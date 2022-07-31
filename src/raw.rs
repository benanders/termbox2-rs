
use std::os::raw::{c_int, c_char};

pub const TB_KEY_CTRL_TILDE: u16       = 0x00;
pub const TB_KEY_CTRL_2: u16           = 0x00; // clash with 'CTRL_TILDE'
pub const TB_KEY_CTRL_A: u16           = 0x01;
pub const TB_KEY_CTRL_B: u16           = 0x02;
pub const TB_KEY_CTRL_C: u16           = 0x03;
pub const TB_KEY_CTRL_D: u16           = 0x04;
pub const TB_KEY_CTRL_E: u16           = 0x05;
pub const TB_KEY_CTRL_F: u16           = 0x06;
pub const TB_KEY_CTRL_G: u16           = 0x07;
pub const TB_KEY_BACKSPACE: u16        = 0x08;
pub const TB_KEY_CTRL_H: u16           = 0x08; // clash with 'CTRL_BACKSPACE'
pub const TB_KEY_TAB: u16              = 0x09;
pub const TB_KEY_CTRL_I: u16           = 0x09; // clash with 'TAB'
pub const TB_KEY_CTRL_J: u16           = 0x0a;
pub const TB_KEY_CTRL_K: u16           = 0x0b;
pub const TB_KEY_CTRL_L: u16           = 0x0c;
pub const TB_KEY_ENTER: u16            = 0x0d;
pub const TB_KEY_CTRL_M: u16           = 0x0d; // clash with 'ENTER'
pub const TB_KEY_CTRL_N: u16           = 0x0e;
pub const TB_KEY_CTRL_O: u16           = 0x0f;
pub const TB_KEY_CTRL_P: u16           = 0x10;
pub const TB_KEY_CTRL_Q: u16           = 0x11;
pub const TB_KEY_CTRL_R: u16           = 0x12;
pub const TB_KEY_CTRL_S: u16           = 0x13;
pub const TB_KEY_CTRL_T: u16           = 0x14;
pub const TB_KEY_CTRL_U: u16           = 0x15;
pub const TB_KEY_CTRL_V: u16           = 0x16;
pub const TB_KEY_CTRL_W: u16           = 0x17;
pub const TB_KEY_CTRL_X: u16           = 0x18;
pub const TB_KEY_CTRL_Y: u16           = 0x19;
pub const TB_KEY_CTRL_Z: u16           = 0x1a;
pub const TB_KEY_ESC: u16              = 0x1b;
pub const TB_KEY_CTRL_LSQ_BRACKET: u16 = 0x1b; // clash with 'ESC'
pub const TB_KEY_CTRL_3: u16           = 0x1b; // clash with 'ESC'
pub const TB_KEY_CTRL_4: u16           = 0x1c;
pub const TB_KEY_CTRL_BACKSLASH: u16   = 0x1c; // clash with 'CTRL_4'
pub const TB_KEY_CTRL_5: u16           = 0x1d;
pub const TB_KEY_CTRL_RSQ_BRACKET: u16 = 0x1d; // clash with 'CTRL_5'
pub const TB_KEY_CTRL_6: u16           = 0x1e;
pub const TB_KEY_CTRL_7: u16           = 0x1f;
pub const TB_KEY_CTRL_SLASH: u16       = 0x1f; // clash with 'CTRL_7'
pub const TB_KEY_CTRL_UNDERSCORE: u16  = 0x1f; // clash with 'CTRL_7'
pub const TB_KEY_SPACE: u16            = 0x20;
pub const TB_KEY_BACKSPACE2: u16       = 0x7f;
pub const TB_KEY_CTRL_8: u16           = 0x7f; // clash with 'BACKSPACE2'

pub const TB_KEY_F1: u16               = 0xffff - 0;
pub const TB_KEY_F2: u16               = 0xffff - 1;
pub const TB_KEY_F3: u16               = 0xffff - 2;
pub const TB_KEY_F4: u16               = 0xffff - 3;
pub const TB_KEY_F5: u16               = 0xffff - 4;
pub const TB_KEY_F6: u16               = 0xffff - 5;
pub const TB_KEY_F7: u16               = 0xffff - 6;
pub const TB_KEY_F8: u16               = 0xffff - 7;
pub const TB_KEY_F9: u16               = 0xffff - 8;
pub const TB_KEY_F10: u16              = 0xffff - 9;
pub const TB_KEY_F11: u16              = 0xffff - 10;
pub const TB_KEY_F12: u16              = 0xffff - 11;
pub const TB_KEY_INSERT: u16           = 0xffff - 12;
pub const TB_KEY_DELETE: u16           = 0xffff - 13;
pub const TB_KEY_HOME: u16             = 0xffff - 14;
pub const TB_KEY_END: u16              = 0xffff - 15;
pub const TB_KEY_PGUP: u16             = 0xffff - 16;
pub const TB_KEY_PGDN: u16             = 0xffff - 17;
pub const TB_KEY_ARROW_UP: u16         = 0xffff - 18;
pub const TB_KEY_ARROW_DOWN: u16       = 0xffff - 19;
pub const TB_KEY_ARROW_LEFT: u16       = 0xffff - 20;
pub const TB_KEY_ARROW_RIGHT: u16      = 0xffff - 21;
pub const TB_KEY_BACK_TAB: u16         = 0xffff - 22;
pub const TB_KEY_MOUSE_LEFT: u16       = 0xffff - 23;
pub const TB_KEY_MOUSE_RIGHT: u16      = 0xffff - 24;
pub const TB_KEY_MOUSE_MIDDLE: u16     = 0xffff - 25;
pub const TB_KEY_MOUSE_RELEASE: u16    = 0xffff - 26;
pub const TB_KEY_MOUSE_WHEEL_UP: u16   = 0xffff - 27;
pub const TB_KEY_MOUSE_WHEEL_DOWN: u16 = 0xffff - 28;

pub const TB_CAP_F1: u16           = 0;
pub const TB_CAP_F2: u16           = 1;
pub const TB_CAP_F3: u16           = 2;
pub const TB_CAP_F4: u16           = 3;
pub const TB_CAP_F5: u16           = 4;
pub const TB_CAP_F6: u16           = 5;
pub const TB_CAP_F7: u16           = 6;
pub const TB_CAP_F8: u16           = 7;
pub const TB_CAP_F9: u16           = 8;
pub const TB_CAP_F10: u16          = 9;
pub const TB_CAP_F11: u16          = 10;
pub const TB_CAP_F12: u16          = 11;
pub const TB_CAP_INSERT: u16       = 12;
pub const TB_CAP_DELETE: u16       = 13;
pub const TB_CAP_HOME: u16         = 14;
pub const TB_CAP_END: u16          = 15;
pub const TB_CAP_PGUP: u16         = 16;
pub const TB_CAP_PGDN: u16         = 17;
pub const TB_CAP_ARROW_UP: u16     = 18;
pub const TB_CAP_ARROW_DOWN: u16   = 19;
pub const TB_CAP_ARROW_LEFT: u16   = 20;
pub const TB_CAP_ARROW_RIGHT: u16  = 21;
pub const TB_CAP_BACK_TAB: u16     = 22;
pub const TB_CAP__COUNT_KEYS: u16  = 23;
pub const TB_CAP_ENTER_CA: u16     = 23;
pub const TB_CAP_EXIT_CA: u16      = 24;
pub const TB_CAP_SHOW_CURSOR: u16  = 25;
pub const TB_CAP_HIDE_CURSOR: u16  = 26;
pub const TB_CAP_CLEAR_SCREEN: u16 = 27;
pub const TB_CAP_SGR0: u16         = 28;
pub const TB_CAP_UNDERLINE: u16    = 29;
pub const TB_CAP_BOLD: u16         = 30;
pub const TB_CAP_BLINK: u16        = 31;
pub const TB_CAP_ITALIC: u16       = 32;
pub const TB_CAP_REVERSE: u16      = 33;
pub const TB_CAP_ENTER_KEYPAD: u16 = 34;
pub const TB_CAP_EXIT_KEYPAD: u16  = 35;
pub const TB_CAP__COUNT: u16       = 36;

pub const TB_HARDCAP_ENTER_MOUSE: &'static str = "\x1b[?1000h\x1b[?1002h\x1b[?1015h\x1b[?1006h";
pub const TB_HARDCAP_EXIT_MOUSE: &'static str  = "\x1b[?1006l\x1b[?1015l\x1b[?1002l\x1b[?1000l";

pub const TB_DEFAULT: u32             = 0x0000;
pub const TB_BLACK: u32               = 0x0001;
pub const TB_RED: u32                 = 0x0002;
pub const TB_GREEN: u32               = 0x0003;
pub const TB_YELLOW: u32              = 0x0004;
pub const TB_BLUE: u32                = 0x0005;
pub const TB_MAGENTA: u32             = 0x0006;
pub const TB_CYAN: u32                = 0x0007;
pub const TB_WHITE: u32               = 0x0008;
pub const TB_BOLD: u32                = 0x0100;
pub const TB_UNDERLINE: u32           = 0x0200;
pub const TB_REVERSE: u32             = 0x0400;
pub const TB_ITALIC: u32              = 0x0800;
pub const TB_BLINK: u32               = 0x1000;
pub const TB_TRUECOLOR_BOLD: u32      = 0x01000000;
pub const TB_TRUECOLOR_UNDERLINE: u32 = 0x02000000;
pub const TB_TRUECOLOR_REVERSE: u32   = 0x04000000;
pub const TB_TRUECOLOR_ITALIC: u32    = 0x08000000;
pub const TB_TRUECOLOR_BLINK: u32     = 0x10000000;

pub const TB_EVENT_KEY: u8    = 1;
pub const TB_EVENT_RESIZE: u8 = 2;
pub const TB_EVENT_MOUSE: u8  = 3;

pub const TB_MOD_ALT: u8    = 1;
pub const TB_MOD_CTRL: u8   = 2;
pub const TB_MOD_SHIFT: u8  = 4;
pub const TB_MOD_MOTION: u8 = 8;

pub const TB_INPUT_CURRENT: c_int = 0;
pub const TB_INPUT_ESC: c_int     = 1;
pub const TB_INPUT_ALT: c_int     = 2;
pub const TB_INPUT_MOUSE: c_int   = 4;

pub const TB_OUTPUT_CURRENT: c_int   = 0;
pub const TB_OUTPUT_NORMAL: c_int    = 1;
pub const TB_OUTPUT_256: c_int       = 2;
pub const TB_OUTPUT_216: c_int       = 3;
pub const TB_OUTPUT_GRAYSCALE: c_int = 4;
pub const TB_OUTPUT_TRUECOLOR: c_int = 5;

pub const TB_OK: c_int                   = 0;
pub const TB_ERR: c_int                  = -1;
pub const TB_ERR_NEED_MORE: c_int        = -2;
pub const TB_ERR_INIT_ALREADY: c_int     = -3;
pub const TB_ERR_INIT_OPEN: c_int        = -4;
pub const TB_ERR_MEM: c_int              = -5;
pub const TB_ERR_NO_EVENT: c_int         = -6;
pub const TB_ERR_NO_TERM: c_int          = -7;
pub const TB_ERR_NOT_INIT: c_int         = -8;
pub const TB_ERR_OUT_OF_BOUNDS: c_int    = -9;
pub const TB_ERR_READ: c_int             = -10;
pub const TB_ERR_RESIZE_IOCTL: c_int     = -11;
pub const TB_ERR_RESIZE_PIPE: c_int      = -12;
pub const TB_ERR_RESIZE_SIGACTION: c_int = -13;
pub const TB_ERR_POLL: c_int             = -14;
pub const TB_ERR_TCGETATTR: c_int        = -15;
pub const TB_ERR_TCSETATTR: c_int        = -16;
pub const TB_ERR_UNSUPPORTED_TERM: c_int = -17;
pub const TB_ERR_RESIZE_WRITE: c_int     = -18;
pub const TB_ERR_RESIZE_POLL: c_int      = -19;
pub const TB_ERR_RESIZE_READ: c_int      = -20;
pub const TB_ERR_RESIZE_SSCANF: c_int    = -21;
pub const TB_ERR_CAP_COLLISION: c_int    = -22;
pub const TB_ERR_SELECT: c_int           = TB_ERR_POLL;
pub const TB_ERR_RESIZE_SELECT: c_int    = TB_ERR_RESIZE_POLL;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Cell {
    pub ch: u32,
    pub fg: u32,
    pub bg: u32,
    pub ech: *mut u32,
    pub nech: usize,
    pub cech: usize,
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct Event {
    pub etype: u8,
    pub emod: u8,
    pub key: u16,
    pub ch: u32,
    pub w: i32,
    pub h: i32,
    pub x: i32,
    pub y: i32,
}

extern "C" {
    pub fn tb_init() -> c_int;
    pub fn tb_shutdown() -> c_int;
    pub fn tb_width() -> c_int;
    pub fn tb_height() -> c_int;
    pub fn tb_clear() -> c_int;
    pub fn tb_set_clear_attrs(fg: u32, bg: u32) -> c_int;
    pub fn tb_present() -> c_int;
    pub fn tb_set_cursor(cx: c_int, cy: c_int) -> c_int;
    pub fn tb_hide_cursor() -> c_int;
    pub fn tb_set_cell(x: c_int, y: c_int, ch: u32, fg: u32, bg: u32) -> c_int;
    pub fn tb_set_cell_ex(x: c_int, y: c_int, ch: *const u32, nch: usize, fg: u32, bg: u32) -> c_int;
    pub fn tb_extend_cell(x: c_int, y: c_int, ch: u32) -> c_int;
    pub fn tb_set_input_mode(mode: c_int) -> c_int;
    pub fn tb_set_output_mode(mode: c_int) -> c_int;
    pub fn tb_peek_event(event: *mut Event, timeout_ms: c_int) -> c_int;
    pub fn tb_poll_event(event: *mut Event) -> c_int;
    pub fn tb_print(x: c_int, y: c_int, fg: u32, bg: u32, s: *const c_char) -> c_int;
    pub fn tb_send(buf: *const c_char, nbuf: usize);

    pub fn tb_utf8_char_length(c: c_char) -> c_int;
    pub fn tb_utf8_char_to_unicode(out: *mut u32, c: *const c_char) -> c_int;
    pub fn tb_utf8_unicode_to_char(out: *mut c_char, c: u32) -> c_int;

    pub fn tb_last_errno() -> c_int;
    pub fn tb_strerror(errno: c_int) -> *const c_char;
    pub fn tb_cell_buffer() -> *mut Cell;
    pub fn tb_has_truecolor() -> c_int;
    pub fn tb_has_egc() -> c_int;
}
