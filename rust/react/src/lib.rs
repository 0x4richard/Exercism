use std::{collections::HashMap, ops::Deref};

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(usize);

impl Deref for InputCellId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(usize);
impl Deref for ComputeCellId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

impl CellId {
    fn get_id(&self) -> usize {
        match self {
            CellId::Input(cell_id) => *cell_id.deref(),
            CellId::Compute(cell_id) => *cell_id.deref(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct InputCell<T>(T);

type ComputeFn<'a, T> = Box<dyn 'a + Fn(&[T]) -> T>;
struct ComputeCell<'a, T> {
    dependencies: Vec<CellId>,
    func: ComputeFn<'a, T>,
}

enum Cell<'a, T> {
    Input(InputCell<T>),
    Compute(ComputeCell<'a, T>),
}

impl<'a, T> Cell<'a, T>
where
    T: Copy + PartialEq,
{
    fn get_value(&self, reactor: &Reactor<T>) -> T {
        match self {
            Cell::Input(input_cell) => input_cell.0,
            Cell::Compute(compute_cell) => {
                let init_vec: Vec<T> = Vec::new();
                let input_vec =
                    compute_cell
                        .dependencies
                        .iter()
                        .fold(init_vec, |new_vec, cell_id| {
                            let input = reactor
                                .cells
                                .get(&cell_id.get_id())
                                .map(|v| v.get_value(reactor))
                                .into_iter()
                                .collect::<Vec<_>>();
                            [new_vec, input].concat()
                        });
                let func = &compute_cell.func;
                func(&input_vec)
            }
        }
    }
}

struct CallbackEntry<'a, T> {
    id: usize,
    callbacks: HashMap<CallbackId, Box<dyn 'a + FnMut(T)>>,
}

pub struct Reactor<'a, T> {
    id: usize,
    cells: HashMap<usize, Cell<'a, T>>,
    callbacks: HashMap<ComputeCellId, CallbackEntry<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        let id = 0;
        let cells = HashMap::new();
        let callbacks = HashMap::new();
        Self {
            id,
            cells,
            callbacks,
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        self.id += 1;
        let input_cell_id = InputCellId(self.id);
        let cell = Cell::Input(InputCell(initial));
        self.cells.insert(self.id, cell);

        input_cell_id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        for cell_id in dependencies {
            if self.value(*cell_id).is_none() {
                return Err(*cell_id);
            }
        }

        self.id += 1;
        let compute_cell = ComputeCell {
            dependencies: dependencies.to_vec(),
            func: Box::new(compute_func),
        };
        let cell = Cell::Compute(compute_cell);
        self.cells.insert(self.id, cell);
        let compute_cell_id = ComputeCellId(self.id);
        Ok(compute_cell_id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(cell_id) => {
                let input_cell = match self.cells.get(&cell_id) {
                    Some(input_cell) => input_cell,
                    None => return None,
                };
                let value = input_cell.get_value(self);
                Some(value)
            }
            CellId::Compute(cell_id) => {
                let compute_cell = match self.cells.get(&cell_id) {
                    Some(compute_cell) => compute_cell,
                    None => return None,
                };
                let value = compute_cell.get_value(self);
                Some(value)
            }
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if let Some(e) = self.cells.get_mut(&id) {
            let new_cell = Cell::Input(InputCell(new_value));
            *e = new_cell;
            true
        } else {
            false
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        if !self.check_if_compute_cell_exist(id) {
            return None;
        }

        let callback_box = Box::new(callback);
        let callback_id = match self.callbacks.get_mut(&id) {
            Some(callback_entry) => {
                callback_entry.id += 1;
                let callback_id = CallbackId(callback_entry.id);
                callback_entry.callbacks.insert(callback_id, callback_box);
                callback_id
            }

            None => {
                let mut callback_entry = CallbackEntry {
                    id: 0,
                    callbacks: HashMap::new(),
                };
                callback_entry.id += 1;
                let callback_id = CallbackId(callback_entry.id);
                callback_entry.callbacks.insert(callback_id, callback_box);
                self.callbacks.insert(id, callback_entry);
                callback_id
            }
        };
        Some(callback_id)
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if !self.check_if_compute_cell_exist(cell) {
            return Err(RemoveCallbackError::NonexistentCell);
        }

        let callbacks = self.callbacks.get_mut(&cell);
        if callbacks.is_none() {
            return Err(RemoveCallbackError::NonexistentCallback);
        }
        let callback_entry = callbacks.unwrap();
        if callback_entry.callbacks.get(&callback).is_none() {
            return Err(RemoveCallbackError::NonexistentCallback);
        }

        callback_entry.callbacks.remove(&callback);

        Ok(())
    }

    fn check_if_compute_cell_exist(&self, cell: ComputeCellId) -> bool {
        self.cells.get(&cell).is_some()
    }
}
