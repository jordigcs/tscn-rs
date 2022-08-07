
use std::{fs::{self, File}, io::{BufReader, Read}};

use crate::scene::{ Scene, SceneError };

// Returns Ok((reader, line_count)) or Err(SceneError)
pub fn load(file_path:&str) -> Result<(BufReader<File>, usize), SceneError> {
    let f = File::open(file_path);
    match f {
        Ok(mut file) => {
            // let mut string = String::new();
            // println!("string {}", string);
            // file.read_to_string(&mut string);
            // println!("string {}", string);
            // println!("f {}", string);
            return Ok((BufReader::new(file), 342));
        },
        Err(file_error) => {
            Err(SceneError::LoadFailed(file_error))
        }
    }
}