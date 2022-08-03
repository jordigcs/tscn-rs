use crate::tokenizer::Token;


#[derive(Debug, Clone)]
pub struct Property();
#[derive(Debug, Clone)]
pub struct ElementData(pub String, pub String); // 0: Name, 1: Value

#[derive(Debug)]
pub enum ExpectedType {
    ElementName,
    ElementDataName,
    ElementDataValue,
    PropertyName,
    PropertyValue,
}

#[derive(Debug, Clone)]
pub struct Element {
    pub element_name:String,
    pub element_type:ElementType,
    pub element_data:Vec<ElementData>,
    pub properties:Vec<Property>,
    pub tokens:Vec<Token>,
}

impl Element {
    pub fn empty() -> Self {
        Element { element_name: String::from("Undefined"), element_type:ElementType::UNKOWN, element_data: Vec::new(), properties: Vec::new(), tokens: Vec::new() }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ElementType {
    UNKOWN,
    SCENE_DATA,
    RESOURCE,
    NODE,
}