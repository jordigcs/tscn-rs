mod loader;
mod tokenizer;
mod scene;
mod element;

#[cfg(test)]
mod tests {
    use crate::{scene::{NodePath, Scene}};

    #[test]
    fn tokenize() {
        let scene = Scene::from_tscn_file(r"./src/test.tscn");
        if let Ok(mut sc) = scene {
            println!("Tokens\n{:#?}", sc.elements[0].tokens);
            sc.elements[0].update_data_by_index(0, r#""Test""#);
            println!("Updated Tokens\n{:#?}", sc.elements[0].tokens);
            println!("{:#?}", sc.get_node_property(NodePath::from("Tree/StaticBody2D/CollisionShape2D"), "test"));
        }
    }
}
