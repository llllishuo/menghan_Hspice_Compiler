pub fn split_equal_sign(bit: &str) -> String {
    if bit.to_string().contains("=") {
        let mut b = bit.split("=");
        b.next();
        let Some(iter) = b.next() else { todo!() };
        iter.to_string()
    } else {
        bit.to_string()
    }
}
pub fn clean_comments_and_end_identifiers(item: Vec<&str>) -> Vec<&str> {
    let mut bits: Vec<&str> = Vec::new();
    for bit in item {
        if bit == "$" || bit.starts_with('$') {
            break;
        }
        if bit == ";" {
            break;
        }

        bits.push(bit);
    }
    bits
}

pub fn get_variables_within_parentheses(bit: &str) -> String {
    let mut date = String::new();
    let mut is_date = false;
    let mut chars = bit.chars();
    while let Some(b) = chars.next() {
        match b {
            '(' => is_date = true,
            ')' => is_date = false,
            ',' => continue,
            _ => {
                if is_date {
                    date.push(b);
                }
            }
        }
    }
    date
}

pub fn organize_special_writing_methods_for_lib(bits: Vec<&str>) -> Vec<&str> {
    let mut new_bits: Vec<&str> = Vec::new();

    let new_lib: Vec<&str> = bits[0].split("\'").collect();
    for lib in new_lib {
        new_bits.push(lib);
    }
    for i in 1..bits.len() {
        new_bits.push(bits[i]);
    }
    new_bits
}
