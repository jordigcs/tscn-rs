use std::{thread::current, fmt::Error, mem::{discriminant, Discriminant}, ops::Deref, rc::Rc};

use crate::{scene::{self, Scene, SceneError}, element::{Element, ExpectedType, ElementData, ElementType}};

// #[derive(PartialEq, Clone)]
// pub enum Value {
//     Undefined,
//     String(String),
//     Resource(String),
//     Int(i32),
// }

type DataIdentifier = String;
#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    UNRESOLVED,
    INVALID,
    BRACKET_LEFT,
    BRACKET_RIGHT,
    NEW_LINE,
    // Elements
    ELEMENT_NAME(Option<String>),
    ELEMENT_DATA_NAME(Option<String>),
    ELEMENT_DATA_VALUE(Option<String>),
    // Element properties
    PROPERTY_NAME(Option<String>),
    PROPERTY_VALUE(Option<String>),
    //Control
    SKIP_TO(Rc<Token>),
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::BRACKET_LEFT => {
                String::from('[')
            },
            Token::BRACKET_RIGHT => {
                String::from(']')
            },
            Token::NEW_LINE => {
                String::from('\n')
            },
            Token::INVALID => {
                String::from("{INVALID}")
            },
            Token::ELEMENT_NAME(val) => {
                if let Some(string) = val {
                    String::from(string)
                }
                else {
                    String::from("{UNRESOLVED}")
                }
            },
            Token::ELEMENT_DATA_NAME(val) => {
                if let Some(string) = val {
                    let mut n = string.clone();
                    n.push('=');
                    String::from(&n)
                }
                else {
                    String::from("{UNRESOLVED}")
                }
            },
            Token::ELEMENT_DATA_VALUE(val) => {
                if let Some(string) = val {
                    String::from(string)
                }
                else {
                    String::from("{UNRESOLVED}")
                }
            },
            Token::PROPERTY_NAME(val) => {
                if let Some(string) = val {
                    let mut n = string.clone();
                    n += " = ";
                    String::from(&n)
                }
                else {
                    String::from("{UNRESOLVED}")
                }
            },
            Token::PROPERTY_VALUE(val) => {
                if let Some(string) = val {
                    string.to_string()
                }
                else {
                    String::from("{UNRESOLVED}")
                }
            },
            Token::UNRESOLVED | _ => {
                 String::from("{UNDEFINED}")
            }
        }
    }
    fn requires_space_suffix(&self) -> bool {
        match self {
            Token::ELEMENT_NAME(..) | Token::ELEMENT_DATA_VALUE(..) => {
                true
            },
            _ => {
                false
            }
        }
    }
}

#[derive(Debug)]
pub struct Tokenizer {
    pub elements:Vec<Element>,
    pub tokens:Vec<Token>,
    current_string:Option<String>,
    in_quote:bool,
    current_string_completed:bool,
}

type Index = usize;
#[derive(Debug)]
pub enum TokenizerError {
    NotFound(ExpectedType),
    InvalidChar(Index),
    EarlyEOF,
    UnexpectedErr,
}

impl Tokenizer {
    fn append_current_string(&mut self, character:char, end_chars:&[char]) {
        if let Some(mut string) = self.current_string.clone() {
            if string.chars().into_iter().filter(|c| !end_chars.contains(c)).count() > 0 { // Check if string has chars other than space or equals.
                if end_chars.contains(&character) {
                    if !self.in_quote {
                        self.current_string_completed = true;
                        return;
                    }
                }
                else {
                    self.current_string_completed = false;
                }
            }
            else if character == '"' || character == '\'' {
                self.in_quote = !self.in_quote; // If we're not in a quote, now we are. If we were already in a quote, now we aren't :O
            }
            string.push(character);
            self.current_string = Some(string.to_string());
        }
        else {
            self.current_string = Some(String::from(character));
        }
    }
    
    fn consume_current_string(&mut self) -> Option<String> {
        let mut new = self.current_string.clone();
        if let Some(string) = new {
            new = Some(string.trim().to_string());
        }
        self.current_string = None;
        self.current_string_completed = false;
        self.in_quote = false;
        return new;
    }

    pub fn tokenize(tscn:&str) -> Result<Tokenizer, TokenizerError> {
        let mut tokenizer = Tokenizer { elements: Vec::new(), tokens: Vec::new(), current_string:None, in_quote: false, current_string_completed: false, };
        let mut next_token:Option<Token> = None;
        for (index, c) in tscn.chars().enumerate() {
            let mut current_token:Token = Token::UNRESOLVED;

            if let Some(next) = &next_token {
                match next {
                    Token::INVALID => {
                        return Err(TokenizerError::InvalidChar(index));
                    },
                    Token::SKIP_TO(new_next_token) => {
                        next_token = Some(new_next_token.deref().clone());
                        continue;
                    },
                    _ => {}
                }
            }

            match c {
                '[' => {
                    current_token = Token::BRACKET_LEFT;
                    let mut is_value:bool = false;
                     if let Some(next) = &next_token {
                         if let Token::PROPERTY_VALUE(_) | Token::ELEMENT_DATA_VALUE(..) = next {
                            is_value = true;
                        }
                    }
                   if !is_value {
                        next_token = Some(Token::ELEMENT_NAME(None));
                  }
                },
                ']' => {
                    if let Some(next) = &next_token {
                        match next {
                            Token::ELEMENT_DATA_VALUE(..) => {
                                next_token = None;
                                let token_value = tokenizer.consume_current_string();
                                tokenizer.tokens.push(Token::ELEMENT_DATA_VALUE(token_value));
                            },
                            Token::PROPERTY_VALUE(..) => {
                                next_token = None;
                                let token_value = tokenizer.consume_current_string();
                                tokenizer.tokens.push(Token::PROPERTY_VALUE(token_value));
                            }
                            _ => {}
                        }
                    }
                    current_token = Token::BRACKET_RIGHT;
                },
                '\n' => {
                    let mut token_mutated:bool = false;
                    if let Some(last) = tokenizer.tokens.last() {
                        match last {
                            Token::BRACKET_RIGHT => {
                                next_token = Some(Token::PROPERTY_NAME(None));
                                token_mutated = true;
                            },
                            _ => {}
                        }
                    }
                    if !token_mutated {
                        if let Some(next) = &next_token {
                            match next {
                                Token::PROPERTY_VALUE(..) => {
                                    next_token = Some(Token::PROPERTY_NAME(None));
                                    let prop_value = tokenizer.consume_current_string();
                                    tokenizer.tokens.push(Token::PROPERTY_VALUE(prop_value));
                                },
                                _ => {}
                            }
                        }
                    }
                    current_token = Token::NEW_LINE;
                }
                _ => {
                    if let Some(next) = &next_token {
                        match next {
                            Token::ELEMENT_NAME(..) => {
                                tokenizer.append_current_string(c, &[' ']);
                                if tokenizer.current_string_completed {
                                    next_token = Some(Token::ELEMENT_DATA_NAME(None));
                                    current_token = Token::ELEMENT_NAME(tokenizer.consume_current_string());
                                }
                                else {
                                    continue;
                                }
                            },
                            Token::ELEMENT_DATA_NAME(..) => {
                                tokenizer.append_current_string(c, &[' ', '=']);
                                if tokenizer.current_string_completed {
                                    // skip '=' and jump to ELEMENT_DATA_VALUE
                                    next_token = Some(Token::ELEMENT_DATA_VALUE(None));
                                    current_token = Token::ELEMENT_DATA_NAME(tokenizer.consume_current_string());
                                }
                                else {
                                    continue;
                                }
                            },
                            Token::ELEMENT_DATA_VALUE(..) => {
                                tokenizer.append_current_string(c,  &[' ']);
                                if tokenizer.current_string_completed {
                                    next_token = Some(Token::ELEMENT_DATA_NAME(None));
                                    current_token = Token::ELEMENT_DATA_VALUE(tokenizer.consume_current_string());
                                }
                                else {
                                    continue;
                                }
                            },
                            Token::PROPERTY_NAME(..) => {
                                tokenizer.append_current_string(c, &[' ', '=']);
                                if tokenizer.current_string_completed {
                                    if let Some(next_char) = tscn.chars().nth(index+1) {
                                        if next_char == '=' {
                                            // skip '=' and jump to PROPERTY_VALUE
                                            next_token = Some(Token::SKIP_TO(Rc::new(Token::PROPERTY_VALUE(None))));
                                            current_token = Token::PROPERTY_NAME(tokenizer.consume_current_string());
                                        }
                                    }
                                    else {
                                        return Err(TokenizerError::EarlyEOF);
                                    }
                                }
                                else {
                                    continue;
                                }
                            },
                            Token::PROPERTY_VALUE(..) => {
                                tokenizer.append_current_string(c, &[]);
                                // current_string is consumed in the '\n' match branch since property values can only be single line.
                                continue;
                            },
                            _ => {
                                continue;
                            }
                        }
                    }
                }
            }
            tokenizer.in_quote = false;
            tokenizer.tokens.push(current_token);
        }
        match tokenizer.elements_from_tokens() {
            Ok(elements) => {
                tokenizer.elements = elements;
            },
            Err(error) => {
                return Err(error);
            },
        }
        return Ok(tokenizer);
    }

    pub fn elements_from_tokens(&self) -> Result<Vec<Element>, TokenizerError> {
        let mut elements:Vec<Element> = Vec::new();
        let mut current_element:Element = Element::empty();
        let mut element_finished:bool = false;
        for token in self.tokens.iter() {
            match token {
                Token::ELEMENT_NAME(name) => {
                    if let Some(string) = name {
                        match &string[..] { // Convert to &[slice] to match against &str 
                            "gd_scene" | "connection" => {
                                current_element.element_type = ElementType::SCENE_DATA;
                            },
                            "ext_resource" | "sub_resource" => {
                                current_element.element_type = ElementType::RESOURCE;
                            },
                            "node" => {
                                current_element.element_type = ElementType::NODE;
                            },
                            _ => {
                                current_element.element_type = ElementType::UNKOWN;
                            }
                        }
                        current_element.element_name = string.to_string();
                    }
                    else {
                        return Err(TokenizerError::NotFound(ExpectedType::ElementDataName));
                    }
                },
                Token::ELEMENT_DATA_NAME(name) => {
                    if let Some(string) = name {
                        current_element.element_data.push(ElementData(string.to_string(), String::new()));
                    }
                    else {
                        return Err(TokenizerError::NotFound(ExpectedType::ElementDataName));
                    }
                },
                Token::ELEMENT_DATA_VALUE(value) => {
                    if let Some(string) = value {
                        if let Some(data) = current_element.element_data.last_mut() {
                            data.1 = string.to_string();
                        }
                        else {
                            return Err(TokenizerError::UnexpectedErr)
                        }
                    }
                    else {
                        return Err(TokenizerError::NotFound(ExpectedType::ElementDataValue));
                    }
                },
                Token::BRACKET_RIGHT => {
                    element_finished = true;
                }
                _ => {}
            }
            //current_element.tokens.push(token.clone());
            if element_finished {
                element_finished = false;
                elements.push(current_element);
                current_element = Element::empty();
            }
        }
        Ok(elements)
    }

    pub fn to_tscn_content(&self) -> String {
        self.tokens.iter().map(|token| {
            token.to_string() + if token.requires_space_suffix() { " " } else { "" }
        }).collect::<String>()
    }

    pub fn reconstruct_tscn_from_tokens(tokens:Vec<Token>) -> String {
        tokens.iter().map(|token| {
            token.to_string() + if token.requires_space_suffix() { " " } else { "" }
        }).collect::<String>()
    }
}