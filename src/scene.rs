use std::{io};

use crate::tokenizer::{Element, Token, ExpectedType, ElementData, Tokenizer, TokenizerError, };

#[derive(Debug)]
pub struct Scene {
    pub elements:Vec<Element>
}

#[derive(Debug)]
pub enum SceneError {
    NotFound(ExpectedType),
    TokenizerError(TokenizerError),
    LoadFailed(io::Error),
    UnexpectedErr,
}

impl Scene {
    pub fn from_elements(elements:Vec<Element>) -> Self {
        Self { elements, }
    }

    pub fn from_tokens(tokens:Vec<Token>) -> Result<Self, SceneError> {
        let mut elements:Vec<Element> = Vec::new();
        let mut current_element:Element = Element::empty();
        let mut element_finished:bool = false;
        for token in tokens.iter() {
            match token {
                Token::ELEMENT_NAME(name) => {
                    if let Some(string) = name {
                        current_element.element_name = string.to_string();
                    }
                    else {
                        return Err(SceneError::NotFound(ExpectedType::ElementDataName));
                    }
                },
                Token::ELEMENT_DATA_NAME(name) => {
                    if let Some(string) = name {
                        current_element.element_data.push(ElementData(string.to_string(), String::new()));
                    }
                    else {
                        return Err(SceneError::NotFound(ExpectedType::ElementDataName));
                    }
                },
                Token::ELEMENT_DATA_VALUE(value) => {
                    if let Some(string) = value {
                        if let Some(data) = current_element.element_data.last_mut() {
                            data.1 = string.to_string();
                        }
                        else {
                            return Err(SceneError::UnexpectedErr)
                        }
                    }
                    else {
                        return Err(SceneError::NotFound(ExpectedType::ElementDataValue));
                    }
                },
                Token::BRACKET_RIGHT => {
                    element_finished = true;
                }
                _ => {}
            }
            if element_finished {
                element_finished = false;
                elements.push(current_element);
                current_element = Element::empty();
            }
        }
        Ok(Self { elements, })
    }

    pub fn from_tscn_content(file_content:&str) -> Result<Self, SceneError> {
        match Tokenizer::tokenize(file_content) {
            Ok(tokenizer) => {
                Scene::from_tokens(tokenizer.tokens)
            },
            Err(error) => {
                Err(SceneError::TokenizerError(error))
            }
        }
    }
}