pub fn is_armstrong_number(num: u32) -> bool {
    let numbers = num
        .to_string()
        .chars()
        .map(|s| s.to_digit(10).unwrap_or_default())
        .collect::<Vec<_>>();
    for ele in numbers.iter() {
        println!("{}", ele);
    }

    let power = numbers.len() as u32;
    let mut is_overflowed = false;
    let number = numbers.iter().fold(0, |acc, n| {
        let result = n.pow(power).checked_add(acc);
        match result {
            Some(v) => v,
            None => {
                is_overflowed = true;
                0
            }
        }
    });

    if is_overflowed {
        return false;
    }

    number == num
}
