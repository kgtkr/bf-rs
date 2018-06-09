use std::io::{Read, Write};
mod ast;
mod parser;
use ast::AST;

fn main() {
    run(parser::Parser::new(
        "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+
++.>-.------------.<++++++++.--------.+++.------.--------.>+."
            .to_string(),
    ).parse()
        .unwrap());
}

fn run(code: Vec<AST>) {
    let mut mem = [0u8; 20000];
    let mut ptr = 0;

    run_code(code, &mut mem, &mut ptr);
}

fn getchar() -> u8 {
    let mut buffer = [0; 1];
    std::io::stdin().read(&mut buffer[..]).unwrap();
    buffer[0]
}

fn putchar(x: u8) {
    std::io::stdout().write(&[x]).unwrap();
}

fn run_code(code: Vec<AST>, mem: &mut [u8; 20000], ptr: &mut usize) {
    for x in code {
        match x {
            AST::PtrInc => {
                *ptr += 1;
            }
            AST::PtrDec => {
                *ptr -= 1;
            }
            AST::ValInc => {
                mem[*ptr] += 1;
            }
            AST::ValDec => {
                mem[*ptr] -= 1;
            }
            AST::Out => {
                putchar(mem[*ptr]);
            }
            AST::In => {
                mem[*ptr] = getchar();
            }
            AST::Loop(v) => while mem[*ptr] != 0 {
                run_code(v.clone(), mem, ptr);
            },
        }
    }
}
