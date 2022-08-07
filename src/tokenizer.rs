use std::{ops::Deref, rc::Rc, io::{BufReader, Read, BufRead}, fs::File};

use crate::{element::{Element, ExpectedType, ElementData, ElementType, Property}};

// #[derive(PartialEq, Clone)]
// pub enum Value {
//     Undefined,
//     String(String),
//     Resource(String),
//     Int(i32),
// }

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Unresolved,
    Invalid,
    BracketLeft,
    BracketRight,
    NewLine,
    // Elements
    ElementName(Option<String>),
    ElementDataName(Option<String>),
    ElementDataValue(Option<String>),
    // Element properties
    PropertyName(Option<String>),
    PropertyValue(Option<String>),
    //Control
    SkipTo(Rc<Token>),
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::BracketLeft => {
                String::from('[')
            },
            Token::BracketRight => {
                String::from(']')
            },
            Token::NewLine => {
                String::from('\n')
            },
            Token::Invalid => {
                String::from("{Invalid}")
            },
            Token::ElementName(val) => {
                if let Some(string) = val {
                    String::from(string)
                }
                else {
                    String::from("{Unresolved}")
                }
            },
            Token::ElementDataName(val) => {
                if let Some(string) = val {
                    let mut n = string.clone();
                    n.push('=');
                    String::from(&n)
                }
                else {
                    String::from("{Unresolved}")
                }
            },
            Token::ElementDataValue(val) => {
                if let Some(string) = val {
                    String::from(string)
                }
                else {
                    String::from("{Unresolved}")
                }
            },
            Token::PropertyName(val) => {
                if let Some(string) = val {
                    let mut n = string.clone();
                    n += " = ";
                    String::from(&n)
                }
                else {
                    String::from("{Unresolved}")
                }
            },
            Token::PropertyValue(val) => {
                if let Some(string) = val {
                    string.to_string()
                }
                else {
                    String::from("{Unresolved}")
                }
            },
            Token::Unresolved | _ => {
                 String::from("{UNDEFINED}")
            }
        }
    }
    fn requires_space_suffix(&self) -> bool {
        match self {
            Token::ElementName(..) | Token::ElementDataValue(..) => {
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
        if character == '"' || character == '\'' {
            self.in_quote = !self.in_quote; // If we're not in a quote, now we are. If we were already in a quote, now we aren't :O
        }
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
        new
    }

    pub fn tokenize(mut reader:BufReader<File>, line_count:usize) -> Result<Tokenizer, TokenizerError> {
        let mut tokenizer = Tokenizer { elements: Vec::new(), tokens: Vec::new(), current_string:None, in_quote: false, current_string_completed: false, };
        let mut next_token:Option<Token> = None;
        let mut current_line:u16 = 0;
        let mut line_beginning_char_index:usize = 0;
        let mut index:usize = 0;
        
        'lines: for _ in 0..line_count {
            let mut line = String::new();
            reader.read_line(&mut line);
            println!("{}", line);
            'chars: for (index, c) in line.chars().enumerate() {
                let mut current_token:Token = Token::Unresolved;

                if let Some(next) = &next_token {
                    match next {
                        Token::Invalid => {
                            return Err(TokenizerError::InvalidChar(index));
                        },
                        Token::SkipTo(new_next_token) => {
                            next_token = Some(new_next_token.deref().clone());
                            continue 'chars;
                        },
                        _ => {}
                    }
                }
                match c {
                    '[' => {
                        let mut is_value:bool = false;
                        if let Some(next) = &next_token {
                            if let Token::PropertyValue(_) | Token::ElementDataValue(..) = next {
                                is_value = true;
                            }
                        }
                    if !is_value {
                            current_token = Token::BracketLeft;
                            next_token = Some(Token::ElementName(None));
                    }
                    },
                    ']' => {
                        if let Some(next) = &next_token {
                            match next {
                                Token::ElementDataValue(..) => {
                                    next_token = None;
                                    let token_value = tokenizer.consume_current_string();
                                    tokenizer.tokens.push(Token::ElementDataValue(token_value));
                                },
                                Token::PropertyValue(..) => {
                                    next_token = None;
                                    let token_value = tokenizer.consume_current_string();
                                    tokenizer.tokens.push(Token::PropertyValue(token_value));
                                }
                                _ => {}
                            }
                        }
                        current_token = Token::BracketRight;
                    },
                    '\n' => {
                        current_line += 1;
                        line_beginning_char_index = index;
                        let mut token_mutated:bool = false;
                        if let Some(last) = tokenizer.tokens.last() {
                            match last {
                                Token::BracketRight => {
                                    next_token = Some(Token::PropertyName(None));
                                    token_mutated = true;
                                },
                                _ => {}
                            }
                        }
                        current_token = Token::NewLine;
                    }
                    _ => {
                        if let Some(next) = &next_token {
                            match next {
                                Token::ElementName(..) => {
                                    tokenizer.append_current_string(c, &[' ']);
                                    if tokenizer.current_string_completed {
                                        next_token = Some(Token::ElementDataName(None));
                                        current_token = Token::ElementName(tokenizer.consume_current_string());
                                    }
                                    else {
                                        continue 'chars;
                                    }
                                },
                                Token::ElementDataName(..) => {
                                    tokenizer.append_current_string(c, &[' ', '=']);
                                    if tokenizer.current_string_completed {
                                        // skip '=' and jump to ElementDataValue
                                        next_token = Some(Token::ElementDataValue(None));
                                        current_token = Token::ElementDataName(tokenizer.consume_current_string());
                                    }
                                    else {
                                        continue 'chars;
                                    }
                                },
                                Token::ElementDataValue(..) => {
                                    tokenizer.append_current_string(c,  &[' ']);
                                    if tokenizer.current_string_completed {
                                        next_token = Some(Token::ElementDataName(None));
                                        current_token = Token::ElementDataValue(tokenizer.consume_current_string());
                                    }
                                    else {
                                        continue 'chars;
                                    }
                                },
                                Token::PropertyName(..) => {
                                    tokenizer.append_current_string(c, &[' ', '=']);
                                    if tokenizer.current_string_completed {
                                        if let Some(next_char) = line.chars().nth(index+1) {
                                            if next_char == '=' {
                                                // skip '=' and jump to PropertyValue
                                                next_token = Some(Token::SkipTo(Rc::new(Token::PropertyValue(None))));
                                                current_token = Token::PropertyName(tokenizer.consume_current_string());
                                            }
                                        }
                                        else {
                                            return Err(TokenizerError::EarlyEOF);
                                        }
                                    }
                                    else {
                                        continue 'chars;
                                    }
                                },
                                Token::PropertyValue(..) => {
                                    // Read entire line starting at prop value start, then continue to next.
                                    // Saves time from reading chars we don't need to analyze.
                                    tokenizer.current_string = Some(String::from(&line[index..]));
                                    tokenizer.current_string_completed = true;
                                    next_token = Some(Token::PropertyName(None));
                                    let prop_value = tokenizer.consume_current_string();
                                    tokenizer.tokens.push(Token::PropertyValue(prop_value));
                                    tokenizer.tokens.push(Token::NewLine);
                                    continue 'lines;
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
        }
        match tokenizer.elements_from_tokens() {
            Ok(elements) => {
                tokenizer.elements = elements;
            },
            Err(error) => {
                return Err(error);
            },
        }
        Ok(tokenizer)
    }

    pub fn elements_from_tokens(&self) -> Result<Vec<Element>, TokenizerError> {
        let mut elements:Vec<Element> = Vec::new();
        let mut current_element:Element = Element::empty();
        let mut element_finished:bool = false;
        for token in self.tokens.iter() {
            match token {
                Token::ElementName(name) => {
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
                Token::ElementDataName(name) => {
                    if let Some(string) = name {
                        current_element.element_data.push(ElementData(string.to_string(), String::new()));
                    }
                    else {
                        return Err(TokenizerError::NotFound(ExpectedType::ElementDataName));
                    }
                },
                Token::ElementDataValue(value) => {
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
                Token::PropertyName(name) => {
                    if let Some(string) = name {
                        current_element.properties.push(Property(string.to_string(), String::new()));
                    }
                    else {
                        return Err(TokenizerError::NotFound(ExpectedType::PropertyName));
                    }
                },
                Token::PropertyValue(value) => {
                    if let Some(string) = value {
                        if let Some(data) = current_element.properties.last_mut() {
                            data.1 = string.to_string();
                        }
                        else {
                            return Err(TokenizerError::UnexpectedErr)
                        }
                    }
                    else {
                        return Err(TokenizerError::NotFound(ExpectedType::PropertyValue));
                    }
                },
                Token::BracketRight => {
                    element_finished = true;
                }
                _ => {}
            }
            current_element.tokens.push(token.clone());
            if element_finished {
                element_finished = false;
                elements.push(current_element);
                current_element = Element::empty();
            }
        }
        Ok(elements)
    }

    pub fn to_tscn_content(&self) -> String {
        Tokenizer::reconstruct_tscn_from_tokens(self.tokens.clone())
    }

    pub fn reconstruct_tscn_from_tokens(tokens:Vec<Token>) -> String {
        tokens.iter().map(|token| {
            token.to_string() + if token.requires_space_suffix() { " " } else { "" }
        }).collect::<String>()
    }
}