use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub hotkey: HotkeyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HotkeyConfig {
    pub modifiers: Vec<Modifier>,
    pub key: Key,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Modifier {
    Ctrl,
    Alt,
    Shift,
    Meta, // Command on macOS
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Key {
    Space,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Escape,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            hotkey: HotkeyConfig {
                modifiers: vec![Modifier::Meta, Modifier::Shift],
                key: Key::E,
            },
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        match Self::config_path() {
            Ok(path) => {
                if path.exists() {
                    match fs::read_to_string(&path) {
                        Ok(contents) => match serde_json::from_str(&contents) {
                            Ok(settings) => settings,
                            Err(e) => {
                                eprintln!("Failed to parse settings: {}, using defaults", e);
                                Self::default()
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed to read settings: {}, using defaults", e);
                            Self::default()
                        }
                    }
                } else {
                    Self::default()
                }
            }
            Err(e) => {
                eprintln!("Failed to get config path: {}, using defaults", e);
                Self::default()
            }
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create config dir: {}", e))?;
        }
        let contents =
            serde_json::to_string_pretty(self).map_err(|e| format!("Failed to serialize: {}", e))?;
        fs::write(&path, contents).map_err(|e| format!("Failed to write settings: {}", e))?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf, String> {
        let home = std::env::var("HOME").map_err(|_| "HOME not set".to_string())?;
        Ok(PathBuf::from(home)
            .join(".config")
            .join("almoji")
            .join("settings.json"))
    }
}

impl Modifier {
    pub fn to_string(&self) -> &'static str {
        match self {
            Modifier::Ctrl => "Ctrl",
            Modifier::Alt => "Alt",
            Modifier::Shift => "Shift",
            Modifier::Meta => if cfg!(target_os = "macos") { "Cmd" } else { "Win" },
        }
    }

    pub fn all() -> Vec<Modifier> {
        vec![Modifier::Ctrl, Modifier::Alt, Modifier::Shift, Modifier::Meta]
    }
}

impl Key {
    pub fn to_string(&self) -> &'static str {
        match self {
            Key::Space => "Space",
            Key::E => "E",
            Key::F => "F",
            Key::G => "G",
            Key::H => "H",
            Key::I => "I",
            Key::J => "J",
            Key::K => "K",
            Key::L => "L",
            Key::M => "M",
            Key::N => "N",
            Key::O => "O",
            Key::P => "P",
            Key::Q => "Q",
            Key::R => "R",
            Key::S => "S",
            Key::T => "T",
            Key::U => "U",
            Key::V => "V",
            Key::W => "W",
            Key::X => "X",
            Key::Y => "Y",
            Key::Z => "Z",
            Key::Escape => "Esc",
        }
    }

    pub fn all() -> Vec<Key> {
        vec![
            Key::Space,
            Key::E,
            Key::F,
            Key::G,
            Key::H,
            Key::I,
            Key::J,
            Key::K,
            Key::L,
            Key::M,
            Key::N,
            Key::O,
            Key::P,
            Key::Q,
            Key::R,
            Key::S,
            Key::T,
            Key::U,
            Key::V,
            Key::W,
            Key::X,
            Key::Y,
            Key::Z,
            Key::Escape,
        ]
    }
}

impl HotkeyConfig {
    pub fn display_string(&self) -> String {
        let mut parts: Vec<String> = self.modifiers.iter().map(|m| m.to_string().to_string()).collect();
        parts.push(self.key.to_string().to_string());
        parts.join("+")
    }
}
