use num_bigint::BigInt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    NoToken,
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    StringLiteral(String),
    Number(BigInt),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug)]
pub struct Scanner {
    pub contents: String,
}

impl Scanner {
    pub fn new(buf: &str) -> Self {
        Self {
            contents: buf.to_string(),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        use Token::*;

        let mut res = Vec::new();
        let mut chars = self.contents.chars().peekable();
        let mut build_string = String::new();

        while let Some(c) = chars.next() {
            let token = match c {
                '(' => LeftParen,
                ')' => RightParen,
                '{' => LeftBrace,
                '}' => RightBrace,
                ',' => Comma,
                '-' => Minus,
                '+' => Plus,
                ';' => Semicolon,
                '*' => Star,
                '.' => {
                    if let Ok(_) = build_string.parse::<f64>() {
                        build_string.push('.');
                        NoToken
                    } else {
                        Dot
                    }
                }

                '!' => {
                    if let Some(nc) = chars.peek() {
                        if *nc == '=' {
                            chars.next();
                            BangEqual
                        } else {
                            Bang
                        }
                    } else {
                        Bang
                    }
                }

                '=' => {
                    if let Some(nc) = chars.peek() {
                        if *nc == '=' {
                            chars.next();
                            EqualEqual
                        } else {
                            Equal
                        }
                    } else {
                        Equal
                    }
                }

                '>' => {
                    if let Some(nc) = chars.peek() {
                        if *nc == '=' {
                            chars.next();
                            GreaterEqual
                        } else {
                            Greater
                        }
                    } else {
                        Greater
                    }
                }

                '<' => {
                    if let Some(nc) = chars.peek() {
                        if *nc == '=' {
                            chars.next();
                            LessEqual
                        } else {
                            Less
                        }
                    } else {
                        Less
                    }
                }

                '/' => {
                    if let Some(nc) = chars.peek() {
                        if *nc == '/' {
                            while let Some(c) = chars.next() {
                                if c == '\n' {
                                    break;
                                }
                            }
                            NoToken
                        } else {
                            Slash
                        }
                    } else {
                        Slash
                    }
                }

                '"' | '\'' => {
                    let mut strlit = String::new();
                    let opening_quote = c;
                    let mut closed = false;
                    while let Some(nc) = chars.next() {
                        if nc == opening_quote {
                            closed = true;
                            break;
                        }
                        strlit.push(nc);

                        if nc == '\\' {
                            strlit.push(chars.next().unwrap());
                        }
                    }

                    if !closed {
                        eprintln!("Unclosed String Literal!!!");
                    }

                    StringLiteral(strlit)
                }

                c if c.is_whitespace() => {
                    if !build_string.is_empty() {
                        let ret = self.parse_word(build_string);
                        build_string = String::new();
                        ret
                    } else {
                        NoToken
                    }
                }

                c => {
                    build_string.push(c);
                    NoToken
                }
            };

            if token != NoToken {
                if !build_string.is_empty() {
                    res.push(self.parse_word(build_string));
                    build_string = String::new();
                }
                res.push(token);
            }
        }

        if !build_string.is_empty() {
            res.push(self.parse_word(build_string));
        }

        res.push(Eof);
        res
    }

    fn parse_word(&self, s: String) -> Token {
        use Token::*;

        if let Result::Ok(n) = s.parse::<BigInt>() {
            return Number(n);
        }

        match s.as_str() {
            "and" => And,
            "class" => Class,
            "else" => Else,
            "false" => False,
            "fun" => Fun,
            "for" => For,
            "if" => If,
            "nil" => Nil,
            "or" => Or,
            "print" => Print,
            "return" => Return,
            "super" => Super,
            "this" => This,
            "true" => True,
            "var" => Var,
            "while" => While,

            _ => Identifier(s),
        }
    }
}
