// src/lib.rs

pub fn parse_positive(s: &str) -> Result<u32, String> {
    let n: i32 = s.parse().map_err(|_| "not a number".to_string())?;
    if n > 0 {
        Ok(n as u32)
    } else {
        Err("not positive".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_positive() {
        assert_eq!(parse_positive("42"), Ok(42));
    }

    #[test]
    fn test_negative_number() {
        assert_eq!(parse_positive("-7"), Err("not positive".into()));
    }

    #[test]
    fn test_invalid_input() {
        assert_eq!(parse_positive("abc"), Err("not a number".into()));
    }
}
