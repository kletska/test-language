use crate::inner_representation::token::{Token, TokenKind, RawToken, RawTokenKind};
use crate::utils::tokenizing_constants::{separating_symbols, special_symbols, special_tokens};

fn remove_comments(program_text: String) -> Result<String, String> { // #* *#
    Ok(program_text
        .split("#*")
        .map(|x| x.split("*#").collect::<Vec<&str>>())
        .enumerate()
        .map(|(i, x)| {
            if i == 0 && x.len() == 1 {
                Ok(x[0])
            } else if x.len() == 2 {
                Ok(x[1])
            }else {
                Err(format!("Wrong comment partition at {:?}", x))}
            }
        )
        .collect::<Result<Vec<&str>, String>>()?
        .join(" ")
    )
}

fn first_step_tokenize(program_test: Result<String, String>) -> Result<Vec<RawToken>, String> {
    let result: Vec<RawToken> = program_test?
        .split("/")
        .enumerate()
        .map(|(i, x)| {
            if i % 2 == 0 {
                RawToken::new(String::from(x), RawTokenKind::DontKnow)
            } else {
                RawToken::new(String::from(x), RawTokenKind::StringLiteral)
            }
        })
        .collect();
    if result.len() % 2 == 1 {
        Ok(result)
    } else {
        Err("problem with string literals".to_string())
    }
}

fn token_analyse(raw_token: &str) -> Result<Token, String> { //todo
    for i in 0..special_symbols.len() {
        if special_symbols[i] == raw_token {
            if raw_token == "(" {
                return Ok(
                    Token::new(
                        raw_token.to_string(), 
                        special_tokens[i], 
                    )
                );
            } else {
                return Ok(
                    Token::new(
                        raw_token.to_string(), 
                        special_tokens[i], 
                    )
                );
            }
        }
    }
    if let Ok(_) = raw_token.parse::<i32>() {
        Ok(Token::new(raw_token.to_string(), TokenKind::Int))
    } else {
        Ok(Token::new(raw_token.to_string(), TokenKind::Name))
        //Err(format!("No token for {}", raw_token))
    }
}

fn separate_symbols(text: String) -> String {
    separating_symbols
        .iter()
        .fold(text, |text, sym| {
            text.replace(sym, &format!(" {} ", sym)[..])
        })
}

fn raw_token_analyse(raw_token: RawToken) -> Result<Vec<Token>, String> {
    match raw_token.kind {
        RawTokenKind::StringLiteral => {
            Ok(vec![Token::new(raw_token.text, TokenKind::StringLiteral)])
        },
        RawTokenKind::DontKnow => {
            separate_symbols(raw_token.text)
                .split_whitespace()
                .map(token_analyse)
                .collect()
        }
    }
}

fn second_step_tokenize(tokens: Result<Vec<RawToken>, String>) -> Result<Vec<Token>, String> {
    Ok(tokens?
        .into_iter()
        .map(raw_token_analyse)
        .collect::<Result<Vec<Vec<Token>>, String>>()?
        .concat()
    )
}

pub fn tokenize(program_text: String) -> Result<Vec<Token>, String> {
    second_step_tokenize(first_step_tokenize(remove_comments(program_text)))
}

#[cfg(test)]
mod tokenizing_tests {
    #[test]
    fn unit_tests() {
        println!("{:?}", super::tokenize("123/test/".to_string()));
        println!("{:?}", super::tokenize("$List: @ -> @ = A ~> Nil . + Cons A * List A;".to_string()));
        println!("{:?}", super::tokenize("123#*this is comment*#/this is string literal/".to_string()));
        println!("{:?}", super::tokenize("".to_string()));
        println!("{:?}", super::tokenize("true * false".to_string()) )
    }
}

/* 
fn tokenize(program_text: String) -> Result<Vec<Token>, String> {

    special_symbols
        .iter()
        .fold(
            program_text, 
            |text, sym| text.replace(sym, &format!(" {} ", sym)[..])
        )
        .split_whitespace()
        .map(token_analyse)
        .collect()
}
*/