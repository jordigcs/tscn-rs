use std::{thread::current, fmt::Error, mem::discriminant, ops::Deref};

use crate::scene::{self, Scene};
#[derive(Debug)]
pub struct Property();
#[derive(Debug)]
pub struct ElementData(pub String, pub String); // 0: Name, 1: Value

#[derive(Debug)]
pub enum ExpectedType {
    ElementName,
    ElementDataName,
    ElementDataValue,
    PropertyName,
    PropertyValue,
}

#[derive(Debug)]
pub struct Element {
    pub element_name:String,
    pub element_data:Vec<ElementData>,
    pub properties:Vec<Property>,
    pub tokens:Vec<Token>,
}

impl Element {
    pub fn empty() -> Self {
        Element { element_name: String::from("Undefined"), element_data: Vec::new(), properties: Vec::new(), tokens: Vec::new() }
    }
}


// #[derive(PartialEq, Clone)]
// pub enum Value {
//     Undefined,
//     String(String),
//     Resource(String),
//     Int(i32),
// }

type DataIdentifier = String;
#[derive(PartialEq, Clone)]
#[derive(Debug)]
pub enum Token {
    UNDEFINED,
    INVALID,
    BRACKET_LEFT,
    BRACKET_RIGHT,
    NEW_LINE,
    // Elements
    ELEMENT_NAME(Option<String>),
    ELEMENT_DATA_NAME(Option<String>),
    ELEMENT_DATA_VALUE(Option<String>),
    // Element properties
    PROPERTY_NAME,
    PROPERTY_VALUE(Option<String>),
}


pub struct Tokenizer {
    pub tokens:Vec<Token>,
}

type Index = usize;
#[derive(Debug)]
pub enum TokenizerError {
    InvalidChar(Index),
}

fn append_current_string(current_string:&mut Option<String>, character:char) {
    if let Some(string) = current_string {
        string.push(character);
        *current_string = Some(string.to_string());
    }
    else {
        *current_string = Some(String::from(character));
    }
}

fn consume_current_string(current_string:&mut Option<String>) -> Option<String> {
    let new = current_string.deref().clone();
    *current_string = None;
    return new;
}

impl Tokenizer {
    pub fn tokenize(tscn:&str) -> Result<Tokenizer, TokenizerError> {
        let mut tokenizer = Tokenizer { tokens: Vec::new(), };
        let mut next_token:Option<Token> = None;
        let mut current_string:Option<String> = None;
        for (index, c) in tscn.chars().enumerate() {
            let mut current_token:Token = Token::UNDEFINED;
            if let Some(next) = &next_token {
                if *next == Token::INVALID {
                    return Err(TokenizerError::InvalidChar(index));
                }
            }
            match c {
                '[' => {
                    current_token = Token::BRACKET_LEFT;
                    let is_prop:bool = false;
                    if let Some(next) = &next_token {
                        if let Token::PROPERTY_VALUE(_) = next {
                            todo!()
                        }
                    }
                    if !is_prop {
                        next_token = Some(Token::ELEMENT_NAME(None));
                    }
                },
                ']' => {
                    if let Some(next) = &next_token {
                        match next {
                            Token::ELEMENT_DATA_VALUE(..) => {
                                next_token = None;
                                tokenizer.tokens.push(Token::ELEMENT_DATA_VALUE(consume_current_string(&mut current_string)));
                            },
                            _ => {}
                        }
                    }
                    current_token = Token::BRACKET_RIGHT;
                },
                // Unused code for Quotes as tokens. Removed because you can have types other than Strings as property values.
                // '"' | '\'' => {
                //     if let Some(next) = &next_token {
                //         if *next == Token::QUOTE_RIGHT {
                //             current_token = next.clone();
                //             next_token = None;
                //         }
                //     }
                //     else {
                //         current_token = Token::QUOTE_LEFT;
                //         next_token = Some(Token::QUOTE_RIGHT);
                //     }
                // },
                _ => {
                    if let Some(next) = &next_token {
                        match next {
                            Token::ELEMENT_NAME(..) => {
                                if c == ' ' {
                                    next_token = Some(Token::ELEMENT_DATA_NAME(None));
                                    current_token = Token::ELEMENT_NAME(consume_current_string(&mut current_string));
                                    current_string = None;
                                }
                                else {
                                    append_current_string(&mut current_string, c);
                                    continue;
                                }
                            },
                            Token::ELEMENT_DATA_NAME(..) => {
                                match c {
                                    '=' => {
                                        next_token = Some(Token::ELEMENT_DATA_VALUE(None));
                                        current_token = Token::ELEMENT_DATA_NAME(consume_current_string(&mut current_string));
                                    },
                                    ' ' => {
                                        next_token = Some(Token::INVALID);
                                        continue;
                                    },
                                    _ => {
                                        append_current_string(&mut current_string, c);
                                        continue;
                                    },
                                }
                            },
                            Token::ELEMENT_DATA_VALUE(..) => {
                                if c == ' ' {
                                    next_token = Some(Token::ELEMENT_DATA_NAME(None));
                                    current_token = Token::ELEMENT_DATA_VALUE(consume_current_string(&mut current_string));
                                }
                                else {
                                    append_current_string(&mut current_string, c);
                                    continue;
                                }
                            },
                            _ => {
                                continue;
                            }
                        }
                    }
                }
            }
            tokenizer.tokens.push(current_token);
        }
        return Ok(tokenizer);
    }
}