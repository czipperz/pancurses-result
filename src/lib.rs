//! `pancurses-result` is a wrapper for `pancurses` that aims to
//! provide a safe interface to `curses`.  This library aims to
//! guarantee thread and memory safety, whereas `pancurses` just
//! provides direct C bindings.
//!
//! Click here to about [`Window`]s.
//!
//! To initialize `pancurses-result`, run [`initscr`].
//!
//! [`Window`]: struct.Window.html
//! [`initscr`]: fn.initscr.html

extern crate pancurses;
#[macro_use]
extern crate lazy_static;

mod general;
mod initialize;
pub use initialize::*;
mod point;
pub use point::*;
mod curses;
pub use curses::*;
mod color;
pub use color::*;
mod window;
pub use window::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn type_assert_send<T: Send>() {}
    fn type_assert_sync<T: Sync>() {}

    #[test]
    fn window_is_send() {
        type_assert_send::<Window>();
    }

    #[test]
    fn window_is_sync() {
        type_assert_sync::<Window>();
    }

    #[test]
    fn curses_is_send() {
        type_assert_send::<Curses>();
    }

    #[test]
    fn curses_is_sync() {
        type_assert_sync::<Curses>();
    }
}
