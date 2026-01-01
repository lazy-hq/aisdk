use aisdk::prompt::{Prompt, PromptEnvironment, Promptable};
use std::collections::HashMap;

pub struct ProductUnClassified {
    description: String,
}

/// A classification output and its confidence score
pub type Classification = (String, i32);

pub struct ProductClassified {
    name: String,
    description: String,
    sold: Option<bool>,
    price: Option<f32>,
    category: Classification,
    attribute_values: Vec<HashMap<String, Option<Classification>>>,
}

pub trait Classifier {
    fn classify(&self, product: &ProductUnClassified) -> Result<ProductClassified, reqwest::Error>;
}

#[derive(Debug, Default)]
pub struct CategoryClassifier {
    candidates: Vec<String>,
}

impl CategoryClassifier {
    pub fn candidates(mut self, candidates: Vec<String>) -> Self {
        self.candidates = candidates;
        self
    }

    pub fn add_candidate(mut self, candidate: String) -> Self {
        self.candidates.push(candidate);
        self
    }
}

impl Classifier for CategoryClassifier {
    fn classify(&self, product: &ProductUnClassified) -> Result<ProductClassified, reqwest::Error> {
        // Set up prompt environment pointing to the prompts directory
        // let env = PromptEnvironment::from_directory("examples/product-classifier/src/prompts");

        // Format categories as a comma-separated list
        let categories_str = self.candidates.join(", ");

        // Load and generate the prompt
        let prompt = Prompt::new("category")
            .with_extension("md")
            .with("text", &product.description)
            .with("categories", &categories_str)
            .generate();

        // Log the formatted prompt
        println!("Generated prompt:\n{}", prompt);

        todo!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_classifier() {
        let classifier = CategoryClassifier::default().add_candidate("Category 1".to_string());
        let product = ProductUnClassified {
            description: "Product description".to_string(),
        };
        let _ = classifier.classify(&product);
    }
}
