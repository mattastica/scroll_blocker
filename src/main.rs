#![cfg(target_os = "macos")]
#![allow(unexpected_cfgs)]

#[macro_use]
extern crate objc;

use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivationPolicy, NSButton, NSMenu, NSMenuItem,
    NSStatusBar, NSStatusItem,
};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSString};

use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};
use core_graphics::event::{
    CallbackResult, CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement,
    CGEventType,
};

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);

        // ---- Menu bar icon ----
        let status_item: id = NSStatusBar::systemStatusBar(nil).statusItemWithLength_(-1.0);
        let button: id = status_item.button();
        if button != nil {
            let title = NSString::alloc(nil).init_str("🧱");
            button.setTitle_(title);
        }

        // Menu with Quit
        let menu = NSMenu::new(nil).autorelease();
        let quit_title = NSString::alloc(nil).init_str("Quit");
        let key_equiv = NSString::alloc(nil).init_str("q");

        let quit_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(quit_title, sel!(terminate:), key_equiv)
            .autorelease();
        menu.addItem_(quit_item);
        status_item.setMenu_(menu);

        // ---- Event tap (drops scroll) ----
        let event_tap = CGEventTap::new(
            CGEventTapLocation::Session,
            CGEventTapPlacement::HeadInsertEventTap,
            CGEventTapOptions::Default,
            vec![CGEventType::ScrollWheel],
            |_proxy, event_type, _event| {
                if matches!(event_type, CGEventType::ScrollWheel) {
                    CallbackResult::Drop
                } else {
                    CallbackResult::Keep
                }
            },
        )
        .expect("Failed to install event tap. Grant Accessibility permission (Privacy & Security → Accessibility).");

        let loop_source = event_tap
            .mach_port()
            .create_runloop_source(0)
            .expect("Runloop source creation failed");
        CFRunLoop::get_current().add_source(&loop_source, kCFRunLoopCommonModes);
        event_tap.enable();

        // Keep these alive for the lifetime of the app.
        let _keep_alive = (status_item, menu, event_tap, loop_source);

        app.run(); // keeps run loop alive
    }
}
