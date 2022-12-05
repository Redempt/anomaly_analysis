use std::collections::HashMap;
use super::chartree::CharTree;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Classifier {
    names: HashMap<String, usize>,
    models: Vec<CharTree>
}