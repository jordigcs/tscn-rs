use std::{io};

use crate::tokenizer::{Token, Tokenizer, TokenizerError, };
use crate::element::{ElementData, Element, ExpectedType, ElementType,};

#[derive(Debug)]
pub struct Scene {
    pub scene_data:Vec<Element>,
    pub resources:Vec<Element>,
    pub nodes:Vec<Element>,
    pub unkown_elements:Vec<Element>,
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

    pub fn add_elements(&mut self, elements:Vec<Element>) {
        self.nodes.append(&mut Scene::filter_elements(elements, ElementType::NODE));
    }

    pub fn from_tscn_content(file_content:&str) -> Result<Self, SceneError> {
        match Tokenizer::tokenize(file_content) {
            Ok(tokenizer) => {
                Ok(Self {
                    scene_data: Scene::filter_elements(tokenizer.elements.clone(), ElementType::SCENE_DATA),
                    resources: Scene::filter_elements(tokenizer.elements.clone(), ElementType::RESOURCE),
                    nodes: Scene::filter_elements(tokenizer.elements.clone(), ElementType::NODE),
                    unkown_elements: Scene::filter_elements(tokenizer.elements.clone(), ElementType::UNKOWN),
                    tokenizer
                })
            },
            Err(error) => {
                Err(SceneError::TokenizerError(error))
            }
        }
    }
}