use curses::Curses;
use general::*;
use std::sync::Mutex;
use window::Window;

lazy_static! {
    static ref INITIALIZED: Mutex<bool> = Mutex::new(false);
}

pub(crate) fn end_window() -> Result<(), ()> {
    let mut initialized = INITIALIZED.lock().unwrap();
    if *initialized {
        check(pancurses::endwin())?;
        *initialized = false;
        Ok(())
    } else {
        Err(())
    }
}

/// This function initializes the [`Curses`] system.
///
/// This is a correlary of the c function `initscr`.
///
/// [`Curses`]: struct.Curses.html
pub fn initscr() -> Result<Curses, ()> {
    {
        let mut initialized = INITIALIZED.lock().unwrap();
        if *initialized {
            return Err(());
        } else {
            *initialized = true;
        }
    }
    let w = pancurses::initscr();
    Ok(Curses::new(Window::new(w)))
}
