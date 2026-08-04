use crossterm::style::Color;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Theme {
    pub name: String,
    pub layout: String,
    pub prompt_symbol: String,

    pub show_user: bool,
    pub show_hostname: bool,
    pub show_directory: bool,
    pub show_git: bool,
    pub show_time: bool,

    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub background: String,
    pub foreground: String,
    pub directory: String,
    pub git: String,
    pub time: String,
    pub success: String,
    pub error: String,
    pub separator: String,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            name: "Astra Dark".into(),
            layout: "power".into(),
            prompt_symbol: "❯".into(),

            show_user: true,
            show_hostname: true,
            show_directory: true,
            show_git: true,
            show_time: true,

            primary: "#8AB4F8".into(),
            secondary: "#5A6BDC".into(),
            accent: "#C792EA".into(),
            background: "#0E1117".into(),
            foreground: "#E8EAED".into(),
            directory: "#7C9EFF".into(),
            git: "#82CFF3".into(),
            time: "#B0B9C1".into(),
            success: "#7DD47D".into(),
            error: "#F04438".into(),
            separator: "│".into(),
        }
    }
}

impl Theme {
    pub fn parse_color(&self, color: &str) -> Color {
        let text = color.trim().trim_start_matches('#');
        if text.len() == 6 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&text[0..2], 16),
                u8::from_str_radix(&text[2..4], 16),
                u8::from_str_radix(&text[4..6], 16),
            ) {
                return Color::Rgb { r, g, b };
            }
        }

        match text.to_lowercase().as_str() {
            "black" => Color::Black,
            "white" => Color::White,
            "red" => Color::DarkRed,
            "green" => Color::DarkGreen,
            "blue" => Color::DarkBlue,
            "yellow" => Color::Yellow,
            "cyan" => Color::Cyan,
            "magenta" => Color::Magenta,
            _ => Color::White,
        }
    }

    pub fn primary_color(&self) -> Color {
        self.parse_color(&self.primary)
    }

    pub fn secondary_color(&self) -> Color {
        self.parse_color(&self.secondary)
    }

    pub fn accent_color(&self) -> Color {
        self.parse_color(&self.accent)
    }

    pub fn directory_color(&self) -> Color {
        self.parse_color(&self.directory)
    }

    pub fn git_color(&self) -> Color {
        self.parse_color(&self.git)
    }

    pub fn time_color(&self) -> Color {
        self.parse_color(&self.time)
    }
}

pub fn load(name: &str) -> Theme {
    let canonical = name.trim().to_ascii_lowercase();

    let source = match canonical.as_str() {
        "crimson" => include_str!("../themes/crimson.toml"),
        "midnight" => include_str!("../themes/midnight.toml"),
        "matrix" => include_str!("../themes/matrix.toml"),
        "obsidian" => include_str!("../themes/obsidian.toml"),
        "light" => include_str!("../themes/light.toml"),
        "dark" => include_str!("../themes/dark.toml"),
        "minimal" => include_str!("../themes/minimal.toml"),
        _ => include_str!("../themes/default.toml"),
    };

    toml::from_str(source).unwrap_or_default()
}
