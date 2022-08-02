use std::env;
use std::fs;

use crate::scene::{ Scene, SceneError };

pub fn load(file_path:&str) -> Result<Scene, SceneError> {
    match fs::read_to_string(file_path) {
        Ok(file_content) => {
            Scene::from_tscn_content(&file_content)
        },
        Err(file_error) => {
            Err(SceneError::LoadFailed(file_error))
        }
    }
}