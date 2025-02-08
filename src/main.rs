// src/main.rs
// Bring in Cocoa and objc macros.
#[macro_use]
extern crate cocoa;
#[macro_use]
extern crate objc;

use cocoa::appkit::{
    NSApplication,
    NSStatusBar,
    NSRunningApplication,
    NSApplicationActivateIgnoringOtherApps,
    NSVariableStatusItemLength,
    // Note: We remove NSFont from the import to resolve the error.
    // NSColor is available if needed.
};
use cocoa::base::{id, nil, NO, YES};
use cocoa::foundation::{NSAutoreleasePool, NSRect, NSSize, NSPoint, NSString};

use objc::class;
use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use objc::runtime::{Object, Sel};

//
// GLOBAL_POPOVER: a mutable global to store the NSPopover instance.
//
static mut GLOBAL_POPOVER: id = nil;

//
// togglePopover: Called by our toggle delegate when the tray button is clicked.
// It toggles the global popover (show/hide).
//
#[no_mangle]
extern "C" fn togglePopover(_this: &Object, _cmd: Sel, sender: id) -> () {
    unsafe {
        if GLOBAL_POPOVER != nil {
            // Retrieve the popover's isShown property as an i32.
            let is_shown: i32 = msg_send![GLOBAL_POPOVER, isShown];
            if is_shown != 0 {
                let _: () = msg_send![GLOBAL_POPOVER, close];
            } else {
                let bounds: NSRect = msg_send![sender, bounds];
                let _: () = msg_send![GLOBAL_POPOVER, showRelativeToRect: bounds ofView: sender preferredEdge: 1];
            }
        }
    }
}

//
// create_toggle_delegate: Creates a custom Objective-C delegate that implements togglePopover:.
//
fn create_toggle_delegate() -> id {
    use objc::declare::ClassDecl;
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new("ToggleDelegate", superclass).unwrap();
    unsafe {
        decl.add_method(
            sel!(togglePopover:),
            togglePopover as extern "C" fn(&Object, Sel, id) -> (),
        );
    }
    decl.register();
    unsafe { msg_send![class!(ToggleDelegate), new] }
}

//
// create_popover_content_view: Constructs the popover's content view.
// This view features a blurred background, a header, a scrollable list of dummy rows, and a footer.
//
fn create_popover_content_view() -> id {
    unsafe {
        // Create the main content view.
        let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(420.0, 400.0));
        let content_view: id = msg_send![class!(NSView), alloc];
        let content_view: id = msg_send![content_view, initWithFrame: frame];

        // --- Background: Visual Effect View ---
        let effect_view: id = msg_send![class!(NSVisualEffectView), alloc];
        let effect_view: id = msg_send![effect_view, initWithFrame: frame];
        let _: () = msg_send![effect_view, setMaterial: 9]; // Material 9 approximates a "menu" style.
        let _: () = msg_send![effect_view, setBlendingMode: 0];
        let _: () = msg_send![effect_view, setState: 1];
        let _: () = msg_send![content_view, addSubview: effect_view];

        // --- Header ---
        let header_frame = NSRect::new(NSPoint::new(12.0, 360.0), NSSize::new(396.0, 30.0));
        let header: id = msg_send![class!(NSTextField), alloc];
        let header: id = msg_send![header, initWithFrame: header_frame];
        let header_str = NSString::alloc(nil).init_str("Security Status");
        let _: () = msg_send![header, setStringValue: header_str];
        // Retrieve the bold system font by sending a message directly to NSFont.
        let header_font: id = msg_send![class!(NSFont), boldSystemFontOfSize: 16.0];
        let _: () = msg_send![header, setFont: header_font];
        let _: () = msg_send![header, setBezeled: NO];
        let _: () = msg_send![header, setDrawsBackground: NO];
        let _: () = msg_send![header, setEditable: NO];
        let _: () = msg_send![header, setSelectable: NO];
        let _: () = msg_send![content_view, addSubview: header];

        // --- Divider ---
        let divider_frame = NSRect::new(NSPoint::new(12.0, 350.0), NSSize::new(396.0, 1.0));
        let divider: id = msg_send![class!(NSBox), alloc];
        let divider: id = msg_send![divider, initWithFrame: divider_frame];
        let _: () = msg_send![divider, setBoxType: 1]; // Separator style.
        let _: () = msg_send![content_view, addSubview: divider];

        // --- Menu Items List (Scroll View) ---
        let scroll_frame = NSRect::new(NSPoint::new(12.0, 100.0), NSSize::new(396.0, 240.0));
        let scroll_view: id = msg_send![class!(NSScrollView), alloc];
        let scroll_view: id = msg_send![scroll_view, initWithFrame: scroll_frame];
        let doc_frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(396.0, 240.0));
        let doc_view: id = msg_send![class!(NSView), alloc];
        let doc_view: id = msg_send![doc_view, initWithFrame: doc_frame];

        // Create three dummy rows.
        for i in 0..3 {
            let row_y = 240.0 - ((i + 1) as f64 * 50.0);
            let row_frame = NSRect::new(NSPoint::new(0.0, row_y), NSSize::new(396.0, 40.0));
            let row: id = msg_send![class!(NSView), alloc];
            let row: id = msg_send![row, initWithFrame: row_frame];

            // Icon: NSImageView using system symbol "doc.text".
            let icon_frame = NSRect::new(NSPoint::new(10.0, 10.0), NSSize::new(20.0, 20.0));
            let icon_view: id = msg_send![class!(NSImageView), alloc];
            let icon_view: id = msg_send![icon_view, initWithFrame: icon_frame];
            let icon_str = NSString::alloc(nil).init_str("doc.text");
            let icon_img: id = msg_send![class!(NSImage), imageWithSystemSymbolName: icon_str accessibilityDescription: nil];
            let _: () = msg_send![icon_view, setImage: icon_img];
            let _: () = msg_send![icon_img, setTemplate: YES];
            let _: () = msg_send![row, addSubview: icon_view];

            // Label.
            let label_frame = NSRect::new(NSPoint::new(35.0, 10.0), NSSize::new(250.0, 20.0));
            let label: id = msg_send![class!(NSTextField), alloc];
            let label: id = msg_send![label, initWithFrame: label_frame];
            let text = NSString::alloc(nil).init_str(&format!("Menu Item {}", i + 1));
            let _: () = msg_send![label, setStringValue: text];
            let label_font: id = msg_send![class!(NSFont), systemFontOfSize: 14.0];
            let _: () = msg_send![label, setFont: label_font];
            let _: () = msg_send![label, setBezeled: NO];
            let _: () = msg_send![label, setDrawsBackground: NO];
            let _: () = msg_send![label, setEditable: NO];
            let _: () = msg_send![label, setSelectable: NO];
            let _: () = msg_send![row, addSubview: label];

            // Circular indicator on the right.
            let circle_frame = NSRect::new(NSPoint::new(320.0, 10.0), NSSize::new(20.0, 20.0));
            let circle: id = msg_send![class!(NSView), alloc];
            let circle: id = msg_send![circle, initWithFrame: circle_frame];
            let _: () = msg_send![circle, setWantsLayer: YES];
            let layer: id = msg_send![circle, layer];
            let indicator_color: id = msg_send![class!(NSColor), systemGreenColor];
            let _: () = msg_send![layer, setBackgroundColor: indicator_color];
            let _: () = msg_send![layer, setCornerRadius: 10.0];
            let _: () = msg_send![row, addSubview: circle];

            let _: () = msg_send![doc_view, addSubview: row];
        }
        let _: () = msg_send![scroll_view, setDocumentView: doc_view];
        let _: () = msg_send![content_view, addSubview: scroll_view];

        // --- Footer ---
        let footer_frame = NSRect::new(NSPoint::new(12.0, 12.0), NSSize::new(396.0, 60.0));
        let footer: id = msg_send![class!(NSView), alloc];
        let footer: id = msg_send![footer, initWithFrame: footer_frame];

        // Logo: NSImageView using system symbol "sparkles".
        let logo_frame = NSRect::new(NSPoint::new(10.0, 20.0), NSSize::new(40.0, 40.0));
        let logo: id = msg_send![class!(NSImageView), alloc];
        let logo: id = msg_send![logo, initWithFrame: logo_frame];
        let logo_str = NSString::alloc(nil).init_str("sparkles");
        let logo_img: id = msg_send![class!(NSImage), imageWithSystemSymbolName: logo_str accessibilityDescription: nil];
        let _: () = msg_send![logo, setImage: logo_img];
        let _: () = msg_send![logo_img, setTemplate: YES];
        let _: () = msg_send![footer, addSubview: logo];

        // Version label.
        let version_frame = NSRect::new(NSPoint::new(60.0, 30.0), NSSize::new(100.0, 20.0));
        let version: id = msg_send![class!(NSTextField), alloc];
        let version: id = msg_send![version, initWithFrame: version_frame];
        let version_str = NSString::alloc(nil).init_str("v4.7.5");
        let _: () = msg_send![version, setStringValue: version_str];
        let version_font: id = msg_send![class!(NSFont), systemFontOfSize: 12.0];
        let _: () = msg_send![version, setFont: version_font];
        let _: () = msg_send![version, setBezeled: NO];
        let _: () = msg_send![version, setDrawsBackground: NO];
        let _: () = msg_send![version, setEditable: NO];
        let _: () = msg_send![version, setSelectable: NO];
        let _: () = msg_send![footer, addSubview: version];

        // Refresh button.
        let button_frame = NSRect::new(NSPoint::new(320.0, 20.0), NSSize::new(60.0, 30.0));
        let refresh: id = msg_send![class!(NSButton), alloc];
        let refresh: id = msg_send![refresh, initWithFrame: button_frame];
        let btn_title = NSString::alloc(nil).init_str("Refresh");
        let _: () = msg_send![refresh, setTitle: btn_title];
        let _: () = msg_send![footer, addSubview: refresh];

        let _: () = msg_send![content_view, addSubview: footer];

        content_view
    }
}

//
// create_popover_view_controller: Wraps our content view in an NSViewController.
//
fn create_popover_view_controller() -> id {
    unsafe {
        let view = create_popover_content_view();
        let vc: id = msg_send![class!(NSViewController), alloc];
        let vc: id = msg_send![vc, init];
        let _: () = msg_send![vc, setView: view];
        vc
    }
}

//
// setup_status_item_and_popover: Creates the tray icon and associates a popover with it.
//
fn setup_status_item_and_popover() {
    unsafe {
        let _app = NSApplication::sharedApplication(nil);
        let status_bar: id = NSStatusBar::systemStatusBar(nil);
        let status_item: id = status_bar.statusItemWithLength_(NSVariableStatusItemLength);

        // Set the tray icon using "shield.fill".
        let symbol_name = NSString::alloc(nil).init_str("shield.fill");
        let image: id = msg_send![class!(NSImage), imageWithSystemSymbolName: symbol_name accessibilityDescription: nil];
        let _: () = msg_send![status_item, setImage: image];
        let _: () = msg_send![image, setTemplate: YES];

        // Create an NSPopover.
        let popover: id = msg_send![class!(NSPopover), alloc];
        let popover: id = msg_send![popover, init];
        let vc = create_popover_view_controller();
        let _: () = msg_send![popover, setContentViewController: vc];

        // Store the popover globally.
        GLOBAL_POPOVER = popover;

        // Create a toggle delegate.
        let toggle_delegate: id = create_toggle_delegate();

        // Set the status item button's target and action.
        let button: id = msg_send![status_item, button];
        if button != nil {
            let _: () = msg_send![button, setTarget: toggle_delegate];
            let _: () = msg_send![button, setAction: sel!(togglePopover:)];
        }

        NSRunningApplication::currentApplication(nil)
            .activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
    }
}

//
// Main entry point.
//
fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let app = NSApplication::sharedApplication(nil);
        app.setActivationPolicy_(cocoa::appkit::NSApplicationActivationPolicyRegular);
        setup_status_item_and_popover();
        app.run();
    }
}
