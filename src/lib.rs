mod loader;
mod tokenizer;
mod scene;

#[cfg(test)]
mod tests {
    use crate::{scene::Scene, loader};

    #[test]
    fn tokenize() {
        println!("{:#?}", loader::load(r"C:\Users\jordi\Projects\tscn\src\test.tscn"));
    }
}
