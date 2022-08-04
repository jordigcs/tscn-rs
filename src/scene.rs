use std::{io};

use crate::tokenizer::{Token, Tokenizer, TokenizerError, };
use crate::element::{Element, ElementType,};

#[derive(Debug)]
pub struct Scene {
    pub elements:Vec<Element>,
    pub tokenizer:Tokenizer,
}

#[derive(Debug)]
pub enum SceneError {
    TokenizerError(TokenizerError),
    LoadFailed(io::Error),
    UnexpectedErr,
}

impl Scene {
    pub fn filter_elements(elements:Vec<Element>, element_type:ElementType) -> Vec<Element> {
        elements.into_iter().filter(|element| element.element_type == element_type).collect::<Vec<Element>>()
    }

    pub fn add_elements(&mut self, mut elements:Vec<Element>) {
        self.elements.append(&mut elements);
    }

    pub fn to_tscn(self) -> String {
        let mut tokens:Vec<Token> = Vec::new();
        for mut element in self.elements.into_iter() {
            element.update_tokens();
            tokens.append(&mut element.tokens);
        }
        Tokenizer::reconstruct_tscn_from_tokens(tokens)
    }

    pub fn from_tscn_content(file_content:&str) -> Result<Self, SceneError> {
        match Tokenizer::tokenize(file_content) {
            Ok(tokenizer) => {
                Ok(Self {
                    elements:tokenizer.elements.clone(),
                    tokenizer
                })
            },
            Err(error) => {
                Err(SceneError::TokenizerError(error))
            }
        }
    }
}