use crate::signal::sigs;

pub type __off_t = i64;
pub type __ssize_t = i32;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
pub type lbool = u32;
pub type less_off_t = off_t;
pub type POSITION = less_off_t;
pub type LINENUM = off_t;
pub type __syscall_slong_t = i64;
pub type __time_t = i64;
pub type time_t = __time_t;
pub type __uintmax_t = i64;
pub type uintmax_t = __uintmax_t;
pub type uintmax = uintmax_t;
pub type __off64_t = i64;
pub type __mode_t = i32;
pub type mode_t = __mode_t;
pub type __ino_t = u64;
pub type ino_t = __ino_t;
pub type __dev_t = u64;
pub type dev_t = __dev_t;
pub type __blkcnt_t = i64;
pub type wint_t = i32;
pub type ansi_state = i32;
pub type __blksize_t = i32;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __nlink_t = u64;
pub type LWCHAR = i32;

pub const NULL_POSITION: i64 = -1;
pub const EOI: i32 = -1;

pub const OPT_OFF: i32 = 0;
pub const OPT_ON: i32 = 0;
pub const OPT_ONPLUS: i32 = 0;

pub const LTRUE: lbool = 1;
pub const LFALSE: lbool = 0;

pub const S_INTERRUPT: i32 = 1 << 0;
pub const S_SWINTERRUPT: i32 = 1 << 1;
pub const S_STOP: i32 = 1 << 2;
pub const S_WINCH: i32 = 1 << 3;

/* filestate flags */
pub const CH_CANSEEK: i32 = 0o1;
pub const CH_KEEPOPEN: i32 = 0o2;
pub const CH_POPENED: i32 = 0o4;
pub const CH_HELPFILE: i32 = 0o10;
pub const CH_NODATA: i32 = 0o20; /* Special case for zero length files */
pub const CH_NOTRUSTSIZE: i32 = 0o40; /* For files that claim 0 length size falsely */

pub const FAKE_HELPFILE: &'static str = "@/\\less/\\help/\\file/\\@";
pub const FAKE_EMPTYFILE: &'static str = "@/\\less/\\empty/\\file/\\@";

pub const SEEK_SET: i32 = 0;
pub const SEEK_END: i32 = 2;

pub const NUM_LOG_FRAC_DENOM: i32 = 6;

pub unsafe fn abort_sigs() -> bool {
    (sigs & (S_INTERRUPT | S_SWINTERRUPT | S_STOP)) != 0
}

/* Security features. */
pub const SF_EDIT: i32 = 1 << 1; /* Edit file (v) */
pub const SF_EXAMINE: i32 = 1 << 2; /* Examine file (:e) */
pub const SF_GLOB: i32 = 1 << 3; /* Expand file pattern */
pub const SF_HISTORY: i32 = 1 << 4; /* History file */
pub const SF_LESSKEY: i32 = 1 << 5; /* Lesskey files */
pub const SF_LESSOPEN: i32 = 1 << 6; /* LESSOPEN */
pub const SF_LOGFILE: i32 = 1 << 7; /* Log file (s, -o) */
pub const SF_PIPE: i32 = 1 << 8; /* Pipe (|) */
pub const SF_SHELL: i32 = 1 << 9; /* Shell command (!) */
pub const SF_STOP: i32 = 1 << 10; /* Stop signal */
pub const SF_TAGS: i32 = 1 << 11; /* Tags */
pub const SF_OSC8_OPEN: i32 = 1 << 12; /* OSC8 open */

pub const TOP: i32 = 0;
pub const TOP_PLUS_ONE: i32 = 1;
pub const BOTTOM: i32 = -1;
pub const BOTTOM_PLUS_ONE: i32 = -2;
pub const MIDDLE: i32 = -3;
pub const BOTTOM_OFFSET: i32 = -4;

/* How should we search? */
pub const SRCH_FORW: i32 = 1 << 0; /* Search forward from current position */
pub const SRCH_BACK: i32 = 1 << 1; /* Search backward from current position */
pub const SRCH_NO_MOVE: i32 = 1 << 2; /* Highlight, but don't move */
pub const SRCH_INCR: i32 = 1 << 3; /* Incremental search */
pub const SRCH_FIND_ALL: i32 = 1 << 4; /* Find and highlight all matches */
pub const SRCH_NO_MATCH: i32 = 1 << 8; /* Search for non-matching lines */
pub const SRCH_PAST_EOF: i32 = 1 << 9; /* Search past end-of-file, into next file */
pub const SRCH_FIRST_FILE: i32 = 1 << 10; /* Search starting at the first file */
pub const SRCH_NO_REGEX: i32 = 1 << 12; /* Don't use regular expressions */
pub const SRCH_FILTER: i32 = 1 << 13; /* Search is for '&' (filter) command */
pub const SRCH_AFTER_TARGET: i32 = 1 << 14; /* Start search after the target line */
pub const SRCH_WRAP: i32 = 1 << 15; /* Wrap-around search (continue at BOF/EOF) */

/*
 * Argument to a handling function tells what type of activity:
 */
pub const INIT: i32 = 0; /* Initialization (from command line) */
pub const QUERY: i32 = 1; /* Query (from _ or - command) */
pub const TOGGLE: i32 = 2; /* Change value (from - command) */

pub const MIN_LINENUM_WIDTH: i32 = 7; /* Default min printing width of a line number */
pub const MAX_LINENUM_WIDTH: i32 = 16; /* Max width of a line number */
pub const MAX_STATUSCOL_WIDTH: i32 = 4; /* Max width of the status column */
pub const MAX_UTF_CHAR_LEN: i32 = 6; /* Max bytes in one UTF-8 char */
pub const MAX_PRCHAR_LEN: i32 = 31; /* Max chars in prchar() result */

/* Special char bit-flags used to tell put_line() to do something special */
pub const AT_NORMAL: i32 = 0;
pub const AT_UNDERLINE: i32 = 1 << 0;
pub const AT_BOLD: i32 = 1 << 1;
pub const AT_BLINK: i32 = 1 << 2;
pub const AT_STANDOUT: i32 = 1 << 3;
pub const AT_ANSI: i32 = 1 << 4; /* Content-supplied "ANSI" escape sequence */
pub const AT_BINARY: i32 = 1 << 5; /* LESS*BINFMT representation */
pub const AT_HILITE: i32 = 1 << 6; /* Internal highlights (e.g., for search) */

pub const AT_COLOR_SHIFT: i32 = 8;
pub const AT_NUM_COLORS: i32 = 16;
pub const AT_COLOR: i32 = (AT_NUM_COLORS - 1) << AT_COLOR_SHIFT;
pub const AT_COLOR_ATTN: i32 = 1 << AT_COLOR_SHIFT;
pub const AT_COLOR_BIN: i32 = 2 << AT_COLOR_SHIFT;
pub const AT_COLOR_CTRL: i32 = 3 << AT_COLOR_SHIFT;
pub const AT_COLOR_ERROR: i32 = 4 << AT_COLOR_SHIFT;
pub const AT_COLOR_LINENUM: i32 = 5 << AT_COLOR_SHIFT;
pub const AT_COLOR_MARK: i32 = 6 << AT_COLOR_SHIFT;
pub const AT_COLOR_PROMPT: i32 = 7 << AT_COLOR_SHIFT;
pub const AT_COLOR_RSCROLL: i32 = 8 << AT_COLOR_SHIFT;
pub const AT_COLOR_HEADER: i32 = 9 << AT_COLOR_SHIFT;
pub const AT_COLOR_SEARCH: i32 = 10 << AT_COLOR_SHIFT;
pub fn AT_COLOR_SUBSEARCH(i: i32) -> i32 {
    (10 + i) << AT_COLOR_SHIFT
}
pub const NUM_SEARCH_COLORS: i32 = AT_NUM_COLORS - 10 - 1;

pub const FOLLOW_DESC: i32 = 0;
pub const FOLLOW_NAME: i32 = 1;

pub const TABSTOP_MAX: i32 = 128; /* Max number of custom tab stops */

/* How should we prompt? */
pub const PR_SHORT: usize = 0; /* Prompt with colon */
pub const PR_MEDIUM: usize = 1; /* Prompt with message */
pub const PR_LONG: usize = 2; /* Prompt with longer message */

pub const QUIT_SAVED_STATUS: i32 = -1;

/* Paste action codes (from cmd.h) */
pub const A_START_PASTE: i32 = 75;
pub const A_END_PASTE: i32 = 76;

/* Line-editing action codes (from cmd.h) */
pub const EC_BACKSPACE: i32 = 1;
pub const EC_LINEKILL: i32 = 2;
pub const EC_RIGHT: i32 = 3;
pub const EC_LEFT: i32 = 4;
pub const EC_W_LEFT: i32 = 5;
pub const EC_W_RIGHT: i32 = 6;
pub const EC_INSERT: i32 = 7;
pub const EC_DELETE: i32 = 8;
pub const EC_HOME: i32 = 9;
pub const EC_END: i32 = 10;
pub const EC_W_BACKSPACE: i32 = 11;
pub const EC_W_DELETE: i32 = 12;
pub const EC_UP: i32 = 13;
pub const EC_DOWN: i32 = 14;
pub const EC_EXPAND: i32 = 15;
pub const EC_F_COMPLETE: i32 = 17;
pub const EC_B_COMPLETE: i32 = 18;
pub const EC_LITERAL: i32 = 19;
pub const EC_ABORT: i32 = 20;
pub const EC_X11MOUSE: i32 = 21;
pub const EC_X116MOUSE: i32 = 22;
pub const EC_START_PASTE: i32 = A_START_PASTE;
pub const EC_END_PASTE: i32 = A_END_PASTE;

/* Command flags for set_mlist() (from less.h) */
pub const CF_QUIT_ON_ERASE: i32 = 1 << 0; /* Abort cmd if entirely erased */
pub const CF_OPTION: i32 = 1 << 1;        /* A_OPT_TOGGLE */

/* These values must not conflict with any A_* or EC_* value. */
pub const A_INVALID: i32 = 100;
pub const A_NOACTION: i32 = 101;
pub const A_UINVALID: i32 = 102;
pub const A_END_LIST: i32 = 103;
pub const A_SPECIAL_KEY: i32 = 104;
pub const A_PREFIX: i32 = 105;
pub const A_SKIP: i32 = 127;

pub const A_EXTRA: i32 = 0o200;

pub const NO_MCA: i32 = 0;
pub const MCA_DONE: i32 = 1;
pub const MCA_MORE: i32 = 2;

pub const CC_OK: i32 = 0; /* Char was accepted & processed */
pub const CC_QUIT: i32 = 1; /* Char was a request to abort current cmd */
pub const CC_ERROR: i32 = 2; /* Char could not be accepted due to error */
pub const CC_PASS: i32 = 3; /* Char was rejected (internal) */

#[inline]
pub fn CONTROL(c: char) -> char {
    ((c as u8) & 0o37) as char
}

/* Flags for editchar() */
pub const ECF_PEEK: i32 = 0o1;
pub const ECF_NOHISTORY: i32 = 0o2;
pub const ECF_NOCOMPLETE: i32 = 0o4;
pub const ECF_NORIGHTLEFT: i32 = 0o10;

/// Search for subpattern
pub const fn SRCH_SUBSEARCH(i: i32) -> i32 {
    1 << (17 + i)
}

pub const SRCH_SUBSEARCH_ALL: i32 = {
    let mut v: i32 = 0;
    let mut i = 1;
    while i <= NUM_SEARCH_COLORS {
        v |= SRCH_SUBSEARCH(i);
        i += 1;
    }
    v
};

/* Flag to toggle_option to specify how to "toggle" */
pub const OPT_NO_TOGGLE: i32 = 0;
pub const OPT_TOGGLE: i32 = 1;
pub const OPT_UNSET: i32 = 2;
pub const OPT_SET: i32 = 3;
pub const OPT_NO_PROMPT: i32 = 0o100;
