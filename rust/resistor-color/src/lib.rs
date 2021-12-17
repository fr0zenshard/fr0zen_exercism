use int_enum::IntEnum;
use std::fmt;
use enum_iterator::IntoEnumIterator;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, IntoEnumIterator)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self)
    }
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    usize::from(_color.int_value())
}   

pub fn value_to_color_string(value: u8) -> String {
    match ResistorColor::from_int(value) {
        Ok(color) => color.to_string(),
        Err(_) => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut colors: Vec<ResistorColor> = Vec::new();
    let mut iter = ResistorColor::into_enum_iter();
    for _ in 0..10 {
        colors.push(iter.next().unwrap());
    };
    return colors
}
