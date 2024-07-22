use csv::StringRecord;

#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    Equal,
    NotEqual,
    Contains,
    StringLiteral,
    And,
    Or,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Token {
    val: String,
    token_type: TokenType,
}

impl Token {
    fn to_node(self) -> Node {
        return Node {
            token: self,
            left: None,
            right: None,
        };
    }

    fn is_comparison(self) -> bool {
        match self.token_type {
            TokenType::Equal => true,
            TokenType::NotEqual => true,
            TokenType::Contains => true,
            _ => false,
        }
    }

    fn is_combination(self) -> bool {
        match self.token_type {
            TokenType::And => true,
            TokenType::Or => true,
            _ => false,
        }
    }
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
    headers: Vec<String>,
) -> Box<dyn Iterator<Item = StringRecord>> {
    let token_list = tokenize(query);

    // sanitize the token list to check if there are any problems with the syntax
    if !validate_token_list(&token_list) {
        panic!("Error in find syntax");
    }

    // build the execution tree
    let ast = generate_ast(token_list);
    if ast.is_none() {
        return Box::new(i);
    }

    let check = move |i: &usize, x: &str| {
        return Node::check_conditions(ast.clone().unwrap(), headers[*i].clone(), x.to_string());
    };

    let res = i.filter(move |x| {
        return x
            .iter()
            .enumerate()
            .find(|(i, x)| {
                return check(i, x);
            })
            .is_some();
    });

    return Box::new(res);
}

#[derive(Clone, Debug)]
struct Node {
    token: Token,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    // fn print(self) {
    // println!("{}", Node::print_rec(self));
    // }

    // fn print_rec(node: Node) -> String {
    // if let Some(left) = node.left {
    // return Node::print_rec(*left)
    // + " "
    // + &node.token.token_type.to_string()
    // + " "
    // + &Node::print_rec(*node.right.unwrap().to_owned());
    // } else {
    // return node.token.val;
    // }
    // }

    fn check_conditions(node: Node, column_name: String, val: String) -> bool {
        // if the current node is a leaf node then we return true;

        if node.token.clone().is_comparison() {
            let left = node.left.clone().unwrap();
            let right = node.right.clone().unwrap();

            if left.token.val != column_name {
                return false;
            }

            return match node.token.token_type {
                TokenType::Equal => right.token.val == val,
                TokenType::NotEqual => right.token.val != val,
                TokenType::Contains => right.token.val.contains(&val),
                _ => false,
            };
        } else if node.token.clone().is_combination() {
            let left = node.left.clone().unwrap();
            let right = node.right.clone().unwrap();

            let left_res = Node::check_conditions(*left, column_name.clone(), val.clone());
            let right_res = Node::check_conditions(*right, column_name, val);
            return match node.token.token_type {
                TokenType::And => left_res && right_res,
                TokenType::Or => left_res || right_res,
                _ => false,
            };
        }

        return true;
    }
}

enum NodeType {
    Comparison,
    Combination,
}

fn generate_ast(token_list: Vec<Token>) -> Option<Node> {
    // get the nodes
    // get the execution order (Precedence)
    //
    //
    // Intuition: There will be no 2 operations that are linked to each other.
    // so we need not create a very complex tree structure

    // you also always start with 3 then the extension which is 1 then 3 again
    //
    //
    // TODO: Change the data type of children to an array of fixed length 2

    let mut curr_node_type = NodeType::Comparison;
    let mut i = 0;

    let mut comp_node_list: Vec<Node> = Vec::new();
    let mut comb_token_list: Vec<Token> = Vec::new();

    loop {
        match curr_node_type {
            NodeType::Comparison => {
                if i + 2 >= token_list.len() {
                    break;
                }

                comp_node_list.push(Node {
                    token: token_list[i + 1].clone(),
                    left: Some(Box::new(token_list[i].clone().to_node())),
                    right: Some(Box::new(token_list[i + 2].clone().to_node())),
                });

                curr_node_type = NodeType::Combination;
                i += 3;
                continue;
            }
            NodeType::Combination => {
                if i + 1 >= token_list.len() {
                    break;
                }
                comb_token_list.push(token_list[i].clone());
                curr_node_type = NodeType::Comparison;
                i += 1;
            }
        };
    }

    // go through the res_node_list and create the actual tree structure
    // we get the first 2 in the comp node list and add them using the first comb token
    // we then add this node to the next comp node using the next comb token and so on

    // a = b and c = d or d = a

    if comp_node_list.len() == 0 {
        return None;
    }

    if comp_node_list.len() - 1 != comb_token_list.len() {
        // Ideally should throw an error here
        return None;
    }

    let mut res_node = comp_node_list[0].clone();
    for i in 1..comp_node_list.len() {
        res_node = Node {
            token: comb_token_list[i - 1].clone(),
            left: Some(Box::new(res_node)),
            right: Some(Box::new(comp_node_list[i].clone())),
        }
    }

    Some(res_node)
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
            add_comparison_token(&mut raw_token_list, &mut buffer, TokenType::Equal);
            continue;
        }

        if c == EXCLAMATION {
            if let Some(c) = chars_iter.next() {
                if c != EQUALS {
                    panic!("Error in find syntax")
                }

                add_comparison_token(&mut raw_token_list, &mut buffer, TokenType::NotEqual);
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
