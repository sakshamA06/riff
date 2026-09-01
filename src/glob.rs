pub enum Token {
    Literal(u8),
    Any,
    Star,
}

pub enum Pattern {
    Literal(Vec<u8>),
    Suffix(Vec<u8>),
    Prefix(Vec<u8>),
    Contains(Vec<u8>),
    Tokens {
        tokens: Vec<Token>,
        min_length: usize,
    },
}

impl Pattern {
    pub fn parse(pattern: &[u8]) -> Self {
        let toks = tokenize(pattern);
        classify(toks)
    }

    pub fn matches(&self, name: &[u8]) -> bool {
        match self {
            Pattern::Literal(literal) => name == literal.as_slice(),
            Pattern::Prefix(prefix) => name.starts_with(prefix.as_slice()),
            Pattern::Suffix(suffix) => name.ends_with(suffix.as_slice()),
            Pattern::Contains(middle) => name.windows(middle.len()).any(|window| middle == window),
            Pattern::Tokens { tokens, min_length } => {
                name.len() >= *min_length && matches_tokens(name, tokens)
            }
        }
    }
}

fn tokenize(pattern: &[u8]) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut escape = false;
    for &byte in pattern {
        if escape {
            tokens.push(Token::Literal(byte));
            escape = false;
            continue;
        }
        match byte {
            b'\\' => escape = true,
            b'*' => {
                if !matches!(tokens.last(), Some(Token::Star)) {
                    tokens.push(Token::Star);
                }
            }
            b'?' => tokens.push(Token::Any),
            b => tokens.push(Token::Literal(b)),
        }
    }

    tokens
}

fn classify(tokens: Vec<Token>) -> Pattern {
    let length = tokens.len();

    let leading = matches!(tokens.first(), Some(Token::Star));
    let trailing = matches!(tokens.last(), Some(Token::Star));

    let expected_count = usize::from(leading) + usize::from(trailing);
    let actual_count = tokens
        .iter()
        .filter(|item| matches!(item, Token::Star))
        .count();

    if expected_count != actual_count {
        return Pattern::Tokens {
            tokens,
            min_length: length - actual_count,
        };
    }

    // The early return above makes sure the range below is not invalid([1..0]) for a pattern with a single '*'
    let middle_slice = &tokens[usize::from(leading)..length - usize::from(trailing)];
    if !middle_slice
        .iter()
        .all(|tok| matches!(tok, Token::Literal(_)))
    {
        return Pattern::Tokens {
            tokens,
            min_length: length - actual_count,
        };
    }

    // Convert tokens back to bytes
    let bytes: Vec<u8> = middle_slice
        .iter()
        .map(|tok| match tok {
            Token::Literal(lit) => *lit,
            _ => unreachable!(),
        })
        .collect();

    match (leading, trailing) {
        (false, false) => Pattern::Literal(bytes),
        (false, true) => Pattern::Prefix(bytes),
        (true, false) => Pattern::Suffix(bytes),
        (true, true) => Pattern::Contains(bytes),
    }
}

fn matches_tokens(haystack: &[u8], needle: &[Token]) -> bool {
    let h = haystack.len();

    let mut i: usize = 0;
    let mut j: usize = 0;

    let mut star_pos: Option<(usize, usize)> = None;

    fn can_advance(token: &Token, byte: u8) -> bool {
        match token {
            Token::Any => true,
            Token::Literal(literal) => *literal == byte,
            _ => unreachable!(),
        }
    }

    loop {
        match needle.get(j) {
            Some(Token::Star) => {
                star_pos = Some((i + 1, j + 1));
                j += 1;
            }
            Some(tok) if i < h && can_advance(tok, haystack[i]) => {
                i += 1;
                j += 1;
            }
            None if i == h => return true,
            _ => {
                match star_pos {
                    Some(pos) if i <= h => (i, j) = pos,
                    _ => return false,
                }
                star_pos = Some((i + 1, j));
            }
        }
    }
}
