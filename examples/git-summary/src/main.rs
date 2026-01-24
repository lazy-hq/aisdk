use std::collections::HashMap;

use aisdk::core::LanguageModelRequest;
use aisdk::prompt;
use aisdk::providers::OpenAI;

mod git_tools {
    use aisdk::core::tools::Tool;
    use aisdk::macros::tool;

    /// Returns the git diff from two commits or branches
    /// # Arguments
    /// * `left` - The left side of the diff
    /// * `right` - The right side of the diff
    ///
    /// # Examples
    /// ```rust
    /// use aisdk::core::tools::Tool;
    /// use aisdk::macros::tool;
    ///
    /// // an example repository where the diff between HEAD and master is
    /// // the following
    /// //
    /// // ```diff
    /// //  fn main() {
    /// // -    println!("Hello, world!");
    /// // +    println!("Hello, aisdk!");
    /// //      println!("Hello, world!");
    /// //  }
    /// // ```
    /// let diff = diff("HEAD", "master");
    ///
    /// assert_eq!(diff, "diff --git a/src/main.rs b/src/main.rs\n index 0e0e7d0..f2b0b3b 100644\n --- a/src/main.rs\n +++ b/src/main.rs\n @@ -1,3 +1,3 @@\n  fn main() {\n-    println!(\"Hello, world!\");\n+    println!(\"Hello, aisdk!\");\n     println!(\"Hello, world!\");\n }\n\\ No newline at end of file\n");
    /// ```
    ///
    /// # Returns
    /// The git diff between the two commits or branches
    #[tool]
    pub fn diff(left: String, right: String) -> Tool {
        let output = std::process::Command::new("git")
            .arg("diff")
            .arg(&left)
            .arg(&right)
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    Err(String::from_utf8_lossy(&output.stderr).to_string())
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }

    /// Returns the git status for the current repository
    ///
    /// # Arguments
    /// * `repository` - The repository to get the status for
    ///
    /// # Examples
    /// ```rust
    /// use aisdk::core::tools::Tool;
    /// use aisdk::macros::tool;
    ///
    /// // an example repository where the diff between HEAD and master is
    /// // the following
    /// //
    /// // ```diff
    /// //  fn main() {
    /// // -    println!("Hello, world!");
    /// // +    println!("Hello, aisdk!");
    /// //      println!("Hello, world!");
    /// //  }
    /// // ```
    /// let status = status("https://github.com/surafat/aisdk.git");
    ///
    /// assert_eq!(status, "M src/main.rs");
    /// ```
    ///
    /// # Returns
    /// The git status for the current repository
    #[tool]
    pub fn status() -> Tool {
        let output = std::process::Command::new("git")
            .arg("status")
            .arg("--porcelain")
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    Err(String::from_utf8_lossy(&output.stderr).to_string())
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }

    /// Returns the git log for the current repository
    /// # Arguments
    /// * `repository` - The repository to get the log for
    ///
    /// # Examples
    /// ```rust
    /// use aisdk::core::tools::Tool;
    /// use aisdk::macros::tool;
    ///
    /// // an example repository where the diff between HEAD and master is
    /// // the following
    /// //
    /// // ```diff
    /// //  fn main() {
    /// // -    println!("Hello, world!");
    /// // +    println!("Hello, aisdk!");
    /// //      println!("Hello, world!");
    /// //  }
    /// // ```
    /// let log = log("https://github.com/surafat/aisdk.git");
    ///
    /// assert_eq!(log, "commit 1234567890\nAuthor: Surafel\nDate:   Sat Jan 20 10:12:00 2023 +0530\n\n    Initial commit");
    /// ```
    ///
    /// # Returns
    /// The git log for the current repository
    #[tool]
    pub fn log() -> Tool {
        let output = std::process::Command::new("git")
            .arg("log")
            .arg("-1")
            .arg("--format=%B")
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    Err(String::from_utf8_lossy(&output.stderr).to_string())
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let prompt_env = prompt::PromptEnv::default();
    let system_prompt = prompt_env
        .generate_prompt("system.md", HashMap::default())
        .unwrap();

    let text = LanguageModelRequest::builder()
        .model(OpenAI::gpt_5_nano())
        .system(system_prompt)
        .prompt("What is the previous commit message and what changes does the most recent commit message do to it?") // TODO: add a prompt from cli input
        .with_tool(git_tools::diff())
        .with_tool(git_tools::status())
        .with_tool(git_tools::log())
        .build()
        .generate_text()
        .await
        .unwrap();

    println!("{}", text.text().unwrap());
}
