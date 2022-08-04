mod loader;
mod tokenizer;
mod scene;
mod element;

#[cfg(test)]
mod tests {
    use crate::{loader};

    #[test]
    fn tokenize() {
        let scene = loader::load(r"C:\Users\jordi\Projects\tscn\src\test.tscn");
        if let Ok(mut sc) = scene {
            println!("Tokens\n{:#?}", sc.elements[0].tokens);
            sc.elements[0].element_data[0].1 = String::from("Test");
            sc.elements[0].update_tokens();
            println!("Updated Tokens\n{:#?}", sc.elements[0].tokens);
            println!("RECONSTRUCT!\n{}", sc.to_tscn());
        }
    }
}
