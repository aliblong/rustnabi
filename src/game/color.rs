pub type ColorIndex = i8;
pub type ColorResult = Result<Color, ColorIndex>;

pub enum Color {
    Blue,
    Green,
    Yellow,
    Red,
    Purple,
    Orange,
    Black,
}
