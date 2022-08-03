mod loader;
mod tokenizer;
mod scene;
mod element;

#[cfg(test)]
mod tests {
    use crate::{scene::Scene, loader, tokenizer::Tokenizer};

    #[test]
    fn tokenize() {
        let scene = loader::load(r"C:\Users\jordi\Projects\tscn\src\test.tscn");
        if let Ok(sc) = scene {
            println!("SCENE!\n{:?}", sc.scene_data);
            println!("RECONSTRUCT!\n{}", Tokenizer::reconstruct_tscn_from_tokens(sc.tokenizer.tokens));
        }
    }
}
