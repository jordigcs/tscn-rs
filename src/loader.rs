
use std::{fs::{File}, io::{BufReader, BufRead}};

use crate::scene::{ SceneError };

// Returns Ok((reader, line_count)) or Err(SceneError)
pub fn load(file_path:&str) -> Result<(BufReader<File>, usize), SceneError> {
    let f = File::open(file_path);
    match f {
        Ok(file) => {
            // Unfortunately we need to open the file twice in order to get the line count without consuming the file ref.
            let line_count = BufReader::new(
                    File::open(file_path).expect("An unexpected error occured during file loading.")
                )
                .lines()
                .count();
            Ok((BufReader::new(file), line_count))
        },
        Err(file_error) => {
            Err(SceneError::LoadFailed(file_error))
        }
    }
}