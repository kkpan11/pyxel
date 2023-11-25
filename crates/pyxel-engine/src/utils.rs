macro_rules! shared_type {
    ($type: ty) => {
        std::sync::Arc<parking_lot::Mutex<$type>>
    }
}

macro_rules! new_shared_type {
    ($value: expr) => {
        std::sync::Arc::new(parking_lot::Mutex::new($value))
    };
}

macro_rules! string_loop {
    ($index: ident, $piece: ident, $string: ident, $step: expr, $block: block) => {
        for $index in 0..($string.len() / $step) {
            let index = $index * $step;
            let $piece = $string[index..index + $step].to_string();
            $block
        }
    };
}

pub fn f64_to_i32(x: f64) -> i32 {
    x.round() as i32
}

pub fn f64_to_u32(x: f64) -> u32 {
    x.round() as u32
}

pub fn remove_whitespace(string: &str) -> String {
    string.replace(&[' ', '\n', '\r', '\t'][..], "")
}

pub fn simplify_string(string: &str) -> String {
    remove_whitespace(string).to_ascii_lowercase()
}

pub fn parse_hex_string(string: &str) -> Result<u32, &str> {
    let string = string.to_ascii_lowercase();
    let mut result: u32 = 0;
    for c in string.chars() {
        result *= 0x10;
        if c.is_ascii_digit() {
            result += c as u32 - '0' as u32;
        } else if ('a'..='f').contains(&c) {
            result += 10 + c as u32 - 'a' as u32;
        } else {
            return Err("invalid hex string");
        }
    }
    Ok(result)
}

pub fn add_file_extension(filename: &str, ext: &str) -> String {
    if filename.to_lowercase().ends_with(ext) {
        filename.to_string()
    } else {
        filename.to_string() + ext
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_loop_() {
        let test_string = "TEST_STRING";
        string_loop!(i, value, test_string, 1, {
            match i {
                3 => {
                    assert_eq!(value, "T");
                }
                7 => {
                    assert_eq!(value, "R");
                }
                _ => {}
            }
        });
        string_loop!(i, value, test_string, 3, {
            match i {
                0 => {
                    assert_eq!(value, "TES");
                }
                2 => {
                    assert_eq!(value, "TRI");
                }
                _ => {}
            }
        });
    }

    #[test]
    fn f64_to_i32_() {
        assert_eq!(f64_to_i32(0.1), 0);
        assert_eq!(f64_to_i32(0.49), 0);
        assert_eq!(f64_to_i32(0.5), 1);
        assert_eq!(f64_to_i32(1.49), 1);
        assert_eq!(f64_to_i32(-0.1), 0);
        assert_eq!(f64_to_i32(-0.49), 0);
        assert_eq!(f64_to_i32(-0.50), -1);
        assert_eq!(f64_to_i32(-1.49), -1);
    }

    #[test]
    fn f64_to_u32_() {
        assert_eq!(f64_to_u32(0.1), 0);
        assert_eq!(f64_to_u32(0.49), 0);
        assert_eq!(f64_to_u32(0.5), 1);
        assert_eq!(f64_to_u32(1.49), 1);
        assert_eq!(f64_to_u32(-0.1), 0);
        assert_eq!(f64_to_u32(-3.0), 0);
    }

    #[test]
    fn remove_whitespace_() {
        assert_eq!(remove_whitespace(" a\n b\r c\t d "), "abcd");
    }

    #[test]
    fn simplify_string_() {
        assert_eq!(simplify_string(" 0\n 1\r 2\t 3 A\n b\r c\t d "), "0123abcd");
    }

    #[test]
    fn parse_hex_string_() {
        assert_eq!(parse_hex_string("100"), Ok(256));
        assert_eq!(parse_hex_string("a2"), Ok(162));
        assert_eq!(parse_hex_string("BC"), Ok(188));
        assert_eq!(parse_hex_string(" "), Err("invalid hex string"));
    }
}
