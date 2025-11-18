#[cfg(target_os = "macos")]
use cocoa::appkit::{NSApp, NSApplication, NSMenu, NSMenuItem, NSStatusBar, NSStatusItem, NSWindow};
#[cfg(target_os = "macos")]
use cocoa::base::{id, nil};
#[cfg(target_os = "macos")]
use cocoa::foundation::{NSAutoreleasePool, NSString};
#[cfg(target_os = "macos")]
use objc::runtime::{Object, Sel};
#[cfg(target_os = "macos")]
use objc::{class, msg_send, sel, sel_impl};
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
    status_item: id,
    _pool: id,
}

#[cfg(target_os = "macos")]
impl MacOSStatusBar {
    pub fn new(action_sender: mpsc::Sender<MenubarAction>) -> Self {
        unsafe {
            let _pool = NSAutoreleasePool::new(nil);

            // Get the system status bar
            let status_bar = NSStatusBar::systemStatusBar(nil);
            let status_item = status_bar.statusItemWithLength_(-1.0); // NSVariableStatusItemLength

            // Create the menu
            let menu = NSMenu::new(nil);
            menu.setAutoenablesItems_(0); // Don't auto-enable/disable

            // Search menu item
            let search_title = NSString::alloc(nil).init_str("Search Emoji");
            let search_item = NSMenuItem::alloc(nil)
                .initWithTitle_action_keyEquivalent_(search_title, sel!(searchAction:), NSString::alloc(nil).init_str(""));

            // Settings menu item
            let settings_title = NSString::alloc(nil).init_str("Settings...");
            let settings_item = NSMenuItem::alloc(nil)
                .initWithTitle_action_keyEquivalent_(settings_title, sel!(settingsAction:), NSString::alloc(nil).init_str(""));

            // Separator
            let separator = NSMenuItem::separatorItem(nil);

            // Quit menu item
            let quit_title = NSString::alloc(nil).init_str("Quit");
            let quit_item = NSMenuItem::alloc(nil)
                .initWithTitle_action_keyEquivalent_(quit_title, sel!(quitAction:), NSString::alloc(nil).init_str("q"));

            // Set up the delegate for menu actions
            let delegate = create_menu_delegate(action_sender);
            let () = msg_send![search_item, setTarget: delegate];
            let () = msg_send![settings_item, setTarget: delegate];
            let () = msg_send![quit_item, setTarget: delegate];

            // Add items to menu
            menu.addItem_(search_item);
            menu.addItem_(settings_item);
            menu.addItem_(separator);
            menu.addItem_(quit_item);

            // Set the menu on the status item
            status_item.setMenu_(menu);

            // Set the icon/title
            let button: id = msg_send![status_item, button];
            let emoji_icon = NSString::alloc(nil).init_str("😊");
            let () = msg_send![button, setTitle: emoji_icon];

            Self {
                status_item,
                _pool,
            }
        }
    }
}

#[cfg(target_os = "macos")]
fn create_menu_delegate(sender: mpsc::Sender<MenubarAction>) -> id {
    use std::sync::Arc;
    use objc::declare::ClassDecl;
    use objc::runtime::{Class, Object};

    unsafe {
        // Check if class already exists
        let superclass = class!(NSObject);
        let mut decl = match ClassDecl::new("AlmojiMenuDelegate", superclass) {
            Some(decl) => decl,
            None => {
                // Class already registered, get it
                return create_delegate_instance(sender);
            }
        };

        // Add an ivar to store the sender
        decl.add_ivar::<usize>("_sender_ptr");

        extern "C" fn search_action(this: &Object, _cmd: Sel, _sender: id) {
            unsafe {
                let sender_ptr: usize = *this.get_ivar("_sender_ptr");
                if sender_ptr != 0 {
                    let sender = &*(sender_ptr as *const mpsc::Sender<MenubarAction>);
                    let _ = sender.send(MenubarAction::Search);
                }
            }
        }

        extern "C" fn settings_action(this: &Object, _cmd: Sel, _sender: id) {
            unsafe {
                let sender_ptr: usize = *this.get_ivar("_sender_ptr");
                if sender_ptr != 0 {
                    let sender = &*(sender_ptr as *const mpsc::Sender<MenubarAction>);
                    let _ = sender.send(MenubarAction::Settings);
                }
            }
        }

        extern "C" fn quit_action(this: &Object, _cmd: Sel, _sender: id) {
            unsafe {
                let sender_ptr: usize = *this.get_ivar("_sender_ptr");
                if sender_ptr != 0 {
                    let sender = &*(sender_ptr as *const mpsc::Sender<MenubarAction>);
                    let _ = sender.send(MenubarAction::Quit);
                }
            }
        }

        decl.add_method(
            sel!(searchAction:),
            search_action as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(settingsAction:),
            settings_action as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(quitAction:),
            quit_action as extern "C" fn(&Object, Sel, id),
        );

        decl.register();

        create_delegate_instance(sender)
    }
}

#[cfg(target_os = "macos")]
unsafe fn create_delegate_instance(sender: mpsc::Sender<MenubarAction>) -> id {
    use objc::runtime::Class;

    let class = Class::get("AlmojiMenuDelegate").unwrap();
    let delegate: id = msg_send![class, alloc];
    let delegate: id = msg_send![delegate, init];

    // Store the sender pointer in the delegate
    let sender_box = Box::new(sender);
    let sender_ptr = Box::into_raw(sender_box) as usize;
    (*delegate).set_ivar("_sender_ptr", sender_ptr);

    delegate
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
