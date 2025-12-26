use crate::datatypes::{ast_statements::VariableType, token::{BuildInCommand, BuildInFunctions, Identifiers, Keywords, Operators, Punctuations, Token, TokenType}};

// Tokenzer struct
pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
    col: usize,
    line: usize,
}

impl<'a> Tokenizer<'a> {
    // Initialize the tokenizer.
    pub fn new(input: &'a str) -> Self {
        Self {input, position: 0, col: 1, line: 1}
    }

    pub fn tokenize_all(&mut self) -> Vec<Token> {
        let mut res : Vec<Token> = Vec::new();

        loop {
            let token = self.next_token();
            
            match token {
                Some(tkn) => {
                    res.push(tkn.clone());
                    if matches!(&tkn.kind, TokenType::EOF) {
                        return res;
                    }
                },
                None => {}
            };
        }
    }
    
    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.input.len() <= self.position {
            return Some(Token{kind: TokenType::EOF, col: self.col, line: self.line, start_pos: self.position, end_pos: self.position});
        }

        let mut res = String::new();

        let start_pos = self.position;

        match self.current_char() {
            '\n' | ';' | '(' | ')' | ',' | ':' => {
                res = String::from(self.current_char());
                self.advance(1);
            },
            '/' => {
                if self.char_at_offset(1) == '/' {
                    self.advance(2);
                    
                    loop {
                        if self.current_char() == '/' && self.char_at_offset(1) == '/' {
                            self.advance(2);

                            break;
                        }

                        self.advance(1);  
                    };

                    return None;
                }
            }
            '"' => {
                let mut str = String::new();

                self.advance(1);

                while self.position < self.input.len() && self.current_char() != '"' {
                    str.push(self.current_char());
                    self.advance(1);
                };

                self.advance(1);

                return Some(Token{kind: TokenType::Identifiers(Identifiers::StringLiteral(str)), col: self.col, line: self.line, start_pos, end_pos: self.position});
            },
            _ => {
                while self.position < self.input.len() && self.current_char().is_whitespace() == false && matches!(self.current_char(), ';' | '(' | ')' | ',') == false {
                    res.push(self.current_char());
                    self.advance(1);
                };
            }
        }

        let token_default = Token{kind: TokenType::EOF, col: self.col, line: self.line, start_pos, end_pos: self.position};

        match &res as &str {
            "\n" => {
                self.line += 1;
                self.col = 1;
            },
            "term" => {
                return Some(Token{kind: TokenType::BuildInCommand(BuildInCommand::Terminate), ..token_default})
            },
            ";" => {
                return Some(Token{kind: TokenType::Semicolon, ..token_default});
            },
            ":" => {
                return Some(Token{kind: TokenType::Punctuation(Punctuations::Colon), ..token_default})
            }
            "=" => {
                return Some(Token{kind: TokenType::Operator(Operators::Assignment), ..token_default});
            },
            "(" => {
                return Some(Token{kind: TokenType::Punctuation(Punctuations::OpenParenthesis), ..token_default})
            },
            ")" => {
                return Some(Token{kind: TokenType::Punctuation(Punctuations::ClosedParenthesis), ..token_default})
            },
            "," => {
                return Some(Token{kind: TokenType::Punctuation(Punctuations::Comma), ..token_default})
            },
            "i32" | "i16" | "i8" | "void" => {
                return Some(Token{kind: TokenType::Keyword(Keywords::VariableType(
                    match &res as &str {
                        "i32" => VariableType::I32,
                        "i16" => VariableType::I16,
                        "i8" => VariableType::I8,
                        "void" => VariableType::Void,
                        _ => unreachable!()
                    }
                )), ..token_default})
            },
            "compare" => {
                return Some(Token{kind: TokenType::BuildInFunctions(BuildInFunctions::Compare), ..token_default})
            },
            "loop" => {
                return Some(Token{kind: TokenType::BuildInFunctions(BuildInFunctions::Loop), ..token_default})
            },
            _ => {
                match res.parse::<i32>() {
                    Ok(num) => {
                        return Some(Token{kind: TokenType::Identifiers(Identifiers::NumberLiteral(num)), ..token_default});
                    },
                    Err(_) => {
                        return Some(Token{kind: TokenType::Identifiers(Identifiers::Identifier(res)), ..token_default});

                    }
                }
            }
        } 

        return None;
    }

    pub fn advance(&mut self, num : usize) {
        let mut new_position : usize = self.position;
        let mut new_col : usize = self.col;
        let mut new_line : usize = self.line;

        for i in 0..num {
            if self.char_at_offset(i as i32) == '\n' {
                new_col = 1;
                new_line += 1;
            } else {
                new_col += 1;
            }

            new_position += 1;
        }

        self.position = new_position;
        self.col = new_col;
        self.line = new_line;
    }

    // Skips whitespace.
    pub fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.current_char().is_whitespace() && self.current_char() != '\n' {
            self.col += 1;
            self.position += 1;
        }
    }

    // Get current char of input.
    pub fn current_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap()
    }

    pub fn char_at_offset(&self, offset : i32) -> char {
        self.input[((self.position as i32) + offset) as usize..].chars().next().unwrap()
    }
 }
