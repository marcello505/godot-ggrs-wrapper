#![warn(missing_docs)]
//! # Godot-GGRS-Wrapper
//! Godot-GGRS-Wrapper exposes different function to interact with GGRS inside Godot.
//! All documentation written is explicitly targeted towards use inside Godot, any functions that are usable in Godot have parameters that start with `(&mut self, _owner: &Node)`.
//! When interacting with the function in Godot you can ignore these 2 parameters and just use what comes after.
//! For example the [GodotGGRSP2PSession::add_remote_player()] method would just be used like this in Godot: `p2p.add_remote_player("127.0.0.1:7070")`.

use gdnative::prelude::*;
pub use godotggrs_p2psession::GodotGGRSP2PSession;
pub use godotggrs_p2pspectatorsession::GodotGGRSP2PSpectatorSession;
pub use godotggrs_synctest::GodotGGRSSyncTest;

mod godotggrs_p2psession;
mod godotggrs_p2pspectatorsession;
mod godotggrs_synctest;
mod helper_functions;

pub const ERR_MESSAGE_NO_SESSION_MADE: &str = "No session was made.";
pub const ERR_MESSAGE_NO_CALLBACK_NODE: &str = "No callback node was specified.";
pub const CALLBACK_FUNC_SAVE_GAME_STATE: &str = "ggrs_save_game_state";
pub const CALLBACK_FUNC_LOAD_GAME_STATE: &str = "ggrs_load_game_state";
pub const CALLBACK_FUNC_ADVANCE_FRAME: &str = "ggrs_advance_frame";

pub fn init_panic_hook() {
    // To enable backtrace, you will need the `backtrace` crate to be included in your cargo.toml, or
    // a version of rust where backtrace is included in the standard library (e.g. Rust nightly as of the date of publishing)
    // use backtrace::Backtrace;
    // use std::backtrace::Backtrace;
    let old_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let loc_string;
        if let Some(location) = panic_info.location() {
            loc_string = format!("file '{}' at line {}", location.file(), location.line());
        } else {
            loc_string = "unknown location".to_owned()
        }

        let error_message;
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            error_message = format!("[RUST] {}: panic occurred: {:?}", loc_string, s);
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            error_message = format!("[RUST] {}: panic occurred: {:?}", loc_string, s);
        } else {
            error_message = format!("[RUST] {}: unknown panic occurred", loc_string);
        }
        godot_error!("{}", error_message);
        // Uncomment the following line if backtrace crate is included as a dependency
        // godot_error!("Backtrace:\n{:?}", Backtrace::new());
        (*(old_hook.as_ref()))(panic_info);

        unsafe {
            if let Some(gd_panic_hook) =
                gdnative::api::utils::autoload::<gdnative::api::Node>("RustPanicHook")
            {
                gd_panic_hook.call(
                    "rust_panic_hook",
                    &[GodotString::from_str(error_message).to_variant()],
                );
            }
        }
    }));
}

fn init(handle: InitHandle) {
    handle.add_class::<GodotGGRSP2PSession>();
    handle.add_class::<GodotGGRSSyncTest>();
    handle.add_class::<GodotGGRSP2PSpectatorSession>();
    init_panic_hook()
}

godot_init!(init);
