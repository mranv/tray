// Bring in Cocoa and objc macros.
#[macro_use]
extern crate cocoa;
#[macro_use]
extern crate objc;

use cocoa::appkit::{
    NSApplication,
    NSStatusBar,
    NSMenu,
    NSMenuItem,
    NSRunningApplication,
    NSApplicationActivateIgnoringOtherApps,
    NSVariableStatusItemLength,
};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSString};

use objc::class;
use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use objc::runtime::{Object, Sel};

//
// Helper: Send a notification using the (deprecated) NSUserNotificationCenter API
//
// Note: Notifications from a command‑line tool (or non‑bundled app) may not appear!
//
fn send_notification(title: &str, message: &str) {
    unsafe {
        let center: id =
            msg_send![class!(NSUserNotificationCenter), defaultUserNotificationCenter];
        let notification: id = msg_send![class!(NSUserNotification), new];
        let ns_title = NSString::alloc(nil).init_str(title);
        let ns_message = NSString::alloc(nil).init_str(message);
        let _: () = msg_send![notification, setTitle: ns_title];
        let _: () = msg_send![notification, setInformativeText: ns_message];
        let _: () = msg_send![center, deliverNotification: notification];
    }
}

//
// Delegate action for "Update Status"
//
extern "C" fn update_status_action(_this: &Object, _cmd: Sel, _sender: id) -> () {
    send_notification("Update Status", "Status updated successfully.");
}

//
// Delegate action for "Security Preferences..."
//
extern "C" fn open_security_prefs_action(_this: &Object, _cmd: Sel, _sender: id) -> () {
    send_notification("Security Preferences", "Opening Security Preferences...");
}

//
// Delegate action for "Quit" – sends a notification then terminates the app.
//
extern "C" fn quit_action(_this: &Object, _cmd: Sel, _sender: id) -> () {
    send_notification("Quit", "Application is quitting.");
    unsafe {
        let app: id = NSApplication::sharedApplication(nil);
        // Annotate the return type to help type inference.
        let _: () = msg_send![app, terminate: nil];
    }
}

//
// Create an Objective-C class "AppDelegate" that implements updateStatus:,
// openSecurityPrefs:, and quitAction:.
//
fn create_app_delegate() -> id {
    use objc::declare::ClassDecl;
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new("AppDelegate", superclass).unwrap();
    unsafe {
        decl.add_method(
            sel!(updateStatus:),
            update_status_action as extern "C" fn(&Object, Sel, id) -> (),
        );
        decl.add_method(
            sel!(openSecurityPrefs:),
            open_security_prefs_action as extern "C" fn(&Object, Sel, id) -> (),
        );
        decl.add_method(
            sel!(quitAction:),
            quit_action as extern "C" fn(&Object, Sel, id) -> (),
        );
    }
    decl.register();
    unsafe { msg_send![class!(AppDelegate), new] }
}

//
// Main: Set up the tray icon and menu
//
fn main() {
    unsafe {
        // Create an autorelease pool.
        let _pool = NSAutoreleasePool::new(nil);
        let app = NSApplication::sharedApplication(nil);

        // --- SETUP STATUS ITEM (TRAY ICON) ---
        let status_bar: id = NSStatusBar::systemStatusBar(nil);
        let status_item: id = status_bar.statusItemWithLength_(NSVariableStatusItemLength);

        // Create a system image using an SF Symbol ("shield.fill").
        let symbol_name = NSString::alloc(nil).init_str("shield.fill");
        let image: id =
            msg_send![class!(NSImage), imageWithSystemSymbolName: symbol_name accessibilityDescription: nil];
        let _: () = msg_send![status_item, setImage: image];
        let _: () = msg_send![image, setTemplate: true];

        // --- SETUP APP DELEGATE FOR MENU ACTIONS ---
        let delegate: id = create_app_delegate();

        // --- CREATE AN NSMENU ---
        let menu: id = NSMenu::new(nil);

        // "Update Status" menu item.
        let update_title = NSString::alloc(nil).init_str("Update Status");
        let update_item: id = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(update_title, sel!(updateStatus:), NSString::alloc(nil).init_str(""));
        let _: () = msg_send![update_item, setTarget: delegate];
        let update_icon_name = NSString::alloc(nil).init_str("arrow.clockwise");
        let update_image: id =
            msg_send![class!(NSImage), imageWithSystemSymbolName: update_icon_name accessibilityDescription: nil];
        let _: () = msg_send![update_item, setImage: update_image];
        let _: () = msg_send![update_image, setTemplate: true];
        menu.addItem_(update_item);

        // "Security Preferences..." menu item.
        let prefs_title = NSString::alloc(nil).init_str("Security Preferences...");
        let prefs_item: id = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(prefs_title, sel!(openSecurityPrefs:), NSString::alloc(nil).init_str(""));
        let _: () = msg_send![prefs_item, setTarget: delegate];
        let prefs_icon_name = NSString::alloc(nil).init_str("gearshape.fill");
        let prefs_image: id =
            msg_send![class!(NSImage), imageWithSystemSymbolName: prefs_icon_name accessibilityDescription: nil];
        let _: () = msg_send![prefs_item, setImage: prefs_image];
        let _: () = msg_send![prefs_image, setTemplate: true];
        menu.addItem_(prefs_item);

        // Add a separator.
        let separator: id = msg_send![class!(NSMenuItem), separatorItem];
        menu.addItem_(separator);

        // "Quit" menu item.
        let quit_title = NSString::alloc(nil).init_str("Quit");
        let quit_item: id = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(quit_title, sel!(quitAction:), NSString::alloc(nil).init_str("q"));
        let _: () = msg_send![quit_item, setTarget: delegate];
        let quit_icon_name = NSString::alloc(nil).init_str("xmark.circle.fill");
        let quit_image: id =
            msg_send![class!(NSImage), imageWithSystemSymbolName: quit_icon_name accessibilityDescription: nil];
        let _: () = msg_send![quit_item, setImage: quit_image];
        let _: () = msg_send![quit_image, setTemplate: true];
        menu.addItem_(quit_item);

        // --- ASSIGN THE MENU TO THE STATUS ITEM ---
        let _: () = msg_send![status_item, setMenu: menu];

        // Activate the application so the tray icon becomes active.
        NSRunningApplication::currentApplication(nil)
            .activateWithOptions_(NSApplicationActivateIgnoringOtherApps);

        // Run the application event loop.
        app.run();
    }
}
