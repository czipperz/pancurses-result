use color::Color;
use general::*;
use initialize::end_window;
use std::sync::Mutex;
use window::Window;

/// The visibility of the cursor
#[repr(i32)]
pub enum CursorVisibility {
    Invisible = 0,
    Normal,
    HighlyVisible,
}

/// A number of bits per second
pub struct BitsPerSecond {
    bps: i32,
}
impl BitsPerSecond {
    pub fn new(bps: i32) -> Self {
        BitsPerSecond { bps }
    }

    pub fn bits_per_second(&self) -> i32 {
        self.bps
    }
}

/// A number of milliseconds
pub struct Milliseconds {
    ms: i32,
}
impl Milliseconds {
    pub fn new(ms: i32) -> Self {
        Milliseconds { ms }
    }

    pub fn milliseconds(&self) -> i32 {
        self.ms
    }
}

/// The curses instance.  To initialize the curses instance, call [`initscr`].
///
/// [`initscr`]: fn.initscr.html
pub struct Curses {
    window: Window,
    key_name_mutex: Mutex<()>,
    already_ended: bool,
    color: Option<Color>,
}

impl Curses {
    pub(crate) fn new(window: Window) -> Self {
        Curses {
            window,
            key_name_mutex: Mutex::new(()),
            already_ended: false,
            color: None,
        }
    }

    /// Get a reference to the main [`Window`] of the cursees instance.
    ///
    /// [`Window`]: struct.Window.html`]
    pub fn window(&self) -> &Window {
        &self.window
    }
    /// Get a mutable reference to the main [`Window`] of the cursees instance.
    ///
    /// [`Window`]: struct.Window.html`]
    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    /// Get the output rate of the terminal in bits per second.
    pub fn output_rate(&self) -> BitsPerSecond {
        BitsPerSecond {
            bps: pancurses::baudrate(),
        }
    }

    /// Attempt to beep the terminal
    pub fn beep(&mut self) -> Result<(), ()> {
        check(pancurses::beep())
    }

    /// Check if the terminal has support for colors.
    pub fn has_colors(&self) -> bool {
        pancurses::has_colors()
    }
    /// Start the color subsystem
    pub fn start_color(&mut self) -> Result<&mut Color, ()> {
        check(pancurses::start_color())?;
        self.color = Some(Color::new());
        Ok(self.color_mut())
    }
    /// Get an immutable reference to the color subsystem.
    pub fn color(&self) -> &Color {
        self.color
            .as_ref()
            .expect("Color subsystem has not yet been successfully started")
    }
    /// Get a mutable reference to the color subsystem.
    pub fn color_mut(&mut self) -> &mut Color {
        self.color
            .as_mut()
            .expect("Color subsystem has not yet been successfully started")
    }

    /// Enable cbreak mode, causing input to immediately be ready for
    /// [`read`] after it is typed.
    ///
    /// This overrides `raw`.
    ///
    /// [`read`]: struct.Curses.html#method.read
    pub fn enable_cbreak(&mut self) -> Result<(), ()> {
        check(pancurses::cbreak())
    }
    /// Disable cbreak mode, causing input to be buffered after it is
    /// typed until a newline or carriage return.
    ///
    /// This overrides `raw`.
    ///
    /// [`read`]: struct.Curses.html#method.read
    pub fn disable_cbreak(&mut self) -> Result<(), ()> {
        check(pancurses::nocbreak())
    }

    /// Set the visibility of the cursor.
    pub fn set_cursor_visibility(&mut self, visibility: CursorVisibility) -> Result<(), ()> {
        check(pancurses::curs_set(unsafe {
            std::mem::transmute(visibility)
        }))
    }

    /// Save the current terminal state as program mode (in curses).
    ///
    /// This is done automatically by [`initscr`].
    ///
    /// [`initscr`]: fn.initscr.html
    pub fn define_program_mode(&mut self) -> Result<(), ()> {
        check(pancurses::def_prog_mode())
    }
    /// Save the current terminal state as shell mode (not in curses).
    ///
    /// This is done automatically by [`initscr`].
    ///
    /// [`initscr`]: fn.initscr.html
    pub fn define_shell_mode(&mut self) -> Result<(), ()> {
        check(pancurses::def_shell_mode())
    }

    /// Insert a millisecond pause in output.  Don't use this extensively.
    pub fn delay_output(&mut self, time: Milliseconds) -> Result<(), ()> {
        check(pancurses::delay_output(time.milliseconds()))
    }

    /// Push updates from the virtual screen to the physical screen.
    ///
    /// This is the corollary of `doupdate`.
    pub fn update(&mut self) -> Result<(), ()> {
        check(pancurses::doupdate())
    }

    /// Characters typed by the user are written to the screen as they are typed.
    pub fn enable_echo(&mut self) -> Result<(), ()> {
        check(pancurses::echo())
    }
    /// Characters typed by the user are interpretted by the program
    /// and not echoed to the screen.
    pub fn disable_echo(&mut self) -> Result<(), ()> {
        check(pancurses::noecho())
    }

    /// Dispose of this window.
    ///
    /// This allows for the user to deal with error conditions instead
    /// of [`Result::unwrap`] being called.
    ///
    /// This is the corollary of `endwin`.
    pub fn end_window(mut self) -> Result<(), ()> {
        if self.already_ended {
            Err(())
        } else {
            self.already_ended = true;
            end_window()
        }
    }

    /// Flash the terminal screen.  If not possible, an alert is sounded.
    ///
    /// Returns `Ok` if flashing succeeds, and `Err` otherwise.
    pub fn flash(&mut self) -> Result<(), ()> {
        check(pancurses::flash())
    }

    /// Get a string representing a key code.
    ///
    /// This is the corollary of `keyname`.
    pub fn key_name(&self, key_code: i32) -> Option<String> {
        let _key_name = self.key_name_mutex.lock().unwrap();
        pancurses::keyname(key_code)
    }

    /// Get the status of the mouse.
    ///
    /// This is the corollary of `getmouse`.
    pub fn mouse_read(&self) -> Result<MouseEvent, ()> {
        pancurses::getmouse().map_err(|_| ())
    }
    /// Get the maximum time between press and release events for it
    /// to be recognized as a click.
    ///
    /// This is the corollary of `mouseinterval(-1)`.
    pub fn mouse_interval(&self) -> Milliseconds {
        Milliseconds {
            ms: pancurses::mouseinterval(-1),
        }
    }
    /// Set the maximum time between press and release events for it
    /// to be recognized as a click.
    ///
    /// As of right now this will always succeed, but it is possible
    /// this behavior will change in the future.
    ///
    /// This is the corollary of `mouseinterval`.
    pub fn set_mouse_interval(&mut self, interval: Milliseconds) -> Result<(), ()> {
        pancurses::mouseinterval(interval.milliseconds());
        Ok(())
    }
    /// Set the mouse events to be reported.
    ///
    /// Returns the masks that were applied.
    ///
    /// As of right now this will always succeed, but it is possible
    /// this behavior will change in the future.
    ///
    /// If `mask == 0` then the mouse pointer may be turned off.
    ///
    /// This is the corollary of `mousemask`.
    pub fn set_mouse_mask(
        &mut self,
        mask: MouseMask,
        old_mask: Option<&mut MouseMask>,
    ) -> Result<MouseMask, ()> {
        let old_mask_ptr = match old_mask {
            Some(mask) => mask,
            None => std::ptr::null_mut(),
        };
        Ok(pancurses::mousemask(mask, old_mask_ptr))
    }

    /// Sleep for a certain number of milliseconds.
    ///
    /// This is the corollary of `napms`.
    pub fn suspend_milliseconds(&mut self, duration: Milliseconds) -> Result<(), ()> {
        check(pancurses::napms(duration.milliseconds()))
    }

    /// Enable new line translations.
    ///
    /// When enabled, the return *key* is translated into newline on
    /// input and return and line-feed on output.
    ///
    /// This is enabled by default.  Disabling this can cause curses
    /// to make better use of the line-feed capability, will have
    /// faster cursor motion, and will detect the return key (see
    /// `Input::KeyEnter`).
    ///
    /// This is the corollary of `nl`.
    pub fn enable_new_line_translations(&mut self) -> Result<(), ()> {
        check(pancurses::nl())
    }
    /// Disable new line translations.
    ///
    /// See [`enable_new_line_translations`].
    ///
    /// [`enable_new_line_translations`]: struct.Curses.html#method.enable_new_line_translations
    ///
    /// This is the corollary of `nonl`.
    pub fn disable_new_line_translations(&mut self) -> Result<(), ()> {
        check(pancurses::nonl())
    }

    /// Enable raw input mode.  Characters are immediately passed
    /// through to the user program and no signals are generated.
    ///
    /// This is similar to cbreak mode (see [`enable_cbreak`]).
    ///
    /// [`enable_cbreak`]: struct.Curses.html#method.enable_cbreak
    ///
    /// This is the corollary of `raw`.
    pub fn enable_raw_input_mode(&mut self) -> Result<(), ()> {
        check(pancurses::raw())
    }
    /// Disable raw input mode.
    ///
    /// For discussion of how raw mode works, see [`enable_raw_input_mode`].
    ///
    /// [`enable_raw_input_mode`]: struct.Curses.html#method.enable_raw_input_mode
    ///
    /// This is the corollary of `noraw`.
    pub fn disable_raw_input_mode(&mut self) -> Result<(), ()> {
        check(pancurses::noraw())
    }

    /// Restore the terminal to program mode (in curses).
    ///
    /// This is the corollary of `reset_prog_mode`.
    pub fn restore_program_mode(&mut self) -> Result<(), ()> {
        check(pancurses::reset_prog_mode())
    }
    /// Restore the terminal to program mode (not in curses).
    ///
    /// This is the corollary of `reset_shell_mode`.
    pub fn restore_shell_mode(&mut self) -> Result<(), ()> {
        check(pancurses::reset_shell_mode())
    }

    /// Attempt to resize the terminal.
    ///
    /// This is the corollary of `resize_term`.
    pub fn resize_terminal(&mut self, rows: i32, columns: i32) -> Result<(), ()> {
        check(pancurses::resize_term(rows, columns))
    }

    /// Force the screen to actually blink instead of setting a high
    /// intensity background.
    ///
    /// This is only supported on Windows.
    ///
    /// This is the corollary of `set_blink(true)`
    pub fn enable_force_blink(&mut self) -> Result<(), ()> {
        check(pancurses::set_blink(true))
    }
    /// Allow the terminal to either actually blink or set a high
    /// intensity background when `blink` is called.
    ///
    /// This is the corollary of `set_blink(false)`
    pub fn disable_force_blink(&mut self) -> Result<(), ()> {
        check(pancurses::set_blink(false))
    }

    /// Set the title of the terminal.
    ///
    /// This is only supported on Windows.
    pub fn set_title<T: AsRef<str>>(&mut self, title: T) -> Result<(), ()> {
        Ok(pancurses::set_title(title.as_ref()))
    }
}

/// Call [`end_window`] and `unwrap` the result.
///
/// [`end_window`]: struct.Curses.html#method.end_window
impl Drop for Curses {
    fn drop(&mut self) {
        if !self.already_ended {
            end_window().unwrap();
        }
    }
}
