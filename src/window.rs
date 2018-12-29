use general::*;
use point::*;
use std::time::Duration;

pub enum EndOfLineOrNumber {
    EndOfLine,
    Number(i32),
}

impl EndOfLineOrNumber {
    pub fn unwrap_number_or(self, eol: i32) -> i32 {
        match self {
            EndOfLineOrNumber::EndOfLine => eol,
            EndOfLineOrNumber::Number(n) => n,
        }
    }
}

impl From<i32> for EndOfLineOrNumber {
    fn from(n: i32) -> Self {
        EndOfLineOrNumber::Number(n)
    }
}

/// A curses window.
///
/// It will clean up itself on destruction.
///
/// Many curses functions have been renamed for one reason or another.  All
/// renamed functions state the curses function they corollate to.
pub struct Window {
    w: pancurses::Window,
}

impl Window {
    pub(crate) fn new(w: pancurses::Window) -> Self {
        Window { w }
    }

    /// Put a character at the point.
    ///
    /// This corresponds to `addch`.
    pub fn put_char<T: Into<Chtype>>(&mut self, ch: T) -> Result<(), ()> {
        check(self.w.addch(ch.into()))
    }
    /// Put a string at the point.
    ///
    /// This corresponds to `addch`.
    pub fn put_str<T: AsRef<str>>(&mut self, string: T) -> Result<(), ()> {
        check(self.w.addstr(string))
    }
    /// Print a formatted string at the point.
    ///
    /// This corresponds to `printw`.  It does not use `printw`
    /// under the hood because that function cannot be safe to use
    /// within rust code because rust does not allow for variadic
    /// arguments.
    pub fn printw(&mut self, args: std::fmt::Arguments) -> Result<(), ()> {
        self.put_str(args.to_string())
    }
    /// Put the contents of `source` that overlap with this `Window`.
    ///
    /// The two `Window`s are not required to be the same size;
    /// overlapping portions are copied.
    ///
    /// This corresponds to `overwrite` but *with the arguments flipped*.
    pub fn put_window(&mut self, source: &Window) -> Result<(), ()> {
        source.overwrite_onto(self)
    }
    /// Put the contents of `source` in a region at a region of this `Window`.
    ///
    /// The two regions are not required to be the same size; overlapping
    /// portions are copied.
    ///
    /// This corresponds to `copywin` but *with the arguments flipped* and a
    /// final argument of `true`.
    pub fn put_window_region<P1: Into<Point>, P2: Into<Point>, P3: Into<Point>>(
        &mut self,
        destination_start: P1,
        destination_end: P2,
        source: &Window,
        source_start: P3,
    ) -> Result<(), ()> {
        source.overwrite_region_onto(source_start, self, destination_start, destination_end)
    }
    /// Put the non-blank contents of `source` that overlap with this
    /// `Window`.
    ///
    /// The two `Window`s are not required to be the same size;
    /// overlapping portions are copied.
    ///
    /// This corresponds to `overlay` but *with the arguments flipped*.
    pub fn put_window_text(&mut self, source: &Window) -> Result<(), ()> {
        source.overlay_onto(self)
    }
    /// Put the non-blank contents of `source` in a region at a region of this `Window`.
    ///
    /// The two regions are not required to be the same size; overlapping
    /// portions are copied.
    ///
    /// This corresponds to `copywin` but *with the arguments flipped* and a
    /// final argument of `true`.
    pub fn put_window_text_region<P1: Into<Point>, P2: Into<Point>, P3: Into<Point>>(
        &mut self,
        destination_start: P1,
        destination_end: P2,
        source: &Window,
        source_start: P3,
    ) -> Result<(), ()> {
        source.overlay_region_onto(source_start, self, destination_start, destination_end)
    }

    /// Put the contents of this `Window` onto `destination` where they overlap.
    ///
    /// This corresponds to `overwrite`.
    pub fn overwrite_onto(&self, destination: &mut Self) -> Result<(), ()> {
        check(self.w.overwrite(&destination.w))
    }
    /// Put the non-blank contents of this `Window` onto `destination` where they overlap.
    ///
    /// This corresponds to `overlay`.
    pub fn overlay_onto(&self, destination: &mut Self) -> Result<(), ()> {
        check(self.w.overlay(&destination.w))
    }

    /// Overwrite this `Window` on top of the `destination`.
    ///
    /// For more information on how this works, see [`put_window`].
    ///
    /// This corresponds to `copywin` with a final argument of `true`.
    ///
    /// [`put_window`]: struct.Window.html#method.put_window
    pub fn overwrite_region_onto<P1: Into<Point>, P2: Into<Point>, P3: Into<Point>>(
        &self,
        source_start: P1,
        destination: &mut Window,
        destination_start: P2,
        destination_end: P3,
    ) -> Result<(), ()> {
        let source_start = source_start.into();
        let destination_start = destination_start.into();
        let destination_end = destination_end.into();
        check(self.w.copywin(
            &destination.w,
            source_start.y,
            source_start.x,
            destination_start.y,
            destination_start.x,
            destination_end.y,
            destination_end.x,
            true,
        ))
    }
    /// Overlay this `Window`'s text on top of the `destination`.
    ///
    /// For more information on how this works, see [`put_window_text`].
    ///
    /// This corresponds to `copywin` with a final argument of `true`.
    ///
    /// [`put_window_text`]: struct.Window.html#method.put_window_text
    pub fn overlay_region_onto<P1: Into<Point>, P2: Into<Point>, P3: Into<Point>>(
        &self,
        source_start: P1,
        destination: &mut Window,
        destination_start: P2,
        destination_end: P3,
    ) -> Result<(), ()> {
        let source_start = source_start.into();
        let destination_start = destination_start.into();
        let destination_end = destination_end.into();
        check(self.w.copywin(
            &destination.w,
            source_start.y,
            source_start.x,
            destination_start.y,
            destination_start.x,
            destination_end.y,
            destination_end.x,
            true,
        ))
    }

    /// Get the attributes of the character at the point.
    ///
    /// This corresponds to `attrget`.
    pub fn attributes(&self) -> (Chtype, i16) {
        self.w.attrget()
    }
    /// Turn off the following attributes of the character at the point.
    ///
    /// This corresponds to `attroff`.
    pub fn turn_off_attributes<T: Into<Chtype>>(&mut self, attributes: T) -> Result<(), ()> {
        check(self.w.attroff(attributes))
    }
    /// Turn on the following attributes of the character at the point.
    ///
    /// This corresponds to `attron`.
    pub fn turn_on_attributes<T: Into<Chtype>>(&mut self, attributes: T) -> Result<(), ()> {
        check(self.w.attron(attributes))
    }
    /// Set the attributes of the character at the point.
    ///
    /// This corresponds to `attrset`.
    pub fn set_attributes<T: Into<Chtype>>(&mut self, attributes: T) -> Result<(), ()> {
        check(self.w.attrset(attributes))
    }
    /// Turn off the following attributes of the character at the point.
    ///
    /// This corresponds to `chgat`.
    pub fn change_attributes<T: Into<Chtype>, N: Into<EndOfLineOrNumber>>(
        &mut self,
        n: N,
        attributes: T,
        color_pair: i16,
    ) -> Result<(), ()> {
        check(
            self.w
                .chgat(n.into().unwrap_number_or(-1), attributes.into(), color_pair),
        )
    }

    /// Set the background of the `Window`.
    ///
    /// This corresponds to `bkgdset`.
    pub fn set_background<T: Into<Chtype>>(&mut self, ch: T) {
        self.w.bkgdset(ch)
    }
    /// Set the background of the `Window` and apply it.
    ///
    /// This sets the attributes of every character to the attributes
    /// of `background` and replaces old background characters with
    /// `background`.
    ///
    /// This corresponds to `bkgd`.
    pub fn set_background_and_apply<T: Into<Chtype>>(&mut self, background: T) -> Result<(), ()> {
        check(self.w.bkgd(background))
    }

    /// Clear the screen.
    ///
    /// This fills the screen with background characters.  This can be
    /// customized via [`set_background`].
    ///
    /// Unlike [`erase`], `clear` will also invoke force a the next call to
    /// [`refresh`] to clear the screen (as if by [`refresh_force_clear`]).
    ///
    /// [`set_background`]: struct.Window.html#method.set_background
    /// [`erase`]: struct.Window.html#method.erase
    /// [`refresh`]: struct.Window.html#method.refresh
    /// [`refresh_force_clear`]: struct.Window.html#method.refresh_force_clear
    pub fn clear(&mut self) -> Result<(), ()> {
        check(self.w.clear())
    }
    /// Clear the virtual screen.
    ///
    /// See [`clear`] for a comparison of these two methods.
    ///
    /// [`clear`]: struct.Window.html#method.clear
    pub fn erase(&mut self) -> Result<(), ()> {
        check(self.w.erase())
    }
    /// Erase all characters after the point.
    ///
    /// This will clear to the end of the line, then clear each line
    /// after the one the point is on.
    ///
    /// This corresponds to `clrtobot`.
    pub fn clear_to_bottom(&mut self) -> Result<(), ()> {
        check(self.w.clrtobot())
    }
    /// Erase all characters to the right of the point on this line.
    ///
    /// This corresponds to `clrtobot`.
    pub fn clear_to_end_of_line(&mut self) -> Result<(), ()> {
        check(self.w.clrtoeol())
    }

    /// Set the current color of the given window to the
    /// foregrond/background pair `color_pair`.
    ///
    /// This corresponds to `color_set`.
    pub fn set_color(&mut self, color_pair: i16) -> Result<(), ()> {
        check(self.w.color_set(color_pair))
    }

    /// Delete the character at the point.
    ///
    /// This will shift left the characters on the rest of the line.
    ///
    /// This corresponds to `delch`.
    pub fn delete_char(&mut self) -> Result<(), ()> {
        check(self.w.delch())
    }
    /// Delete the line the point is on.
    ///
    /// This will shift left the characters on the rest of the line.
    ///
    /// This corresponds to `delch`.
    pub fn delete_line(&mut self) -> Result<(), ()> {
        check(self.w.deleteln())
    }
    /// Delete this `Window`, allowing for error handling outside of
    /// panicking.
    pub fn delete_window(self) -> Result<(), ()> {
        check(self.w.delwin())
    }

    /// Draw a border around the edges of the `Window`.
    ///
    /// This corresponds to `border`.
    pub fn draw_border<
        LS: Into<Chtype>,
        RS: Into<Chtype>,
        TS: Into<Chtype>,
        BS: Into<Chtype>,
        TLC: Into<Chtype>,
        TRC: Into<Chtype>,
        BLC: Into<Chtype>,
        BRC: Into<Chtype>,
    >(
        &mut self,
        left_side: LS,
        right_side: RS,
        top_side: TS,
        bottom_side: BS,
        top_left_corner: TLC,
        top_right_corner: TRC,
        bottom_left_corner: BLC,
        bottom_right_corner: BRC,
    ) -> Result<(), ()> {
        check(self.w.border(
            left_side.into(),
            right_side.into(),
            top_side.into(),
            bottom_side.into(),
            top_left_corner.into(),
            top_right_corner.into(),
            bottom_left_corner.into(),
            bottom_right_corner.into(),
        ))
    }
    /// Drow a box around the edges of the `Window`.
    ///
    /// This is shorthand for [`draw_border`] with default corners.
    ///
    /// This corresponds to `border`.
    ///
    /// [`draw_border`]: struct.Window.html#method.draw_border
    pub fn draw_box<VT: Into<Chtype>, HT: Into<Chtype>>(
        &mut self,
        vertical: VT,
        horizontal: HT,
    ) -> Result<(), ()> {
        check(self.w.draw_box(vertical.into(), horizontal.into()))
    }
    /// Draw a horizontal line starting at the point.
    ///
    /// This corresponds to `hline`.
    pub fn draw_horizontal_line<T: Into<Chtype>>(
        &mut self,
        ch: T,
        max_length: i32,
    ) -> Result<(), ()> {
        check(self.w.hline(ch.into(), max_length))
    }
    /// Draw a vertical line starting at the point.
    ///
    /// This corresponds to `vline`.
    pub fn draw_vertical_line<T: Into<Chtype>>(
        &mut self,
        ch: T,
        max_length: i32,
    ) -> Result<(), ()> {
        check(self.w.vline(ch.into(), max_length))
    }

    /// Test if `p` is enclosed in this `Window`.
    pub fn encloses<P: Into<Point>>(&self, p: P) -> bool {
        let p = p.into();
        self.w.enclose(p.y, p.x)
    }

    /// Get the start of the `Window` on the physical screen.
    ///
    /// This corresponds to `get_beg_yx`.
    pub fn beginning(&self) -> Point {
        self.w.get_beg_yx().into()
    }
    /// Get the ending of the `Window` on the physical screen.
    pub fn ending(&self) -> Point {
        let (y, x) = self.beginning().into();
        let (h, w) = self.size().into();
        Point { y: y + h, x: x + w }
    }

    /// Get the position of the point.
    ///
    /// This corresponds to `get_cur_yx`.
    pub fn point(&self) -> Point {
        self.w.get_cur_yx().into()
    }

    /// Get the size of the `Window`.
    ///
    /// This corresponds to `get_max_yx`.
    pub fn size(&self) -> Dimension {
        self.w.get_max_yx().into()
    }

    /// Insert `n` blank lines above the cursor.
    ///
    /// If `n > 0`, then `n` blank lines are inserted above the
    /// cursor.  The last `n` lines of the screen are lost.
    ///
    /// If `n < 0`, then `n` lines below the cursor are deleted and
    /// the rest are shifted up.  This clears the bottom `n` lines of
    /// the screen.
    ///
    /// The point remains the same after this operation.
    ///
    /// This corresponds to `insdelln`.
    pub fn insert_lines(&mut self, n: i32) -> Result<(), ()> {
        check(self.w.insdelln(n))
    }
    /// Insert a blank line above the current line.
    ///
    /// This effectively erases the last line on the screen.
    ///
    /// The point remains the same after this operation.
    ///
    /// This corresponds to `insertln`.
    pub fn insert_line(&mut self) -> Result<(), ()> {
        check(self.w.insertln())
    }
    /// Insert a character into the current line.
    ///
    /// This shifts to the right characters after the cursor.  The
    /// rightmost character will thus be lost.
    ///
    /// The point remains the same after this operation.
    ///
    /// This corresponds to `insch`.
    pub fn insert_char<T: Into<Chtype>>(&self, ch: T) -> Result<(), ()> {
        check(self.w.insch(ch.into()))
    }

    /// Transform the point `p` from `Window`-relative to screen-relative.
    ///
    /// This corresponds to `mouse_trafo`.
    pub fn window_to_screen<P: Into<Point>>(&self, p: P) -> Point {
        let p = p.into();
        self.w.mouse_trafo(p.y, p.x, true).into()
    }
    /// Transform the point `p` from screen-relative to `Window`-relative.
    ///
    /// This corresponds to `mouse_trafo`.
    pub fn screen_to_window<P: Into<Point>>(&self, p: P) -> Point {
        let p = p.into();
        self.w.mouse_trafo(p.y, p.x, false).into()
    }

    /// Move to the point to `p`.
    ///
    /// This corresponds to `mv`.
    pub fn move_to<P: Into<Point>>(&mut self, p: P) -> Result<(), ()> {
        let p = p.into();
        check(self.w.mv(p.y, p.x))
    }
    /// Move to the point `p` then put `ch` at that point.
    ///
    /// This corresponds to `mvaddch`.
    pub fn move_put_char<P: Into<Point>, T: Into<Chtype>>(
        &mut self,
        p: P,
        ch: T,
    ) -> Result<(), ()> {
        let p = p.into();
        check(self.w.mvaddch(p.y, p.x, ch.into()))
    }
    /// Move to the point `p` then put `string` at that point.
    ///
    /// This corresponds to `mvaddstr`.
    pub fn move_put_str<P: Into<Point>, T: AsRef<str>>(
        &mut self,
        p: P,
        string: T,
    ) -> Result<(), ()> {
        let p = p.into();
        check(self.w.mvaddstr(p.y, p.x, string))
    }
    /// Move to the point `p` then change the attributes of `n` characters after that point.
    ///
    /// This corresponds to `mvchgat`.
    pub fn move_change_attributes<P: Into<Point>, N: Into<EndOfLineOrNumber>, T: Into<Chtype>>(
        &mut self,
        p: P,
        n: N,
        attributes: T,
        color_pair: i16,
    ) -> Result<(), ()> {
        let p = p.into();
        check(self.w.mvchgat(
            p.y,
            p.x,
            n.into().unwrap_number_or(-1),
            attributes.into(),
            color_pair,
        ))
    }
    /// Move to `p` then get the character at the point.
    ///
    /// This corresponds to `mvinch`.
    pub fn move_get_char<P: Into<Point>>(&mut self, p: P) -> Chtype {
        let p = p.into();
        self.w.mvinch(p.y, p.x)
    }
    /// Move to `p` then insert the character at the point.
    ///
    /// This corresponds to `mvinsch`.
    pub fn move_insert_char<P: Into<Point>, T: Into<Chtype>>(
        &mut self,
        p: P,
        ch: T,
    ) -> Result<(), ()> {
        let p = p.into();
        check(self.w.mvinsch(p.y, p.x, ch.into()))
    }
    // /// Move to `p` then insert the character at the point.
    // ///
    // /// This corresponds to `mvinsch`.
    // pub fn move_derived_window<P: Into<Point>>(&mut self, p: P) -> Result<(), ()> {
    //     let p = p.into();
    //     check(self.w.mvderwin(p.y, p.x))
    // }
    /// Move the `Window` such that it starts at `p` on the screen.
    ///
    /// This corresponds to `mvwin`.
    pub fn move_window<P: Into<Point>>(&mut self, p: P) -> Result<(), ()> {
        let p = p.into();
        check(self.w.mvwin(p.y, p.x))
    }

    /// Read a key event from the `Window`.
    ///
    /// The exact behavior of this procedure depends on other configuration
    /// procedures.
    ///
    /// * To set how we handle a lack of input (with regard to
    ///   blocking/unblocking) [`set_block_on_read`], [`set_timeout`], and
    ///   [`Curses::set_timeout`].
    ///
    /// * To set how function keys are handled, see
    ///   [`read_interpolate_function_keys`].
    ///
    /// * To set how new lines are handled, see
    ///   [`Curses::set_translate_new_lines`].
    ///
    /// * To set how input is buffered, see [`Curses::set_input_buffering_mode`].
    ///
    /// * To control whether curses will automatically echo characters see
    ///   [`Curses::set_echo_input`].
    ///
    /// This corresponds to `getch`.
    ///
    /// [`set_block_on_read`]: struct.Window.html#method.set_block_on_read
    /// [`set_timeout`]: struct.Window.html#method.set_timeout
    /// [`Curses::set_timeout`]: struct.Curses.html#method.set_timeout
    /// [`read_interpolate_function_keys`]: struct.Window.html#method.read_interpolate_function_keys
    /// [`Curses::set_translate_new_lines`]: struct.Curses.html#method.set_translate_new_lines
    /// [`Curses::set_input_buffering_mode`]: struct.Curses.html#method.set_input_buffering_mode
    /// [`Curses::set_echo_input`]: struct.Curses.html#method.set_echo_input
    pub fn read_char(&mut self) -> Option<Input> {
        self.w.getch()
    }
    /// Place `input` into the front of the input queue.
    ///
    /// Thus the next call to [`read_char`] will return `input`.
    ///
    /// This corresponds to `ungetch`.
    ///
    /// [`read_char`]: struct.Window.html#method.read_char
    pub fn unread_char(&mut self, input: &Input) -> Result<(), ()> {
        check(self.w.ungetch(input))
    }
    /// Set whether [`read_char`] will block until an input is ready.
    ///
    /// With an argument `true`, [`read_char`] will block until it receives an
    /// input to yield.
    ///
    /// With an argument `false`, [`read_char`] will immediately return `None`
    /// if there is no input to yield.
    ///
    /// This corresponds to `nodelay(!block)`.
    pub fn set_block_on_read(&mut self, block: bool) -> Result<(), ()> {
        check(self.w.nodelay(!block))
    }
    /// [`read_char`] will block for at most `duration` and wait for input.
    ///
    /// `duration` is rounded down to the nearest millisecond.  This will only
    /// change the way input is read in this `Window`.  To set it for *all*
    /// `Window`s, see [`Curses::set_timeout`].
    ///
    /// From reading the ncurses source code, I have deduced that this is overriden
    /// by the global [`Curses`]'s timeout (see [`Curses::set_timeout`]).
    ///
    /// Use `None` as the timeout to stop this.
    ///
    /// This corresponds to `timeout`.
    ///
    /// [`Curses::read_char`]: struct.Curses.html#method.read_char
    /// [`Curses::set_timeout`]: struct.Curses.html#method.set_timeout
    /// [`Curses`]: struct.Curses.html
    pub fn set_timeout(&self, duration: Option<Duration>) {
        self.w.timeout(duration.map(as_millis).unwrap_or(-1))
    }
    /// Enable or disable function key interpolation.
    ///
    /// When enabled and a function key is pressed, [`read_char`] will
    /// return a single value representing the function key instead of
    /// a series of escape sequences.
    ///
    /// When disabled and a function key is pressed, [`read_char`]
    /// will return a series of escape sequences instead of a single
    /// value representing the function key.
    ///
    /// It is disabled by default.
    ///
    /// This corresponds to `keypad`.
    ///
    /// [`read_char`]: struct.Window.html#method.read_char
    pub fn read_interpolate_function_keys(&mut self, interpolate: bool) -> Result<(), ()> {
        check(self.w.keypad(interpolate))
    }

    /// Copy this `Window` to the physical screen.
    ///
    /// This corresponds to `wrefresh`.
    pub fn refresh(&mut self) -> Result<(), ()> {
        check(self.w.refresh())
    }
    /// Refresh the virtual screen.
    ///
    /// This is much faster if multiple `Window`s have to be refreshed.
    ///
    /// To push the virtual screen changes to the physical screen, use
    /// [`Curses::update`].
    ///
    /// This corresponds to `wnoutrefresh`.
    ///
    /// [`Curses::update`]: struct.Curses.html#method.update
    pub fn refresh_virtual_screen(&mut self) -> Result<(), ()> {
        check(self.w.noutrefresh())
    }
    /// Make the next call to [`refresh`] clear and then rerender.
    ///
    /// This corresponds to `clearok`.
    ///
    /// [`refresh`]: struct.Window.html#method.refresh
    pub fn refresh_force_clear(&mut self, force_clear: bool) -> Result<(), ()> {
        check(self.w.clearok(force_clear))
    }

    /// Enable or disable scrolling.
    ///
    /// This corresponds to `scrollok`.
    pub fn set_scroll_enabled(&mut self, scroll: bool) -> Result<(), ()> {
        check(self.w.scrollok(scroll))
    }
    /// Set a software scrolling region.
    ///
    /// This will do nothing unless scrolling has been enabled (see
    /// [`set_scroll_enabled`]).
    ///
    /// This corresponds to `setscrreg`.
    ///
    /// [`set_scroll_enabled`]: struct.Window.html#method.set_scroll_enabled
    pub fn set_scroll_region(&mut self, start: i32, end: i32) -> Result<(), ()> {
        check(self.w.setscrreg(start, end))
    }

    /// Create a new window
    ///
    /// This corresponds to `subwin`.  Note that the arguments have been
    /// reordered to be more consistent with other functions.
    pub fn create_sub_window<P: Into<Point>, D: Into<Dimension>>(
        &self,
        point: P,
        size: D,
    ) -> Result<Window, ()> {
        let p = point.into();
        let d = size.into();
        match self.w.subwin(d.rows, d.columns, p.y, p.x) {
            Ok(w) => Ok(Window { w }),
            Err(_) => Err(()),
        }
    }

    /// Test if this `Window` has been modified since the last call to
    /// [`refresh`].
    ///
    /// This corresponds to `is_wintouched`.
    ///
    /// [`refresh`]: struct.Window.html#method.refresh
    pub fn touched(&self) -> bool {
        self.w.is_touched()
    }
    /// Test if the specified line has been modified since the last call to
    /// [`refresh`].
    ///
    /// This corresponds to `is_linetouched`.
    ///
    /// [`refresh`]: struct.Window.html#method.refresh
    pub fn line_touched(&self, line: i32) -> bool {
        self.w.is_linetouched(line)
    }
    /// Force the entire `Window` to be redrawn upon the next call to
    /// [`refresh`].
    ///
    /// This corresponds to `touchwin`.
    ///
    /// [`refresh`]: struct.Window.html#method.refresh
    pub fn touch(&mut self) -> Result<(), ()> {
        check(self.w.touch())
    }
    /// Force the specified lines to be redrawn upon the next call to
    /// [`refresh`].
    ///
    /// This corresponds to `touchline`.
    ///
    /// [`refresh`]: struct.Window.html#method.refresh
    pub fn touch_lines(&mut self, start: i32, count: i32) -> Result<(), ()> {
        check(self.w.touchline(start, count))
    }
    /// Pretend this `Window` hasn't changed and thus won't redraw it upon the
    /// next call to [`refresh`].
    ///
    /// This corresponds to `touchline`.
    ///
    /// [`refresh`]: struct.Window.html#method.refresh
    pub fn untouch(&mut self) -> Result<(), ()> {
        check(self.w.untouch())
    }
    /// Pretend the specified lines haven't changed and thus won't redraw it
    /// upon the next call to [`refresh`].
    ///
    /// This corresponds to `touchline`.
    ///
    /// [`refresh`]: struct.Window.html#method.refresh
    pub fn untouch_lines(&mut self, start: i32, count: i32) -> Result<(), ()> {
        check(self.w.touchln(start, count, false))
    }
}

/// Duplicate this `Window`.
///
/// This corresponds to `dupwin`.
impl Clone for Window {
    fn clone(&self) -> Self {
        Window { w: self.w.dupwin() }
    }
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}
