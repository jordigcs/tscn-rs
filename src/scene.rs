use std::rc::Rc;
use std::{io};

use crate::loader;
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

#[derive(Debug)]
pub enum NodePathError {
    NodeNotFound,
    PropertyNotFound,
}

#[derive(Debug)]
pub enum NodePathStatus {
    VALID,
    INVALID(String),
}

#[derive(Debug)]
pub struct NodePath {
    path: Vec<String>,
    node_name: String,
    status: NodePathStatus,
}

impl NodePath {
    fn return_invalid(reason:&str) -> NodePath {
        NodePath { path: Vec::default(), node_name: String::default(), status: NodePathStatus::INVALID(reason.into()) }
    }
}

impl From<&str> for NodePath {
    fn from(string: &str) -> Self {
        if string.contains('.') {
            return NodePath::return_invalid("Path cannot be relative.")
        }
        let mut path = string.split(|c| c == '/' || c == '\\').map(|string| string.to_string()).collect::<Vec<String>>();
        let mut node_name:String = String::new();
        match path.pop() {
            Some(name) => {
                node_name = String::from(name);
            },
            None => {
                return NodePath::return_invalid("Path not formatted correctly.")
            }
        }
        NodePath { path, node_name, status: NodePathStatus::VALID }
    }
}

impl Scene {
    pub fn filter_elements(elements:&Vec<Element>, element_type:ElementType) -> Vec<&Element> {
        elements.into_iter().filter(|element| element.element_type == element_type).collect::<Vec<&Element>>()
    }

    pub fn add_elements(&mut self, mut elements:Vec<Element>) {
        self.elements.append(&mut elements);
    }

    pub fn get_node_property(&self, node_path:NodePath, property_name:&str) -> Result<String, NodePathError> {
        let mut parent:String = if node_path.path.is_empty() { ".".into() } else { node_path.path.concat() };
        
        for node in Scene::filter_elements(&self.elements, ElementType::NODE).iter() {
            if let Ok(node_parent) = node.get_data_value("parent") {
                if node_parent == parent {
                    if let Ok(node_name) = node.get_data_value("name") {
                        if node_name == node_path.node_name {
                            println!("Found node!");
                        }
                    }
                }
            }
        }
        Ok(String::new())
    }

    pub fn to_tscn(self) -> String {
        let mut tokens:Vec<Token> = Vec::new();
        for mut element in self.elements.into_iter() {
            element.force_update_tokens();
            tokens.append(&mut element.tokens);
        }
        Tokenizer::reconstruct_tscn_from_tokens(tokens)
    }

    pub fn from_tscn_file(file_path:&str) -> Result<Self, SceneError> {
        let r = loader::load(file_path)?;
        match Tokenizer::tokenize(r.0, r.1) {
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