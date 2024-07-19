use csv::StringRecord;

#[derive(Debug, PartialEq)]
enum TokenType {
    Equal,
    NotEqual,
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
const EQUALS: char = '=';
const EXCLAMATION: char = '!';
const WHITESPACE: char = ' ';

pub fn find(
    i: Box<dyn Iterator<Item = StringRecord>>,
    query: String,
) -> Box<dyn Iterator<Item = StringRecord>> {
    let token_list = tokenize(query);
    println!("{:?}", token_list);

    // build the execution tree

    i
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

    // NOTE: this will cause the column names to be restricted to not having = and ! in their names
    //
    //
    // first check for any whitespaces

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

        if is_escaped_string {
            if c == SINGLE_QUOTE {
                is_escaped_string = false;
                add_string_literal(&mut raw_token_list, &mut buffer);
                continue;
            }

            buffer.push(c);
            continue;
        }

        if c == SINGLE_QUOTE {
            is_escaped_string = true;
            continue;
        }

        if c == EQUALS {
            add_string_literal(&mut raw_token_list, &mut buffer);
            raw_token_list.push(Token {
                val: String::new(),
                token_type: TokenType::Equal,
            });

            continue;
        }

        if c == EXCLAMATION {
            if let Some(c) = chars_iter.next() {
                if c != EQUALS {
                    panic!("Error in find syntax")
                }

                add_string_literal(&mut raw_token_list, &mut buffer);
                raw_token_list.push(Token {
                    val: String::new(),
                    token_type: TokenType::NotEqual,
                });

                continue;
            }
            panic!("Error in find syntax")
        }

        buffer.push(c);
    }

    if buffer.len() > 0 {
        raw_token_list.push(Token {
            val: buffer.clone(),
            token_type: TokenType::StringLiteral,
        })
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
