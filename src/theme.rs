use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Theme {
    pub name: String,
    pub prompt_symbol: String,

    pub show_user: bool,
    pub show_hostname: bool,
    pub show_directory: bool,
    pub show_git: bool,
    pub show_time: bool,
}


impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "Crimson".into(),
            prompt_symbol: "❯".into(),

            show_user: true,
            show_hostname: true,
            show_directory: true,
            show_git: true,
            show_time: true,
        }
    }
}


pub fn load(name: &str) -> Theme {
    match name {

        "midnight" => Theme {
            name: "Midnight".into(),
            prompt_symbol: "❯".into(),

            show_user: true,
            show_hostname: true,
            show_directory: true,
            show_git: true,
            show_time: true,
        },


        "matrix" => Theme {
            name: "Matrix".into(),
            prompt_symbol: ">".into(),

            show_user: true,
            show_hostname: false,
            show_directory: true,
            show_git: true,
            show_time: false,
        },


        "obsidian" => Theme {
            name: "Obsidian".into(),
            prompt_symbol: "›".into(),

            show_user: false,
            show_hostname: false,
            show_directory: true,
            show_git: true,
            show_time: false,
        },


        _ => Theme::default(),
    }
}
