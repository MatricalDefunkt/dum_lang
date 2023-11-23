use core::panic;

use crate::{
    tokenizer::{Token, Tokens, Types, VarData},
    utilities::{get_string, is_string},
};

pub fn parse(tokens: Vec<Token>) -> String {
    let mut variables_asm = String::from("section .data\n");
    let mut main_asm = String::from("section .text\n_start:\n");
    let mut heap: Vec<VarData> = vec![];
    let mut set_vars: Vec<String> = vec![];
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

                for var in &heap {
                    if var.var_name == token.value.clone().unwrap() {
                        var_flag = true;
                    }
                }

                if !var_flag {
                    print_value = token.value.clone().unwrap();

                    if is_string(print_value.clone()) {
                        print_value = get_string(print_value);
                    }

                    if print_value == "true" {
                        print_value = String::from("1");
                    } else if print_value == "false" {
                        print_value = String::from("0");
                    }

                    let temp_var_name = format!("temp_{}", set_vars.len());
                    set_vars.push(temp_var_name.clone());

                    variables_asm.push_str(
                        format!(
                            "
                                {}: db {}
                            ",
                            temp_var_name, print_value
                        )
                        .as_str(),
                    );

                    if is_string(print_value.clone()) {
                        main_asm.push_str(
                            format!(
                                "
                                mov rax, 1
                                mov rdi, 1
                                mov rsi, {}
                                mov rdx, {}
                                syscall
                            ",
                                set_vars[set_vars.len() - 1],
                                print_value.len()
                            )
                            .as_str(),
                        );
                    } else {
                        todo!();
                    }
                } else {
                    let mut var: VarData = VarData {
                        var_type: Types::String,
                        var_name: String::from("NOT FOUND"),
                        var_value: String::new(),
                    };
                    for var_data in &heap {
                        if var_data.var_name == token.value.clone().unwrap() {
                            var = var_data.clone();
                            print_value = var.var_value.clone();
                        }
                    }

                    if var.var_name == "NOT FOUND" {
                        panic!("Variable not found: {}", token.value.clone().unwrap());
                    }

                    if print_value == "true" {
                        print_value = String::from("1");
                    } else if print_value == "false" {
                        print_value = String::from("0");
                    }

                    if is_string(print_value.clone()) {
                        main_asm.push_str(
                            format!(
                                "
                                mov rax, 1
                                mov rdi, 1
                                mov rsi, {}
                                mov rdx, {}
                                syscall
                            ",
                                var.var_name,
                                print_value.len()
                            )
                            .as_str(),
                        );
                    } else {
                        unimplemented!("Printing non-string variables is not yet supported!");
                    }
                }
            }
            Tokens::Add => {
                todo!()
            }
            
        }
    }

    format!(
        "
        global _start
        {}
        {}
        ",
        variables_asm, main_asm
    )
}
