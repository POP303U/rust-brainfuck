use std::io::{self, Read, Write};

fn main() {
    println!("rust-brainfuck [version 1.0]");
    loop { read_input(); }
}

fn read_input() {
    let mut input = String::from("");
    print!(">>> ");
    io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .expect("couldn't read input");
    print!(">>> {}\n",parse_commands(input));
}

fn parse_commands(input_string: String) -> String {
    let char_array: Vec<char> = input_string.chars().collect();
    let bf_code: &[char] = &char_array;

    let mut memory: [u8; 30000] = [0; 30000];
    let mut mem_ptr = 0;
    let mut tok_ptr = 0;
    let mut output = String::from("");
    while tok_ptr < bf_code.len() {
        match bf_code[tok_ptr] {
            '>' => {
                if mem_ptr == 30000 {
                    mem_ptr = 0;
                } else {
                    mem_ptr += 1;
                } 
            }
            '<' => {
                if mem_ptr == 0 {
                    mem_ptr = 30000;
                } else {
                    mem_ptr -= 1;
                }
            }
            '+' => {
                if memory[mem_ptr] == 255 {
                    memory[mem_ptr] = 0;
                } else {
                    memory[mem_ptr] = memory[mem_ptr] + 1;
                }
            }
            '-' => {
                if memory[mem_ptr] == 0 {
                    memory[mem_ptr] = 255;
                } else {
                    memory[mem_ptr] = memory[mem_ptr] - 1;
                }
            }
            '.' => output += &(memory[mem_ptr] as char).to_string(),
            ',' => {
                let mut input = [0u8; 1];
                io::stdin().read_exact(&mut input).expect("Failed to read input");
                memory[mem_ptr] = input[0];
            }
            '[' => {
                if memory[mem_ptr] == 0 {
                    let mut layers = 0;
                    loop {
                        if bf_code[tok_ptr] == ']' {
                            if layers == 0 {
                                break;
                            }
                            layers -= 1
                        }
                        tok_ptr += 1;
                        if bf_code[tok_ptr] == '[' {
                            layers += 1
                        }
                    }
                }
            }
            ']' => {
                if memory[mem_ptr] != 0 {
                    let mut layers = 0;
                    loop {
                        if bf_code[tok_ptr] == '[' {
                            if layers == 0 {
                                break;
                            }
                            layers -= 1
                        }
                        tok_ptr -= 1;
                        if bf_code[tok_ptr] == ']' {
                            layers += 1
                        }
                    }
                }
            }
            _ => (),
        }
        tok_ptr += 1;
    }
    return output;
}
