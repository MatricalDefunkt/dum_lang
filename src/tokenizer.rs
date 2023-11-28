use std::char;

use regex::Regex;

use crate::utilities::{is_float, is_int, is_var};

#[derive(Debug, PartialEq, Clone)]
pub enum VarTypes {
    Int,
    Float,
    String,
    Bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarData {
    pub var_type: Option<VarTypes>,
    pub var_name: String,
    pub var_value: String,
}

const PUNCTUATIONS: [char; 9] = [';', ':', ',', '(', ')', '{', '}', '[', ']'];

#[derive(Debug, PartialEq, Clone)]
pub enum PunctuationType {
    Semicolon,
    Colon,
    Comma,
    LeftParen,
    RightParen,
    // LeftBrace,
    // RightBrace,
    // LeftBracket,
    // RightBracket,
    DoubleQuote,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tokens {
    Let,
    Exit,
    Print,
    Add,
    Sub,
    Mul,
    Div,
    Identifier { name: String },
    IntegerLiteral { value: i32 },
    StringLiteral { value: String },
    FloatLiteral { value: f32 },
    BoolLiteral { value: bool },
    Punctuation { value: PunctuationType },
}

pub const KEYWORDS: [&str; 9] = [
    "let", "exit", "print", "add", "sub", "mul", "div", "true", "false",
];

pub struct Tokenizer {
    tokens: Vec<Tokens>,
    current_char: char,
    current_line: u32,
    current_column: u32,
    current_index: usize,
    input_string: String,
}

impl Tokenizer {
    pub fn new(input_string: String) -> Self {
        let mut tokenizer = Self {
            tokens: Vec::new(),
            current_char: '\0',
            current_line: 1,
            current_column: 0,
            current_index: 0,
            input_string,
        };

        tokenizer.current_char = tokenizer.input_string.chars().nth(0).unwrap_or('\0');
        tokenizer
    }

    fn next_char(&mut self) {
        self.current_index += 1;
        self.current_column += 1;
        self.current_char = self
            .input_string
            .chars()
            .nth(self.current_index)
            .unwrap_or('\0');
    }

    fn next_line(&mut self) {
        self.current_line += 1;
        self.current_column = 0;

        while self.peek() != '\n' {
            self.next_char();
        }
    }

    fn peek(&self) -> char {
        self.input_string
            .chars()
            .nth(self.current_index + 1)
            .unwrap_or('\0')
    }

    pub fn tokenize(&mut self) -> Vec<Tokens> {
        let mut buf = String::new();

        while self.peek() != '\0' {
            if self.current_char == '\n' || self.current_char == ';' {
                self.next_line();
            }

            let mut found_string = false;

            while !self.current_char.is_whitespace() {
                if PUNCTUATIONS.contains(&self.current_char) {
                    match self.current_char {
                        ';' => self.tokens.push(Tokens::Punctuation {
                            value: PunctuationType::Semicolon,
                        }),
                        ':' => self.tokens.push(Tokens::Punctuation {
                            value: PunctuationType::Colon,
                        }),
                        ',' => self.tokens.push(Tokens::Punctuation {
                            value: PunctuationType::Comma,
                        }),
                        '(' => self.tokens.push(Tokens::Punctuation {
                            value: PunctuationType::LeftParen,
                        }),
                        ')' => self.tokens.push(Tokens::Punctuation {
                            value: PunctuationType::RightParen,
                        }),
                        // '{' => tokens.push(Tokens::Punctuation {
                        //     value: PunctuationType::LeftBrace,
                        // }),
                        // '}' => tokens.push(Tokens::Punctuation {
                        //     value: PunctuationType::RightBrace,
                        // }),
                        // '[' => tokens.push(Tokens::Punctuation {
                        //     value: PunctuationType::LeftBracket,
                        // }),
                        // ']' => tokens.push(Tokens::Punctuation {
                        //     value: PunctuationType::RightBracket,
                        // }),
                        '"' => {
                            self.tokens.push(Tokens::Punctuation {
                                value: PunctuationType::DoubleQuote,
                            });
                            self.next_char();
                            while self.current_char != '"' {
                                buf.push(self.current_char);
                                self.next_char();

                                if self.current_char == '\\' {
                                    self.next_char();
                                    match self.current_char {
                                        'n' => buf.push('\n'),
                                        't' => buf.push('\t'),
                                        'r' => buf.push('\r'),
                                        '\\' => buf.push('\\'),
                                        '"' => buf.push('"'),
                                        '\'' => buf.push('\''),
                                        _ => panic!(
                                            "Unknown escape character: {}",
                                            self.current_char
                                        ),
                                    }
                                }
                            }
                            self.tokens
                                .push(Tokens::StringLiteral { value: buf.clone() });
                            self.tokens.push(Tokens::Punctuation {
                                value: PunctuationType::DoubleQuote,
                            });
                            buf.clear();
                            self.next_char();
                            found_string = true;
                            break;
                        }
                        _ => panic!("Unknown punctuation: {}", self.current_char),
                    }
                    self.next_char();
                    break;
                }
                buf.push(self.current_char);
                self.next_char();
            }

            if found_string {
                continue;
            }

            if buf.is_empty() {
                continue;
            };

            if KEYWORDS.contains(&buf.as_str()) {
                match buf.as_str() {
                    "let" => {
                        self.tokens.insert(self.tokens.len() - 1, Tokens::Let);

                        let mut data_type = String::new();

                        while !self.current_char.is_whitespace()
                            || PUNCTUATIONS.contains(&self.current_char)
                        {
                            data_type.push(self.current_char);
                            self.next_char();
                        }

                        self.tokens.push(Tokens::Identifier { name: data_type });
                    }
                    "exit" => self.tokens.push(Tokens::Exit),
                    "print" => self.tokens.push(Tokens::Print),
                    "add" => self.tokens.push(Tokens::Add),
                    "sub" => self.tokens.push(Tokens::Sub),
                    "mul" => self.tokens.push(Tokens::Mul),
                    "div" => self.tokens.push(Tokens::Div),
                    "true" => self.tokens.push(Tokens::BoolLiteral { value: true }),
                    "false" => self.tokens.push(Tokens::BoolLiteral { value: false }),
                    _ => panic!("Unknown keyword: {}", buf),
                }
            } else if is_int(&buf) {
                self.tokens.push(Tokens::IntegerLiteral {
                    value: buf.parse::<i32>().unwrap(),
                });
            } else if is_float(&buf) {
                self.tokens.push(Tokens::FloatLiteral {
                    value: buf.parse::<f32>().unwrap(),
                });
            } else if is_var(&buf) {
                self.tokens.push(Tokens::Identifier { name: buf.clone() });
            } else if buf.is_empty() {
                continue;
            } else {
                panic!("Unknown token: {}", buf);
            }

            self.next_char();
            buf.clear();
        }

        self.tokens.clone()
    }
}
