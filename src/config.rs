use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub repo: String,
    pub author: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const BASIC: &str = indoc! {r#"
        repo = "https://github.com/MyAccount/snatches"
        author = "dzejkop <jakubtrad@gmail.com>"
    "#};

    #[test]
    fn basic() {
        toml::from_str::<Config>(BASIC).unwrap();
    }
}
