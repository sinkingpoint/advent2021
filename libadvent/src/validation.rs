pub fn is_valid_hex_char(c: char) -> bool {
    return ('0'..='9').contains(&c) || ('a'..='f').contains(&c) || ('A'..='F').contains(&c);
}

#[test]
fn test_is_valid_hex_char() {
    for i in '0'..='9' {
        assert!(is_valid_hex_char(i), "{} should be a hex char", i)
    }

    for i in 'a'..='f' {
        assert!(is_valid_hex_char(i), "{} should be a hex char", i)
    }

    for i in 'A'..='F' {
        assert!(is_valid_hex_char(i), "{} should be a hex char", i)
    }

    for i in 'G'..='Z'{
        assert!(!is_valid_hex_char(i), "{} shouldn't be a hex char", i)
    }
}

pub fn is_valid_hex_string(s: &str) -> bool {
    return s.chars().all(|c| is_valid_hex_char(c))
}

#[test]
fn test_is_valid_hex_string() {
    for i in '0'..='9' {
        assert!(is_valid_hex_string(&String::from(i)), "{} should be a hex char", i)
    }

    for i in 'a'..='f' {
        assert!(is_valid_hex_string(&String::from(i)), "{} should be a hex char", i)
    }

    for i in 'A'..='F' {
        assert!(is_valid_hex_string(&String::from(i)), "{} should be a hex char", i)
    }

    for i in 'G'..='Z'{
        assert!(!is_valid_hex_string(&String::from(i)), "{} shouldn't be a hex char", i)
    }

    assert!(is_valid_hex_string("0123456789ABCDEF"));
    assert!(!is_valid_hex_string("ABCDEFG"));
}

pub fn is_valid_year(s: &str, min: u32, max: u32) -> bool {
    let num: Result<u32, _> = s.parse();
    if s.len() != 4 || num.is_err() {
        return false;
    }

    let num = num.unwrap();

    return num >= min && num <= max;
}

#[test]
fn test_is_valid_year() {
    assert!(is_valid_year("1994", 1993, 1995));
    assert!(is_valid_year("1995", 1993, 1995));
    assert!(!is_valid_year("1996", 1993, 1995));
    assert!(!is_valid_year("ABCD", 1993, 1995));
    assert!(!is_valid_year("123", 1993, 1995));
}