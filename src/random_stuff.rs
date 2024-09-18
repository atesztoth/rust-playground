#[derive(PartialOrd, PartialEq, Debug)]
enum ParseResult {
    Integer(i64),
    Float(f64),
}

fn parse_number_str(s: &str) -> Option<ParseResult> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return None;
    }

    let seems_float = trimmed.contains('.') || trimmed.contains(',');

    if seems_float {
        return match trimmed.replace(",", ".").parse::<f64>() {
            Ok(num) => Some(ParseResult::Float(num)),
            _ => None,
        };
    }

    match trimmed.parse::<i64>() {
        Ok(num) => Some(ParseResult::Integer(num)),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use crate::random_stuff::{parse_number_str, ParseResult};

    #[test]
    fn test() {
        let cases = vec![
            ("0", ParseResult::Integer(0)),
            ("-0", ParseResult::Integer(-0)),
            ("0.12345", ParseResult::Float(0.12345)),
            ("-0.12345", ParseResult::Float(-0.12345)),
            ("1", ParseResult::Integer(1)),
            ("10000000000", ParseResult::Integer(10000000000)),
        ];

        // DISCLAIMER: I know it's not a release build and the code is being run by a test
        // runner! These were important, and known circumstances.

        // So I wanted to do to this test repeatedly to see how well it performs,
        // and stumbled upon the problem of having to make ParseResult Copy to be able to use in the inner loop.
        // But I was like NA-AH, clearly, the same memory can be read safely in every iteration, so
        // let's come over this. This is how this unsafe block ended up here.
        for (string, ref result) in cases {
            let result_ptr = result as *const ParseResult;
            for i in 0..10_000 {
                unsafe {
                    assert_eq!(parse_number_str(string), Some(result_ptr.read()));
                }
            }
        }
    }
}
