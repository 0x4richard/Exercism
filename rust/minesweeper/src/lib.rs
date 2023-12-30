const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let minefield = minefield
        .iter()
        .map(|row| row.as_bytes())
        .collect::<Vec<&[u8]>>();

    minefield
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(y, col)| {
                    if col.eq(&b'*') {
                        return *col;
                    }

                    let count = count_mines(&minefield, (x, y));
                    if count > 0 {
                        return count.to_string().as_bytes()[0];
                    }

                    *col
                })
                .collect::<Vec<u8>>()
        })
        .flat_map(|row| String::from_utf8(row).ok())
        .collect::<Vec<String>>()
}

fn count_mines(minefield: &[&[u8]], position: (usize, usize)) -> usize {
    let height = minefield.len();
    let width = minefield[0].len();
    let (x, y) = position;

    let mut count = 0;
    for (dx, dy) in DIRECTIONS {
        let (x, y) = (x as isize + dx, y as isize + dy);
        if x < 0 || y < 0 || x >= height as isize || y >= width as isize {
            continue;
        }
        let (x, y) = (x as usize, y as usize);
        if minefield[x][y].eq(&b'*') {
            count += 1;
        }
    }
    count
}
