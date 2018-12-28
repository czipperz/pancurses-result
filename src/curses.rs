use color::Color;
use general::*;
use initialize::end_window;
use std::sync::Mutex;
use std::time::Duration;
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

/// The input buffering mode.
///
/// This allows us to control which of `raw` and `cbreak` modes are applied.
pub enum InputBufferingMode {
    /// This is the default and means that curses buffers input until a new line
    /// is read.
    ///
    /// This corresponds to no `cbreak` mode and no `raw` mode.
    Buffered,
    /// Allow for unbuffered input while generating signals upon seeing control
    /// characters.
    ///
    /// For example, this allows `Control+C` to cause the program to exit.
    ///
    /// This corresponds to `cbreak` mode without `raw` mode.
    UnbufferedWithSignals,
    /// Allow for unbuffered input without interpreting the meaning of any keys.
    ///
    /// For example, this allows `Control+C` to be typed *without* forcing the
    /// program to exit.
    ///
    /// This corresponds to `cbreak` mode and `raw` mode.
    UnbufferedNoSignals,
}

/// The curses instance.  To initialize the curses instance, call [`initscr`].
///
/// Many curses functions have been renamed for one reason or another.  All
/// renamed functions state the curses function they corollate to.
///
/// [`initscr`]: fn.initscr.html
pub struct Curses {
    window: Window,
    key_name_mutex: Mutex<()>,
    color: Option<Color>,
}

impl Curses {
    pub(crate) fn new(window: Window) -> Self {
        Curses {
            window,
            key_name_mutex: Mutex::new(()),
            color: None,
        }
    }

    /// Get a reference to the main [`Window`] of the curses instance.
    ///
    /// This corresponds to `stdscr`.
    ///
    /// [`Window`]: struct.Window.html`]
    pub fn window(&self) -> &Window {
        &self.window
    }
    /// Get a mutable reference to the main [`Window`] of the curses instance.
    ///
    /// This corresponds to `stdscr`.
    ///
    /// [`Window`]: struct.Window.html`]
    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    /// Check if the terminal has support for colors.
    pub fn has_colors(&self) -> bool {
        pancurses::has_colors()
    }
    /// Start the color subsystem.
    ///
    /// If it has already been started, this does nothing.
    pub fn start_color(&mut self) -> Result<(), ()> {
        if self.color.is_none() {
            check(pancurses::start_color())?;
            self.color = Some(Color::new());
        }
        Ok(())
    }
    /// Get an immutable reference to the [`Color`] subsystem.
    ///
    /// This method will panic if [`start_color`] has not successfully completed yet.
    ///
    /// [`Color`]: struct.Color.html
    /// [`start_color`]: struct.Curses.html#method.start_color
    pub fn color(&self) -> &Color {
        self.color
            .as_ref()
            .expect("Color subsystem has not yet been successfully started")
    }
    /// Get a mutable reference to the [`Color`] subsystem.
    ///
    /// This method will panic if [`start_color`] has not successfully completed yet.
    ///
    /// [`Color`]: struct.Color.html
    /// [`start_color`]: struct.Curses.html#method.start_color
    pub fn color_mut(&mut self) -> &mut Color {
        self.color
            .as_mut()
            .expect("Color subsystem has not yet been successfully started")
    }

    /// Set the visibility of the cursor.
    ///
    /// This corresponds of `curs_set`.
    pub fn set_cursor_visibility(&mut self, visibility: CursorVisibility) -> Result<(), ()> {
        check(pancurses::curs_set(unsafe {
            std::mem::transmute(visibility)
        }))
    }

    /// Save the current terminal state as program mode (in curses).
    ///
    /// This is done automatically by [`initscr`].
    ///
    /// This corresponds of `def_prog_mode`.
    ///
    /// [`initscr`]: fn.initscr.html
    pub fn define_program_mode(&mut self) -> Result<(), ()> {
        check(pancurses::def_prog_mode())
    }
    /// Save the current terminal state as shell mode (not in curses).
    ///
    /// This is done automatically by [`initscr`].
    ///
    /// This corresponds of `def_shell_mode`.
    ///
    /// [`initscr`]: fn.initscr.html
    pub fn define_shell_mode(&mut self) -> Result<(), ()> {
        check(pancurses::def_shell_mode())
    }
    /// Restore the terminal to program mode (in curses).
    ///
    /// This corresponds of `reset_prog_mode`.
    pub fn restore_program_mode(&mut self) -> Result<(), ()> {
        check(pancurses::reset_prog_mode())
    }
    /// Restore the terminal to program mode (not in curses).
    ///
    /// This corresponds of `reset_shell_mode`.
    pub fn restore_shell_mode(&mut self) -> Result<(), ()> {
        check(pancurses::reset_shell_mode())
    }

    /// Get the output rate of the terminal in bits per second.
    ///
    /// This corresponds to `baudrate`.
    pub fn output_rate(&self) -> BitsPerSecond {
        BitsPerSecond {
            bps: pancurses::baudrate(),
        }
    }
    /// Insert a millisecond pause in output.  *Don't use this extensively.*
    pub fn delay_output(&mut self, time: Duration) -> Result<(), ()> {
        check(pancurses::delay_output(as_millis(time)))
    }

    /// Push updates from the virtual screen to the physical screen.
    ///
    /// This corresponds of `doupdate`.
    pub fn update(&mut self) -> Result<(), ()> {
        check(pancurses::doupdate())
    }

    /// Control whether characters typed by the user are written to the screen
    /// as they are typed.
    ///
    /// If enabled, characters typed by the user are written to the screen as
    /// they are typed.
    ///
    /// If disabled, characters typed by the user are interpretted by the
    /// program and not echoed to the screen.
    ///
    /// This corresponds of `echo` and `noecho`.
    pub fn set_echo_input(&mut self, echo: bool) -> Result<(), ()> {
        if echo {
            check(pancurses::echo())
        } else {
            check(pancurses::noecho())
        }
    }
    /// Set the input buffering mode.
    ///
    /// See [`InputBufferingMode`] for more informatation.
    ///
    /// This corresponds of `cbreak`, `nocbreak`, `raw`, and `noraw`.
    ///
    /// [`InputBufferingMode`]: enum.InputBufferingMode.html
    pub fn set_input_buffering_mode(&mut self, mode: InputBufferingMode) -> Result<(), ()> {
        match mode {
            InputBufferingMode::Buffered => {
                let l = check(pancurses::noraw());
                let r = check(pancurses::nocbreak());
                l.and(r)
            }
            InputBufferingMode::UnbufferedWithSignals => {
                let l = check(pancurses::noraw());
                let r = check(pancurses::cbreak());
                l.and(r)
            }
            InputBufferingMode::UnbufferedNoSignals => {
                let l = check(pancurses::cbreak());
                let r = check(pancurses::raw());
                l.and(r)
            }
        }
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
    /// This corresponds of `nl` and `nonl`.
    pub fn set_translate_new_lines(&mut self, translate: bool) -> Result<(), ()> {
        if translate {
            check(pancurses::nl())
        } else {
            check(pancurses::nonl())
        }
    }
    /// Throw away all unread key events.
    ///
    /// This corresponds of `flushinp`.
    pub fn flush_input(&mut self) -> Result<(), ()> {
        check(pancurses::flushinp())
    }
    /// [`read_char`] will block for at most `duration` and wait for input.
    ///
    /// This will fail if `duration` is not inbetween 0.1 and 2.55 seconds.
    /// `duration` is rounded down to the nearest tenth of a second.  For more
    /// fidelity, see [`Window::set_timeout`].
    ///
    /// From reading the ncurses source code, I have deduced that this overrides
    /// the specific [`Window`]'s timeout.
    ///
    /// Use [`disable_cbreak`] to stop this.
    ///
    /// This corresponds of `halfdelay`.
    ///
    /// [`disable_cbreak`]: struct.Curses.html#method.disable_cbreak
    /// [`Window::set_timeout`]: struct.Window.html#method.set_timeout
    /// [`Window`]: struct.Window.html
    pub fn set_timeout(&mut self, duration: Duration) -> Result<(), ()> {
        let tenths = as_millis(duration) / 100;
        if tenths < 1 || tenths > 255 {
            Err(())?
        }
        check(pancurses::half_delay(tenths))
    }

    /// End the instance of curses, allowing for error handling outside of
    /// panicking.
    ///
    /// This returns the terminal to shell mode.
    ///
    /// This disposes of the main `Window`.
    ///
    /// This corresponds of `endwin`.
    pub fn end_curses(self) -> Result<(), ()> {
        let r = end_window();
        std::mem::forget(self);
        r
    }

    /// Flash the terminal screen.  If not possible, an alert is sounded.
    ///
    /// Returns `Ok` if flashing succeeds, and `Err` otherwise.
    pub fn flash(&mut self) -> Result<(), ()> {
        check(pancurses::flash())
    }

    /// Get a string representing a key code.
    ///
    /// This corresponds of `keyname`.
    pub fn key_name(&self, key_code: i32) -> Option<String> {
        let _key_name = self.key_name_mutex.lock().unwrap();
        pancurses::keyname(key_code)
    }

    /// Get the status of the mouse.
    ///
    /// This corresponds of `getmouse`.
    pub fn mouse_read(&self) -> Result<MouseEvent, ()> {
        pancurses::getmouse().map_err(|_| ())
    }
    /// Get the maximum time between press and release events for it
    /// to be recognized as a click.
    ///
    /// This corresponds of `mouseinterval(-1)`.
    pub fn mouse_interval(&self) -> Duration {
        Duration::from_millis(pancurses::mouseinterval(-1) as u64)
    }
    /// Set the maximum time between press and release events for it
    /// to be recognized as a click.
    ///
    /// As of right now this will always succeed, but it is possible
    /// this behavior will change in the future.
    ///
    /// This corresponds of `mouseinterval`.
    pub fn set_mouse_interval(&mut self, interval: Duration) -> Result<(), ()> {
        pancurses::mouseinterval(as_millis(interval));
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
    /// This corresponds of `mousemask`.
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
    /// This corresponds of `napms`.
    pub fn sleep(&mut self, duration: Duration) -> Result<(), ()> {
        check(pancurses::napms(as_millis(duration)))
    }

    /// Attempt to resize the terminal.
    ///
    /// This corresponds of `resize_term`.
    pub fn resize_terminal(&mut self, rows: i32, columns: i32) -> Result<(), ()> {
        check(pancurses::resize_term(rows, columns))
    }

    /// Attempt to beep the terminal.
    pub fn beep(&mut self) -> Result<(), ()> {
        check(pancurses::beep())
    }

    /// Control whether characters with `A_BLINK` will actually blink the screen
    /// or if it will set a high intensity background.
    ///
    /// When enabled, the screen will actually blink instead of setting a high
    /// intensity background.
    ///
    /// When disabled, Allow the terminal to either actually blink or set a high
    /// intensity background when `blink` is called.
    ///
    /// This is only supported on Windows.
    ///
    /// This corresponds of `set_blink`.
    pub fn set_force_blink(&mut self, blink: bool) -> Result<(), ()> {
        check(pancurses::set_blink(blink))
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
        end_window().unwrap();
    }
}
