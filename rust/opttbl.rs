use crate::decode::lgetenv;
use crate::defs::*;
use crate::main::sprefix;
use crate::xbuf::XBuffer;
use bitflags::bitflags;

const OLETTER_NONE: char = '\x01';

struct Options {
    // Variables controlled by command line options.
    /* Should we suppress the audible bell? */
    pub quiet: i32,
    /* Should we suppress the visual bell? */
    pub no_vbell: i32,
    /* Where should forward searches start? */
    pub how_search: i32,
    /* Repaint screen from top?
     * (alternative is scroll from bottom)
     */
    pub top_scroll: i32,
    /* Type of prompt (short, medium, long) */
    pub pr_type: i32,
    /* How to process backspaces */
    pub bs_mode: i32,
    /* Don't complain about dumb terminals */
    pub know_dumb: i32,
    /* Quit after hitting end of file twice */
    pub quit_at_eof: i32,
    /* Quit if EOF on first screen */
    pub quit_if_one_screen: i32,
    /* Squeeze multiple blank lines into one */
    pub squeeze: i32,
    /* Tab settings */
    pub tabstop: i32,
    /* Repaint screen on backwards movement */
    pub back_scroll: i32,
    /* Repaint screen on forward movement */
    pub forw_scroll: i32,
    /* Do "caseless" searches */
    pub caseless: i32,
    /* Use line numbers */
    pub linenums: i32,
    /* Automatically allocate buffers as needed */
    pub autobuf: i32,
    /* Max buffer space per file (K) */
    pub bufspace: i32,
    /* Send control chars to screen untranslated */
    pub ctldisp: i32,
    /* Open the file even if not regular file */
    pub force_open: i32,
    /* Size of scrolling window */
    pub swindow: i32,
    /* Screen line of "jump target" */
    pub jump_sline: i32,
    pub jump_sline_fraction: i64,
    /* Number of positions to shift horizontally */
    pub shift_count: i32,
    pub shift_count_fraction: i64,
    /* Truncate displayed lines at screen width */
    pub chopline: i32,
    /* Wrap lines at space */
    pub wordwrap: i32,
    /* Disable sending ti/te termcap strings */
    pub no_init: i32,
    /* Disable sending ks/ke termcap strings */
    pub no_keypad: i32,
    /* Show tildes after EOF */
    pub twiddle: i32,
    /* Hilite first unread line */
    pub show_attn: i32,
    /* Display a status column */
    pub status_col: i32,
    /* Use the LESSOPEN filter */
    pub use_lessopen: i32,
    /* Quit on interrupt */
    pub quit_on_intr: i32,
    /* F cmd Follows file desc or file name? */
    pub follow_mode: i32,
    /* Old bottom of screen behavior {{REMOVE}} */
    pub oldbot: i32,
    /* Use backslash escaping in option parsing */
    pub opt_use_backslash: i32,
    /* Char which marks chopped lines with -S */
    pub rscroll_char: LWCHAR,
    /* Attribute of rscroll_char */
    pub rscroll_attr: i32,
    /* Remove dups from history list */
    pub no_hist_dups: i32,
    /* Allow mouse for scrolling */
    pub mousecap: i32,
    /* Number of lines to scroll on mouse wheel scroll */
    pub wheel_lines: i32,
    /* Save marks in history file */
    pub perma_marks: i32,
    /* Width of line numbers */
    pub linenum_width: i32,
    /* Width of status column */
    pub status_col_width: i32,
    /* Incremental search */
    pub incr_search: i32,
    /* Use UI color */
    pub use_color: i32,
    /* Scan to EOF if necessary to get file size */
    pub want_filesize: i32,
    /* Highlight entire marked lines */
    pub status_line: i32,
    /* Freeze header lines at top of screen */
    pub header_lines: i32,
    /* Freeze header columns at left of screen */
    pub header_cols: i32,
    /* Don't give headers line numbers */
    pub nonum_headers: i32,
    /* Don't search in header lines */
    pub nosearch_header_lines: i32,
    /* Don't search in header columns */
    pub nosearch_header_cols: i32,
    /* Redraw last screen after term deinit */
    pub redraw_on_quit: i32,
    /* */
    pub def_search_type: i32,
    /* Exit F command when input closes */
    pub exit_F_on_close: i32,
    /* Lines to read looking for modelines */
    pub modelines: i32,
    /* Display msg when preproc exits with error */
    pub show_preproc_error: i32,
    /* Special handling of backspace */
    pub proc_backspace: i32,
    /* Special handling of tab */
    pub proc_tab: i32,
    /* Special handling of carriage return */
    pub proc_return: i32,
    /* Extra horizontal shift on search match */
    pub match_shift: i32,
    /* Don't accept pasted input */
    pub no_paste: i32,
    /* Don't warn when editing a LESSOPENed file */
    pub no_edit_warn: i32,
    /* Stop scrolling on a line starting with form feed */
    pub stop_on_form_feed: i32,
    /* 1/2 of screen width */
    pub match_shift_fraction: i64,
    /* Char to interrupt reads */
    pub intr_char: u8,
    //#[cfg(feature = "hilite_search")]
    /* Highlight matched search patterns? */
    pub hilite_search: i32,
    /* Make compatible with POSIX more */
    pub less_is_more: i32,

    pub option_table: Vec<LOption>,
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct OptFlags: u32 {
        const BOOL = 0x01;
        const TRIPLE   = 0x02;
        const NUMBER   = 0x04;
        const STRING   = 0x10;
        const NOVAR    = 0x20;
        const REPAINT  = 0x40;
        const NO_TOGGLE  = 0x100;
        const HL_REPAINT  = 0x200;
        const NO_QUERY  = 0x400;
        const INIT_HANDLER  = 0x1000;
        const UNSUPPORTED  = 0x2000;

    }
}

#[derive(Copy, Clone)]
pub enum OptAction {
    Init,
    Query,
    Toggle,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ToggleHow {
    NoToggle,
    Toggle,
    Unset,
    Set,
}

#[derive(Clone)]
pub struct LOption {
    /// The controlling letter (a-z)
    pub oletter: char,
    /// Long (GNU-style) option name
    pub onames: [&'static str; 2],
    /// Type of the option
    pub otype: OptFlags,
    /// Default value
    pub odefault: i32,
    /// Associated variable
    pub ovar: Option<i32>,
    /// Special handling function
    pub ofunc: Option<unsafe fn(OptAction, Option<&str>)>,
    /// Description fore each value
    pub odesc: [&'static str; 3],
}

/*
 * Long option names.
 */
pub static A_OPTNAME: &'static str = "search-skip-screen";
pub static B_OPTNAME: &'static str = "buffers";
pub static B__OPTNAME: &'static str = "auto-buffers";
pub static C_OPTNAME: &'static str = "clear-screen";
pub static D_OPTNAME: &'static str = "dumb";
pub static D__OPTNAME: &'static str = "color";
pub static E_OPTNAME: &'static str = "quit-at-eof";
pub static F_OPTNAME: &'static str = "force";
pub static F__OPTNAME: &'static str = "quit-if-one-screen";
pub static H_OPTNAME: &'static str = "max-back-scroll";
pub static I_OPTNAME: &'static str = "ignore-case";
pub static J_OPTNAME: &'static str = "jump-target";
pub static J__OPTNAME: &'static str = "status-column";
pub static K__OPTNAME: &'static str = "quit-on-intr";
pub static L__OPTNAME: &'static str = "no-lessopen";
pub static M_OPTNAME: &'static str = "long-prompt";
pub static N_OPTNAME: &'static str = "line-numbers";
pub static P_OPTNAME: &'static str = "pattern";
pub static P__OPTNAME: &'static str = "prompt";
pub static Q2_OPTNAME: &'static str = "silent";
pub static Q_OPTNAME: &'static str = "quiet";
pub static R_OPTNAME: &'static str = "raw-control-chars";
pub static S_OPTNAME: &'static str = "squeeze-blank-lines";
pub static S__OPTNAME: &'static str = "chop-long-lines";
pub static U_OPTNAME: &'static str = "underline-special";
pub static V__OPTNAME: &'static str = "version";
pub static W_OPTNAME: &'static str = "hilite-unread";
pub static X_OPTNAME: &'static str = "tabs";
pub static X__OPTNAME: &'static str = "no-init";
pub static Y_OPTNAME: &'static str = "max-forw-scroll";
pub static Z_OPTNAME: &'static str = "window";
pub static QUOTE_OPTNAME: &'static str = "quotes";
pub static TILDE_OPTNAME: &'static str = "tilde";
pub static QUERY_OPTNAME: &'static str = "help";
pub static POUND_OPTNAME: &'static str = "shift";
pub static KEYPAD_OPTNAME: &'static str = "no-keypad";
pub static OLDBOT_OPTNAME: &'static str = "old-bot";
pub static FOLLOW_OPTNAME: &'static str = "follow-name";
pub static USE_BACKSLASH_OPTNAME: &'static str = "use-backslash";
pub static RSCROLL_OPTNAME: &'static str = "rscroll";
pub static NOHISTDUPS_OPTNAME: &'static str = "no-histdups";
pub static MOUSECAP_OPTNAME: &'static str = "mouse";
pub static WHEEL_LINES_OPTNAME: &'static str = "wheel-lines";
pub static PERMA_MARKS_OPTNAME: &'static str = "save-marks";
pub static LINENUM_WIDTH_OPTNAME: &'static str = "line-num-width";
pub static STATUS_COL_WIDTH_OPTNAME: &'static str = "status-col-width";
pub static INCR_SEARCH_OPTNAME: &'static str = "incsearch";
pub static USE_COLOR_OPTNAME: &'static str = "use-color";
pub static WANT_FILESIZE_OPTNAME: &'static str = "file-size";
pub static STATUS_LINE_OPTNAME: &'static str = "status-line";
pub static HEADER_OPTNAME: &'static str = "header";
pub static NO_PASTE_OPTNAME: &'static str = "no-paste";
pub static FORM_FEED_OPTNAME: &'static str = "form-feed";
pub static NO_EDIT_WARN_OPTNAME2: &'static str = "no-warn-edit";
pub static NO_EDIT_WARN_OPTNAME: &'static str = "no-edit-warn";
pub static NONUM_HEADERS_OPTNAME: &'static str = "no-number-headers";
pub static NOSEARCH_HEADERS_OPTNAME: &'static str = "no-search-headers";
pub static NOSEARCH_HEADER_LINES_OPTNAME: &'static str = "no-search-header-lines";
pub static NOSEARCH_HEADER_COLS_OPTNAME: &'static str = "no-search-header-columns";
pub static REDRAW_ON_QUIT_OPTNAME: &'static str = "redraw-on-quit";
pub static SEARCH_TYPE_OPTNAME: &'static str = "search-options";
pub static EXIT_F_ON_CLOSE_OPTNAME: &'static str = "exit-follow-on-close";
pub static NO_VBELL_OPTNAME: &'static str = "no-vbell";
pub static MODELINES_OPTNAME: &'static str = "modelines";
pub static INTR_OPTNAME: &'static str = "intr";
pub static WORDWRAP_OPTNAME: &'static str = "wordwrap";
pub static SHOW_PREPROC_ERROR_OPTNAME: &'static str = "show-preproc-errors";
pub static PROC_BACKSPACE_OPTNAME: &'static str = "proc-backspace";
pub static PROC_TAB_OPTNAME: &'static str = "proc-tab";
pub static PROC_RETURN_OPTNAME: &'static str = "proc-return";
pub static MATCH_SHIFT_OPTNAME: &'static str = "match-shift";

/*
 * Table of all options and their semantics.
 *
 * For BOOL and TRIPLE options, odesc[0], odesc[1], odesc[2] are
 * the description of the option when set to 0, 1 or 2, respectively.
 * For NUMBER options, odesc[0] is the prompt to use when entering
 * a new value, and odesc[1] is the description, which should contain
 * one %d which is replaced by the value of the number.
 * For STRING options, odesc[0] is the prompt to use when entering
 * a new value, and odesc[1], if not NULL, is the set of characters
 * that are valid in the string.
 */

unsafe fn is_optchar(c: char) -> bool {
    if c.is_ascii_uppercase() {
        return true;
    }
    if c.is_ascii_lowercase() {
        return true;
    }
    if c == '-' {
        return true;
    }
    return false;
}

impl Options {
    fn new() -> Self {
        Options {
            quiet: 0,
            no_vbell: 0,
            how_search: 0,
            top_scroll: 0,
            pr_type: 0,
            bs_mode: 0,
            know_dumb: 0,
            quit_at_eof: 0,
            quit_if_one_screen: 0,
            squeeze: 0,
            tabstop: 0,
            back_scroll: 0,
            forw_scroll: 0,
            caseless: 0,
            linenums: 0,
            autobuf: 0,
            bufspace: 0,
            ctldisp: 0,
            force_open: 0,
            swindow: 0,
            jump_sline: 0,
            jump_sline_fraction: 0,
            shift_count: 0,
            shift_count_fraction: 0,
            chopline: 0,
            wordwrap: 0,
            no_init: 0,
            no_keypad: 0,
            twiddle: 0,
            show_attn: 0,
            status_col: 0,
            use_lessopen: 0,
            quit_on_intr: 0,
            follow_mode: 0,
            oldbot: 0,
            opt_use_backslash: 0,
            rscroll_char: 0,
            rscroll_attr: 0,
            no_hist_dups: 0,
            mousecap: 0,
            wheel_lines: 0,
            perma_marks: 0,
            linenum_width: 0,
            status_col_width: 0,
            incr_search: 0,
            use_color: 0,
            want_filesize: 0,
            status_line: 0,
            header_lines: 0,
            header_cols: 0,
            nonum_headers: 0,
            nosearch_header_lines: 0,
            nosearch_header_cols: 0,
            redraw_on_quit: 0,
            def_search_type: 0,
            exit_F_on_close: 0,
            modelines: 0,
            show_preproc_error: 0,
            proc_backspace: 0,
            proc_tab: 0,
            proc_return: 0,
            match_shift: 0,
            no_paste: 0,
            no_edit_warn: 0,
            stop_on_form_feed: 0,
            match_shift_fraction: 0,
            intr_char: 0,
            hilite_search: 0,
            less_is_more: 0,
            option_table: vec![
                LOption {
                    oletter: 'a',
                    onames: &a_optname,
                    otype: OptFlags::TRIPLE,
                    odefault: OPT_ONPLUS,
                    ovar: Some(&how_search),
                    ofunc: None,
                    odesc: [
                        "Search includes displayed screen",
                        "Search skips displayed screen",
                        "Search includes all of displayed screen",
                    ],
                },
                LOption {
                    oletter: 'b',
                    onames: &b_optname,
                    otype: OptFlags::NUMBER | OptFlags::INIT_HANDLER,
                    odefault: 64,
                    ovar: Some(&bufspace),
                    ofunc: Some(opt_b),
                    odesc: [
                        "Max buffer space per file (K): ",
                        "Max buffer space per file: %dK",
                        "",
                    ],
                },
                LOption {
                    oletter: 'B',
                    onames: &B__optname,
                    otype: OptFlags::BOOL,
                    odefault: OPT_ON,
                    ovar: Some(&autobuf),
                    ofunc: None,
                    odesc: [
                        "Don't automatically allocate buffers",
                        "Automatically allocate buffers when needed",
                        "",
                    ],
                },
                LOption {
                    oletter: 'c',
                    onames: &c_optname,
                    otype: OptFlags::TRIPLE,
                    odefault: OPT_OFF,
                    ovar: Some(&top_scroll),
                    ofunc: None,
                    odesc: [
                        "Repaint by scrolling from bottom of screen",
                        "Repaint by painting from top of screen",
                        "Repaint by painting from top of screen",
                    ],
                },
                LOption {
                    oletter: 'd',
                    onames: &d_optname,
                    otype: OptFlags::BOOL | OptFlags::NO_TOGGLE,
                    odefault: OPT_OFF,
                    ovar: Some(&know_dumb),
                    ofunc: None,
                    odesc: ["Assume intelligent terminal", "Assume dumb terminal", ""],
                },
                LOption {
                    oletter: 'D',
                    onames: &D__optname,
                    otype: OptFlags::STRING | OptFlags::REPAINT | OptFlags::NO_QUERY,
                    odefault: 0,
                    ovar: None,
                    ofunc: Some(opt_D),
                    odesc: ["color desc: ", "s", ""],
                },
                LOption {
                    oletter: 'e',
                    onames: &e_optname,
                    otype: OptFlags::TRIPLE,
                    odefault: OPT_OFF,
                    ovar: Some(&quit_at_eof),
                    ofunc: None,
                    odesc: [
                        "Don't quit at end-of-file",
                        "Quit at end-of-file",
                        "Quit immediately at end-of-file",
                    ],
                },
                LOption {
                    oletter: 'f',
                    onames: &f_optname,
                    otype: OptFlags::BOOL,
                    odefault: OPT_OFF,
                    ovar: Some(&force_open),
                    ofunc: None,
                    odesc: ["Open only regular files", "Open even non-regular files", ""],
                },
                LOption {
                    oletter: 'F',
                    onames: &F__optname,
                    otype: OptFlags::BOOL,
                    odefault: OPT_OFF,
                    ovar: Some(&quit_if_one_screen),
                    ofunc: None,
                    odesc: [
                        "Don't quit if end-of-file on first screen",
                        "Quit if end-of-file on first screen",
                        "",
                    ],
                },
                LOption {
                    oletter: 'h',
                    onames: &h_optname,
                    otype: OptFlags::NUMBER,
                    odefault: -1,
                    ovar: Some(&back_scroll),
                    ofunc: None,
                    odesc: [
                        "Backwards scroll limit: ",
                        "Backwards scroll limit is %d lines",
                        "",
                    ],
                },
                LOption {
                    oletter: 'i',
                    onames: &i_optname,
                    otype: OptFlags::TRIPLE | OptFlags::HL_REPAINT,
                    odefault: OPT_OFF,
                    ovar: Some(&caseless),
                    ofunc: Some(opt_i),
                    odesc: [
                        "Case is significant in searches",
                        "Ignore case in searches",
                        "Ignore case in searches and in patterns",
                    ],
                },
                LOption {
                    oletter: 'j',
                    onames: &j_optname,
                    otype: OptFlags::STRING,
                    odefault: 0,
                    ovar: None,
                    ofunc: Some(opt_j),
                    odesc: ["Target line: ", "-.d", ""],
                },
                LOption {
                    oletter: 'J',
                    onames: &J__optname,
                    otype: OptFlags::BOOL | OptFlags::REPAINT,
                    odefault: OPT_OFF,
                    ovar: Some(&status_col),
                    ofunc: None,
                    odesc: [
                        "Don't display a status column",
                        "Display a status column",
                        "",
                    ],
                },
                LOption {
                    oletter: 'K',
                    onames: &K__optname,
                    otype: OptFlags::BOOL,
                    odefault: OPT_OFF,
                    ovar: Some(&quit_on_intr),
                    ofunc: None,
                    odesc: [
                        "Interrupt (ctrl-C) returns to prompt",
                        "Interrupt (ctrl-C) exits less",
                        "",
                    ],
                },
                LOption {
                    oletter: 'L',
                    onames: &L__optname,
                    otype: OptFlags::BOOL,
                    odefault: OPT_ON,
                    ovar: Some(&use_lessopen),
                    ofunc: None,
                    odesc: [
                        "Don't use the LESSOPEN filter",
                        "Use the LESSOPEN filter",
                        "",
                    ],
                },
                LOption {
                    oletter: 'm',
                    onames: &m_optname,
                    otype: OptFlags::TRIPLE,
                    odefault: OPT_OFF,
                    ovar: Some(&pr_type),
                    ofunc: None,
                    odesc: ["Short prompt", "Medium prompt", "Long prompt"],
                },
                LOption {
                    oletter: 'n',
                    onames: &n_optname,
                    otype: OptFlags::TRIPLE | OptFlags::REPAINT,
                    odefault: OPT_ON,
                    ovar: Some(&linenums),
                    ofunc: None,
                    odesc: [
                        "Don't use line numbers",
                        "Use line numbers",
                        "Constantly display line numbers",
                    ],
                },
                LOption {
                    oletter: 'q',
                    onames: &q_optname,
                    otype: OptFlags::TRIPLE,
                    odefault: OPT_OFF,
                    ovar: Some(&quiet),
                    ofunc: None,
                    odesc: [
                        "Ring the bell for errors AND at eof/bof",
                        "Ring the bell for errors but not at eof/bof",
                        "Never ring the bell",
                    ],
                },
                LOption {
                    oletter: 'r',
                    onames: &r_optname,
                    otype: OptFlags::TRIPLE | OptFlags::REPAINT,
                    odefault: OPT_OFF,
                    ovar: Some(&ctldisp),
                    ofunc: None,
                    odesc: [
                        "Display control characters as ^X",
                        "Display control characters directly (not recommended)",
                        "Display ANSI sequences directly, other control characters as ^X",
                    ],
                },
                LOption {
                    oletter: 's',
                    onames: &s_optname,
                    otype: OptFlags::BOOL | OptFlags::REPAINT,
                    odefault: OPT_OFF,
                    ovar: Some(&squeeze),
                    ofunc: None,
                    odesc: [
                        "Display all blank lines",
                        "Squeeze multiple blank lines",
                        "",
                    ],
                },
                LOption {
                    oletter: 'S',
                    onames: &S__optname,
                    otype: OptFlags::BOOL | OptFlags::REPAINT,
                    odefault: OPT_OFF,
                    ovar: Some(&chopline),
                    ofunc: Some(opt__S),
                    odesc: ["Fold long lines", "Chop long lines", ""],
                },
                LOption {
                    oletter: 'u',
                    onames: &u_optname,
                    otype: OptFlags::TRIPLE | OptFlags::REPAINT | OptFlags::HL_REPAINT,
                    odefault: OPT_OFF,
                    ovar: Some(&bs_mode),
                    ofunc: None,
                    odesc: [
                        "Display underlined text in underline mode",
                        "Backspaces cause overstrike",
                        "Print backspace as ^H",
                    ],
                },
                LOption {
                    oletter: 'x',
                    onames: &x_optname,
                    otype: OptFlags::STRING | OptFlags::REPAINT,
                    odefault: 0,
                    ovar: None,
                    ofunc: Some(opt_x),
                    odesc: ["Tab stops: ", "d,", ""],
                },
                LOption {
                    oletter: 'X',
                    onames: &X__optname,
                    otype: OptFlags::BOOL | OptFlags::NO_TOGGLE,
                    odefault: OPT_OFF,
                    ovar: Some(&no_init),
                    ofunc: None,
                    odesc: [
                        "Send init/deinit strings to terminal",
                        "Don't use init/deinit strings",
                        "",
                    ],
                },
                LOption {
                    oletter: 'y',
                    onames: &y_optname,
                    otype: OptFlags::NUMBER,
                    odefault: -1,
                    ovar: Some(&forw_scroll),
                    ofunc: None,
                    odesc: [
                        "Forward scroll limit: ",
                        "Forward scroll limit is %d lines",
                        "",
                    ],
                },
                LOption {
                    oletter: 'z',
                    onames: &z_optname,
                    otype: OptFlags::NUMBER,
                    odefault: -1,
                    ovar: Some(&swindow),
                    ofunc: None,
                    odesc: ["Scroll window size: ", "Scroll window size is %d lines", ""],
                },
                LOption {
                    oletter: '"',
                    onames: &quote_optname,
                    otype: OptFlags::STRING,
                    odefault: 0,
                    ovar: None,
                    ofunc: Some(opt_quote),
                    odesc: ["quotes: ", "s", ""],
                },
                LOption {
                    oletter: '~',
                    onames: &tilde_optname,
                    otype: OptFlags::BOOL | OptFlags::REPAINT,
                    odefault: OPT_ON,
                    ovar: Some(&twiddle),
                    ofunc: None,
                    odesc: [
                        "Don't show tildes after end of file",
                        "Show tildes after end of file",
                        "",
                    ],
                },
                LOption {
                    oletter: '#',
                    onames: &pound_optname,
                    otype: OptFlags::STRING,
                    odefault: 0,
                    ovar: None,
                    ofunc: Some(opt_shift),
                    odesc: ["Horizontal shift: ", ".d", ""],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &keypad_optname,
                    otype: OptFlags::BOOL | OptFlags::NO_TOGGLE,
                    odefault: OPT_OFF,
                    ovar: Some(&no_keypad),
                    ofunc: None,
                    odesc: ["Use keypad mode", "Don't use keypad mode", ""],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &oldbot_optname,
                    otype: OptFlags::BOOL,
                    odefault: OPT_OFF,
                    ovar: Some(&oldbot),
                    ofunc: None,
                    odesc: [
                        "Use new bottom of screen behavior",
                        "Use old bottom of screen behavior",
                        "",
                    ],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &follow_optname,
                    otype: OptFlags::BOOL,
                    odefault: FOLLOW_DESC,
                    ovar: Some(&follow_mode),
                    ofunc: None,
                    odesc: [
                        "F command follows file descriptor",
                        "F command follows file name",
                        "",
                    ],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &use_backslash_optname,
                    otype: OptFlags::BOOL,
                    odefault: OPT_OFF,
                    ovar: Some(&opt_use_backslash),
                    ofunc: None,
                    odesc: [
                        "Use backslash escaping in command line parameters",
                        "Don't use backslash escaping in command line parameters",
                        "",
                    ],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &rscroll_optname,
                    otype: OptFlags::STRING | OptFlags::REPAINT | OptFlags::INIT_HANDLER,
                    odefault: 0,
                    ovar: None,
                    ofunc: Some(opt_rscroll),
                    odesc: ["rscroll character: ", "s", ""],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &nohistdups_optname,
                    otype: OptFlags::BOOL,
                    odefault: OPT_OFF,
                    ovar: Some(&no_hist_dups),
                    ofunc: None,
                    odesc: [
                        "Allow duplicates in history list",
                        "Remove duplicates from history list",
                        "",
                    ],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &mousecap_optname,
                    otype: OptFlags::TRIPLE,
                    odefault: OPT_OFF,
                    ovar: Some(&mousecap),
                    ofunc: Some(opt_mousecap),
                    odesc: [
                        "Ignore mouse input",
                        "Use the mouse for scrolling",
                        "Use the mouse for scrolling (reverse)",
                    ],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &wheel_lines_optname,
                    otype: OptFlags::NUMBER | OptFlags::INIT_HANDLER,
                    odefault: 0,
                    ovar: Some(&wheel_lines),
                    ofunc: Some(opt_wheel_lines),
                    odesc: [
                        "Lines to scroll on mouse wheel: ",
                        "Scroll %d line(s) on mouse wheel",
                        "",
                    ],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &perma_marks_optname,
                    otype: OptFlags::BOOL,
                    odefault: OPT_OFF,
                    ovar: Some(&perma_marks),
                    ofunc: None,
                    odesc: [
                        "Don't save marks in history file",
                        "Save marks in history file",
                        "",
                    ],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &linenum_width_optname,
                    otype: OptFlags::NUMBER | OptFlags::REPAINT,
                    odefault: MIN_LINENUM_WIDTH,
                    ovar: Some(&linenum_width),
                    ofunc: Some(opt_linenum_width),
                    odesc: ["Line number width: ", "Line number width is %d chars", ""],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &status_col_width_optname,
                    otype: OptFlags::NUMBER | OptFlags::REPAINT,
                    odefault: 2,
                    ovar: Some(&status_col_width),
                    ofunc: Some(opt_status_col_width),
                    odesc: [
                        "Status column width: ",
                        "Status column width is %d chars",
                        "",
                    ],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &incr_search_optname,
                    otype: OptFlags::BOOL,
                    odefault: OPT_OFF,
                    ovar: Some(&incr_search),
                    ofunc: None,
                    odesc: ["Incremental search is off", "Incremental search is on", ""],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &use_color_optname,
                    otype: OptFlags::BOOL | OptFlags::REPAINT,
                    odefault: OPT_OFF,
                    ovar: Some(&use_color),
                    ofunc: None,
                    odesc: ["Don't use color", "Use color", ""],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &want_filesize_optname,
                    otype: OptFlags::BOOL | OptFlags::REPAINT,
                    odefault: OPT_OFF,
                    ovar: Some(&want_filesize),
                    ofunc: Some(opt_filesize),
                    odesc: ["Don't get size of each file", "Get size of each file", ""],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &status_line_optname,
                    otype: OptFlags::BOOL | OptFlags::REPAINT,
                    odefault: OPT_OFF,
                    ovar: Some(&status_line),
                    ofunc: None,
                    odesc: [
                        "Don't color each line with its status column color",
                        "Color each line with its status column color",
                        "",
                    ],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &header_optname,
                    otype: OptFlags::STRING | OptFlags::REPAINT,
                    odefault: 0,
                    ovar: None,
                    ofunc: Some(opt_header),
                    odesc: ["Header lines: ", "d,", ""],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &no_paste_optname,
                    otype: OptFlags::BOOL,
                    odefault: OPT_OFF,
                    ovar: Some(&no_paste),
                    ofunc: Some(opt_no_paste),
                    odesc: ["Accept pasted input", "Ignore pasted input", ""],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &form_feed_optname,
                    otype: OptFlags::BOOL,
                    odefault: OPT_OFF,
                    ovar: Some(&stop_on_form_feed),
                    ofunc: None,
                    odesc: ["Don't stop on form feed", "Stop on form feed", ""],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &no_edit_warn_optname,
                    otype: OptFlags::BOOL,
                    odefault: OPT_OFF,
                    ovar: Some(&no_edit_warn),
                    ofunc: None,
                    odesc: [
                        "Warn when editing a file opened via LESSOPEN",
                        "Don't warn when editing a file opened via LESSOPEN",
                        "",
                    ],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &nonum_headers_optname,
                    otype: OptFlags::BOOL | OptFlags::REPAINT,
                    odefault: 0,
                    ovar: Some(&nonum_headers),
                    ofunc: None,
                    odesc: ["Number header lines", "Don't number header lines", ""],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &redraw_on_quit_optname,
                    otype: OptFlags::BOOL,
                    odefault: OPT_OFF,
                    ovar: Some(&redraw_on_quit),
                    ofunc: None,
                    odesc: [
                        "Don't redraw screen when quitting",
                        "Redraw last screen when quitting",
                        "",
                    ],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &search_type_optname,
                    otype: OptFlags::STRING,
                    odefault: 0,
                    ovar: None,
                    ofunc: Some(opt_search_type),
                    odesc: ["Search options: ", "s", ""],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &exit_F_on_close_optname,
                    otype: OptFlags::BOOL,
                    odefault: OPT_OFF,
                    ovar: Some(&exit_F_on_close),
                    ofunc: None,
                    odesc: [
                        "Don't exit F command when input closes",
                        "Exit F command when input closes",
                        "",
                    ],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &no_vbell_optname,
                    otype: OptFlags::BOOL,
                    odefault: OPT_OFF,
                    ovar: Some(&no_vbell),
                    ofunc: None,
                    odesc: ["Display visual bell", "Don't display visual bell", ""],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &modelines_optname,
                    otype: OptFlags::NUMBER,
                    odefault: 0,
                    ovar: Some(&modelines),
                    ofunc: None,
                    odesc: [
                        "Lines to read looking for modelines: ",
                        "Read %d lines looking for modelines",
                        "",
                    ],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &intr_optname,
                    otype: OptFlags::STRING,
                    odefault: 0,
                    ovar: None,
                    ofunc: Some(opt_intr),
                    odesc: ["interrupt character: ", "s", ""],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &wordwrap_optname,
                    otype: OptFlags::BOOL | OptFlags::REPAINT,
                    odefault: OPT_OFF,
                    ovar: Some(&wordwrap),
                    ofunc: None,
                    odesc: ["Wrap lines at any character", "Wrap lines at spaces", ""],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &show_preproc_error_optname,
                    otype: OptFlags::BOOL,
                    odefault: OPT_OFF,
                    ovar: Some(&show_preproc_error),
                    ofunc: None,
                    odesc: [
                        "Don't show error message if preprocessor fails",
                        "Show error message if preprocessor fails",
                        "",
                    ],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &proc_backspace_optname,
                    otype: OptFlags::TRIPLE | OptFlags::REPAINT | OptFlags::HL_REPAINT,
                    odefault: OPT_OFF,
                    ovar: Some(&proc_backspace),
                    ofunc: None,
                    odesc: [
                        "Backspace handling is specified by the -U option",
                        "Display underline text in underline mode",
                        "Print backspaces as ^H",
                    ],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &proc_tab_optname,
                    otype: OptFlags::TRIPLE | OptFlags::REPAINT | OptFlags::HL_REPAINT,
                    odefault: OPT_OFF,
                    ovar: Some(&proc_tab),
                    ofunc: None,
                    odesc: [
                        "Tab handling is specified by the -U option",
                        "Expand tabs to spaces",
                        "Print tabs as ^I",
                    ],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &proc_return_optname,
                    otype: OptFlags::TRIPLE | OptFlags::REPAINT | OptFlags::HL_REPAINT,
                    odefault: OPT_OFF,
                    ovar: Some(&proc_return),
                    ofunc: None,
                    odesc: [
                        "Carriage return handling is specified by the -U option",
                        "Delete carriage return before newline",
                        "Print carriage return as ^M",
                    ],
                },
                LOption {
                    oletter: OLETTER_NONE,
                    onames: &match_shift_optname,
                    otype: OptFlags::STRING | OptFlags::INIT_HANDLER,
                    odefault: 0,
                    ovar: None,
                    ofunc: Some(opt_match_shift),
                    odesc: ["Search match shift: ", ".d", ""],
                },
            ],
        }
    }

    /// Initialize each option to its default value.
    pub unsafe fn init_option(&mut self) {
        let mut p = lgetenv("LESS_IS_MORE");
        if let Ok(lm) = p {
            // Set each variable to its default.
            if !lm.is_empty() && lm != "0" {
                self.less_is_more = 1;
            }
        }
        for o in self.option_table.iter_mut() {
            if o.ovar.is_none() {
                o.ovar = Some(o.odefault);
            }
            if !(o.otype & OptFlags::INIT_HANDLER).is_empty() {
                o.ofunc.unwrap()(OptAction::Init, None);
            }
        }
    }

    /// Find an option in the option table, given its option letter.
    pub unsafe extern "C" fn findopt<'a>(&'a mut self, c: char) -> Option<&'a mut LOption> {
        for o in self.option_table.iter_mut() {
            if o.oletter == c {
                return Some(o);
            }
            if !(o.otype & OptFlags::TRIPLE).is_empty()
                && o.oletter.to_uppercase().next() == Some(c)
            {
                return Some(o);
            }
        }
        None
    }

    /*
     * Find an option in the option table, given its option name.
     * p_optname is the (possibly partial) name to look for, and
     * is updated to point after the matched name.
     * p_oname if non-NULL is set to point to the full option name.
     */
    pub unsafe fn findopt_name<'a>(
        &'a mut self,
        optname: &'a str,
        p_oname: Option<&'a str>,
    ) -> (Option<&'a mut LOption>, bool) {
        let mut o: Option<&'a mut LOption> = None;
        let mut maxo_idx = 0;
        let mut uppercase: usize;
        let mut maxoname: &str;
        let mut maxlen = 0;
        let mut ambig = false;
        let mut exact = false;
        let mut len = 0;

        // Check all options
        for (i, o) in self.option_table.iter_mut().enumerate() {
            // Check all names for this option
            for oname in o.onames.iter_mut() {
                if *oname == "" {
                    break;
                }
                // Try normal match first (uppercase == 0),
                // then, then if it's a TRIPLE option,
                // try uppercase match (uppercase == 1).
                for uppercase in 0..=1 {
                    len = sprefix(optname, oname, uppercase == 1);
                    let opt_ch = optname.chars().nth(len).unwrap();
                    if len == 0 || is_optchar(opt_ch) {
                        // We didn't use all of the option name.
                        continue;
                    }
                    if !exact && len == maxlen {
                        // Already had a partial match,
                        // and now there's another one that
                        // matches the same length.
                        ambig = true;
                    } else if len > maxlen {
                        // Found a better match than the one we had
                        maxo_idx = i;
                        // XXX is this needed
                        // maxoname = oname;
                        ambig = false;
                        exact = len == oname.len();
                    }
                    let flags = o.otype & OptFlags::TRIPLE;
                    if flags.is_empty() {
                        break;
                    }
                }
            }
            if ambig {
                // Name matched more than one option
                return (None, ambig);
            }
        }
        (Some(&mut self.option_table[maxo_idx]), ambig)
    }

    /// Find all toggleable options whose names begin with a specified string.
    /// Return them in a space-separated string.
    pub unsafe extern "C" fn findopts_name(&self, pfx: &str) -> String {
        let mut xbuf = XBuffer::new(16);
        for o in self.option_table.iter() {
            if !(o.otype & OptFlags::NO_TOGGLE).is_empty() {
                continue;
            }
            for oname in o.onames {
                if oname == "" {
                    break;
                }
                for uppercase in 0..=1 {
                    let len = sprefix(pfx, oname, uppercase == 1);
                    if len as usize > pfx.len() {
                        let mut chars = oname.chars();
                        while let Some(np) = chars.next() {
                            let c = if uppercase != 0 && np.is_ascii_lowercase() {
                                np.to_uppercase().next().unwrap()
                            } else {
                                np
                            };
                            xbuf.xbuf_add_char(c as i8);
                        }
                        xbuf.xbuf_add_char(b' ' as i8);
                    }
                    if (o.otype & OptFlags::TRIPLE).is_empty() {
                        break;
                    }
                }
            }
        }
        xbuf.pop();
        String::from(str::from_utf8_unchecked(&xbuf.data))
    }
}
