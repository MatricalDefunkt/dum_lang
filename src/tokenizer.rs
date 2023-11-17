use std::fmt::Display;

use regex::Regex;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct VarData {
    pub var_type: Types,
    pub var_name: String,
    pub var_value: String,
}

#[derive(Debug)]
/**
 * Tokens: Exit, IntLit, Var
 * Exit: exit <exit_code>
 * IntLit: <int_value>
 * Var: let:<var_type> <var_name> <var_value>
 */
pub enum Tokens {
    Exit,
    IntLit,
    Var(VarData),
    Print,
    Add,
}

#[derive(Debug)]
pub struct Token {
    pub token: Tokens,
    pub value: Option<String>,
}

pub fn tokenize(input_string: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for line in input_string.lines() {
        let function = line.split(" ").nth(0);
        let operand1 = line.split(" ").nth(1);
        let operand2 = line.split(" ").nth(2);
        // Rest of the line
        let overflow = line.split(" ").skip(3).collect::<Vec<&str>>().concat();

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

                let var_name = operand1.unwrap();
                let var_value = operand2.unwrap();

                tokens.push(Token {
                    token: Tokens::Var(VarData {
                        var_type,
                        var_name: var_name.to_string(),
                        var_value: var_value.to_string(),
                    }),
                    value: Some(var_value.to_string()),
                });
            } else if Regex::new(r"exit").unwrap().is_match(&function) {
                let exit_code = operand1.unwrap();
                if operand2.is_some() {
                    panic!("Too many arguments for exit");
                };

                tokens.push(Token {
                    token: Tokens::Exit,
                    value: Some(exit_code.to_string()),
                });
            }
        }
    }

    tokens
}
