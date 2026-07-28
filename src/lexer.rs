#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Word(String),

    Pipe,          // |
    And,           // &&
    Or,            // ||

    RedirectOut,   // >
    RedirectAppend,// >>
    RedirectIn,    // <
    RedirectErr,   // 2>
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let chars: Vec<char> = input.chars().collect();
    let mut current = String::new();

    let mut i = 0;

    let mut in_single = false;
    let mut in_double = false;

    while i < chars.len() {
        let c = chars[i];

        if in_single {
            if c == '\'' {
                in_single = false;
            } else {
                current.push(c);
            }

            i += 1;
            continue;
        }

        if in_double {
            if c == '"' {
                in_double = false;
            } else {
                current.push(c);
            }

            i += 1;
            continue;
        }

        match c {
            '\'' => {
                in_single = true;
            }

            '"' => {
                in_double = true;
            }

            ' ' | '\t' => {
                if !current.is_empty() {
                    tokens.push(Token::Word(current.clone()));
                    current.clear();
                }
            }

            '|' => {
                if !current.is_empty() {
                    tokens.push(Token::Word(current.clone()));
                    current.clear();
                }

                if i + 1 < chars.len() && chars[i + 1] == '|' {
                    tokens.push(Token::Or);
                    i += 1;
                } else {
                    tokens.push(Token::Pipe);
                }
            }

            '&' => {
                if !current.is_empty() {
                    tokens.push(Token::Word(current.clone()));
                    current.clear();
                }

                if i + 1 < chars.len() && chars[i + 1] == '&' {
                    tokens.push(Token::And);
                    i += 1;
                } else {
                    current.push('&');
                }
            }

            '>' => {
                if !current.is_empty() {
                    tokens.push(Token::Word(current.clone()));
                    current.clear();
                }

                if i + 1 < chars.len() && chars[i + 1] == '>' {
                    tokens.push(Token::RedirectAppend);
                    i += 1;
                } else {
                    tokens.push(Token::RedirectOut);
                }
            }

            '<' => {
                if !current.is_empty() {
                    tokens.push(Token::Word(current.clone()));
                    current.clear();
                }

                tokens.push(Token::RedirectIn);
            }

            '2' => {
                if i + 1 < chars.len() && chars[i + 1] == '>' {
                    if !current.is_empty() {
                        tokens.push(Token::Word(current.clone()));
                        current.clear();
                    }

                    tokens.push(Token::RedirectErr);
                    i += 1;
                } else {
                    current.push('2');
                }
            }

            _ => current.push(c),
        }

        i += 1;
    }

    if !current.is_empty() {
        tokens.push(Token::Word(current));
    }

    tokens
}
