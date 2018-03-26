use std::io::{self, Read, Write};

fn main() {
    run(to_ast(&mut to_token(
        "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+
++.>-.------------.<++++++++.--------.+++.------.--------.>+.",
    ).into_iter()));
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

fn to_token(s: &str) -> Vec<Token> {
    s.chars()
        .filter_map(|x| match x {
            '>' => Some(Token::PtrInc),
            '<' => Some(Token::PtrDec),
            '+' => Some(Token::ValInc),
            '-' => Some(Token::ValDec),
            '.' => Some(Token::Out),
            ',' => Some(Token::In),
            '[' => Some(Token::LoopBegin),
            ']' => Some(Token::LoopEnd),
            _ => None,
        })
        .collect()
}

fn to_ast<T: Iterator<Item = Token>>(ts: &mut T) -> Vec<AST> {
    let mut r = Vec::new();
    while let Some(t) = ts.next() {
        let v = match t {
            Token::PtrInc => AST::PtrInc,
            Token::PtrDec => AST::PtrDec,
            Token::ValInc => AST::ValInc,
            Token::ValDec => AST::ValDec,
            Token::Out => AST::Out,
            Token::In => AST::In,
            Token::LoopBegin => AST::Loop(to_ast(ts)),
            Token::LoopEnd => {
                return r;
            }
        };
        r.push(v);
    }
    r
}

#[derive(PartialEq, Debug, Clone)]
enum Token {
    PtrInc,
    PtrDec,
    ValInc,
    ValDec,
    Out,
    In,
    LoopBegin,
    LoopEnd,
}

#[derive(PartialEq, Debug, Clone)]
enum AST {
    PtrInc,
    PtrDec,
    ValInc,
    ValDec,
    Out,
    In,
    Loop(Vec<AST>),
}
