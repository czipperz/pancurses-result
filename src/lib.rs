extern crate pancurses;
#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

pub use pancurses::{Attribute, Attributes, ColorPair, Input};
pub use pancurses::{
    ACS_BBSS, ACS_BLOCK, ACS_BOARD, ACS_BSBS, ACS_BSSB, ACS_BSSS, ACS_BTEE, ACS_BULLET,
    ACS_CKBOARD, ACS_DARROW, ACS_DEGREE, ACS_DIAMOND, ACS_GEQUAL, ACS_HLINE, ACS_LANTERN,
    ACS_LARROW, ACS_LEQUAL, ACS_LLCORNER, ACS_LRCORNER, ACS_LTEE, ACS_NEQUAL, ACS_PI, ACS_PLMINUS,
    ACS_PLUS, ACS_RARROW, ACS_RTEE, ACS_S1, ACS_S3, ACS_S7, ACS_S9, ACS_SBBS, ACS_SBSB, ACS_SBSS,
    ACS_SSBB, ACS_SSBS, ACS_SSSB, ACS_SSSS, ACS_STERLING, ACS_TTEE, ACS_UARROW, ACS_ULCORNER,
    ACS_URCORNER, ACS_VLINE,
};
pub use pancurses::{
    ALL_MOUSE_EVENTS, A_ALTCHARSET, A_ATTRIBUTES, A_BLINK, A_BOLD, A_CHARTEXT, A_COLOR, A_DIM,
    A_INVIS, A_ITALIC, A_LEFTLINE, A_NORMAL, A_OVERLINE, A_REVERSE, A_RIGHTLINE, A_STANDOUT,
    A_STRIKEOUT, A_UNDERLINE, BUTTON1_CLICKED, BUTTON1_DOUBLE_CLICKED, BUTTON1_PRESSED,
    BUTTON1_RELEASED, BUTTON1_TRIPLE_CLICKED, BUTTON2_CLICKED, BUTTON2_DOUBLE_CLICKED,
    BUTTON2_PRESSED, BUTTON2_RELEASED, BUTTON2_TRIPLE_CLICKED, BUTTON3_CLICKED,
    BUTTON3_DOUBLE_CLICKED, BUTTON3_PRESSED, BUTTON3_RELEASED, BUTTON3_TRIPLE_CLICKED,
    BUTTON4_CLICKED, BUTTON4_DOUBLE_CLICKED, BUTTON4_PRESSED, BUTTON4_RELEASED,
    BUTTON4_TRIPLE_CLICKED, BUTTON5_CLICKED, BUTTON5_DOUBLE_CLICKED, BUTTON5_PRESSED,
    BUTTON5_RELEASED, BUTTON5_TRIPLE_CLICKED, BUTTON_ALT, BUTTON_CTRL, BUTTON_SHIFT, COLOR_BLACK,
    COLOR_BLUE, COLOR_CYAN, COLOR_GREEN, COLOR_MAGENTA, COLOR_RED, COLOR_WHITE, COLOR_YELLOW, ERR,
    KEY_EVENT, KEY_F15, KEY_OFFSET, KEY_RESIZE, OK, REPORT_MOUSE_POSITION, SPECIAL_KEY_CODES,
};
pub type Chtype = pancurses::chtype;
pub type MouseMask = pancurses::mmask_t;
pub type MouseEvent = pancurses::MEVENT;

fn check(r: i32) -> Result<(), ()> {
    if r == pancurses::ERR {
        Err(())
    } else {
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
    y: i32,
    x: i32,
}

impl From<(i32, i32)> for Point {
    fn from(v: (i32, i32)) -> Self {
        Point { y: v.0, x: v.1 }
    }
}

impl Into<(i32, i32)> for Point {
    fn into(self) -> (i32, i32) {
        (self.y, self.x)
    }
}

#[repr(i32)]
pub enum CursorVisibility {
    Invisible = 0,
    Normal,
    HighlyVisible,
}

pub struct Curses {
    window: Window,
    key_name_mutex: Mutex<()>,
}

impl Curses {
    pub fn window(&self) -> &Window {
        &self.window
    }
    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn baudrate(&self) -> i32 {
        pancurses::baudrate()
    }

    pub fn beep(&mut self) -> Result<(), ()> {
        check(pancurses::beep())
    }

    pub fn has_colors(&self) -> bool {
        pancurses::has_colors()
    }
    pub fn start_color(&mut self) -> Result<(), ()> {
        check(pancurses::start_color())
    }
    pub fn colors(&self) -> i32 {
        pancurses::COLORS()
    }
    pub fn color_pair<T: Into<Chtype>>(&self, n: T) -> Chtype {
        pancurses::COLOR_PAIR(n.into())
    }
    pub fn color_pairs(&self) -> i32 {
        pancurses::COLOR_PAIRS()
    }
    pub fn color_content(&self, color: i16) -> (i16, i16, i16) {
        pancurses::color_content(color)
    }
    pub fn can_change_color(&self) -> bool {
        pancurses::can_change_color()
    }
    pub fn use_default_colors(&mut self) -> Result<(), ()> {
        check(pancurses::use_default_colors())
    }
    pub fn set_color(&mut self, color: i16, r: i16, g: i16, b: i16) -> Result<(), ()> {
        check(pancurses::init_color(color, r, g, b))
    }
    pub fn set_color_pair(
        &mut self,
        pair: i16,
        foreground: i16,
        background: i16,
    ) -> Result<(), ()> {
        check(pancurses::init_pair(pair, foreground, background))
    }

    pub fn enable_cbreak_mode(&mut self) -> Result<(), ()> {
        check(pancurses::cbreak())
    }
    pub fn disable_cbreak_mode(&mut self) -> Result<(), ()> {
        check(pancurses::nocbreak())
    }

    pub fn set_cursor_visibility(&mut self, visibility: CursorVisibility) -> Result<(), ()> {
        check(pancurses::curs_set(unsafe {
            std::mem::transmute(visibility)
        }))
    }

    pub fn define_program_mode(&mut self) -> Result<(), ()> {
        check(pancurses::def_prog_mode())
    }
    pub fn define_shell_mode(&mut self) -> Result<(), ()> {
        check(pancurses::def_shell_mode())
    }

    pub fn delay_output_milliseconds(&mut self, milliseconds: i32) -> Result<(), ()> {
        check(pancurses::delay_output(milliseconds))
    }

    pub fn update(&mut self) -> Result<(), ()> {
        check(pancurses::doupdate())
    }

    pub fn enable_echo_mode(&mut self) -> Result<(), ()> {
        check(pancurses::echo())
    }
    pub fn disable_echo_mode(&mut self) -> Result<(), ()> {
        check(pancurses::noecho())
    }

    pub fn end_window(&mut self) -> Result<(), ()> {
        let mut initialized = INITIALIZED.lock().unwrap();
        if *initialized {
            check(pancurses::endwin())?;
            *initialized = false;
        }
        Ok(())
    }

    pub fn flash(&mut self) -> Result<(), ()> {
        check(pancurses::flash())
    }

    pub fn key_name(&self, key_code: i32) -> Option<String> {
        let _key_name = self.key_name_mutex.lock().unwrap();
        pancurses::keyname(key_code)
    }

    pub fn mouse_status(&self) -> Result<MouseEvent, ()> {
        pancurses::getmouse().map_err(|_| ())
    }
    pub fn mouse_interval_milliseconds(&self) -> i32 {
        pancurses::mouseinterval(-1)
    }
    pub fn set_mouse_interval_milliseconds(&mut self, milliseconds: i32) {
        pancurses::mouseinterval(milliseconds);
    }
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

    pub fn suspend_milliseconds(&mut self, milliseconds: i32) -> Result<(), ()> {
        check(pancurses::napms(milliseconds))
    }

    pub fn enable_new_line_mode(&mut self) -> Result<(), ()> {
        check(pancurses::nl())
    }
    pub fn disable_new_line_mode(&mut self) -> Result<(), ()> {
        check(pancurses::nonl())
    }

    pub fn enable_raw_mode(&mut self) -> Result<(), ()> {
        check(pancurses::raw())
    }
    pub fn disable_raw_mode(&mut self) -> Result<(), ()> {
        check(pancurses::noraw())
    }

    pub fn restore_program_mode(&mut self) -> Result<(), ()> {
        check(pancurses::reset_prog_mode())
    }
    pub fn restore_shell_mode(&mut self) -> Result<(), ()> {
        check(pancurses::reset_shell_mode())
    }

    pub fn resize_terminal(&mut self, rows: i32, columns: i32) -> Result<(), ()> {
        check(pancurses::resize_term(rows, columns))
    }

    pub fn enable_force_blink(&mut self) -> Result<(), ()> {
        check(pancurses::set_blink(true))
    }
    pub fn disable_force_blink(&mut self) -> Result<(), ()> {
        check(pancurses::set_blink(false))
    }

    pub fn set_title<T: AsRef<str>>(&mut self, title: T) -> Result<(), ()> {
        Ok(pancurses::set_title(title.as_ref()))
    }
}

impl Drop for Curses {
    fn drop(&mut self) {
        self.end_window().unwrap();
    }
}

lazy_static! {
    static ref INITIALIZED: Mutex<bool> = Mutex::new(false);
}

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
    Ok(Curses {
        window: Window { w },
        key_name_mutex: Mutex::new(()),
    })
}

pub struct Window {
    w: pancurses::Window,
}

impl Window {
    pub fn add_char<T: Into<Chtype>>(&mut self, ch: T) -> Result<(), ()> {
        check(self.w.addch(ch.into()))
    }
    pub fn add_str<T: AsRef<str>>(&mut self, string: T) -> Result<(), ()> {
        check(self.w.addstr(string))
    }
    pub fn add_str_n<T: AsRef<str>>(&mut self, string: T, length: usize) -> Result<(), ()> {
        check(self.w.addnstr(string, length))
    }

    pub fn attributes(&self) -> (Chtype, i16) {
        self.w.attrget()
    }
    pub fn turn_off_attributes<T: Into<Chtype>>(&mut self, attributes: T) -> Result<(), ()> {
        check(self.w.attroff(attributes))
    }
    pub fn turn_on_attributes<T: Into<Chtype>>(&mut self, attributes: T) -> Result<(), ()> {
        check(self.w.attron(attributes))
    }
    pub fn set_attributes<T: Into<Chtype>>(&mut self, attributes: T) -> Result<(), ()> {
        check(self.w.attrset(attributes))
    }
    pub fn change_attributes<T: Into<Chtype>>(
        &mut self,
        n: i32,
        attributes: T,
        color_pair: i16,
    ) -> Result<(), ()> {
        check(self.w.chgat(n, attributes.into(), color_pair))
    }

    pub fn set_background<T: Into<Chtype>>(&mut self, ch: T) {
        self.w.bkgdset(ch)
    }
    pub fn set_background_and_apply<T: Into<Chtype>>(&mut self, ch: T) -> Result<(), ()> {
        check(self.w.bkgd(ch))
    }

    pub fn clear(&mut self) -> Result<(), ()> {
        check(self.w.clear())
    }
    pub fn enable_clear_after_next_refresh(&mut self) -> Result<(), ()> {
        check(self.w.clearok(true))
    }
    pub fn disable_clear_after_next_refresh(&mut self) -> Result<(), ()> {
        check(self.w.clearok(false))
    }
    pub fn clear_to_bottom(&mut self) -> Result<(), ()> {
        check(self.w.clrtobot())
    }
    pub fn clear_to_end_of_line(&mut self) -> Result<(), ()> {
        check(self.w.clrtoeol())
    }

    pub fn set_color(&mut self, color_pair: i16) -> Result<(), ()> {
        check(self.w.color_set(color_pair))
    }

    pub fn copy_overlay<P1: Into<Point>, P2: Into<Point>, P3: Into<Point>>(
        &self,
        destination: &Window,
        source_start: P1,
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
    pub fn copy_overwrite<P1: Into<Point>, P2: Into<Point>, P3: Into<Point>>(
        &self,
        destination: &mut Window,
        source_start: P1,
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

    pub fn delete_char(&mut self) -> Result<(), ()> {
        check(self.w.delch())
    }
    pub fn delete_line(&mut self) -> Result<(), ()> {
        check(self.w.deleteln())
    }
    pub fn delete_window(self) -> Result<(), ()> {
        check(self.w.delwin())
    }

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
    pub fn draw_box<VT: Into<Chtype>, HT: Into<Chtype>>(
        &mut self,
        vertical: VT,
        horizontal: HT,
    ) -> Result<(), ()> {
        check(self.w.draw_box(vertical.into(), horizontal.into()))
    }
    pub fn draw_horizontal_line<T: Into<Chtype>>(
        &mut self,
        ch: T,
        max_length: i32,
    ) -> Result<(), ()> {
        check(self.w.hline(ch.into(), max_length))
    }
    pub fn draw_vertical_line<T: Into<Chtype>>(
        &mut self,
        ch: T,
        max_length: i32,
    ) -> Result<(), ()> {
        check(self.w.vline(ch.into(), max_length))
    }

    pub fn encloses(&self, y: i32, x: i32) -> bool {
        self.w.enclose(y, x)
    }

    pub fn erase(&mut self) -> Result<(), ()> {
        check(self.w.erase())
    }

    pub fn start(&self) -> Point {
        self.w.get_beg_yx().into()
    }
    pub fn start_y(&self) -> i32 {
        self.w.get_beg_y()
    }
    pub fn start_x(&self) -> i32 {
        self.w.get_beg_x()
    }

    pub fn current(&self) -> Point {
        self.w.get_cur_yx().into()
    }
    pub fn current_y(&self) -> i32 {
        self.w.get_cur_y()
    }
    pub fn current_x(&self) -> i32 {
        self.w.get_cur_x()
    }

    pub fn end(&self) -> Point {
        self.w.get_max_yx().into()
    }
    pub fn end_y(&self) -> i32 {
        self.w.get_max_y()
    }
    pub fn end_x(&self) -> i32 {
        self.w.get_max_x()
    }

    pub fn insert_lines(&mut self, n: i32) -> Result<(), ()> {
        check(self.w.insdelln(n))
    }
    pub fn insert_line(&mut self) -> Result<(), ()> {
        check(self.w.insertln())
    }
    pub fn insert_char<T: Into<Chtype>>(&self, ch: T) -> Result<(), ()> {
        check(self.w.insch(ch.into()))
    }

    pub fn enable_keypad(&mut self) -> Result<(), ()> {
        check(self.w.keypad(true))
    }
    pub fn disable_keypad(&mut self) -> Result<(), ()> {
        check(self.w.keypad(false))
    }

    pub fn window_to_screen<P: Into<Point>>(&self, p: P) -> Point {
        let p = p.into();
        self.w.mouse_trafo(p.y, p.x, true).into()
    }
    pub fn screen_to_window<P: Into<Point>>(&self, p: P) -> Point {
        let p = p.into();
        self.w.mouse_trafo(p.y, p.x, false).into()
    }

    pub fn move_to<P: Into<Point>>(&mut self, p: P) -> Result<(), ()> {
        let p = p.into();
        check(self.w.mv(p.y, p.x))
    }
    pub fn move_add_char<P: Into<Point>, T: Into<Chtype>>(
        &mut self,
        p: P,
        ch: T,
    ) -> Result<(), ()> {
        let p = p.into();
        check(self.w.mvaddch(p.y, p.x, ch.into()))
    }
    pub fn move_add_str<P: Into<Point>, T: AsRef<str>>(
        &mut self,
        p: P,
        string: T,
    ) -> Result<(), ()> {
        let p = p.into();
        check(self.w.mvaddstr(p.y, p.x, string))
    }
    pub fn move_add_str_n<P: Into<Point>, T: AsRef<str>>(
        &mut self,
        p: P,
        string: T,
        n: i32,
    ) -> Result<(), ()> {
        let p = p.into();
        check(self.w.mvaddnstr(p.y, p.x, string, n))
    }
    pub fn move_change_attributes<P: Into<Point>, T: Into<Chtype>>(
        &mut self,
        p: P,
        n: i32,
        attributes: T,
        color_pair: i16,
    ) -> Result<(), ()> {
        let p = p.into();
        check(self.w.mvchgat(p.y, p.x, n, attributes.into(), color_pair))
    }
    pub fn move_get_char<P: Into<Point>>(&mut self, p: P) -> Chtype {
        let p = p.into();
        self.w.mvinch(p.y, p.x)
    }
    pub fn move_insert_char<P: Into<Point>, T: Into<Chtype>>(
        &mut self,
        p: P,
        ch: T,
    ) -> Result<(), ()> {
        let p = p.into();
        check(self.w.mvinsch(p.y, p.x, ch.into()))
    }
    pub fn move_derived_window<P: Into<Point>>(&mut self, p: P) -> Result<(), ()> {
        let p = p.into();
        check(self.w.mvderwin(p.y, p.x))
    }
    pub fn move_window<P: Into<Point>>(&mut self, p: P) -> Result<(), ()> {
        let p = p.into();
        check(self.w.mvwin(p.y, p.x))
    }

    pub fn read(&mut self) -> Option<Input> {
        self.w.getch()
    }
    pub fn flush_input(&mut self) -> Result<(), ()> {
        check(pancurses::flushinp())
    }
    pub fn unread(&mut self, input: &Input) -> Result<(), ()> {
        check(self.w.ungetch(input))
    }
    pub fn enable_read_delay(&mut self) -> Result<(), ()> {
        check(self.w.nodelay(false))
    }
    pub fn disable_read_delay(&mut self) -> Result<(), ()> {
        check(self.w.nodelay(true))
    }
    pub fn set_timeout_milliseconds(&self, milliseconds: i32) {
        self.w.timeout(milliseconds)
    }
    pub fn set_input_timeout_tenths(&mut self, tenths: i32) -> Result<(), ()> {
        check(pancurses::half_delay(tenths))
    }

    pub fn overlay(&self, destination: &Window) -> Result<(), ()> {
        check(self.w.overlay(&destination.w))
    }
    pub fn overwrite(&self, destination: &mut Window) -> Result<(), ()> {
        check(self.w.overwrite(&destination.w))
    }

    pub fn refresh(&self) -> Result<(), ()> {
        check(self.w.refresh())
    }
    pub fn refresh_virtual_screen(&self) -> Result<(), ()> {
        check(self.w.noutrefresh())
    }

    pub fn enable_scroll(&mut self) -> Result<(), ()> {
        check(self.w.scrollok(true))
    }
    pub fn disable_scroll(&mut self) -> Result<(), ()> {
        check(self.w.scrollok(false))
    }
    pub fn set_scroll_region(&mut self, start: i32, end: i32) -> Result<(), ()> {
        check(self.w.setscrreg(start, end))
    }

    pub fn create_sub_window<P: Into<Point>>(
        &self,
        lines: i32,
        columns: i32,
        p: P,
    ) -> Result<Window, ()> {
        let p = p.into();
        match self.w.subwin(lines, columns, p.y, p.x) {
            Ok(w) => Ok(Window { w }),
            Err(_) => Err(()),
        }
    }

    pub fn line_touched(&self, line: i32) -> bool {
        self.w.is_linetouched(line)
    }
    pub fn touched(&self) -> bool {
        self.w.is_touched()
    }
    pub fn touch(&mut self) -> Result<(), ()> {
        check(self.w.touch())
    }
    pub fn touch_lines(&mut self, start: i32, count: i32) -> Result<(), ()> {
        check(self.w.touchline(start, count))
    }
    pub fn untouch(&mut self) -> Result<(), ()> {
        check(self.w.untouch())
    }
    pub fn untouch_lines(&mut self, start: i32, count: i32) -> Result<(), ()> {
        check(self.w.touchln(start, count, false))
    }
}

impl Clone for Window {
    fn clone(&self) -> Self {
        Window { w: self.w.dupwin() }
    }
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}

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
