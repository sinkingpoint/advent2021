use std::collections::HashMap;

fn does_have_multiple_root_layers(s: &str, entry_char: char, exit_char: char) -> bool {
    let mut depth = 0;
    let mut i= 0;
    for c in s.chars() {
        if c == entry_char {
            depth += 1;
        }
        else if c == exit_char {
            depth -= 1;
        }

        if depth == 0 && i != s.len() - 1 {
            return true;
        }
        i += 1;
    }

    return false;
}

pub fn split_ors_with_multiple_layers(s: &str, split_char: char, entry_char: char, exit_char: char) -> Vec<String> {
    let mut depth = 0;

    let start = s.chars().next().unwrap();
    let end = s.chars().rev().next().unwrap();
    let base_depth = if start == entry_char && end == exit_char && !does_have_multiple_root_layers(s, entry_char, exit_char) {
        1
    }
    else {
        0
    };

    println!("{} with {}", s, base_depth);

    let mut last_proccessed = 'a';

    let mut out = Vec::new();
    let mut current = String::new();
    for c in s.chars() {
        if c == entry_char {
            depth += 1;
            if depth > base_depth {
                current.push(c);
            }
        }
        else if c == exit_char {
            depth -= 1;
            if depth > 0 {
                current.push(c);
            }

            if depth == base_depth && (current.len() != 0 || last_proccessed == split_char) {
                out.push(current);
                current = String::new();
            }
        }
        else if c == split_char && (depth == base_depth) {
            if current.len() != 0 || last_proccessed == split_char {
                out.push(current);
                current = String::new();
            }
        }
        else {
            current.push(c);
        }
        last_proccessed = c;
    }

    if current.len() != 0 || last_proccessed == split_char {
        out.push(current);
    }

    assert_eq!(depth, 0);
    return out;
}

#[test]
fn test_split_ors_with_multiple_layers() {
    assert_eq!(split_ors_with_multiple_layers("(a|b)", '|', '(', ')'), vec!["a", "b"]);
    assert_eq!(split_ors_with_multiple_layers("a|b", '|', '(', ')'), vec!["a", "b"]);
    assert_eq!(split_ors_with_multiple_layers("a|(b|c)", '|', '(', ')'), vec!["a", "b|c"]);
    assert_eq!(split_ors_with_multiple_layers("a|(b|(c|d))", '|', '(', ')'), vec!["a", "b|(c|d)"]);
    assert_eq!(split_ors_with_multiple_layers("(a|(b|(c|d)))", '|', '(', ')'), vec!["a", "(b|(c|d))"]);

    assert_eq!(split_ors_with_multiple_layers("(ab|(c|d))", '|', '(', ')'), vec!["ab", "(c|d)"]);
    assert_eq!(split_ors_with_multiple_layers("(ab|(c|d))|e", '|', '(', ')'), vec!["ab|(c|d)", "e"]);
    assert_eq!(split_ors_with_multiple_layers("(ab|(c|d))|e|fg|(h|(i|j))", '|', '(', ')'), vec!["ab|(c|d)", "e", "fg", "h|(i|j)"]);

    assert_eq!(split_ors_with_multiple_layers("ab(c|d)e", '|', '(', ')'), vec!["ab(c|d)e"]);
}

pub fn count_chars_at(strs: &Vec<String>, i: usize) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for str in strs {
        let c = str.chars().nth(i).unwrap();
        let current = map.get(&c).map(|s| *s).unwrap_or(0) + 1;
        map.insert(c, current);
    }

    return map;
}