use crate::history::HistoryEntry;

fn digits_only(input: &str) -> String {
    input.chars().filter(|c| c.is_ascii_digit()).collect()
}

fn format_digits(digits: &str) -> String {
    match digits.len() {
        0..=2 => digits.to_string(),
        3..=4 => format!("{}/{}", &digits[0..2], &digits[2..]),
        _ => format!("{}/{}/{}", &digits[0..2], &digits[2..4], &digits[4..]),
    }
}

pub fn append_digit(input: &str, c: char) -> String {
    if !c.is_ascii_digit() {
        return input.to_string();
    }

    let mut digits = digits_only(input);
    if digits.len() >= 8 {
        return format_digits(&digits);
    }

    digits.push(c);
    format_digits(&digits)
}

pub fn backspace(input: &str) -> String {
    let mut digits = digits_only(input);
    digits.pop();
    format_digits(&digits)
}

pub fn parse_date_input(input: &str) -> Option<HistoryEntry> {
    let parts: Vec<&str> = input.split('/').collect();
    if parts.len() != 3 {
        return None;
    }

    let day = parts[0].parse::<u32>().ok()?;
    let month = parts[1].parse::<u32>().ok()?;
    let year = parts[2].parse::<i32>().ok()?;

    HistoryEntry::new(year, month, day)
}

pub fn is_valid(input: &str) -> bool {
    parse_date_input(input).is_some()
}

#[cfg(test)]
mod tests {
    use super::{append_digit, backspace, is_valid, parse_date_input};

    #[test]
    fn auto_formats_digits() {
        let mut input = String::new();
        for ch in "25032026".chars() {
            input = append_digit(&input, ch);
        }
        assert_eq!(input, "25/03/2026");
    }

    #[test]
    fn backspace_keeps_format() {
        let mut input = "25/03/2026".to_string();
        input = backspace(&input);
        input = backspace(&input);
        assert_eq!(input, "25/03/20");
    }

    #[test]
    fn validates_real_dates() {
        assert!(is_valid("29/02/2024"));
        assert!(!is_valid("29/02/2023"));
        assert!(parse_date_input("31/12/2025").is_some());
    }
}
