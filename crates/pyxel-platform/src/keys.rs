use paste::paste;

use crate::sdl2_sys::*;

pub type Key = u32;
pub type KeyValue = i32;

// Unknown
pub const KEY_UNKNOWN: Key = SDLK_UNKNOWN as Key;

// Keyboard
pub const KEY_RETURN: Key = SDLK_RETURN as Key;
pub const KEY_ESCAPE: Key = SDLK_ESCAPE as Key;
pub const KEY_BACKSPACE: Key = SDLK_BACKSPACE as Key;
pub const KEY_TAB: Key = SDLK_TAB as Key;
pub const KEY_SPACE: Key = SDLK_SPACE as Key;
pub const KEY_EXCLAIM: Key = SDLK_EXCLAIM as Key;
pub const KEY_QUOTEDBL: Key = SDLK_QUOTEDBL as Key;
pub const KEY_HASH: Key = SDLK_HASH as Key;
pub const KEY_PERCENT: Key = SDLK_PERCENT as Key;
pub const KEY_DOLLAR: Key = SDLK_DOLLAR as Key;
pub const KEY_AMPERSAND: Key = SDLK_AMPERSAND as Key;
pub const KEY_QUOTE: Key = SDLK_QUOTE as Key;
pub const KEY_LEFTPAREN: Key = SDLK_LEFTPAREN as Key;
pub const KEY_RIGHTPAREN: Key = SDLK_RIGHTPAREN as Key;
pub const KEY_ASTERISK: Key = SDLK_ASTERISK as Key;
pub const KEY_PLUS: Key = SDLK_PLUS as Key;
pub const KEY_COMMA: Key = SDLK_COMMA as Key;
pub const KEY_MINUS: Key = SDLK_MINUS as Key;
pub const KEY_PERIOD: Key = SDLK_PERIOD as Key;
pub const KEY_SLASH: Key = SDLK_SLASH as Key;
pub const KEY_0: Key = SDLK_0 as Key;
pub const KEY_1: Key = SDLK_1 as Key;
pub const KEY_2: Key = SDLK_2 as Key;
pub const KEY_3: Key = SDLK_3 as Key;
pub const KEY_4: Key = SDLK_4 as Key;
pub const KEY_5: Key = SDLK_5 as Key;
pub const KEY_6: Key = SDLK_6 as Key;
pub const KEY_7: Key = SDLK_7 as Key;
pub const KEY_8: Key = SDLK_8 as Key;
pub const KEY_9: Key = SDLK_9 as Key;
pub const KEY_COLON: Key = SDLK_COLON as Key;
pub const KEY_SEMICOLON: Key = SDLK_SEMICOLON as Key;
pub const KEY_LESS: Key = SDLK_LESS as Key;
pub const KEY_EQUALS: Key = SDLK_EQUALS as Key;
pub const KEY_GREATER: Key = SDLK_GREATER as Key;
pub const KEY_QUESTION: Key = SDLK_QUESTION as Key;
pub const KEY_AT: Key = SDLK_AT as Key;

pub const KEY_LEFTBRACKET: Key = SDLK_LEFTBRACKET as Key;
pub const KEY_BACKSLASH: Key = SDLK_BACKSLASH as Key;
pub const KEY_RIGHTBRACKET: Key = SDLK_RIGHTBRACKET as Key;
pub const KEY_CARET: Key = SDLK_CARET as Key;
pub const KEY_UNDERSCORE: Key = SDLK_UNDERSCORE as Key;
pub const KEY_BACKQUOTE: Key = SDLK_BACKQUOTE as Key;
pub const KEY_A: Key = SDLK_a as Key;
pub const KEY_B: Key = SDLK_b as Key;
pub const KEY_C: Key = SDLK_c as Key;
pub const KEY_D: Key = SDLK_d as Key;
pub const KEY_E: Key = SDLK_e as Key;
pub const KEY_F: Key = SDLK_f as Key;
pub const KEY_G: Key = SDLK_g as Key;
pub const KEY_H: Key = SDLK_h as Key;
pub const KEY_I: Key = SDLK_i as Key;
pub const KEY_J: Key = SDLK_j as Key;
pub const KEY_K: Key = SDLK_k as Key;
pub const KEY_L: Key = SDLK_l as Key;
pub const KEY_M: Key = SDLK_m as Key;
pub const KEY_N: Key = SDLK_n as Key;
pub const KEY_O: Key = SDLK_o as Key;
pub const KEY_P: Key = SDLK_p as Key;
pub const KEY_Q: Key = SDLK_q as Key;
pub const KEY_R: Key = SDLK_r as Key;
pub const KEY_S: Key = SDLK_s as Key;
pub const KEY_T: Key = SDLK_t as Key;
pub const KEY_U: Key = SDLK_u as Key;
pub const KEY_V: Key = SDLK_v as Key;
pub const KEY_W: Key = SDLK_w as Key;
pub const KEY_X: Key = SDLK_x as Key;
pub const KEY_Y: Key = SDLK_y as Key;
pub const KEY_Z: Key = SDLK_z as Key;

pub const KEY_CAPSLOCK: Key = SDLK_CAPSLOCK as Key;

pub const KEY_F1: Key = SDLK_F1 as Key;
pub const KEY_F2: Key = SDLK_F2 as Key;
pub const KEY_F3: Key = SDLK_F3 as Key;
pub const KEY_F4: Key = SDLK_F4 as Key;
pub const KEY_F5: Key = SDLK_F5 as Key;
pub const KEY_F6: Key = SDLK_F6 as Key;
pub const KEY_F7: Key = SDLK_F7 as Key;
pub const KEY_F8: Key = SDLK_F8 as Key;
pub const KEY_F9: Key = SDLK_F9 as Key;
pub const KEY_F10: Key = SDLK_F10 as Key;
pub const KEY_F11: Key = SDLK_F11 as Key;
pub const KEY_F12: Key = SDLK_F12 as Key;

pub const KEY_PRINTSCREEN: Key = SDLK_PRINTSCREEN as Key;
pub const KEY_SCROLLLOCK: Key = SDLK_SCROLLLOCK as Key;
pub const KEY_PAUSE: Key = SDLK_PAUSE as Key;
pub const KEY_INSERT: Key = SDLK_INSERT as Key;
pub const KEY_HOME: Key = SDLK_HOME as Key;
pub const KEY_PAGEUP: Key = SDLK_PAGEUP as Key;
pub const KEY_DELETE: Key = SDLK_DELETE as Key;
pub const KEY_END: Key = SDLK_END as Key;
pub const KEY_PAGEDOWN: Key = SDLK_PAGEDOWN as Key;
pub const KEY_RIGHT: Key = SDLK_RIGHT as Key;
pub const KEY_LEFT: Key = SDLK_LEFT as Key;
pub const KEY_DOWN: Key = SDLK_DOWN as Key;
pub const KEY_UP: Key = SDLK_UP as Key;

pub const KEY_NUMLOCKCLEAR: Key = SDLK_NUMLOCKCLEAR as Key;
pub const KEY_KP_DIVIDE: Key = SDLK_KP_DIVIDE as Key;
pub const KEY_KP_MULTIPLY: Key = SDLK_KP_MULTIPLY as Key;
pub const KEY_KP_MINUS: Key = SDLK_KP_MINUS as Key;
pub const KEY_KP_PLUS: Key = SDLK_KP_PLUS as Key;
pub const KEY_KP_ENTER: Key = SDLK_KP_ENTER as Key;
pub const KEY_KP_1: Key = SDLK_KP_1 as Key;
pub const KEY_KP_2: Key = SDLK_KP_2 as Key;
pub const KEY_KP_3: Key = SDLK_KP_3 as Key;
pub const KEY_KP_4: Key = SDLK_KP_4 as Key;
pub const KEY_KP_5: Key = SDLK_KP_5 as Key;
pub const KEY_KP_6: Key = SDLK_KP_6 as Key;
pub const KEY_KP_7: Key = SDLK_KP_7 as Key;
pub const KEY_KP_8: Key = SDLK_KP_8 as Key;
pub const KEY_KP_9: Key = SDLK_KP_9 as Key;
pub const KEY_KP_0: Key = SDLK_KP_0 as Key;
pub const KEY_KP_PERIOD: Key = SDLK_KP_PERIOD as Key;

pub const KEY_APPLICATION: Key = SDLK_APPLICATION as Key;
pub const KEY_POWER: Key = SDLK_POWER as Key;
pub const KEY_KP_EQUALS: Key = SDLK_KP_EQUALS as Key;
pub const KEY_F13: Key = SDLK_F13 as Key;
pub const KEY_F14: Key = SDLK_F14 as Key;
pub const KEY_F15: Key = SDLK_F15 as Key;
pub const KEY_F16: Key = SDLK_F16 as Key;
pub const KEY_F17: Key = SDLK_F17 as Key;
pub const KEY_F18: Key = SDLK_F18 as Key;
pub const KEY_F19: Key = SDLK_F19 as Key;
pub const KEY_F20: Key = SDLK_F20 as Key;
pub const KEY_F21: Key = SDLK_F21 as Key;
pub const KEY_F22: Key = SDLK_F22 as Key;
pub const KEY_F23: Key = SDLK_F23 as Key;
pub const KEY_F24: Key = SDLK_F24 as Key;
pub const KEY_EXECUTE: Key = SDLK_EXECUTE as Key;
pub const KEY_HELP: Key = SDLK_HELP as Key;
pub const KEY_MENU: Key = SDLK_MENU as Key;
pub const KEY_SELECT: Key = SDLK_SELECT as Key;
pub const KEY_STOP: Key = SDLK_STOP as Key;
pub const KEY_AGAIN: Key = SDLK_AGAIN as Key;
pub const KEY_UNDO: Key = SDLK_UNDO as Key;
pub const KEY_CUT: Key = SDLK_CUT as Key;
pub const KEY_COPY: Key = SDLK_COPY as Key;
pub const KEY_PASTE: Key = SDLK_PASTE as Key;
pub const KEY_FIND: Key = SDLK_FIND as Key;
pub const KEY_MUTE: Key = SDLK_MUTE as Key;
pub const KEY_VOLUMEUP: Key = SDLK_VOLUMEUP as Key;
pub const KEY_VOLUMEDOWN: Key = SDLK_VOLUMEDOWN as Key;
pub const KEY_KP_COMMA: Key = SDLK_KP_COMMA as Key;
pub const KEY_KP_EQUALSAS400: Key = SDLK_KP_EQUALSAS400 as Key;

pub const KEY_ALTERASE: Key = SDLK_ALTERASE as Key;
pub const KEY_SYSREQ: Key = SDLK_SYSREQ as Key;
pub const KEY_CANCEL: Key = SDLK_CANCEL as Key;
pub const KEY_CLEAR: Key = SDLK_CLEAR as Key;
pub const KEY_PRIOR: Key = SDLK_PRIOR as Key;
pub const KEY_RETURN2: Key = SDLK_RETURN2 as Key;
pub const KEY_SEPARATOR: Key = SDLK_SEPARATOR as Key;
pub const KEY_OUT: Key = SDLK_OUT as Key;
pub const KEY_OPER: Key = SDLK_OPER as Key;
pub const KEY_CLEARAGAIN: Key = SDLK_CLEARAGAIN as Key;
pub const KEY_CRSEL: Key = SDLK_CRSEL as Key;
pub const KEY_EXSEL: Key = SDLK_EXSEL as Key;

pub const KEY_KP_00: Key = SDLK_KP_00 as Key;
pub const KEY_KP_000: Key = SDLK_KP_000 as Key;
pub const KEY_THOUSANDSSEPARATOR: Key = SDLK_THOUSANDSSEPARATOR as Key;
pub const KEY_DECIMALSEPARATOR: Key = SDLK_DECIMALSEPARATOR as Key;
pub const KEY_CURRENCYUNIT: Key = SDLK_CURRENCYUNIT as Key;
pub const KEY_CURRENCYSUBUNIT: Key = SDLK_CURRENCYSUBUNIT as Key;
pub const KEY_KP_LEFTPAREN: Key = SDLK_KP_LEFTPAREN as Key;
pub const KEY_KP_RIGHTPAREN: Key = SDLK_KP_RIGHTPAREN as Key;
pub const KEY_KP_LEFTBRACE: Key = SDLK_KP_LEFTBRACE as Key;
pub const KEY_KP_RIGHTBRACE: Key = SDLK_KP_RIGHTBRACE as Key;
pub const KEY_KP_TAB: Key = SDLK_KP_TAB as Key;
pub const KEY_KP_BACKSPACE: Key = SDLK_KP_BACKSPACE as Key;
pub const KEY_KP_A: Key = SDLK_KP_A as Key;
pub const KEY_KP_B: Key = SDLK_KP_B as Key;
pub const KEY_KP_C: Key = SDLK_KP_C as Key;
pub const KEY_KP_D: Key = SDLK_KP_D as Key;
pub const KEY_KP_E: Key = SDLK_KP_E as Key;
pub const KEY_KP_F: Key = SDLK_KP_F as Key;
pub const KEY_KP_XOR: Key = SDLK_KP_XOR as Key;
pub const KEY_KP_POWER: Key = SDLK_KP_POWER as Key;
pub const KEY_KP_PERCENT: Key = SDLK_KP_PERCENT as Key;
pub const KEY_KP_LESS: Key = SDLK_KP_LESS as Key;
pub const KEY_KP_GREATER: Key = SDLK_KP_GREATER as Key;
pub const KEY_KP_AMPERSAND: Key = SDLK_KP_AMPERSAND as Key;
pub const KEY_KP_DBLAMPERSAND: Key = SDLK_KP_DBLAMPERSAND as Key;
pub const KEY_KP_VERTICALBAR: Key = SDLK_KP_VERTICALBAR as Key;
pub const KEY_KP_DBLVERTICALBAR: Key = SDLK_KP_DBLVERTICALBAR as Key;
pub const KEY_KP_COLON: Key = SDLK_KP_COLON as Key;
pub const KEY_KP_HASH: Key = SDLK_KP_HASH as Key;
pub const KEY_KP_SPACE: Key = SDLK_KP_SPACE as Key;
pub const KEY_KP_AT: Key = SDLK_KP_AT as Key;
pub const KEY_KP_EXCLAM: Key = SDLK_KP_EXCLAM as Key;
pub const KEY_KP_MEMSTORE: Key = SDLK_KP_MEMSTORE as Key;
pub const KEY_KP_MEMRECALL: Key = SDLK_KP_MEMRECALL as Key;
pub const KEY_KP_MEMCLEAR: Key = SDLK_KP_MEMCLEAR as Key;
pub const KEY_KP_MEMADD: Key = SDLK_KP_MEMADD as Key;
pub const KEY_KP_MEMSUBTRACT: Key = SDLK_KP_MEMSUBTRACT as Key;
pub const KEY_KP_MEMMULTIPLY: Key = SDLK_KP_MEMMULTIPLY as Key;
pub const KEY_KP_MEMDIVIDE: Key = SDLK_KP_MEMDIVIDE as Key;
pub const KEY_KP_PLUSMINUS: Key = SDLK_KP_PLUSMINUS as Key;
pub const KEY_KP_CLEAR: Key = SDLK_KP_CLEAR as Key;
pub const KEY_KP_CLEARENTRY: Key = SDLK_KP_CLEARENTRY as Key;
pub const KEY_KP_BINARY: Key = SDLK_KP_BINARY as Key;
pub const KEY_KP_OCTAL: Key = SDLK_KP_OCTAL as Key;
pub const KEY_KP_DECIMAL: Key = SDLK_KP_DECIMAL as Key;
pub const KEY_KP_HEXADECIMAL: Key = SDLK_KP_HEXADECIMAL as Key;

pub const KEY_LCTRL: Key = SDLK_LCTRL as Key;
pub const KEY_LSHIFT: Key = SDLK_LSHIFT as Key;
pub const KEY_LALT: Key = SDLK_LALT as Key;
pub const KEY_LGUI: Key = SDLK_LGUI as Key;
pub const KEY_RCTRL: Key = SDLK_RCTRL as Key;
pub const KEY_RSHIFT: Key = SDLK_RSHIFT as Key;
pub const KEY_RALT: Key = SDLK_RALT as Key;
pub const KEY_RGUI: Key = SDLK_RGUI as Key;

pub const KEY_MODE: Key = SDLK_MODE as Key;

pub const KEY_AUDIONEXT: Key = SDLK_AUDIONEXT as Key;
pub const KEY_AUDIOPREV: Key = SDLK_AUDIOPREV as Key;
pub const KEY_AUDIOSTOP: Key = SDLK_AUDIOSTOP as Key;
pub const KEY_AUDIOPLAY: Key = SDLK_AUDIOPLAY as Key;
pub const KEY_AUDIOMUTE: Key = SDLK_AUDIOMUTE as Key;
pub const KEY_MEDIASELECT: Key = SDLK_MEDIASELECT as Key;
pub const KEY_WWW: Key = SDLK_WWW as Key;
pub const KEY_MAIL: Key = SDLK_MAIL as Key;
pub const KEY_CALCULATOR: Key = SDLK_CALCULATOR as Key;
pub const KEY_COMPUTER: Key = SDLK_COMPUTER as Key;
pub const KEY_AC_SEARCH: Key = SDLK_AC_SEARCH as Key;
pub const KEY_AC_HOME: Key = SDLK_AC_HOME as Key;
pub const KEY_AC_BACK: Key = SDLK_AC_BACK as Key;
pub const KEY_AC_FORWARD: Key = SDLK_AC_FORWARD as Key;
pub const KEY_AC_STOP: Key = SDLK_AC_STOP as Key;
pub const KEY_AC_REFRESH: Key = SDLK_AC_REFRESH as Key;
pub const KEY_AC_BOOKMARKS: Key = SDLK_AC_BOOKMARKS as Key;

pub const KEY_BRIGHTNESSDOWN: Key = SDLK_BRIGHTNESSDOWN as Key;
pub const KEY_BRIGHTNESSUP: Key = SDLK_BRIGHTNESSUP as Key;
pub const KEY_DISPLAYSWITCH: Key = SDLK_DISPLAYSWITCH as Key;
pub const KEY_KBDILLUMTOGGLE: Key = SDLK_KBDILLUMTOGGLE as Key;
pub const KEY_KBDILLUMDOWN: Key = SDLK_KBDILLUMDOWN as Key;
pub const KEY_KBDILLUMUP: Key = SDLK_KBDILLUMUP as Key;
pub const KEY_EJECT: Key = SDLK_EJECT as Key;
pub const KEY_SLEEP: Key = SDLK_SLEEP as Key;
pub const KEY_APP1: Key = SDLK_APP1 as Key;
pub const KEY_APP2: Key = SDLK_APP2 as Key;

pub const KEY_AUDIOREWIND: Key = SDLK_AUDIOREWIND as Key;
pub const KEY_AUDIOFASTFORWARD: Key = SDLK_AUDIOFASTFORWARD as Key;

pub const KEY_SOFTLEFT: Key = SDLK_SOFTLEFT as Key;
pub const KEY_SOFTRIGHT: Key = SDLK_SOFTRIGHT as Key;
pub const KEY_CALL: Key = SDLK_CALL as Key;
pub const KEY_ENDCALL: Key = SDLK_ENDCALL as Key;

pub const SPECIAL_KEY_START_INDEX: Key = 10000;
pub const KEY_NONE: Key = SPECIAL_KEY_START_INDEX;
pub const KEY_SHIFT: Key = SPECIAL_KEY_START_INDEX + 1;
pub const KEY_CTRL: Key = SPECIAL_KEY_START_INDEX + 2;
pub const KEY_ALT: Key = SPECIAL_KEY_START_INDEX + 3;
pub const KEY_GUI: Key = SPECIAL_KEY_START_INDEX + 4;

// Mouse
pub const MOUSE_KEY_START_INDEX: Key = 11000;
pub const MOUSE_POS_X: Key = MOUSE_KEY_START_INDEX;
pub const MOUSE_POS_Y: Key = MOUSE_KEY_START_INDEX + 1;
pub const MOUSE_WHEEL_X: Key = MOUSE_KEY_START_INDEX + 2;
pub const MOUSE_WHEEL_Y: Key = MOUSE_KEY_START_INDEX + 3;
pub const MOUSE_BUTTON_LEFT: Key = MOUSE_KEY_START_INDEX + 4;
pub const MOUSE_BUTTON_MIDDLE: Key = MOUSE_KEY_START_INDEX + 5;
pub const MOUSE_BUTTON_RIGHT: Key = MOUSE_KEY_START_INDEX + 6;
pub const MOUSE_BUTTON_X1: Key = MOUSE_KEY_START_INDEX + 7;
pub const MOUSE_BUTTON_X2: Key = MOUSE_KEY_START_INDEX + 8;

// Gamepad
pub const GAMEPAD_KEY_START_INDEX: Key = 12000;
pub const GAMEPAD_KEY_INDEX_INTERVAL: Key = 1000;

macro_rules! define_gamepad_keys {
    ($gamepad_name:ident, $start_index:expr) => {
        paste! {
            pub const [<$gamepad_name _AXIS_LEFTX>]: Key  = $start_index + 0;
            pub const [<$gamepad_name _AXIS_LEFTY>]: Key  = $start_index + 1;
            pub const [<$gamepad_name _AXIS_RIGHTX>]: Key  = $start_index + 2;
            pub const [<$gamepad_name _AXIS_RIGHTY>]: Key  = $start_index + 3;
            pub const [<$gamepad_name _AXIS_TRIGGERLEFT>]: Key  = $start_index + 4;
            pub const [<$gamepad_name _AXIS_TRIGGERRIGHT>]: Key  = $start_index + 5;
            pub const [<$gamepad_name _BUTTON_A>]: Key  = $start_index + 6;
            pub const [<$gamepad_name _BUTTON_B>]: Key  = $start_index + 7;
            pub const [<$gamepad_name _BUTTON_X>]: Key  = $start_index + 8;
            pub const [<$gamepad_name _BUTTON_Y>]: Key  = $start_index + 9;
            pub const [<$gamepad_name _BUTTON_BACK>]: Key  = $start_index + 10;
            pub const [<$gamepad_name _BUTTON_GUIDE>]: Key  = $start_index + 11;
            pub const [<$gamepad_name _BUTTON_START>]: Key  = $start_index + 12;
            pub const [<$gamepad_name _BUTTON_LEFTSTICK>]: Key  = $start_index + 13;
            pub const [<$gamepad_name _BUTTON_RIGHTSTICK>]: Key  = $start_index + 14;
            pub const [<$gamepad_name _BUTTON_LEFTSHOULDER>]: Key  = $start_index + 15;
            pub const [<$gamepad_name _BUTTON_RIGHTSHOULDER>]: Key  = $start_index + 16;
            pub const [<$gamepad_name _BUTTON_DPAD_UP>]: Key  = $start_index + 17;
            pub const [<$gamepad_name _BUTTON_DPAD_DOWN>]: Key  = $start_index + 18;
            pub const [<$gamepad_name _BUTTON_DPAD_LEFT>]: Key  = $start_index + 19;
            pub const [<$gamepad_name _BUTTON_DPAD_RIGHT>]: Key  = $start_index + 20;
            pub const [<$gamepad_name _BUTTON_MISC1>]: Key  = $start_index + 21;
            pub const [<$gamepad_name _BUTTON_PADDLE1>]: Key  = $start_index + 22;
            pub const [<$gamepad_name _BUTTON_PADDLE2>]: Key  = $start_index + 23;
            pub const [<$gamepad_name _BUTTON_PADDLE3>]: Key  = $start_index + 24;
            pub const [<$gamepad_name _BUTTON_PADDLE4>]: Key  = $start_index + 25;
            pub const [<$gamepad_name _BUTTON_TOUCHPAD>]: Key  = $start_index + 26;
        }
    };
}

define_gamepad_keys!(GAMEPAD1, GAMEPAD_KEY_START_INDEX);
define_gamepad_keys!(
    GAMEPAD2,
    GAMEPAD_KEY_START_INDEX + GAMEPAD_KEY_INDEX_INTERVAL
);
define_gamepad_keys!(
    GAMEPAD3,
    GAMEPAD_KEY_START_INDEX + GAMEPAD_KEY_INDEX_INTERVAL * 2
);
define_gamepad_keys!(
    GAMEPAD4,
    GAMEPAD_KEY_START_INDEX + GAMEPAD_KEY_INDEX_INTERVAL * 3
);
