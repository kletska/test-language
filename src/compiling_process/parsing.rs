use std::collections::VecDeque;

use crate::inner_representation::token_tree::TreeBuilder;
use crate::inner_representation::token::{Token, TokenKind, TokenDir};

pub fn preparse(mut program: Vec<Token>) -> Vec<Token> {
    let mut result = Vec::new();
    let mut last: Option<Token> = None;

    for val in program.into_iter() {
        match last {
            Some(last_inside) => {
                if TokenKind::is_applicate(last_inside.get_kind(), val.get_kind()) {
                    result.push(last_inside);
                    result.push(Token::new("<-".to_string(), TokenKind::Application));
                } else {
                    result.push(last_inside);
                }
                last = Some(val);

            },
            None => last = Some(val),
        }
    }

    match last {
        Some(last_inside) => result.push(last_inside),
        None => (),
    }
    result
}

fn reverse_polish_notation(mut program: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut result: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();

    while let Some(first) = program.pop() {
        match stack.pop() {
            Some(top) => {
                match top.get_kind().compare(first.get_kind())? {
                    TokenDir::Pair => (),
                    TokenDir::Down => {
                        result.push(top);
                        program.push(first);
                    },
                    TokenDir::Forward => {
                        stack.push(top);
                        result.push(first);
                    },
                    TokenDir::Stay => {
                        stack.push(top);
                        stack.push(first);
                    },
                }
            },

            None => {
                match first.get_kind().get_prior() {
                    n if n > 0 => {
                        stack.push(first);
                    }, 
                    n if n == 0 => {
                        result.push(first);
                    },
                    _ => return Err("no such operation".to_string()),
                } 
            },
        }
    }
    
    while let Some(token) = stack.pop() {
        result.push(token);
    }
    Ok(result)
}


fn parse_RPN(builder: &mut TreeBuilder, mut program: Vec<Token>) -> Result<(), String> {
    let mut stack :Vec<i32> = Vec::new(); 

    while let Some(last) = program.pop() {
        while let Some(0) = stack.last() {
            stack.pop();
            builder.pop();
        } 
        let n = last.get_kind().nary_operation();
        if n >= 0 {
            match stack.last_mut() {
                Some(last) => *last -= 1,
                None => (),
            }
            stack.push(n);
            builder.push(last);
        } else {
            return Err("unexpected operation".to_string());
        } 

    }
    Ok(())
}

fn type_token_check(program: &Vec<Token>) -> Result<(), String> {
    let result = program
        .iter()
        .map(|x| x.get_kind().allow_in_type())
        .fold(true, |x, y| x && y);
    if result {
        Ok(())
    } else {
        Err("not all tokens allowed in type".to_string())
    }
}

pub fn parse_type(builder: &mut TreeBuilder, mut program: Vec<Token>) -> Result<(), String> {
    type_token_check(&program)?;
    program = preparse(program);
    program = reverse_polish_notation(program)?;
    parse_RPN(builder, program)?;
    Ok(()) 
}

fn value_token_check(program: &Vec<Token>) -> Result<(), String> {
    let result = program
        .iter()
        .map(|x| x.get_kind().allow_in_value())
        .fold(true, |x, y| x && y);
    if result {
        Ok(())
    } else {
        Err("not all tokens allowed in value".to_string())
    }
}

pub fn parse_value(builder: &mut TreeBuilder, mut program: Vec<Token>) -> Result<(), String> {
    value_token_check(&program)?;
    program = preparse(program);
    program = reverse_polish_notation(program)?;
    parse_RPN(builder, program)?;
    Ok(()) 
}

fn get_first_kind(input: &Vec<Token>) -> Result<TokenKind, String> {
    Ok(input.last().ok_or("empty input")?.get_kind())
}

fn parse_let(builder: &mut TreeBuilder, mut program: Vec<Token>) -> Result<(), String> {
    let mut typ: Vec<Token> = Vec::new();
    
    if TokenKind::Name == get_first_kind(&program)? {
        builder.push_one(program.pop().unwrap());
    } 

    if TokenKind::Type == get_first_kind(&program)? {
        builder.push(program.pop().unwrap())
    } else {
        return Err("can't parse let".to_string());
    }

    loop {
        match get_first_kind(&program)? {
            TokenKind::Eq => {
                typ.reverse();
                parse_type(builder, typ)?;
                builder.pop();
                builder.push(program.pop().unwrap());
                parse_expr(builder, program)?;
                builder.pop();
                break;
            },
            _ => typ.push(program.pop().unwrap()),
        }
    } 
    Ok(())
}

fn parse_expr(builder: &mut TreeBuilder, mut program: Vec<Token>) -> Result<(), String> {
    todo!()
}

#[cfg(test)]
mod parsing_tests {
    use crate::inner_representation::token_tree::TreeBuilder;
    use crate::compiling_process::tokenizing::tokenize;
    use super::parse_type;

    #[test]
    fn unit_tests() {
        let mut builder = TreeBuilder::new();
        let mut tokens = tokenize("(d + a * c) * b + a * c * d -> b + d * c".to_string()).unwrap();
        tokens.reverse();
        parse_type(&mut builder, tokens);
        println!("{}", builder);
        //println!("{}", result.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(" "));

        let mut builder = TreeBuilder::new();
        let mut tokens = tokenize("A (a * c)".to_string()).unwrap();
        tokens.reverse();
        parse_type(&mut builder, tokens);
        println!("{}", builder);

        let mut builder = TreeBuilder::new();
        let mut tokens = tokenize("a -> b -> c".to_string()).unwrap();
        tokens.reverse();
        parse_type(&mut builder, tokens);
        println!("{}", builder);

        let mut builder = TreeBuilder::new();
        let mut tokens = tokenize("a * b * c".to_string()).unwrap();
        tokens.reverse();
        parse_type(&mut builder, tokens);
        println!("{}", builder);

        let mut builder = TreeBuilder::new();
        let mut tokens = tokenize("(A B C) + C".to_string()).unwrap();
        tokens.reverse();
        parse_type(&mut builder, tokens);
        println!("{}", builder);

        let mut builder = TreeBuilder::new();
        let mut tokens = tokenize("A B (A + C)".to_string()).unwrap();
        tokens.reverse();
        parse_type(&mut builder, tokens);
        println!("{}", builder);
    }
}
