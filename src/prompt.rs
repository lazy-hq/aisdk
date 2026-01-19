//! Prompt template processing module.
//!
//! This module provides utilities for managing and rendering prompt templates
//! using the Tera templating engine.

use crate::error::Error;
use derive_builder::Builder;
use std::{collections::HashMap, path::PathBuf};
use tera::Tera;

/// A lightweight wrapper around the Tera templating engine.
pub struct PromptEnv {
    template: Tera,
}

impl Default for PromptEnv {
    /// Creates a new prompt environment from in ./prompts/**/*.
    fn default() -> Self {
        PromptEnv::new(PathBuf::from("prompts/**/*"))
    }
}

impl PromptEnv {
    /// Creates a new prompt environment from `name/**/*`
    pub fn new(path: PathBuf) -> Self {
        PromptEnv {
            template: Tera::new(&format!("{}/**/*", path.display()))
                .expect("failed to load templates to tera"),
        }
    }
}

/// A prompt template with variables for rendering.
#[derive(Builder)]
#[builder(pattern = "owned", setter(into), build_fn(error = "Error"))]
#[allow(missing_docs)]
pub struct Prompt {
    path: String,
    variables: HashMap<String, String>,
}

impl Prompt {
    /// Creates a new prompt builder.
    pub fn builder() -> PromptBuilder {
        PromptBuilder::default()
    }
}

impl PromptBuilder {
    /// Adds a variable to the prompt template.
    pub fn with_variable(mut self, key: String, value: String) -> Self {
        if let Some(vars) = self.variables.as_mut() {
            vars.insert(key, value);
        } else {
            self.variables = Some(HashMap::from([(key, value)]));
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_env_creation_with_examples_path() {
        // Test creating a PromptEnv with the examples/prompts path
        let path = PathBuf::from("examples/prompts");
        let env = PromptEnv::new(path);

        // Verify the environment is created successfully
        assert!(
            !env.template
                .get_template_names()
                .collect::<Vec<_>>()
                .is_empty(),
            "PromptEnv should have loaded templates from examples/prompts"
        );

        // Test with a subdirectory path
        let path = PathBuf::from("examples/prompts/system");
        let env = PromptEnv::new(path);

        // Verify templates are loaded from the subdirectory
        let template_names: Vec<&str> = env.template.get_template_names().collect();
        assert!(
            !template_names.is_empty(),
            "PromptEnv should have loaded templates from examples/prompts/system"
        );

        // Test with user prompts subdirectory
        let path = PathBuf::from("examples/prompts/user");
        let env = PromptEnv::new(path);

        let template_names: Vec<&str> = env.template.get_template_names().collect();
        assert!(
            !template_names.is_empty(),
            "PromptEnv should have loaded templates from examples/prompts/user"
        );
    }
}
