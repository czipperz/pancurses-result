use general::*;
use std::marker::PhantomData;

/// A color value represented as RGB
pub struct ColorContent {
    pub red: i16,
    pub green: i16,
    pub blue: i16,
}
impl From<(i16, i16, i16)> for ColorContent {
    fn from(p: (i16, i16, i16)) -> Self {
        ColorContent {
            red: p.0,
            green: p.1,
            blue: p.2,
        }
    }
}
impl From<ColorContent> for (i16, i16, i16) {
    fn from(color_content: ColorContent) -> (i16, i16, i16) {
        (color_content.red, color_content.green, color_content.blue)
    }
}

pub struct Color {
    marker: PhantomData<()>,
}

impl Color {
    pub(crate) fn new() -> Self {
        Color { marker: PhantomData }
    }

    /// The maximum number of colors supported
    pub fn max_colors(&self) -> i32 {
        pancurses::COLORS()
    }
    /// Get the `n`th color pair
    pub fn color_pair<T: Into<Chtype>>(&self, n: T) -> Chtype {
        pancurses::COLOR_PAIR(n.into())
    }
    /// Get the number of color pairs
    pub fn color_pairs(&self) -> i32 {
        pancurses::COLOR_PAIRS()
    }
    /// Get the [`ColorContent`] of a certain color
    ///
    /// [`ColorContent`]: struct.ColorContent.html
    pub fn color_content(&self, color: i16) -> ColorContent {
        pancurses::color_content(color).into()
    }
    /// Is it possible to change colors?
    pub fn can_change_color(&self) -> bool {
        pancurses::can_change_color()
    }
    /// Tell the curses instance to use default colors
    pub fn use_default_colors(&mut self) -> Result<(), ()> {
        check(pancurses::use_default_colors())
    }
    /// Set the nth color to a certain [`ColorContent`]
    ///
    /// [`ColorContent`]: struct.ColorContent.html
    pub fn set_color(&mut self, color: i16, color_content: ColorContent) -> Result<(), ()> {
        check(pancurses::init_color(
            color,
            color_content.red,
            color_content.green,
            color_content.blue,
        ))
    }
    /// Set the `color_pair` to a combination of the `foregrond` and `background` colors.
    pub fn set_color_pair(
        &mut self,
        color_pair: i16,
        foreground: i16,
        background: i16,
    ) -> Result<(), ()> {
        check(pancurses::init_pair(color_pair, foreground, background))
    }
}
