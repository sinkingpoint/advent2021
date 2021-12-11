use libadvent::*;

#[derive(Debug)]
enum ChunkType {
    Circle,
    Square,
    Squiggly,
    Angle
}

impl From<char> for ChunkType {
    fn from(c: char) -> Self {
        match c {
            '{' => ChunkType::Squiggly,
            '(' => ChunkType::Circle,
            '[' => ChunkType::Square,
            '<' => ChunkType::Angle,
            _ => panic!("{}", c)
        }
    }
}

impl ChunkType {
    fn get_closer(&self) -> char {
        match &self {
            ChunkType::Angle => '>',
            ChunkType::Circle => ')',
            ChunkType::Square => ']',
            ChunkType::Squiggly => '}',
        }
    }
}

#[derive(Debug)]
struct Chunk {
    ty: ChunkType,
    children: Vec<Chunk>,
    needed: String
}

impl Chunk {
    fn new(ty: ChunkType) -> Chunk {
        return Self {
            ty,
            children: Vec::new(),
            needed: String::new()
        }
    }
}

fn parse_chunk(s: &[char]) -> Result<(usize, Chunk), (usize, String)> {
    let mut start = Chunk::new(ChunkType::from(s[0]));
    let mut i = 1;
    let mut closed = false;
    while i < s.len() {
        if ['<', '{', '(', '['].contains(&s[i]) {
            let (len, new_chunk) = match parse_chunk(&s[i..]) {
                Ok(o) => o,
                Err(e) => return Err((e.0 + i, e.1))
            };

            i += len;
            start.needed.push_str(&new_chunk.needed);
            start.children.push(new_chunk);
        }
        else if s[i] == start.ty.get_closer(){
            closed = true;
            break;
        }
        else {
            return Err((i, format!("invalid chunk closer (expected {}): {}", start.ty.get_closer(), s[i])))
        }
        i += 1
    }

    if !closed {
        start.needed.push(start.ty.get_closer());
    }

    return Ok((i, start));
}

fn parse_everything(s: &[char]) -> Result<Vec<Chunk>, (usize, String)> {
    let mut chunks = Vec::new();
    let mut i = 0;
    while i < s.len() {
        match parse_chunk(&s[i..]) {
            Ok((len, chunk)) => {
                i += len;
                chunks.push(chunk);
            },
            Err(e) => return Err(e),
        };

        i += 1;
    }

    return Ok(chunks);
}

fn explode(s: &str) -> Vec<char> {
    return s.chars().collect();
}

fn main() {
    let input = must_read_input_to_lines();

    let mut invalid = 0;
    let mut score = 0;
    let mut scores = Vec::new();
    for line in input.iter() {
        let chars = &explode(line);
        match parse_everything(chars) {
            Ok(chunks) => {
                let mut score2: u128 = 0;
                for c in chunks[chunks.len()-1].needed.chars() {
                    score2 *= 5;
                    score2 += match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!()
                    };
                }

                scores.push(score2);
            },
            Err((i, _)) => {
                invalid += 1;

                score += match chars[i] {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => panic!()
                };
            }
        }
    }

    scores.sort();

    println!("Found {} invalid lines with a score of {}", invalid, score);
    println!("Autocomplete Score: {}", scores[scores.len()/2]);
}
