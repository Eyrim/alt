use crate::escape::ESCAPE;

pub mod bg;
pub mod fg;

const SEMICOLON: u8 = 0x3b;
const OPEN_SQUARE_BRACKET: u8 = 0x5b;
const M: u8 = 0x6d;

#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Colours {
    Grey,
    Black,
    Red,
}

impl Colours {
    pub const DEFAULT: &[u8] = &[ ESCAPE, 0x27, SEMICOLON, 0x2f, M ];
    
    pub fn get_full_ansi_code(fg: Self, bg: Self) -> String {
        let fg_ansi = Self::get_ansi_fg(fg);
        let bg_ansi = Self::get_ansi_bg(bg);

        format!(r#"\x1b[{fg_ansi};{bg_ansi}m"#)
        // vec![ESCAPE, OPEN_SQUARE_BRACKET, fg_ansi, SEMICOLON, bg_ansi, M]
        //     .join("")
    }

    pub fn get_ansi_fg(foreground: Self) -> u8 {
        match foreground {
            Self::Grey => 0x5a,
            Self::Black => 0x1e,
            Self::Red => 0x1f,
        }
    }

    pub fn get_ansi_bg(background: Self) -> u8 {
        match background {
            Self::Grey => 0x5a + 10,
            Self::Black => 0x1e + 10,
            Self::Red => 0x1f + 10,
        }
    }
}

