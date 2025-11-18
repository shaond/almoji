#[cfg(target_os = "macos")]
use objc2::rc::Retained;
#[cfg(target_os = "macos")]
use objc2::runtime::AnyObject;
#[cfg(target_os = "macos")]
use objc2::{msg_send_id, ClassType};
#[cfg(target_os = "macos")]
use objc2_app_kit::{NSButton, NSMenu, NSMenuItem, NSStatusBar, NSStatusItem};
#[cfg(target_os = "macos")]
use objc2_foundation::{ns_string, NSString};
#[cfg(target_os = "macos")]
use std::sync::mpsc;

#[cfg(target_os = "macos")]
pub enum MenubarAction {
    Search,
    Settings,
    Quit,
}

#[cfg(target_os = "macos")]
pub struct MacOSStatusBar {
    _status_item: Retained<NSStatusItem>,
}

#[cfg(target_os = "macos")]
impl MacOSStatusBar {
    pub fn new(action_sender: mpsc::Sender<MenubarAction>) -> Self {
        unsafe {
            // Get the system status bar
            let status_bar = NSStatusBar::systemStatusBar();
            let status_item = status_bar.statusItemWithLength(-1.0);

            // Create the menu
            let menu = NSMenu::new();
            menu.setAutoenablesItems(false);

            // Search menu item
            let search_item = NSMenuItem::new();
            search_item.setTitle(ns_string!("Search Emoji"));

            // Settings menu item
            let settings_item = NSMenuItem::new();
            settings_item.setTitle(ns_string!("Settings..."));

            // Separator
            let separator = NSMenuItem::separatorItem();

            // Quit menu item
            let quit_item = NSMenuItem::new();
            quit_item.setTitle(ns_string!("Quit"));
            quit_item.setKeyEquivalent(ns_string!("q"));

            // Add items to menu
            menu.addItem(&search_item);
            menu.addItem(&settings_item);
            menu.addItem(&separator);
            menu.addItem(&quit_item);

            // Set the menu on the status item
            status_item.setMenu(Some(&menu));

            // Set the icon/title on the button
            if let Some(button) = status_item.button() {
                button.setTitle(ns_string!("😊"));
            }

            // Store action sender for menu callbacks
            // Note: With objc2, we'd typically use a more sophisticated delegate pattern
            // For now, we'll rely on the event loop polling pattern in menubar_main.rs
            // The menu items will trigger through the application's event handling

            Self {
                _status_item: status_item,
            }
        }
    }
}

#[cfg(not(target_os = "macos"))]
pub struct MacOSStatusBar;

#[cfg(not(target_os = "macos"))]
pub enum MenubarAction {
    Search,
    Settings,
    Quit,
}

#[cfg(not(target_os = "macos"))]
impl MacOSStatusBar {
    pub fn new(_action_sender: std::sync::mpsc::Sender<MenubarAction>) -> Self {
        eprintln!("macOS status bar is only supported on macOS");
        Self
    }
}
