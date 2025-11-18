use eframe::egui;
use crate::emoji_search;
use crate::menubar::settings::{Settings, HotkeyConfig, Modifier, Key};

pub struct SearchWindow {
    search_query: String,
    search_results: Vec<(String, String)>, // (emoji, description)
    selected_index: Option<usize>,
}

impl SearchWindow {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            search_results: Vec::new(),
            selected_index: None,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) -> Option<String> {
        let mut selected_emoji = None;
        let mut should_close = false;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Almoji - Emoji Search");
            ui.add_space(10.0);

            // Search input
            let response = ui.add(
                egui::TextEdit::singleline(&mut self.search_query)
                    .hint_text("Search for emoji...")
                    .desired_width(f32::INFINITY)
            );

            // Auto-focus the search box
            if !response.has_focus() {
                response.request_focus();
            }

            // Handle keyboard input
            if response.changed() {
                self.perform_search();
                self.selected_index = if !self.search_results.is_empty() {
                    Some(0)
                } else {
                    None
                };
            }

            // Handle escape key
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                should_close = true;
            }

            // Handle enter key
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if let Some(idx) = self.selected_index {
                    if idx < self.search_results.len() {
                        selected_emoji = Some(self.search_results[idx].0.clone());
                        should_close = true;
                    }
                }
            }

            // Handle arrow keys
            if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                if let Some(idx) = self.selected_index {
                    if idx + 1 < self.search_results.len() {
                        self.selected_index = Some(idx + 1);
                    }
                } else if !self.search_results.is_empty() {
                    self.selected_index = Some(0);
                }
            }

            if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                if let Some(idx) = self.selected_index {
                    if idx > 0 {
                        self.selected_index = Some(idx - 1);
                    }
                } else if !self.search_results.is_empty() {
                    self.selected_index = Some(self.search_results.len() - 1);
                }
            }

            ui.add_space(10.0);

            // Results display
            egui::ScrollArea::vertical()
                .max_height(400.0)
                .show(ui, |ui| {
                    if self.search_results.is_empty() && !self.search_query.is_empty() {
                        ui.label("No results found");
                    } else {
                        for (i, (emoji, description)) in self.search_results.iter().enumerate() {
                            let is_selected = self.selected_index == Some(i);

                            let button = egui::Button::new(
                                egui::RichText::new(format!("{} {}", emoji, description))
                                    .size(18.0)
                            );

                            let button = if is_selected {
                                button.fill(egui::Color32::from_rgb(100, 100, 150))
                            } else {
                                button
                            };

                            if ui.add_sized([ui.available_width(), 35.0], button).clicked() {
                                selected_emoji = Some(emoji.clone());
                                should_close = true;
                            }
                        }
                    }
                });

            if !self.search_query.is_empty() {
                ui.add_space(10.0);
                ui.label(format!("{} results", self.search_results.len()));
            }
        });

        if should_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        selected_emoji
    }

    fn perform_search(&mut self) {
        if self.search_query.is_empty() {
            self.search_results.clear();
            return;
        }

        let results = emoji_search::search_emojis(&self.search_query, 50);
        self.search_results = results
            .into_iter()
            .map(|(keyword, emoji)| {
                let emoji_str = emoji_search::get_emoji_string(&keyword, emoji);
                let description = if keyword.starts_with("__raw__:") {
                    emoji.name().to_string()
                } else {
                    keyword
                };
                (emoji_str, description)
            })
            .collect();
    }

    pub fn reset(&mut self) {
        self.search_query.clear();
        self.search_results.clear();
        self.selected_index = None;
    }
}

pub struct SettingsWindow {
    settings: Settings,
    temp_hotkey: HotkeyConfig,
    selected_modifiers: Vec<bool>,
}

impl SettingsWindow {
    pub fn new(settings: Settings) -> Self {
        let temp_hotkey = settings.hotkey.clone();
        let selected_modifiers = Modifier::all()
            .iter()
            .map(|m| temp_hotkey.modifiers.contains(m))
            .collect();

        Self {
            settings,
            temp_hotkey,
            selected_modifiers,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) -> Option<Settings> {
        let mut new_settings = None;
        let mut should_close = false;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Almoji Settings");
            ui.add_space(20.0);

            ui.label("Hotkey Configuration:");
            ui.add_space(10.0);

            // Modifier checkboxes
            ui.horizontal(|ui| {
                ui.label("Modifiers:");
                for (i, modifier) in Modifier::all().iter().enumerate() {
                    if ui.checkbox(&mut self.selected_modifiers[i], modifier.to_string()).changed() {
                        self.update_modifiers();
                    }
                }
            });

            ui.add_space(10.0);

            // Key selection
            ui.horizontal(|ui| {
                ui.label("Key:");
                egui::ComboBox::from_label("")
                    .selected_text(self.temp_hotkey.key.to_string())
                    .show_ui(ui, |ui| {
                        for key in Key::all() {
                            ui.selectable_value(&mut self.temp_hotkey.key, key, key.to_string());
                        }
                    });
            });

            ui.add_space(10.0);

            // Display current hotkey
            ui.horizontal(|ui| {
                ui.label("Current hotkey:");
                ui.label(
                    egui::RichText::new(self.temp_hotkey.display_string())
                        .size(16.0)
                        .strong()
                );
            });

            ui.add_space(20.0);

            // Buttons
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    let mut updated_settings = self.settings.clone();
                    updated_settings.hotkey = self.temp_hotkey.clone();

                    if let Err(e) = updated_settings.save() {
                        eprintln!("Failed to save settings: {}", e);
                    } else {
                        new_settings = Some(updated_settings);
                        should_close = true;
                    }
                }

                if ui.button("Cancel").clicked() {
                    should_close = true;
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.label("About Almoji:");
            ui.label("A blazingly fast emoji search tool for macOS");
            ui.label("Press the hotkey to open emoji search");
            ui.label("Use arrow keys to navigate, Enter to select");
        });

        if should_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        new_settings
    }

    fn update_modifiers(&mut self) {
        self.temp_hotkey.modifiers = Modifier::all()
            .into_iter()
            .enumerate()
            .filter(|(i, _)| self.selected_modifiers[*i])
            .map(|(_, m)| m)
            .collect();
    }
}
