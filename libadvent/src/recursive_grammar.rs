use crate::split_ors_with_multiple_layers;

const DEFAULT_MAP_SEP: &str = ":";
const OR_SEPERATOR: char = '|';

#[derive(Debug, Clone, PartialEq)]
pub enum GrammarEntryType {
    Static(char),
    List(Vec<GrammarEntryType>),
    Ors(Vec<GrammarEntryType>),
    Empty
}

impl GrammarEntryType {
    // Parses lines in the form x: a | b | cd
    // into GrammarEntry::Ors(vec![GrammarEntry::Static(a), GrammarEntry::Static(b), GrammarEntry::List(GrammarEntry::Static(b), GrammarEntry::Static(c))])
    pub fn parse_from_map(line: &str) -> (&str, Self) {
        let (name, parts) = line.split_once(DEFAULT_MAP_SEP).unwrap();
        return (name, Self::parse_from_part(parts.trim()));
    }

    pub fn parse_from_part(s: &str) -> GrammarEntryType {
        if s == "" {
            return GrammarEntryType::Empty;
        }

        let parts = split_ors_with_multiple_layers(s, OR_SEPERATOR, '(', ')');
        if parts.len() > 1 {
            return GrammarEntryType::Ors(parts.into_iter().map(|s| GrammarEntryType::parse_from_part(s.trim())).collect());
        }

        let s = s.trim();
        assert!(s.len() != 0);
        if s.len() == 1 {
            return GrammarEntryType::Static(s.chars().next().unwrap());
        }

        return GrammarEntryType::List(s.chars().map(|c| GrammarEntryType::Static(c)).collect());
    }

    pub fn to_string(&self) -> String {
        match self {
            GrammarEntryType::List(l) => return l.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(""),
            GrammarEntryType::Ors(l) => return format!("({})", l.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("|")),
            GrammarEntryType::Static(c) => String::from(*c),
            &GrammarEntryType::Empty => String::new()
        }
    }

    pub fn walk_grammar(&self, start: &str) -> Vec<String> {
        let mut out = Vec::new();
        match self {
            GrammarEntryType::Static(c) => out.push(format!("{}{}", start, c)),
            GrammarEntryType::List(l) => {
                let mut base = String::from(start);
                for g in l {
                    let next = g.walk_grammar(&base);
                    assert!(next.len() > 0);
                    if next.len() == 1 {
                        // It's a static, just add it
                        base.push_str(&next[0]);
                    }
                    else {
                        // It's an Ors, add each variant

                    }
                }
            },
            GrammarEntryType::Ors(_) => todo!(),
            GrammarEntryType::Empty => todo!(),
        }

        return out;
    }
}

#[test]
fn test_recursive_grammar_parse() {
    let (name, grammar) = GrammarEntryType::parse_from_map("0: ab");
    assert_eq!(name, "0");
    assert_eq!(grammar, GrammarEntryType::List(vec![GrammarEntryType::Static('a'), GrammarEntryType::Static('b')]));

    let (name, grammar) = GrammarEntryType::parse_from_map("1: ab|c");
    assert_eq!(name, "1");
    assert_eq!(grammar, GrammarEntryType::Ors(vec![GrammarEntryType::List(vec![GrammarEntryType::Static('a'), GrammarEntryType::Static('b')]), GrammarEntryType::Static('c')]));

    let (name, grammar) = GrammarEntryType::parse_from_map("1: (ab|(c|d))");
    assert_eq!(name, "1");
    assert_eq!(grammar, GrammarEntryType::Ors(vec![GrammarEntryType::List(vec![GrammarEntryType::Static('a'), GrammarEntryType::Static('b')]), GrammarEntryType::Ors(vec![GrammarEntryType::Static('c'), GrammarEntryType::Static('d')])]));

    let (name, grammar) = GrammarEntryType::parse_from_map("1: (ab|(c|d))|e");
    assert_eq!(name, "1");
    assert_eq!(grammar, GrammarEntryType::Ors(vec![GrammarEntryType::Ors(vec![GrammarEntryType::List(vec![GrammarEntryType::Static('a'), GrammarEntryType::Static('b')]), GrammarEntryType::Ors(vec![GrammarEntryType::Static('c'), GrammarEntryType::Static('d')])]), GrammarEntryType::Static('e')]));

    let (name, grammar) = GrammarEntryType::parse_from_map("1: (ab|(c|d))|e|fg|(h|(i|j))");
    assert_eq!(name, "1");
    assert_eq!(grammar, 
        GrammarEntryType::Ors(vec![
            GrammarEntryType::Ors(vec![
                GrammarEntryType::List(vec![
                    GrammarEntryType::Static('a'), 
                    GrammarEntryType::Static('b')
                ]),
                GrammarEntryType::Ors(vec![
                    GrammarEntryType::Static('c'), 
                    GrammarEntryType::Static('d')
                ])
            ]), 
            GrammarEntryType::Static('e'),
            GrammarEntryType::List(vec![
                GrammarEntryType::Static('f'),
                GrammarEntryType::Static('g'),
            ]),
            GrammarEntryType::Ors(vec![
                GrammarEntryType::Static('h'),
                GrammarEntryType::Ors(vec![
                    GrammarEntryType::Static('i'),
                    GrammarEntryType::Static('j'),
                ])
            ])
        ]));

    let (name, grammar) = GrammarEntryType::parse_from_map("1: (ab|(c|d))|e|fg|(h|(i|j))|");
    assert_eq!(name, "1");
    assert_eq!(grammar, 
        GrammarEntryType::Ors(vec![
            GrammarEntryType::Ors(vec![
                GrammarEntryType::List(vec![
                    GrammarEntryType::Static('a'), 
                    GrammarEntryType::Static('b')
                ]),
                GrammarEntryType::Ors(vec![
                    GrammarEntryType::Static('c'), 
                    GrammarEntryType::Static('d')
                ])
            ]), 
            GrammarEntryType::Static('e'),
            GrammarEntryType::List(vec![
                GrammarEntryType::Static('f'),
                GrammarEntryType::Static('g'),
            ]),
            GrammarEntryType::Ors(vec![
                GrammarEntryType::Static('h'),
                GrammarEntryType::Ors(vec![
                    GrammarEntryType::Static('i'),
                    GrammarEntryType::Static('j'),
                ])
            ]),
            GrammarEntryType::Empty
        ]));
}
