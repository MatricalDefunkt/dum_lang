use std::fmt::Display;

use regex::Regex;

use crate::utilities::{get_string, is_float, is_int, is_string, is_var};

#[derive(Debug, PartialEq, Clone)]
pub enum Types {
    Int,
    Float,
    String,
    Bool,
}

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::Int => write!(f, "dd"),
            Types::Float => write!(f, "dq"),
            Types::String => write!(f, "db"),
            Types::Bool => write!(f, "db"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarData {
    pub var_type: Types,
    pub var_name: String,
    pub var_value: String,
}

#[derive(Debug, PartialEq)]
/**
 * Tokens: Exit, IntLit, Var
 * Exit: exit <exit_code>
 * IntLit: <int_value>
 * Var: let:<var_type> <var_name> <var_value>
 */
pub enum Tokens {
    Exit,
    Var(VarData),
    Print,
    Add,
}

#[derive(Debug)]
pub struct Token {
    pub token: Tokens,
    pub value: Option<String>,
}

pub const KEYWORDS: [&str; 9] = [
    "let", "exit", "print", "add", "sub", "mul", "div", "true", "false",
];

pub fn tokenize(input_string: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for line in input_string.lines() {
        let function = line.split(" ").nth(0);
        let operand1 = line.split(" ").nth(1);
        let operand2 = line.split(" ").nth(2);
        // Rest of the line
        let overflow = line.splitn(4, " ").last().unwrap_or("");

        if let Some(function) = function {
            if Regex::new(r"let:[a-z]+").unwrap().is_match(&function) {
                let var_type = match function.split(":").nth(1).unwrap() {
                    "int" => Types::Int,
                    "float" => Types::Float,
                    "string" => Types::String,
                    "bool" => Types::Bool,
                    _ => panic!(
                        "Invalid variable type: {}",
                        function.split(":").nth(1).unwrap()
                    ),
                };

                match var_type {
                    Types::Int => {
                        if !is_int(operand2.unwrap().to_string()) {
                            panic!("Invalid int: {}", operand2.unwrap());
                        }
                    }
                    Types::Float => {
                        if !is_float(operand2.unwrap().to_string()) {
                            panic!("Invalid float: {}", operand2.unwrap());
                        }
                    }
                    Types::String => {
                        let input_string = operand2.unwrap().to_string() + " " + overflow;

                        if !is_string(input_string) {
                            panic!("Invalid float: {}", operand2.unwrap());
                        }
                    }
                    Types::Bool => {
                        if !is_var(operand2.unwrap().to_string()) {
                            panic!("Invalid bool: {}", operand2.unwrap());
                        }
                    }
                }

                if var_type == Types::String {
                    operand2.unwrap().to_string().push_str(overflow);
                    let var_value = operand2.unwrap().to_string() + " " + overflow;
                    let var_name = operand1.unwrap();

                    if KEYWORDS.contains(&var_name) {
                        panic!("Reserved variable name: {}", var_name);
                    }

                    tokens.push(Token {
                        token: Tokens::Var(VarData {
                            var_type,
                            var_name: var_name.to_string(),
                            var_value: var_value.to_string().replace("'", ""),
                        }),
                        value: Some(var_value.to_string()),
                    });
                    continue;
                }

                let var_name = operand1.unwrap();
                let var_value = operand2.unwrap();

                if KEYWORDS.contains(&var_name) {
                    panic!("Reserved variable name: {}", var_name);
                }

                tokens.push(Token {
                    token: Tokens::Var(VarData {
                        var_type,
                        var_name: var_name.to_string(),
                        var_value: var_value.to_string(),
                    }),
                    value: Some(var_value.to_string()),
                });

                dbg!(var_value);
            } else if Regex::new(r"exit").unwrap().is_match(&function) {
                let exit_code = operand1.unwrap();
                if operand2.is_some() {
                    panic!("Too many arguments for exit");
                };

                tokens.push(Token {
                    token: Tokens::Exit,
                    value: Some(exit_code.to_string()),
                });
            } else if Regex::new(r"add").unwrap().is_match(&function) {
                tokens.push(Token {
                    token: Tokens::Add,
                    value: Some(line.to_string()),
                });
            } else if Regex::new(r"print").unwrap().is_match(&function) {
                if is_string(line.to_string()) {
                    tokens.push(Token {
                        token: Tokens::Print,
                        value: Some(get_string(line.to_string())),
                    });
                } else if is_var(operand1.unwrap().to_string()) {
                    tokens.push(Token {
                        token: Tokens::Print,
                        value: Some(operand1.unwrap().to_string()),
                    });
                } else if is_int(operand1.unwrap().to_string()) {
                    tokens.push(Token {
                        token: Tokens::Print,
                        value: Some(operand1.unwrap().to_string()),
                    });
                } else if is_float(operand1.unwrap().to_string()) {
                    tokens.push(Token {
                        token: Tokens::Print,
                        value: Some(operand1.unwrap().to_string()),
                    });
                } else {
                    panic!("Invalid print: {}", line);
                }
            } else {
                panic!("Invalid function: {}", function);
            }
        }
    }

    tokens
}
