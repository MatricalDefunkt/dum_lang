use crate::tokenizer::{Token, Tokens, Types, VarData};

pub fn parse(tokens: Vec<Token>) -> String {
    let mut variables_asm = String::from("section .data\n");
    let mut main_asm = String::new();
    let mut heap: Vec<VarData> = vec![];
    for token in tokens {
        match token.token {
            Tokens::Exit => main_asm.push_str(
                format!(
                    "
                    mov rax, 60
                    mov rdi, {}
                    syscall
                ",
                    token.value.unwrap()
                )
                .as_str(),
            ),
            Tokens::IntLit => {
                todo!()
            }
            Tokens::Var(VarData {
                var_type,
                var_name,
                var_value,
            }) => {
                match var_type {
                    Types::String => {
                        variables_asm.push_str(
                            format!(
                                "
                                    {}: {} '{}'
                                ",
                                var_name, var_type, var_value
                            )
                            .as_str(),
                        );
                    }
                    _ => {
                        variables_asm.push_str(
                            format!(
                                "
                                    {}: {} {}
                                ",
                                var_name, var_type, var_value
                            )
                            .as_str(),
                        );
                    }
                }
                heap.push(VarData {
                    var_type,
                    var_name,
                    var_value,
                })
            }
            Tokens::Print => {
                let mut print_value = String::new();
                let mut var_flag = false;
                let token_value = token.value.clone().unwrap();
                for var in heap.iter() {
                    if var.var_name == token_value {
                        var_flag = true;
                    }
                }
                if !var_flag {
                    print_value = token_value;
                }
                main_asm.push_str(
                    format!(
                        "
                            mov rax, 1
                            mov rdi, 1
                            mov rsi, {}
                            mov rdx, {}
                            syscall
                        ",
                        if !var_flag {
                            format!("'{}'", print_value)
                        } else {
                            print_value.clone()
                        },
                        if var_flag {
                            let var = heap.iter().find(|var| var.var_name == print_value).unwrap();
                            match var.var_type {
                                Types::String => var.var_value.len(),
                                Types::Int => 4,
                                Types::Float => 8,
                                Types::Bool => 1,
                            }
                        } else {
                            print_value.len()
                        }
                    )
                    .as_str(),
                );
            }
            Tokens::Add => {
                todo!()
            }
        }
    }

    format!(
        "
        global _start
        section .text
        {}
        _start:
        {}
        ",
        variables_asm, main_asm
    )
}
