use std::collections::HashMap;

use csv::StringRecord;

use crate::functions::duplicate::get_selected_header_position_map;

#[derive(Debug, PartialEq)]
enum TokenType {
    Equal,
    NotEqual,
    Contains,
    StringLiteral,
    And,
    Or,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Token {
    val: String,
    token_type: TokenType,
}

const ESCAPE: char = '\\';
const SINGLE_QUOTE: char = '\'';
const DOUBLE_QUOTE: char = '\"';
const EQUALS: char = '=';
const EXCLAMATION: char = '!';
const WHITESPACE: char = ' ';

pub fn find(
    i: Box<dyn Iterator<Item = StringRecord>>,
    query: String,
    headers: &Vec<String>,
) -> Box<dyn Iterator<Item = StringRecord>> {
    let token_list = tokenize(query);
    println!("{:?}", token_list);

    // sanitize the token list to check if there are any problems with the syntax
    if !validate_token_list(&token_list) {
        panic!("Error in find syntax");
    }

    // build the execution tree
    // get the conditions


    i.enumerate().filter(|(i, x)| {
        // TODO: Add the conditions here after parsing
    });

    i
}

fn get_header_map(headers: &Vec<String>) -> HashMap<usize, String> {
    let res: HashMap<usize, String> = HashMap::new();

    for (i, h) in headers.iter().enumerate() {
        res.insert(i, h);
    }

    return res;
}

fn add_string_literal(token_list: &mut Vec<Token>, buffer: &mut String) {
    if buffer.len() == 0 {
        return;
    }

    token_list.push(Token {
        val: buffer.clone().trim().to_string(),
        token_type: TokenType::StringLiteral,
    });

    buffer.clear();
}

fn tokenize(query: String) -> Vec<Token> {
    let mut buffer = String::new();
    let mut raw_token_list: Vec<Token> = Vec::new();
    let mut is_escaped_string = false;
    let mut escape_next = false;

    let mut chars_iter = query.chars();
    while let Some(c) = chars_iter.next() {
        if escape_next {
            buffer.push(c);
            escape_next = false;
            continue;
        }

        if c == ESCAPE {
            escape_next = true;
            continue;
        }

        // Intuition: in the terminal you can only use one of either double
        // or single quotes since the other one is for specifying the params
        // and that will lead to having to escape other occurences

        if is_escaped_string {
            if c == SINGLE_QUOTE || c == DOUBLE_QUOTE {
                is_escaped_string = false;
                add_string_literal(&mut raw_token_list, &mut buffer);
                continue;
            }

            buffer.push(c);
            continue;
        }

        if c == SINGLE_QUOTE || c == DOUBLE_QUOTE {
            is_escaped_string = true;
            continue;
        }

        if c == EQUALS {
            add_comparison_token(token_list, &mut buffer, TokenType::Equal);
            continue;
        }

        if c == EXCLAMATION {
            if let Some(c) = chars_iter.next() {
                if c != EQUALS {
                    panic!("Error in find syntax")
                }

                add_comparison_token(token_list, &mut buffer, TokenType::NotEqual);
                continue;
            }
            panic!("Error in find syntax")
        }

        buffer.push(c);
    }

    if buffer.len() > 0 {
        add_string_literal(&mut raw_token_list, &mut buffer);
    }

    let mut token_list: Vec<Token> = Vec::new();

    for t in raw_token_list {
        if t.token_type == TokenType::StringLiteral {
            let mut parts: Vec<String> = Vec::new();
            for p in t.val.split(WHITESPACE) {
                dbg!(p);
                match p.to_lowercase().as_ref() {
                    "and" => add_condition_token(&mut token_list, &mut parts, TokenType::And),
                    "or" => add_condition_token(&mut token_list, &mut parts, TokenType::Or),
                    _ => parts.push(p.to_string()),
                }
            }

            add_string_literal(&mut token_list, &mut parts.join(" "));
        } else {
            token_list.push(t);
        }
    }

    token_list
}

fn add_condition_token(
    token_list: &mut Vec<Token>,
    parts: &mut Vec<String>,
    token_type: TokenType,
) {
    add_string_literal(token_list, &mut parts.join(" "));
    token_list.push(Token {
        val: String::new(),
        token_type,
    });

    parts.clear();
}

fn add_comparison_token(token_list: &mut Vec<Token>, buffer: &mut String, token_type: TokenType) {
    add_string_literal(token_list, buffer);
    token_list.push(Token {
        val: String::new(),
        token_type,
    });
}

fn validate_token_list(token_list: &Vec<Token>) -> bool {
    let op_tokens = [
        TokenType::Equal,
        TokenType::NotEqual,
        TokenType::And,
        TokenType::Or,
        TokenType::Contains,
    ];

    return token_list.windows(3).all(|x| {
        if op_tokens.contains(&x[1].token_type) {
            return x[0].token_type == TokenType::StringLiteral
                && x[2].token_type == TokenType::StringLiteral;
        }

        return true;
    });
}
