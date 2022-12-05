use super::chartree::CharTree;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Classifier {
    models: HashMap<String, CharTree>,
}

impl Classifier {
    fn score(&self, model: &str, input: &str) -> f64 {
        let model = self.models.get(model).unwrap();
        let weirdness = model.get_weirdness(input);
        let avg = weirdness.iter().sum::<u32>() as f64 / weirdness.len() as f64;
        avg / (model.count as f64).log10()
    }

    fn classify(&self, input: &str) -> &str {
        self.models
            .keys()
            .map(|name| (name, self.score(name, input)))
            .max_by(|(_, a), (_, b)| a.total_cmp(&b))
            .unwrap()
            .0
    }

    fn train<S: ToString>(&mut self, model: &str, data: Vec<S>) {
        let model = self.models.get_mut(model).unwrap();
        data.iter().for_each(|s| model.train(s.to_string()));
    }
}
