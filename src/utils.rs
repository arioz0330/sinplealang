use alloc::{
    format,
    string::{String, ToString},
};

pub fn extract_digits(s: &str) -> Result<(&str, &str), String> {
    take_while1(|c| c.is_ascii_digit(), s, "Expected digits".to_string())
}

const WHITESPACE: &[char] = &[' ', '\n'];

pub fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| WHITESPACE.contains(&c), s)
}

pub fn extract_whitespace1(s: &str) -> Result<(&str, &str), String> {
    take_while1(
        |c| WHITESPACE.contains(&c),
        s,
        "Expected whitespace".to_string(),
    )
}

pub fn extract_ident(s: &str) -> Result<(&str, &str), String> {
    let input_starts_with_alphabetic = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false);
    if input_starts_with_alphabetic {
        Ok(take_while(|c| c.is_ascii_alphanumeric(), s))
    } else {
        Err("Error: Expected identifier".to_string())
    }
}

 fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let digits_end = s
        .char_indices()
        .find_map(|(index, c)| if accept(c) { None } else { Some(index) })
        .unwrap_or_else(|| s.len());

    (&s[digits_end..], &s[..digits_end])
}

 fn take_while1(
    accept: impl Fn(char) -> bool,
    s: &str,
    em: String,
) -> Result<(&str, &str), String> {
    let (remainder, extracted) = take_while(accept, s);

    if extracted.is_empty() {
        Err(format!("Error: {}", em))
    } else {
        Ok((remainder, extracted))
    }
}

pub fn tag<'a, 'b>(starting_text: &'a str, s: &'b str, em: &str) -> Result<&'b str, String> {
    if s.starts_with(starting_text) {
        Ok(&s[starting_text.len()..])
    } else {
        Err(format!("Error: {}", em))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), Ok(("+2", "1")));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("10-20"), Ok(("-20", "10")));
    }

    #[test]
    fn do_not_extract_digits_when_input_is_invalid() {
        assert_eq!(
            extract_digits("abcd"),
            Err("Error: Expected digits".to_string())
        );
    }

    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), Ok(("", "100")));
    }

    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespace("    1"), ("1", "    "));
    }

    #[test]
    fn extract_alphabetic_ident() {
        assert_eq!(extract_ident("abcdEFG stop"), Ok((" stop", "abcdEFG")));
    }

    #[test]
    fn extract_alphanumeric_ident() {
        assert_eq!(extract_ident("foobar1()"), Ok(("()", "foobar1")));
    }

    #[test]
    fn ident_must_begin_with_alphabetic() {
        assert_eq!(
            extract_ident("123abc"),
            Err("Error: Expected identifier".to_string())
        );
    }

    #[test]
    fn tag_word() {
        assert_eq!(tag("let", "let a", "(test) Expected `let`"), Ok(" a"));
    }

    #[test]
    fn do_not_extract_spaces1_when_input_does_not_start_with_them() {
        assert_eq!(
            extract_whitespace1("blah"),
            Err("Error: Expected whitespace".to_string()),
        );
    }
}
