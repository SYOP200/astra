use std::collections::HashMap;

pub struct AliasManager {
    aliases: HashMap<String, String>,
}

impl AliasManager {
    pub fn new(aliases: HashMap<String, String>) -> Self {
        Self { aliases }
    }

    pub fn expand(&self, command: &str) -> String {
        let parts: Vec<&str> = command.split_whitespace().collect();

        if parts.is_empty() {
            return command.into();
        }

        match self.aliases.get(parts[0]) {
            Some(value) => {
                let rest = parts[1..].join(" ");

                if rest.is_empty() {
                    value.clone()
                } else {
                    format!("{} {}", value, rest)
                }
            }

            None => command.into(),
        }
    }
}
