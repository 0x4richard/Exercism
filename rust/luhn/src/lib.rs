/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut code_vec: Vec<u32> = Vec::new();

    let chars = code.chars().rev();
    let mut i = 0_u32;
    for c in chars {
        if c.is_whitespace() {
            continue;
        }

        let result = c.to_digit(10);
        match result {
            Some(mut n) => {
                if (i + 1) % 2 == 0 {
                    n *= 2;
                    if n > 9 {
                        n -= 9;
                    }
                }

                code_vec.push(n);
                i += 1;
            }
            None => return false,
        }
    }

    if code_vec.len() == 1 && code_vec[0] == 0 {
        return false;
    }

    code_vec.iter().sum::<u32>() % 10 == 0
}
