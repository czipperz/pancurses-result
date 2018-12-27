use general::*;
use point::Point;

/// A curses window.
///
/// It will clean up itself on destruction.
pub struct Window {
    w: pancurses::Window,
}

impl Window {
    pub(crate) fn new(w: pancurses::Window) -> Self {
        Window { w }
    }

    pub fn add_char<T: Into<Chtype>>(&mut self, ch: T) -> Result<(), ()> {
        check(self.w.addch(ch.into()))
    }
    pub fn add_str<T: AsRef<str>>(&mut self, string: T) -> Result<(), ()> {
        check(self.w.addstr(string))
    }
    pub fn printw<T: AsRef<str>>(&mut self, string: T) -> Result<(), ()> {
        self.add_str(string)
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

    pub fn read_char(&mut self) -> Option<Input> {
        self.w.getch()
    }
    pub fn flush_input(&mut self) -> Result<(), ()> {
        check(pancurses::flushinp())
    }
    pub fn unread_char(&mut self, input: &Input) -> Result<(), ()> {
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
