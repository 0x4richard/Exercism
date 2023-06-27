#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    use Comparison::*;

    let first_list_len = first_list.len();
    let second_list_len = second_list.len();

    if first_list_len == second_list_len && first_list == second_list {
        return Equal;
    }

    if first_list_len > second_list_len {
        if is_contained(first_list, second_list) {
            return Superlist;
        }
    }

    if is_contained(second_list, first_list) {
        return Sublist;
    }

    Unequal
}

fn is_contained<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    let b_len = b.len();
    if b_len == 0 {
        return true;
    }

    let a_len = a.len();
    let first_char_of_b = &b[0];
    for (i, e) in a.iter().enumerate() {
        if e != first_char_of_b {
            continue;
        }
        let end_index = i + b_len;
        if end_index > a_len {
            return false;
        }

        if &a[i..end_index] == b {
            return true;
        }
    }

    false
}
