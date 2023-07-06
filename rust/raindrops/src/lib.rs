pub fn raindrops(n: u32) -> String {
    let factors: [u32; 3] = [3, 5, 7];
    let mut result = String::new();

    for factor in factors {
        if n % factor != 0 {
            continue;
        }

        let res = match factor {
            3_u32 => "Pling",
            5_u32 => "Plang",
            7_u32 => "Plong",
            _ => "",
        };
        result.push_str(res);
    }

    if result.is_empty() {
        result.push_str(n.to_string().as_str());
    }

    result
}
