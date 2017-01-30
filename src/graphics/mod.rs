extern crate aperutils;
use self::aperutils::maths;
use std::cmp;

pub struct Color
{
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

impl Color
{
    /// Creates a new Color who is lighter than original.
    pub fn lighter(&self) -> Color
    {
        let i: u8 = (1.0 / 0.3) as u8;
        let mut red = self.red;
        let mut green = self.green;
        let mut blue = self.blue;

        if red == 0 && green == 0 && blue == 0
            {
                return Color { red: i, green: i, blue: i, alpha: self.alpha };
            }
        if red > 0 && red < i
            {
                red = i;
            }
        if green > 0 && green < i
            {
                green = i;
            }
        if blue > 0 && blue < i
            {
                blue = i;
            }

        red = maths::min(red as f32 / 0.7, 255.0) as u8;
        green = maths::min(green as f32 / 0.7, 255.0) as u8;
        blue = maths::min(blue as f32 / 0.7, 255.0) as u8;

        return Color { red: cmp::min(red, 255),
            green: cmp::min(green, 255),
            blue: cmp::min(blue, 255),
            alpha: self.alpha }
    }

    /// Creates a new Color who is darker than original.
    pub fn darker(&self) -> Color
    {
        Color
            {
                red: cmp::max((self.red as f32 * 0.7) as i8, 0) as u8,
                green: cmp::max((self.green as f32 * 0.7) as i8, 0) as u8,
                blue: cmp::max((self.blue as f32 * 0.7) as i8, 0) as u8,
                alpha: self.alpha
            }
    }

    pub fn to_float(&self) -> (f32, f32, f32, f32)
    {
        (self.red as f32 / 255.0, self.green as f32 / 255.0, self.blue as f32 / 255.0, self.alpha as f32 / 255.0)
    }
}

/// The black color.
pub static COLOR_BLACK: Color = Color { red: 0, green: 0, blue: 0, alpha: 255 };
/// The white color.
pub static COLOR_WHITE: Color = Color { red: 255, green: 255, blue: 255, alpha: 255 };
/// The red color.
pub static COLOR_RED: Color = Color { red: 255, green: 0, blue: 0, alpha: 255 };
/// The green color.
pub static COLOR_GREEN: Color = Color { red: 0, green: 255, blue: 0, alpha: 255 };
/// The blue color.
pub static COLOR_BLUE: Color = Color { red: 0, green: 0, blue: 255, alpha: 255 };

pub struct IonicGraphics
{
    complement_x: f64,
    complement_y: f64
}


pub trait GeneralIonicGraphics
{
    fn set_color(&self, color: Color)
    {
        /*unsafe
            {
                gl::Color
            }*/
    }
}
