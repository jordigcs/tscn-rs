use crate::tokenizer::Token;


#[derive(Debug, Clone)]
pub struct Property(pub String, pub String);
impl Property {
    pub fn to_tokens(&self) -> [Token;2] {
        [Token::PropertyName(Some(self.0.clone())), Token::PropertyValue(Some(self.1.clone()))]
    }
}

#[derive(Debug, Clone)]
pub struct ElementData(pub String, pub String); // 0: Name, 1: Value
impl ElementData {
    pub fn to_tokens(&self) -> [Token;2] {
        [Token::ElementDataName(Some(self.0.clone())), Token::ElementDataValue(Some(self.1.clone()))]
    }
}

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
    
    pub fn force_update_tokens(&mut self) {
        let mut tokens:Vec<Token> = vec![Token::BracketLeft, Token::ElementName(Some(self.element_name.clone()))]; // Elements start with [element_name
        
        // Append ElementData tokens.
        tokens.append(
            &mut self.element_data.iter().flat_map(|element_data| {
                element_data.to_tokens().to_vec()
            }).collect()
        );
        // Close element
        tokens.push(Token::BracketRight);
        tokens.push(Token::NewLine);
        if !self.properties.is_empty() {
            // Append property tokens
            tokens.append(
                &mut self.properties.iter().flat_map(|property| {
                    let mut v = property.to_tokens().to_vec();
                    v.push(Token::NewLine);
                    v
                }).collect()
            );
            tokens.push(Token::NewLine);
        }

        // Update complete.
        self.tokens = tokens;
    }

    pub fn get_data_value(&self, data_name:&str) -> Result<String, ()> {
        for data in self.element_data.iter() {
            if data.0 == data_name {
                return Ok(data.1.clone());
            }
        }
        Err(())
    }

    pub fn update_data(&mut self, data_name:&str, new_value:&str) -> Result<(), ()> {
        let mut found = false;
        self.element_data = self.element_data.iter().map(|data| { 
            if data.0 == data_name {
                found = true;
                return ElementData(data.0.clone(), String::from(new_value));
            }
            data.clone()
        }).collect::<Vec<ElementData>>();
        if found {
            self.force_update_tokens();
            return Ok(());
        }
        Err(())
    }

    pub fn update_data_by_index(&mut self, index:usize, new_value:&str) -> Result<(), ()> {
        if let Some(data) = self.element_data.get_mut(index) {
            data.1 = String::from(new_value);
            self.force_update_tokens();
            return Ok(());
        }
        Err(())
    }

    pub fn update_property(&mut self, property_name:&str, new_value:&str) -> Result<(), ()> {
        let mut found = false;
        self.properties = self.properties.iter().map(|prop| { 
            if prop.0 == property_name {
                found = true;
                return Property(prop.0.clone(), String::from(new_value));
            }
            prop.clone()
        }).collect::<Vec<Property>>();
        if found {
            self.force_update_tokens();
            return Ok(());
        }
        Err(())
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ElementType {
    UNKOWN,
    SCENE_DATA,
    RESOURCE,
    NODE,
}