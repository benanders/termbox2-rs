
#![allow(non_upper_case_globals)]

use std::{fmt, io};
use std::os::raw::c_int;

use gag::Hold;
use bitflags::bitflags;

pub mod raw;
use raw::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum InputMode {
    #[default]
    Esc,
    Alt,
}

impl InputMode {
    fn to_tb(&self) -> c_int {
        match self {
            InputMode::Esc => TB_INPUT_ESC,
            InputMode::Alt => TB_INPUT_ALT,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum Color {
    #[default]
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    RGB(u8, u8, u8),
}

impl Color {
    fn to_8color(&self) -> u32 {
        match self {
            Color::Default => TB_DEFAULT,
            Color::Black => TB_BLACK,
            Color::Red => TB_RED,
            Color::Green => TB_GREEN,
            Color::Yellow => TB_YELLOW,
            Color::Blue => TB_BLUE,
            Color::Magenta => TB_MAGENTA,
            Color::Cyan => TB_CYAN,
            Color::White => TB_WHITE,
            Color::RGB(_, _, _) => panic!("Can't use 256-bit color in normal output mode"),
        }
    }

    fn to_256color(&self) -> u32 {
        match *self {
            Color::Black => 0x000000,
            Color::Red => 0xff0000,
            Color::Green => 0x00ff00,
            Color::Yellow => 0x00ffff,
            Color::Blue => 0x0000ff,
            Color::Magenta => 0xff00ff,
            Color::Cyan => 0x00ffff,
            Color::White => 0xffffff,
            Color::RGB(r, g, b) => (r as u32) << 16 | (g as u32) << 8 | b as u32,
            Color::Default => panic!("Can't use default color in 256-bit color mode"),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Default => write!(f, "default"),
            Color::Black => write!(f, "black"),
            Color::Red => write!(f, "red"),
            Color::Green => write!(f, "green"),
            Color::Yellow => write!(f, "yellow"),
            Color::Blue => write!(f, "blue"),
            Color::Magenta => write!(f, "magenta"),
            Color::Cyan => write!(f, "cyan"),
            Color::White => write!(f, "white"),
            Color::RGB(r, g, b) => write!(f, "rgb({}, {}, {})", r, g, b),
        }
    }
}

bitflags! {
    #[derive(Default)]
    pub struct Style: u32 {
        const None      = 0;
        const Bold      = TB_BOLD;
        const Underline = TB_UNDERLINE;
        const Reverse   = TB_REVERSE;
        const Italic    = TB_ITALIC;
        const Blink     = TB_BLINK;
    }
}

impl Style {
    fn to_8color(&self) -> u32 {
        self.bits
    }

    fn to_256color(&self) -> u32 {
        // Need extra 2 bytes to convert from TB_... to TB_TRUECOLOR_...
        self.bits << 8
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Key {
    Ctrl2OrTilde,
    CtrlA,
    CtrlB,
    CtrlC,
    CtrlD,
    CtrlE,
    CtrlF,
    CtrlG,
    BackspaceOrCtrlH,
    TabOrCtrlI,
    CtrlJ,
    CtrlK,
    CtrlL,
    EnterOrCtrlM,
    CtrlN,
    CtrlO,
    CtrlP,
    CtrlQ,
    CtrlR,
    CtrlS,
    CtrlT,
    CtrlU,
    CtrlV,
    CtrlW,
    CtrlX,
    CtrlY,
    CtrlZ,
    EscOrLSqBracketOr3,
    Ctrl4OrBackslash,
    Ctrl5OrRSqBracket,
    Ctrl6,
    Ctrl7OrSlashOrUnderscore,
    Space,
    Backspace2OrCtrl8,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Insert,
    Delete,
    Home,
    End,
    PgUp,
    PgDn,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    BackTab,
}

impl Key {
    fn from(code: u16) -> Option<Key> {
        match code {
            TB_KEY_CTRL_2 => Some(Key::Ctrl2OrTilde),
            TB_KEY_CTRL_A => Some(Key::CtrlA),
            TB_KEY_CTRL_B => Some(Key::CtrlB),
            TB_KEY_CTRL_C => Some(Key::CtrlC),
            TB_KEY_CTRL_D => Some(Key::CtrlD),
            TB_KEY_CTRL_E => Some(Key::CtrlE),
            TB_KEY_CTRL_F => Some(Key::CtrlF),
            TB_KEY_CTRL_G => Some(Key::CtrlG),
            TB_KEY_BACKSPACE => Some(Key::BackspaceOrCtrlH),
            TB_KEY_TAB => Some(Key::TabOrCtrlI),
            TB_KEY_CTRL_J => Some(Key::CtrlJ),
            TB_KEY_CTRL_K => Some(Key::CtrlK),
            TB_KEY_CTRL_L => Some(Key::CtrlL),
            TB_KEY_ENTER => Some(Key::EnterOrCtrlM),
            TB_KEY_CTRL_N => Some(Key::CtrlN),
            TB_KEY_CTRL_O => Some(Key::CtrlO),
            TB_KEY_CTRL_P => Some(Key::CtrlP),
            TB_KEY_CTRL_Q => Some(Key::CtrlQ),
            TB_KEY_CTRL_R => Some(Key::CtrlR),
            TB_KEY_CTRL_S => Some(Key::CtrlS),
            TB_KEY_CTRL_T => Some(Key::CtrlT),
            TB_KEY_CTRL_U => Some(Key::CtrlU),
            TB_KEY_CTRL_V => Some(Key::CtrlV),
            TB_KEY_CTRL_W => Some(Key::CtrlW),
            TB_KEY_CTRL_X => Some(Key::CtrlX),
            TB_KEY_CTRL_Y => Some(Key::CtrlY),
            TB_KEY_CTRL_Z => Some(Key::CtrlZ),
            TB_KEY_ESC => Some(Key::EscOrLSqBracketOr3),
            TB_KEY_CTRL_4 => Some(Key::Ctrl4OrBackslash),
            TB_KEY_CTRL_5 => Some(Key::Ctrl5OrRSqBracket),
            TB_KEY_CTRL_6 => Some(Key::Ctrl6),
            TB_KEY_CTRL_7 => Some(Key::Ctrl7OrSlashOrUnderscore),
            TB_KEY_SPACE => Some(Key::Space),
            TB_KEY_BACKSPACE2 => Some(Key::Backspace2OrCtrl8),
            TB_KEY_F1 => Some(Key::F1),
            TB_KEY_F2 => Some(Key::F2),
            TB_KEY_F3 => Some(Key::F3),
            TB_KEY_F4 => Some(Key::F4),
            TB_KEY_F5 => Some(Key::F5),
            TB_KEY_F6 => Some(Key::F6),
            TB_KEY_F7 => Some(Key::F7),
            TB_KEY_F8 => Some(Key::F8),
            TB_KEY_F9 => Some(Key::F9),
            TB_KEY_F10 => Some(Key::F10),
            TB_KEY_F11 => Some(Key::F11),
            TB_KEY_F12 => Some(Key::F12),
            TB_KEY_INSERT => Some(Key::Insert),
            TB_KEY_DELETE => Some(Key::Delete),
            TB_KEY_HOME => Some(Key::Home),
            TB_KEY_END => Some(Key::End),
            TB_KEY_PGUP => Some(Key::PgUp),
            TB_KEY_PGDN => Some(Key::PgDn),
            TB_KEY_ARROW_UP => Some(Key::ArrowUp),
            TB_KEY_ARROW_DOWN => Some(Key::ArrowDown),
            TB_KEY_ARROW_LEFT => Some(Key::ArrowLeft),
            TB_KEY_ARROW_RIGHT => Some(Key::ArrowRight),
            TB_KEY_BACK_TAB => Some(Key::BackTab),
            _ => None,
        }
    }
}

bitflags! {
    pub struct Modifiers: u8 {
        const Alt = TB_MOD_ALT;
        const Ctrl = TB_MOD_CTRL;
        const Shift = TB_MOD_SHIFT;
        const Motion = TB_MOD_MOTION;
        const AltShift = Self::Alt.bits | Self::Shift.bits;
        const CtrlShift = Self::Ctrl.bits | Self::Shift.bits;
        const CtrlAlt = Self::Ctrl.bits | Self::Alt.bits;
        const CtrlAltShift = Self::Ctrl.bits | Self::Alt.bits | Self::Shift.bits;
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Mouse {
    Left,
    Middle,
    Right,
    Release,
    WheelUp,
    WheelDown,
}

impl Mouse {
    fn from(code: u16) -> Option<Mouse> {
        match code {
            TB_KEY_MOUSE_LEFT => Some(Mouse::Left),
            TB_KEY_MOUSE_MIDDLE => Some(Mouse::Middle),
            TB_KEY_MOUSE_RIGHT => Some(Mouse::Right),
            TB_KEY_MOUSE_RELEASE => Some(Mouse::Release),
            TB_KEY_MOUSE_WHEEL_UP => Some(Mouse::WheelUp),
            TB_KEY_MOUSE_WHEEL_DOWN => Some(Mouse::WheelDown),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Event {
    Key(Key, Modifiers),
    Char(char),
    Mouse(Mouse, u32, u32),
    Resize(u32, u32),
}

impl Event {
    fn from(ev: raw::Event) -> Option<Event> {
        match ev.etype {
            TB_EVENT_KEY => {
                if ev.key == 0 {
                    Some(Event::Char(char::from_u32(ev.ch)?))
                } else {
                    Some(Event::Key(Key::from(ev.key)?, Modifiers::from_bits(ev.emod)?))
                }
            }
            TB_EVENT_RESIZE => Some(Event::Resize(ev.w as u32, ev.h as u32)),
            TB_EVENT_MOUSE => Some(Event::Mouse(Mouse::from(ev.key)?, ev.x as u32, ev.y as u32)),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum InitError {
    AlreadyInitialized,
    AlreadyOpen,
    BufferStderrFailed(io::Error),
    Other(i32),
}

impl InitError {
    fn from(code: i32) -> InitError {
        match code {
            TB_ERR_INIT_ALREADY => InitError::AlreadyInitialized,
            TB_ERR_INIT_OPEN => InitError::AlreadyOpen,
            _ => InitError::Other(code),
        }
    }
}

pub struct Term {
    _stderr: Hold,
    input_mode: InputMode,
    mouse_input: bool,
    rgb_output: bool,
}

// TODO: proper error checking
impl Term {
    pub fn new() -> Result<Term, InitError> {
        let stderr = Hold::stderr().map_err(|e| InitError::BufferStderrFailed(e))?;
        match unsafe { tb_init() } {
            TB_OK => Ok(Term {
                _stderr: stderr,
                input_mode: Default::default(),
                mouse_input: false,
                rgb_output: false,
            }),
            err => Err(InitError::from(err)),
        }
    }

    pub fn width(&self) -> u32 {
        unsafe { tb_width() as u32 }
    }

    pub fn height(&self) -> u32 {
        unsafe { tb_height() as u32 }
    }

    pub fn clear(&self) {
        unsafe { tb_clear(); }
    }

    fn to_attrs(&self, style: Style, fg: Color, bg: Color) -> (u32, u32) {
        if self.rgb_output {
            (fg.to_256color() | style.to_256color(), bg.to_256color() | style.to_256color())
        } else {
            (fg.to_8color() | style.to_8color(), bg.to_8color() | style.to_8color())
        }
    }

    pub fn set_clear_attrs(&self, style: Style, fg: Color, bg: Color) {
        let (fg, bg) = self.to_attrs(style, fg, bg);
        unsafe { tb_set_clear_attrs(fg, bg); }
    }

    pub fn present(&self) {
        unsafe { tb_present(); }
    }

    pub fn set_cursor(&self, x: u32, y: u32) {
        unsafe { tb_set_cursor(x as c_int, y as c_int); }
    }

    pub fn hide_cursor(&self) {
        unsafe { tb_hide_cursor(); }
    }

    pub fn set_cell(&self, x: u32, y: u32, ch: char, style: Style, fg: Color, bg: Color) {
        let (fg, bg) = self.to_attrs(style, fg, bg);
        unsafe { tb_set_cell(x as c_int, y as c_int, ch as u32, fg, bg); }
    }

    pub fn has_rgb_support(&self) -> bool {
        unsafe { tb_has_truecolor() != 0 }
    }

    pub fn set_rgb(&mut self, use_rgb: bool) {
        self.rgb_output = use_rgb;
        if use_rgb {
            unsafe { tb_set_output_mode(TB_OUTPUT_TRUECOLOR); }
        } else {
            unsafe { tb_set_output_mode(TB_OUTPUT_NORMAL); }
        }
    }

    pub fn set_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
        if self.mouse_input {
            unsafe { tb_set_input_mode(mode.to_tb() | TB_INPUT_MOUSE); }
        } else {
            unsafe { tb_set_input_mode(mode.to_tb()); }
        }
    }

    pub fn set_mouse_input(&mut self, mouse: bool) {
        self.mouse_input = mouse;
        if self.mouse_input {
            unsafe { tb_set_input_mode(self.input_mode.to_tb() | TB_INPUT_MOUSE); }
        } else {
            unsafe { tb_set_input_mode(self.input_mode.to_tb()); }
        }
    }

    pub fn peek_event(&self, timeout_ms: u32) -> Option<Event> {
        let mut raw = Default::default();
        match unsafe { tb_peek_event(&mut raw, timeout_ms as c_int) } {
            TB_ERR_NO_EVENT => None,
            _ => Event::from(raw),
        }
    }

    pub fn poll_event(&self) -> Event {
        loop {
            let mut raw = Default::default();
            unsafe { tb_poll_event(&mut raw); }
            let event = Event::from(raw);
            if let Some(event) = event {
                return event;
            }
        }
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        // tb_shutdown only ever produces TB_OK or TB_ERR_NOT_INIT which we can
        // safely ignore here
        unsafe { tb_shutdown(); }
    }
}
