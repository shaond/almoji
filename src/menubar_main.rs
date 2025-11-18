mod emoji_search;
mod menubar;

use eframe::egui;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, hotkey::{Code, HotKey, Modifiers}};
use menubar::{MacOSStatusBar, MenubarAction, SearchWindow, SettingsWindow, Settings};
use std::sync::mpsc::{channel, Receiver};
use arboard::Clipboard;

struct AppState {
    settings: Settings,
    hotkey_manager: GlobalHotKeyManager,
    current_hotkey_id: Option<u32>,
    menubar_rx: Receiver<MenubarAction>,
    _statusbar: MacOSStatusBar,
}

impl AppState {
    fn new() -> Self {
        let settings = Settings::load();
        let hotkey_manager = GlobalHotKeyManager::new().expect("Failed to create hotkey manager");

        let (menubar_tx, menubar_rx) = channel();
        let _statusbar = MacOSStatusBar::new(menubar_tx);

        let mut state = Self {
            settings,
            hotkey_manager,
            current_hotkey_id: None,
            menubar_rx,
            _statusbar,
        };

        state.register_hotkey();
        state
    }

    fn register_hotkey(&mut self) {
        // Unregister old hotkey if exists
        if let Some(_id) = self.current_hotkey_id {
            let hotkey = HotKey::new(None, Code::KeyE);
            let _ = self.hotkey_manager.unregister(hotkey);
        }

        // Register new hotkey
        let mut modifiers = Modifiers::empty();

        for modifier in &self.settings.hotkey.modifiers {
            match modifier {
                menubar::Modifier::Ctrl => modifiers |= Modifiers::CONTROL,
                menubar::Modifier::Alt => modifiers |= Modifiers::ALT,
                menubar::Modifier::Shift => modifiers |= Modifiers::SHIFT,
                menubar::Modifier::Meta => modifiers |= Modifiers::SUPER,
            }
        }

        let code = match self.settings.hotkey.key {
            menubar::Key::Space => Code::Space,
            menubar::Key::E => Code::KeyE,
            menubar::Key::F => Code::KeyF,
            menubar::Key::G => Code::KeyG,
            menubar::Key::H => Code::KeyH,
            menubar::Key::I => Code::KeyI,
            menubar::Key::J => Code::KeyJ,
            menubar::Key::K => Code::KeyK,
            menubar::Key::L => Code::KeyL,
            menubar::Key::M => Code::KeyM,
            menubar::Key::N => Code::KeyN,
            menubar::Key::O => Code::KeyO,
            menubar::Key::P => Code::KeyP,
            menubar::Key::Q => Code::KeyQ,
            menubar::Key::R => Code::KeyR,
            menubar::Key::S => Code::KeyS,
            menubar::Key::T => Code::KeyT,
            menubar::Key::U => Code::KeyU,
            menubar::Key::V => Code::KeyV,
            menubar::Key::W => Code::KeyW,
            menubar::Key::X => Code::KeyX,
            menubar::Key::Y => Code::KeyY,
            menubar::Key::Z => Code::KeyZ,
            menubar::Key::Escape => Code::Escape,
        };

        let hotkey = HotKey::new(Some(modifiers), code);

        match self.hotkey_manager.register(hotkey) {
            Ok(_) => {
                self.current_hotkey_id = Some(hotkey.id());
                println!("Registered hotkey: {}", self.settings.hotkey.display_string());
            }
            Err(e) => {
                eprintln!("Failed to register hotkey: {}", e);
            }
        }
    }

    fn update_settings(&mut self, new_settings: Settings) {
        self.settings = new_settings;
        self.register_hotkey();
    }
}

fn main() {
    let mut app_state = AppState::new();

    println!("Almoji menubar app started!");
    println!("Press {} to search for emojis", app_state.settings.hotkey.display_string());

    // Main event loop
    loop {
        // Check for menubar actions
        if let Ok(action) = app_state.menubar_rx.try_recv() {
            match action {
                MenubarAction::Search => {
                    show_search_window();
                }
                MenubarAction::Settings => {
                    if let Some(new_settings) = show_settings_window(app_state.settings.clone()) {
                        app_state.update_settings(new_settings);
                    }
                }
                MenubarAction::Quit => {
                    println!("Quitting Almoji...");
                    break;
                }
            }
        }

        // Check for hotkey events
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            if Some(event.id) == app_state.current_hotkey_id {
                show_search_window();
            }
        }

        // Sleep to avoid busy-waiting
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

fn show_search_window() {
    std::thread::spawn(|| {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([600.0, 500.0])
                .with_resizable(true)
                .with_always_on_top()
                .with_decorations(true),
            ..Default::default()
        };

        let _ = eframe::run_simple_native("Almoji Search", options, move |ctx, _frame| {
            let mut search_window = SearchWindow::new();

            // Main render loop
            if let Some(selected_emoji) = search_window.show(ctx) {
                // Copy to clipboard
                if let Ok(mut clipboard) = Clipboard::new() {
                    if let Err(e) = clipboard.set_text(&selected_emoji) {
                        eprintln!("Failed to copy to clipboard: {}", e);
                    } else {
                        println!("Copied emoji to clipboard: {}", selected_emoji);
                    }
                }
            }
        });
    });
}

fn show_settings_window(current_settings: Settings) -> Option<Settings> {
    let (tx, rx) = channel();

    std::thread::spawn(move || {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([500.0, 400.0])
                .with_resizable(false)
                .with_decorations(true),
            ..Default::default()
        };

        let _ = eframe::run_simple_native("Almoji Settings", options, move |ctx, _frame| {
            let mut settings_window = SettingsWindow::new(current_settings.clone());

            if let Some(new_settings) = settings_window.show(ctx) {
                let _ = tx.send(new_settings);
            }
        });
    });

    // Wait a bit for the window to potentially send settings
    std::thread::sleep(std::time::Duration::from_millis(100));

    rx.try_recv().ok()
}
