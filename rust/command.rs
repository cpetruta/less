use crate::cmdbuf::{
    cmd_accept, clear_cmd, cmd_char, cmd_int, cmd_putstr, cmd_repaint, cmd_reset,
    cmd_setstring, cmdbuf_empty, get_cmdbuf, len_cmdbuf, restore_updown_match,
    save_updown_match, set_mlist,
};
use crate::decode::fcmd_decode;
use crate::decode::ActionType;
use crate::decode::{editchar, get_tables_mut};
use crate::defs::*;
use crate::line::load_line;
use crate::ifile::IFileManager;
use crate::mark::Marks;
use crate::opttbl::{get_options, LOption};
use crate::opttbl::Options;
use std::ffi::{CStr, CString};

extern "C" {
    fn snprintf(
        _: *mut std::ffi::c_char,
        _: std::ffi::c_ulong,
        _: *const std::ffi::c_char,
        _: ...
    ) -> std::ffi::c_int;
    fn free(_: *mut std::ffi::c_void);
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn save(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
    fn quit(status: std::ffi::c_int);
    fn secure_allow(features: std::ffi::c_int) -> std::ffi::c_int;
    fn check_winch();
    fn bell();
    fn clear();
    fn clear_eol();
    fn clear_bot();
    fn at_enter(attr: std::ffi::c_int);
    fn at_exit();
    fn match_brac(
        obrac: std::ffi::c_char,
        cbrac: std::ffi::c_char,
        forwdir: std::ffi::c_int,
        n: std::ffi::c_int,
    );
    fn ch_length() -> POSITION;
    fn ch_flush();
    fn ch_set_eof();
    fn ch_getflags() -> std::ffi::c_int;
    fn edit(filename: *const std::ffi::c_char) -> std::ffi::c_int;
    fn edit_ifile(ifile: *mut std::ffi::c_void) -> std::ffi::c_int;
    fn edit_list(filelist: *mut std::ffi::c_char) -> std::ffi::c_int;
    fn edit_first() -> std::ffi::c_int;
    fn edit_last() -> std::ffi::c_int;
    fn edit_next(n: std::ffi::c_int) -> std::ffi::c_int;
    fn edit_prev(n: std::ffi::c_int) -> std::ffi::c_int;
    fn edit_index(n: std::ffi::c_int) -> std::ffi::c_int;
    fn save_curr_ifile() -> *mut std::ffi::c_void;
    fn unsave_ifile(save_ifile: *mut std::ffi::c_void);
    fn reedit_ifile(save_ifile: *mut std::ffi::c_void);
    fn reopen_curr_ifile();
    fn fexpand(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn eof_displayed(offset: lbool) -> lbool;
    fn entire_file_displayed() -> lbool;
    fn forward(n: std::ffi::c_int, force: lbool, only_last: lbool, to_newline: lbool);
    fn backward(n: std::ffi::c_int, force: lbool, only_last: lbool, to_newline: lbool);
    fn del_ifile(h: *mut std::ffi::c_void);
    fn next_ifile(h: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn getoff_ifile(ifile: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn get_filename(ifile: *mut std::ffi::c_void) -> *const std::ffi::c_char;
    fn get_altfilename(ifile: *mut std::ffi::c_void) -> *mut std::ffi::c_char;
    fn set_attnpos(pos: POSITION);
    fn jump_forw();
    fn jump_forw_buffered();
    fn jump_back(linenum: LINENUM);
    fn repaint();
    fn jump_percent(percent: std::ffi::c_int, fraction_0: std::ffi::c_long);
    fn jump_line_loc(pos: POSITION, sline: std::ffi::c_int);
    fn jump_loc(pos: POSITION, sline: std::ffi::c_int);
    fn set_line_contig_pos(pos: POSITION);
    fn rrshift() -> std::ffi::c_int;
    fn clr_linenum();
    fn lsystem(cmd: *const std::ffi::c_char, donemsg: *const std::ffi::c_char);
    fn pipe_mark(c: std::ffi::c_char, cmd: *const std::ffi::c_char) -> std::ffi::c_int;
    fn get_swindow() -> std::ffi::c_int;
    fn propt(c: std::ffi::c_char) -> *const std::ffi::c_char;
    fn toggle_option(
        o: *mut LOption,
        lower: bool,
        s: *const std::ffi::c_char,
        how_toggle: std::ffi::c_int,
    );
    fn opt_has_param(o: *mut LOption) -> std::ffi::c_int;
    fn opt_prompt(o: *mut LOption) -> *const std::ffi::c_char;
    fn opt_toggle_disallowed(c: std::ffi::c_int) -> *const std::ffi::c_char;
    fn get_quit_at_eof() -> std::ffi::c_int;
    fn findopt(c: std::ffi::c_int) -> *mut LOption;
    fn findopt_name(
        p_optname: *mut *const std::ffi::c_char,
        p_oname: *mut *const std::ffi::c_char,
        p_ambig: *mut lbool,
    ) -> *mut LOption;
    fn get_time() -> time_t;
    fn put_line(forw_scroll: lbool);
    fn flush();
    fn putchr(ch: std::ffi::c_int) -> std::ffi::c_int;
    fn putstr(s: *const std::ffi::c_char);
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn position(sindex: std::ffi::c_int) -> POSITION;
    fn empty_screen() -> std::ffi::c_int;
    fn pos_rehead();
    fn pr_expand(proto: *const std::ffi::c_char) -> *const std::ffi::c_char;
    fn eq_message() -> *const std::ffi::c_char;
    fn pr_string() -> *const std::ffi::c_char;
    fn clear_attn();
    fn undo_search(clear_0: lbool);
    fn clr_hilite();
    fn osc8_search(
        search_type_0: std::ffi::c_int,
        param: *const std::ffi::c_char,
        matches: std::ffi::c_int,
    );
    fn osc8_open();
    fn osc8_jump();
    fn search(
        search_type_0: std::ffi::c_int,
        pattern: *const std::ffi::c_char,
        n: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn set_filter_pattern(pattern: *const std::ffi::c_char, search_type_0: std::ffi::c_int);
    fn is_filtering() -> lbool;
    fn psignals();
    fn cleantags();
    fn tagsearch() -> POSITION;
    fn nexttag(n: std::ffi::c_int) -> *const std::ffi::c_char;
    fn prevtag(n: std::ffi::c_int) -> *const std::ffi::c_char;
    fn ntags() -> std::ffi::c_int;
    fn getchr() -> i32;
    static mut erase_char: char;
    static mut erase2_char: char;
    static mut kill_char: char;
    static mut sigs: i32;
    static mut one_screen: std::ffi::c_int;
    static mut sc_width: std::ffi::c_int;
    static mut sc_height: std::ffi::c_int;
    static mut kent: String;
    static mut quitting: lbool;
    static mut wscroll: std::ffi::c_int;
    static mut ignore_eoi: std::ffi::c_int;
    static mut hshift: std::ffi::c_int;
    static mut highest_hilite: POSITION;
    static mut every_first_cmd: *mut std::ffi::c_char;
    static mut version: [std::ffi::c_char; 0];
    static mut initial_scrpos: scrpos;
    static mut curr_ifile: *mut std::ffi::c_void;
    static mut ml_search: *mut std::ffi::c_void;
    static mut ml_examine: *mut std::ffi::c_void;
    static mut search_wrapped: lbool;
    static mut no_poll: lbool;
    static mut pasting: lbool;
    static mut soft_eof: POSITION;
    static mut ml_shell: *mut std::ffi::c_void;
    static mut editproto: *const std::ffi::c_char;
    static mut osc8_uri: *mut std::ffi::c_char;
    static mut forw_prompt: std::ffi::c_int;
    static mut full_screen: std::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scrpos {
    pub pos: POSITION,
    pub ln: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union parg {
    pub p_string: *const std::ffi::c_char,
    pub p_int: std::ffi::c_int,
    pub p_linenum: LINENUM,
    pub p_char: std::ffi::c_char,
}
pub type PARG = parg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loption {
    pub oletter: std::ffi::c_char,
    pub onames: *mut optname,
    pub otype: std::ffi::c_int,
    pub odefault: std::ffi::c_int,
    pub ovar: *mut std::ffi::c_int,
    pub ofunc: Option<unsafe extern "C" fn(std::ffi::c_int, *const std::ffi::c_char) -> ()>,
    pub odesc: [*const std::ffi::c_char; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct optname {
    pub oname: *const std::ffi::c_char,
    pub onext: *mut optname,
}
pub struct Char {
    pub ch: char,
    pub end_command: bool,
}
pub struct Ungot {
    chars: Vec<Char>,
}
static mut shellcmd: *mut std::ffi::c_char = 0 as *const std::ffi::c_char as *mut std::ffi::c_char;
static mut mca: ActionType = ActionType::Null;
static mut search_type: std::ffi::c_int = 0;
static mut last_search_type: std::ffi::c_int = 0;
static mut number: LINENUM = 0;
static mut fraction: std::ffi::c_long = 0;
static mut curropt: *mut LOption = std::ptr::null_mut();
static mut opt_lower: bool = false;
static mut optflag: std::ffi::c_int = 0;
static mut optgetname: bool = false;
static mut bottompos: POSITION = 0;
static mut save_hshift: std::ffi::c_int = 0;
static mut save_bs_mode: std::ffi::c_int = 0;
static mut save_proc_backspace: std::ffi::c_int = 0;
static mut screen_trashed_value: std::ffi::c_int = 0 as std::ffi::c_int;
static mut literal_char: bool = false;
static mut ignoring_input: lbool = LFALSE;
static mut ignoring_input_time: time_t = 0;
static mut pipec: char = ' ';
#[no_mangle]
pub unsafe extern "C" fn cmd_exec() {
    clear_attn();
    clear_bot();
    flush();
}
unsafe extern "C" fn set_mca(action: ActionType) {
    mca = action;
    clear_bot();
    clear_cmd();
}
unsafe extern "C" fn clear_mca() {
    if mca == ActionType::Null {
        return;
    }
    mca = ActionType::Null;
}
unsafe extern "C" fn start_mca(
    action: ActionType,
    mut prompt_0: *const std::ffi::c_char,
    mut mlist: *mut std::ffi::c_void,
    mut cmdflags: std::ffi::c_int,
) {
    set_mca(action);
    cmd_putstr(CStr::from_ptr(prompt_0));
    set_mlist(mlist, cmdflags);
}
#[no_mangle]
pub unsafe extern "C" fn in_mca() -> bool {
    mca != ActionType::Null && mca != ActionType::Prefix
}

/// Set up the display to start a new search command.
unsafe extern "C" fn mca_search1() {
    let mut i: std::ffi::c_int = 0;
    if search_type & SRCH_FILTER != 0 {
        set_mca(ActionType::Filter);
    } else if search_type & SRCH_FORW != 0 {
        set_mca(ActionType::FSearch);
    } else {
        set_mca(ActionType::BSearch);
    }
    if search_type & SRCH_NO_MATCH != 0 {
        cmd_putstr(c"Non-match ");
    }
    if search_type & SRCH_FIRST_FILE != 0 {
        cmd_putstr(c"First-file ");
    }
    if search_type & SRCH_PAST_EOF != 0 {
        cmd_putstr(c"EOF-ignore ");
    }
    if search_type & SRCH_NO_MOVE != 0 {
        cmd_putstr(c"Keep-pos ");
    }
    if search_type & SRCH_NO_REGEX != 0 {
        cmd_putstr(c"Regex-off ");
    }
    if search_type & SRCH_WRAP != 0 {
        cmd_putstr(c"Wrap ");
    }
    i = 1 as std::ffi::c_int;
    for i in 1..=NUM_SEARCH_COLORS {
        if search_type & (1 as std::ffi::c_int) << 17 as std::ffi::c_int + i != 0 {
            let mut buf: [std::ffi::c_char; 19] = [0; 19];
            snprintf(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[std::ffi::c_char; 19]>() as std::ffi::c_ulong,
                b"Sub-%d \0" as *const u8 as *const std::ffi::c_char,
                i,
            );
            cmd_putstr(CStr::from_ptr(buf.as_ptr()));
        }
    }
    if literal_char {
        cmd_putstr(c"Lit ");
    }
    if search_type & SRCH_FILTER != 0 {
        cmd_putstr(c"&/");
    } else if search_type & SRCH_FORW != 0 {
        cmd_putstr(c"/");
    } else {
        cmd_putstr(c"?");
    }
    forw_prompt = 0 as std::ffi::c_int;
}

unsafe extern "C" fn mca_search() {
    mca_search1();
    set_mlist(ml_search, 0 as std::ffi::c_int);
}

/*
 * Set up the display to start a new toggle-option command.
 */
unsafe extern "C" fn mca_opt_toggle() {
    let mut no_prompt = optflag & OPT_NO_PROMPT;
    let mut flag = optflag & !OPT_NO_PROMPT;
    let mut dash: *const std::ffi::c_char = if flag == 0 as std::ffi::c_int {
        b"_\0" as *const u8 as *const std::ffi::c_char
    } else {
        b"-\0" as *const u8 as *const std::ffi::c_char
    };
    set_mca(ActionType::OptToggle);
    cmd_putstr(CStr::from_ptr(dash));
    if optgetname as u64 != 0 {
        cmd_putstr(CStr::from_ptr(dash));
    }
    if no_prompt != 0 {
        cmd_putstr(c"(P)");
    }
    match flag {
        2 => {
            cmd_putstr(c"+");
        }
        3 => {
            cmd_putstr(c"!");
        }
        _ => {}
    }
    forw_prompt = 0 as std::ffi::c_int;
    set_mlist(
        0 as *mut std::ffi::c_void,
        (1 as std::ffi::c_int) << 1 as std::ffi::c_int,
    );
}

/// Execute a multicharacter command.
unsafe extern "C" fn exec_mca() {
    cmd_exec();
    let Some(cbuf_cs) = get_cmdbuf() else { return };
    let mut cbuf: *const std::ffi::c_char = cbuf_cs.as_ptr();
    match mca {
        ActionType::FSearch | ActionType::BSearch => {
            multi_search(cbuf, number as std::ffi::c_int, 0 as std::ffi::c_int);
        }
        ActionType::Filter => {
            search_type ^= SRCH_NO_MATCH;
            set_filter_pattern(cbuf, search_type);
            soft_eof = NULL_POSITION;
        }
        ActionType::FirstCmd => {
            /*
             * Skip leading spaces or + signs in the string.
             */
            while *cbuf as std::ffi::c_int == '+' as i32 || *cbuf as std::ffi::c_int == ' ' as i32 {
                cbuf = cbuf.offset(1);
            }
            if !every_first_cmd.is_null() {
                free(every_first_cmd as *mut std::ffi::c_void);
            }
            if *cbuf as std::ffi::c_int == '\0' as i32 {
                every_first_cmd = 0 as *mut std::ffi::c_char;
            } else {
                every_first_cmd = save(cbuf);
            }
        }
        ActionType::OptToggle => {
            toggle_option(curropt, opt_lower, cbuf, optflag);
            curropt = std::ptr::null_mut();
        }
        ActionType::FBracket => {
            match_brac(
                *cbuf.offset(0 as std::ffi::c_int as isize),
                *cbuf.offset(1 as std::ffi::c_int as isize),
                1 as std::ffi::c_int,
                number as std::ffi::c_int,
            );
        }
        ActionType::BBracket => {
            match_brac(
                *cbuf.offset(1 as std::ffi::c_int as isize),
                *cbuf.offset(0 as std::ffi::c_int as isize),
                0 as std::ffi::c_int,
                number as std::ffi::c_int,
            );
        }
        ActionType::Examine => {
            let mut p: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
            if !(secure_allow((1 as std::ffi::c_int) << 2 as std::ffi::c_int) == 0) {
                p = save(cbuf);
                edit_list(p);
                free(p as *mut std::ffi::c_void);
                /* If tag structure is loaded then clean it up. */
                cleantags();
            }
        }
        ActionType::Shell => {
            /*
             * !! just uses whatever is in shellcmd.
             * Otherwise, copy cmdbuf to shellcmd,
             * expanding any special characters ("%" or "#").
             */
            let mut done_msg: *const std::ffi::c_char =
                if *cbuf as std::ffi::c_int == 'P' as i32 & 0o37 as std::ffi::c_int {
                    0 as *const std::ffi::c_char
                } else {
                    b"!done\0" as *const u8 as *const std::ffi::c_char
                };
            if done_msg.is_null() {
                cbuf = cbuf.offset(1);
            }
            if *cbuf as std::ffi::c_int != '!' as i32 {
                if !shellcmd.is_null() {
                    free(shellcmd as *mut std::ffi::c_void);
                }
                shellcmd = fexpand(cbuf);
            }
            if !(secure_allow((1 as std::ffi::c_int) << 9 as std::ffi::c_int) == 0) {
                if shellcmd.is_null() {
                    shellcmd =
                        b"\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char;
                }
                lsystem(shellcmd, done_msg);
            }
        }
        ActionType::PShell => {
            let mut done_msg_0: *const std::ffi::c_char =
                if *cbuf as std::ffi::c_int == 'P' as i32 & 0o37 as std::ffi::c_int {
                    0 as *const std::ffi::c_char
                } else {
                    b"#done\0" as *const u8 as *const std::ffi::c_char
                };
            if done_msg_0.is_null() {
                cbuf = cbuf.offset(1);
            }
            if !(secure_allow((1 as std::ffi::c_int) << 9 as std::ffi::c_int) == 0) {
                lsystem(pr_expand(cbuf), done_msg_0);
            }
        }
        ActionType::Pipe => {
            let mut done_msg_1: *const std::ffi::c_char =
                if *cbuf as std::ffi::c_int == 'P' as i32 & 0o37 as std::ffi::c_int {
                    0 as *const std::ffi::c_char
                } else {
                    b"|done\0" as *const u8 as *const std::ffi::c_char
                };
            if done_msg_1.is_null() {
                cbuf = cbuf.offset(1);
            }
            if !(secure_allow((1 as std::ffi::c_int) << 8 as std::ffi::c_int) == 0) {
                pipe_mark(pipec as i8, cbuf);
                if !done_msg_1.is_null() {
                    error(done_msg_1, 0 as *mut std::ffi::c_void as *mut PARG);
                }
            }
        }
        _ => {}
    };
}

/*
 * Is a character an erase or kill char?
 */
unsafe extern "C" fn is_erase_char(c: char) -> bool {
    c == erase_char || c == erase2_char || c == kill_char
}

/*
 * Is a character a carriage return or newline?
 */
unsafe extern "C" fn is_newline_char(c: char) -> bool {
    c == '\n' || c == '\r'
}

/*
 * Handle the first char of an option (after the initial dash).
 */
unsafe extern "C" fn mca_opt_first_char(c: char) -> ActionType {
    let mut no_prompt = optflag & OPT_NO_PROMPT;
    let mut flag = optflag & !OPT_NO_PROMPT;
    if flag == 0 {
        match c {
            '_' => {
                /* "__" = long option name. */
                optgetname = true;
                mca_opt_toggle();
                return ActionType::McaMore;
            }
            _ => {}
        }
    } else {
        if c == '+' {
            /* "-+" = UNSET. */
            optflag = no_prompt
                | (if flag == OPT_UNSET {
                    OPT_TOGGLE
                } else {
                    OPT_SET
                });
            mca_opt_toggle();
            return ActionType::McaMore;
        } else if c == '!' {
            /* "-!" = SET */
            optflag = no_prompt | (if flag == OPT_SET { OPT_TOGGLE } else { OPT_SET });
            mca_opt_toggle();
            return ActionType::McaMore;
        } else if c == CONTROL('P') {
            optflag ^= OPT_NO_PROMPT;
            mca_opt_toggle();
            return ActionType::McaMore;
        } else if c == '-' {
            /* "--" = long option name. */
            optgetname = true;
            mca_opt_toggle();
            return ActionType::McaMore;
        }
    }
    /* Char was not handled here. */
    ActionType::NoMca
}

/*
 * Add a char to a long option name.
 * See if we've got a match for an option name yet.
 * If so, display the complete name and stop
 * accepting chars until user hits RETURN.
 */
unsafe extern "C" fn mca_opt_nonfirst_char(c: char) -> ActionType {
    let mut oname: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut ambig: lbool = LFALSE;
    let mut was_curropt: *mut LOption = std::ptr::null_mut();

    if !curropt.is_null() {
        /* Already have a match for the name. */
        if is_erase_char(c) {
            return ActionType::McaDone;
        }
        /* {{ Checking for TAB here is ugly.
         *    Also doesn't extend well -- can't do BACKTAB this way
         *    because it's a multichar sequence. }} */
        if c != '\t' {
            return ActionType::McaMore;
        }
    }
    /*
     * Add char to cmd buffer and try to match
     * the option name.
     */
    if cmd_char(c) == CC_QUIT {
        return ActionType::McaDone;
    }
    let Some(p_cs) = get_cmdbuf() else { return ActionType::McaMore };
    let mut p = p_cs.as_ptr();
    let slice = std::slice::from_raw_parts(p as *const u8, strlen(p) as usize);
    let cmd_b = String::from_utf8_lossy(slice).into_owned();
    if cmd_b.len() == 0 {
        return ActionType::McaMore;
    }
    let first: char = cmd_b.chars().nth(0).unwrap();
    opt_lower = first.is_ascii_lowercase();
    was_curropt = curropt;
    curropt = findopt_name(&mut p, &mut oname, &mut ambig);
    if !curropt.is_null() {
        if was_curropt.is_null() {
            /*
             * Got a match.
             * Remember the option and
             * display the full option name.
             */
            cmd_reset();
            mca_opt_toggle();
            cmd_setstring(CStr::from_ptr(oname).to_str().unwrap_or(""), !opt_lower);
        }
    } else if ambig as u64 == 0 {
        bell();
    }
    ActionType::McaMore
}

/*
 * Handle a char of an option toggle command.
 */
unsafe extern "C" fn mca_opt_char(c: char) -> ActionType {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };

    /*
     * This may be a short option (single char),
     * or one char of a long option name,
     * or one char of the option parameter.
     */
    if curropt.is_null() && cmdbuf_empty() {
        let mut ret = mca_opt_first_char(c);
        if ret != ActionType::NoMca {
            return ret;
        }
    }
    if optgetname as u64 != 0 {
        /* We're getting a long option name.  */
        if is_newline_char(c) as u64 == 0 && c as std::ffi::c_int != '=' as i32 {
            return mca_opt_nonfirst_char(c);
        }
        if curropt.is_null() {
            let Some(cbuf_cs) = get_cmdbuf() else { return ActionType::McaMore };
            parg.p_string = cbuf_cs.as_ptr();
            error(
                b"There is no --%s option\0" as *const u8 as *const std::ffi::c_char,
                &mut parg,
            );
            return ActionType::McaDone;
        }
        optgetname = false;
        cmd_reset();
    } else {
        if is_erase_char(c) as u64 != 0 {
            return ActionType::NoMca;
        }
        if !curropt.is_null() {
            return ActionType::NoMca;
        }
        curropt = findopt(c as std::ffi::c_int);
        if curropt.is_null() {
            parg.p_string = propt(c as i8);
            error(
                b"There is no %s option\0" as *const u8 as *const std::ffi::c_char,
                &mut parg,
            );
            return ActionType::McaDone;
        }
        opt_lower = c.is_ascii_lowercase();
    }

    /*
     * If the option which was entered does not take a
     * parameter, toggle the option immediately,
     * so user doesn't have to hit RETURN.
     */
    if (optflag & !OPT_NO_PROMPT) != OPT_TOGGLE || opt_has_param(curropt) == 0 {
        toggle_option(
            curropt,
            opt_lower,
            b"\0" as *const u8 as *const std::ffi::c_char,
            optflag,
        );
        return ActionType::McaDone;
    }

    /*
     * Display a prompt appropriate for the option parameter.
     */
    start_mca(
        ActionType::OptToggle,
        opt_prompt(curropt),
        0 as *mut std::ffi::c_void,
        (1 as std::ffi::c_int) << 1 as std::ffi::c_int,
    );
    ActionType::McaMore
}

#[no_mangle]
pub unsafe extern "C" fn norm_search_type(mut st: std::ffi::c_int) -> std::ffi::c_int {
    if st
        & ((1 as std::ffi::c_int) << 9 as std::ffi::c_int
            | (1 as std::ffi::c_int) << 15 as std::ffi::c_int)
        == (1 as std::ffi::c_int) << 9 as std::ffi::c_int
            | (1 as std::ffi::c_int) << 15 as std::ffi::c_int
    {
        st ^= (1 as std::ffi::c_int) << 9 as std::ffi::c_int;
    }
    return st;
}

/*
 * Handle a char of a search command.
 */
unsafe extern "C" fn mca_search_char(ungot: &mut Ungot, mut c: char) -> ActionType {
    let mut flag = 0;
    /*
     * Certain characters as the first char of
     * the pattern have special meaning:
     *      !  Toggle the NO_MATCH flag
     *      *  Toggle the PAST_EOF flag
     *      @  Toggle the FIRST_FILE flag
     */
    if !cmdbuf_empty() || literal_char {
        literal_char = false;
        return ActionType::Null;
    }
    if c == '*' || c == CONTROL('E') {
        // ignore END of file
        if mca != ActionType::Filter {
            flag = SRCH_FIRST_FILE;
        }
        search_type &= !SRCH_WRAP;
    } else if c == CONTROL('F') || c == '@' {
        // FIRST file
        if mca != ActionType::Filter {
            flag = SRCH_FIRST_FILE;
        }
    } else if c == CONTROL('K') {
        // KEEP position
        if mca != ActionType::Filter {
            flag = SRCH_NO_MOVE;
        }
    } else if c == CONTROL('S') {
        // SUBSEARCH
        let buf_c = CString::new(format!("Sub-pattern (1-{}):", NUM_SEARCH_COLORS)).unwrap();
        clear_bot();
        cmd_putstr(&buf_c);
        flush();
        c = getcc(ungot);
        if c as u8 >= b'1' && c as u8 <= b'0' + NUM_SEARCH_COLORS as u8 {
            // calls mca_search() below to repaint
            flag = SRCH_SUBSEARCH((c as u8 - b'0').into());
        } else {
            flag = -1;
        }
    } else if c == CONTROL('W') {
        // WRAP around
        if mca != ActionType::Filter {
            flag = SRCH_WRAP;
        }
    } else if c == CONTROL('R') {
        // Don't use REGULAR EXPRESSIONS
        flag = SRCH_NO_REGEX;
    } else if c == CONTROL('N') || c == '!' {
        flag = SRCH_NO_MATCH;
    } else if c == CONTROL('L') {
        literal_char = true;
        flag = -1;
    }

    if flag != 0 {
        if flag != -1 {
            search_type = norm_search_type(search_type ^ flag);
        }
        mca_search();
        return ActionType::McaMore;
    }
    ActionType::NoMca
}

/*
 * Handle a character of a multi-character command.
 */
unsafe extern "C" fn mca_char(ungot: &mut Ungot, c: char) -> ActionType {
    let mut ret = ActionType::Null;
    match mca {
        /*
         * We're not in a multicharacter command.
         */
        ActionType::Null => return ActionType::NoMca,
        /*
         * In the prefix of a command.
         * This not considered a multichar command
         * (even tho it uses cmdbuf, etc.).
         * It is handled in the commands() switch.
         */
        ActionType::Prefix => return ActionType::NoMca,
        /*
         * Entering digits of a number.
         * Terminated by a non-digit.
         */
        ActionType::Digit => {
            if !(c.is_ascii_digit() || c == '.') {
                let tables = get_tables_mut().unwrap();
                match editchar(
                    &tables,
                    c as u8,
                    ECF_PEEK | ECF_NOHISTORY | ECF_NOCOMPLETE | ECF_NORIGHTLEFT,
                ) {
                    /*
                     * Ignore this char and get another one.
                     */
                    ActionType::NoAction => return ActionType::McaMore,
                    ActionType::Invalid => {
                        /*
                         * Not part of the number.
                         * End the number and treat this char
                         * as a normal command character.
                         */
                        let (n, f) = cmd_int();
                        number = n;
                        fraction = f;
                        clear_mca();
                        cmd_accept();
                        return ActionType::NoMca;
                    }
                    _ => {}
                }
            }
        }
        ActionType::OptToggle => {
            ret = mca_opt_char(c);
            if ret != ActionType::NoMca {
                return ret;
            }
        }
        ActionType::FSearch | ActionType::BSearch | ActionType::Filter => {
            ret = mca_search_char(ungot, c);
            if ret != ActionType::NoMca {
                return ret;
            }
        }
        _ => {}
    }

    /*
     * The multichar command is terminated by a newline.
     */
    if is_newline_char(c) {
        let opts = get_options();
        if pasting != 0 && opts.no_paste != 0 {
            /* Ignore pasted input after (and including) the first newline */
            start_ignoring_input();
            return ActionType::McaMore;
        }
        /* Execute the command */
        exec_mca();
        return ActionType::McaDone;
    }

    /*
     * Append the char to the command buffer.
     */
    if cmd_char(c) == CC_QUIT {
        /*
         * Abort the multi-char command.
         */
        return ActionType::McaDone;
    }
    match mca {
        ActionType::FBracket | ActionType::BBracket => {
            if len_cmdbuf() >= 2 {
                /*
                 * Special case for the bracket-matching commands.
                 * Execute the command after getting exactly two
                 * characters from the user.
                 */
                exec_mca();
                return ActionType::McaDone;
            }
        }
        ActionType::FSearch | ActionType::BSearch => {
            let opts = get_options();
            if opts.incr_search != 0 {
                /* Incremental search: do a search after every input char. */
                let mut st = search_type
                    & (SRCH_FORW
                        | SRCH_BACK
                        | SRCH_NO_MATCH
                        | SRCH_NO_REGEX
                        | SRCH_NO_MOVE
                        | SRCH_WRAP
                        | SRCH_SUBSEARCH_ALL);
                let mut save_updown = 0;
                let Some(pattern_cs) = get_cmdbuf() else { return ActionType::McaMore };
                let mut pattern = pattern_cs.as_ptr();
                /*
                 * Must save updown_match because mca_search
                 * reinits it. That breaks history scrolling.
                 * {{ This is ugly. mca_search probably shouldn't call set_mlist. }}
                 */
                save_updown = save_updown_match();
                cmd_exec();
                if *pattern as std::ffi::c_int == '\0' as i32 {
                    /* User has backspaced to an empty pattern. */
                    undo_search(LTRUE);
                } else {
                    /*
                     * Suppress tty polling while searching.
                     * This avoids a problem where tty input
                     * can cause the search to be interrupted.
                     */
                    no_poll = LTRUE;
                    if search(
                        /* No match, invalid pattern, etc. */
                        st | (1 as std::ffi::c_int) << 3 as std::ffi::c_int,
                        pattern,
                        1 as std::ffi::c_int,
                    ) != 0 as std::ffi::c_int
                    {
                        undo_search(LTRUE);
                    }
                    no_poll = LFALSE;
                }
                /* Redraw the search prompt and search string. */
                if is_screen_trashed() != 0 || full_screen == 0 {
                    clear();
                    repaint();
                }
                mca_search1();
                restore_updown_match(save_updown);
                cmd_repaint(None);
            }
        }
        _ => {}
    }
    /*
     * Need another character.
     */
    ActionType::McaMore
}
unsafe extern "C" fn clear_buffers() {
    if ch_getflags() & 0o1 as std::ffi::c_int == 0 {
        return;
    }
    ch_flush();
    clr_linenum();
    clr_hilite();
    set_line_contig_pos(-(1 as std::ffi::c_int) as POSITION);
}
#[no_mangle]
pub unsafe extern "C" fn screen_trashed_num(mut trashed: std::ffi::c_int) {
    screen_trashed_value = trashed;
}
#[no_mangle]
pub unsafe extern "C" fn screen_trashed() {
    screen_trashed_num(1 as std::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn is_screen_trashed() -> std::ffi::c_int {
    return screen_trashed_value;
}
unsafe extern "C" fn make_display() {
    let opts = get_options();
    if full_screen == 0 && !(opts.quit_if_one_screen != 0 && one_screen != 0) {
        clear();
    }
    if empty_screen() != 0 {
        if initial_scrpos.pos == -(1 as std::ffi::c_int) as POSITION {
            jump_loc(0 as std::ffi::c_int as POSITION, 1 as std::ffi::c_int);
        } else {
            jump_loc(initial_scrpos.pos, initial_scrpos.ln);
        }
    } else if is_screen_trashed() != 0 || full_screen == 0 {
        let opts = get_options();
        let mut save_top_scroll: std::ffi::c_int = opts.top_scroll;
        let mut save_ignore_eoi: std::ffi::c_int = ignore_eoi;
        opts.top_scroll = 1 as std::ffi::c_int;
        ignore_eoi = 0 as std::ffi::c_int;
        if is_screen_trashed() == 2 as std::ffi::c_int {
            reopen_curr_ifile();
            jump_forw();
        }
        repaint();
        opts.top_scroll = save_top_scroll;
        ignore_eoi = save_ignore_eoi;
    }
}

/*
 * Display the appropriate prompt.
 */
unsafe extern "C" fn prompt(ungot: &mut Ungot, o: &Options) {
    let mut p: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    if !ungot.is_empty() && ungot.current().unwrap().end_command {
        /*
         * No prompt necessary if commands are from
         * ungotten chars rather than from the user.
         */
        return;
    }
    make_display();
    bottompos = position(-(2 as std::ffi::c_int));
    if get_quit_at_eof() == 2 as std::ffi::c_int
        && eof_displayed(LFALSE) as std::ffi::c_uint != 0
        && ch_getflags() & 0o10 as std::ffi::c_int == 0
        && next_ifile(curr_ifile) == 0 as *mut std::ffi::c_void
    {
        quit(0 as std::ffi::c_int);
    }
    let opts = get_options();
    if opts.quit_if_one_screen != 0
        && entire_file_displayed() as std::ffi::c_uint != 0
        && ch_getflags() & 0o10 as std::ffi::c_int == 0
        && next_ifile(curr_ifile) == 0 as *mut std::ffi::c_void
    {
        quit(0 as std::ffi::c_int);
    }
    opts.quit_if_one_screen = LFALSE as std::ffi::c_int;
    if forw_prompt == 0 {
        clear_bot();
    }
    clear_cmd();
    forw_prompt = 0 as std::ffi::c_int;
    p = pr_string();
    if is_filtering() as u64 != 0 {
        putstr(b"& \0" as *const u8 as *const std::ffi::c_char);
    }
    if search_wrapped as u64 != 0 {
        if search_type & (1 as std::ffi::c_int) << 1 as std::ffi::c_int != 0 {
            error(
                b"Search hit top; continuing at bottom\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
        } else {
            error(
                b"Search hit bottom; continuing at top\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
        }
        search_wrapped = LFALSE;
    }
    if !osc8_uri.is_null() {
        let mut parg: PARG = parg {
            p_string: 0 as *const std::ffi::c_char,
        };
        parg.p_string = osc8_uri;
        error(
            b"Link: %s\0" as *const u8 as *const std::ffi::c_char,
            &mut parg,
        );
        free(osc8_uri as *mut std::ffi::c_void);
        osc8_uri = 0 as *mut std::ffi::c_char;
    }
    if p.is_null() || *p as std::ffi::c_int == '\0' as i32 {
        at_enter(0 as std::ffi::c_int | (7 as std::ffi::c_int) << 8 as std::ffi::c_int);
        putchr(':' as i32);
        at_exit();
    } else {
        load_line(o, CStr::from_ptr(p).to_bytes());
        put_line(LFALSE);
    }
    clear_eol();
}
#[no_mangle]
pub unsafe extern "C" fn dispversion() {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    parg.p_string = version.as_mut_ptr();
    error(
        b"less %s\0" as *const u8 as *const std::ffi::c_char,
        &mut parg,
    );
}

/*
 * Return a character to complete a partial command, if possible.
 */
unsafe extern "C" fn getcc_end_command(ungot: &Ungot) -> char {
    let mut ch = 0i32;
    match mca {
        /* We have a number but no command.  Treat as #g. */
        ActionType::Digit => return 'g',
        ActionType::FSearch | ActionType::BSearch | ActionType::Filter => {
            /* We have "/string" but no newline.  Add the \n. */
            return '\n';
        }
        _ => {
            /* Some other incomplete command.  Let user complete it. */
            if !ungot.is_empty() {
                return '\0';
            }
            ch = getchr();
            if ch < 0 {
                ch = '\0' as i32;
            }
            return char::from_u32(ch as u32).unwrap();
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn getcc_clear(ungot: &mut Ungot) {
    while !ungot.is_empty() {
        ungot.get_ungot(None);
    }
}

/*
 * Get command character.
 * The character normally comes from the keyboard,
 * but may come from ungotten characters
 * (characters previously given to ungetcc or ungetsc).
 */
unsafe fn getccu(ungot: &mut Ungot) -> char {
    let mut c = ' ';
    while c as u8 == 0 && sigs == 0 {
        if ungot.is_empty() {
            /* Normal case: no ungotten chars.
             * Get char from the user. */
            let ch = getchr();
            if ch < 0 {
                c = '\0';
            }
        } else {
            /* Ungotten chars available:
             * Take the top of stack (most recent). */
            let mut end_command = false;
            (c, _) = ungot.get_ungot(Some(end_command));
            if end_command {
                c = getcc_end_command(ungot);
            }
        }
    }
    c
}

/*
 * Get a command character, but if we receive the orig sequence,
 * convert it to the repl sequence.
 */
unsafe extern "C" fn getcc_repl(
    ungot: &mut Ungot,
    orig: Option<&str>,
    repl: Option<&str>,
    gr_getc: Option<unsafe fn(&mut Ungot) -> char>,
    gr_ungetc: Option<unsafe fn(&mut Ungot, char)>,
) -> char {
    let mut keys: [char; 16] = ['\0'; 16];
    let mut ki = 0;

    let mut c = gr_getc.unwrap()(ungot);
    if orig.is_none() || orig.unwrap().len() == 0 {
        return c;
    }
    let orig = orig.unwrap();
    let repl = repl.unwrap();
    loop {
        keys[ki] = c;
        if Some(c) != orig.chars().nth(ki) || ki >= keys.len() {
            /* This is not orig we have been receiving.
             * If we have stashed chars in keys[],
             * unget them and return the first one. */
            while ki > 0 {
                gr_ungetc.expect("non-null function pointer")(ungot, keys[ki]);
                ki -= 1;
            }
            return keys[0];
        }
        ki += 1;
        if orig.chars().nth(ki) == Some('\0') {
            /* We've received the full orig sequence.
             * Return the repl sequence. */
            ki = repl.len();
            while ki > 0 {
                gr_ungetc.expect("non-null function pointer")(
                    ungot,
                    repl.chars().nth(ki).unwrap(),
                );
                ki -= 1;
                return repl.chars().nth(0).unwrap();
            }
            return repl.chars().nth(0).unwrap();
        }
        c = gr_getc.expect("No function defined")(ungot);
    }
}

/*
 * Get command character.
 */
#[no_mangle]
pub unsafe extern "C" fn getcc(ungot: &mut Ungot) -> char {
    /* Replace kent (keypad Enter) with a newline. */
    return getcc_repl(
        ungot,
        Some(&kent),
        Some("\n"),
        Some(getccu),
        Some(Ungot::ungetcc as unsafe fn(&mut Ungot, char)),
    );
}

impl Ungot {
    pub fn new() -> Self {
        Ungot { chars: Vec::new() }
    }

    /// "Unget" a command character.
    /// The next getcc() will return this character.
    pub unsafe fn ungetcc(&mut self, c: char) {
        self.chars.push(Char {
            ch: c,
            end_command: false,
        });
    }

    /*
     * "Unget" a command character.
     * If any other chars are already ungotten, put this one after those.
     */
    unsafe extern "C" fn ungetcc_back1(&mut self, c: char, end_command: bool) {
        self.chars.push(Char {
            ch: c,
            end_command: end_command,
        });
    }

    pub unsafe extern "C" fn ungetcc_back(&mut self, c: char) {
        self.ungetcc_back1(c, false);
    }

    pub unsafe extern "C" fn ungetcc_end_command(&mut self) {
        self.ungetcc_back1('\0', true);
    }

    /*
     * Unget a whole string of command characters.
     * The next sequence of getcc()'s will return this string.
     */
    pub unsafe extern "C" fn ungetsc(&mut self, s: &str) {
        for c in s.chars() {
            self.ungetcc(c);
        }
    }

    /// Peek the next command character, without consuming it.
    pub unsafe extern "C" fn peekcc(&mut self) -> char {
        let mut c = getcc(self);
        self.ungetcc(c);
        c
    }

    /// Get a command character from the ungotten stack.
    unsafe extern "C" fn get_ungot(&mut self, p_end_command: Option<bool>) -> (char, Option<bool>) {
        let ug = self.chars.pop().unwrap();
        if let Some(end_command) = p_end_command {
            return (ug.ch, Some(ug.end_command));
        }
        (ug.ch, None)
    }

    pub unsafe fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }

    pub unsafe fn current(&self) -> Option<&Char> {
        self.chars.last()
    }
}

unsafe extern "C" fn multi_search(
    mut pattern: *const std::ffi::c_char,
    mut n: std::ffi::c_int,
    mut silent: std::ffi::c_int,
) {
    let mut nomore: std::ffi::c_int = 0;
    let mut save_ifile: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    let mut changed_file: lbool = LFALSE;
    changed_file = LFALSE;
    save_ifile = save_curr_ifile();
    if search_type
        & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
            | (1 as std::ffi::c_int) << 1 as std::ffi::c_int)
        == 0 as std::ffi::c_int
    {
        search_type |= (1 as std::ffi::c_int) << 0 as std::ffi::c_int;
    }
    if search_type & (1 as std::ffi::c_int) << 10 as std::ffi::c_int != 0 {
        if search_type & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
            nomore = edit_first();
        } else {
            nomore = edit_last();
        }
        if nomore != 0 {
            unsave_ifile(save_ifile);
            return;
        }
        changed_file = LTRUE;
        search_type &= !((1 as std::ffi::c_int) << 10 as std::ffi::c_int);
    }
    loop {
        n = search(search_type, pattern, n);
        search_type &= !((1 as std::ffi::c_int) << 2 as std::ffi::c_int);
        last_search_type = search_type;
        if n == 0 as std::ffi::c_int {
            unsave_ifile(save_ifile);
            return;
        }
        if n < 0 as std::ffi::c_int {
            break;
        } else {
            if search_type & (1 as std::ffi::c_int) << 9 as std::ffi::c_int == 0 as std::ffi::c_int
            {
                break;
            }
            if search_type & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
                nomore = edit_next(1 as std::ffi::c_int);
            } else {
                nomore = edit_prev(1 as std::ffi::c_int);
            }
            if nomore != 0 {
                break;
            }
            changed_file = LTRUE;
        }
    }
    if n > 0 as std::ffi::c_int && silent == 0 {
        error(
            b"Pattern not found\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
    }
    if changed_file as u64 != 0 {
        reedit_ifile(save_ifile);
    } else {
        unsave_ifile(save_ifile);
    };
}
unsafe extern "C" fn forw_loop(mut until_hilite: std::ffi::c_int) -> ActionType {
    let mut curr_len: POSITION = 0;
    if ch_getflags() & 0o10 as std::ffi::c_int != 0 {
        return ActionType::NoAction;
    }
    cmd_exec();
    jump_forw_buffered();
    curr_len = ch_length();
    highest_hilite = if until_hilite != 0 {
        curr_len
    } else {
        -(1 as std::ffi::c_int) as POSITION
    };
    ignore_eoi = 1 as std::ffi::c_int;
    while sigs == 0 {
        if until_hilite != 0 && highest_hilite > curr_len {
            bell();
            break;
        } else {
            make_display();
            forward(1 as std::ffi::c_int, LFALSE, LFALSE, LFALSE);
        }
    }
    ignore_eoi = 0 as std::ffi::c_int;
    ch_set_eof();
    if sigs != 0
        && sigs
            & ((1 as std::ffi::c_int) << 0 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                | (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
            == 0
    {
        return if until_hilite != 0 {
            ActionType::FUntilHilite
        } else {
            ActionType::FForever
        };
    }
    ActionType::NoAction
}
#[no_mangle]
pub unsafe extern "C" fn start_ignoring_input() {
    ignoring_input = LTRUE;
    ignoring_input_time = get_time();
}
#[no_mangle]
pub unsafe extern "C" fn stop_ignoring_input() {
    ignoring_input = LFALSE;
    pasting = LFALSE;
}
#[no_mangle]
pub unsafe extern "C" fn is_ignoring_input(mut action: ActionType) -> bool {
    if ignoring_input as u64 == 0 {
        return false;
    }
    if action == ActionType::EndPaste {
        stop_ignoring_input();
    }
    if get_time() >= ignoring_input_time + 5 as std::ffi::c_int as time_t {
        stop_ignoring_input();
    }
    action != ActionType::Prefix
}
#[no_mangle]
pub unsafe extern "C" fn commands(marks: &mut Marks, ifiles: &mut IFileManager, ungot: &mut Ungot, o: &Options) {
    let mut current_block: u64;
    let mut c = ' ';
    let mut action = ActionType::NoAction;
    let mut cbuf: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut msg: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut save_jump_sline: std::ffi::c_int = 0;
    let mut save_search_type: std::ffi::c_int = 0;
    let mut extra = None;
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    let mut old_ifile: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    let mut new_ifile: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    let mut tagfile: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    search_type = (1 as std::ffi::c_int) << 0 as std::ffi::c_int;
    wscroll = (sc_height + 1 as std::ffi::c_int) / 2 as std::ffi::c_int;
    let mut newaction = ActionType::NoAction;
    's_39: loop {
        clear_mca();
        cmd_accept();
        number = 0 as std::ffi::c_int as LINENUM;
        curropt = std::ptr::null_mut();
        if sigs != 0 {
            psignals();
            if quitting as u64 != 0 {
                quit(-(1 as std::ffi::c_int));
            }
        }
        check_winch();
        cmd_reset();
        prompt(ungot, o);
        if sigs != 0 {
            continue;
        }
        if newaction == ActionType::NoAction {
            c = getcc(ungot);
        }
        loop {
            if sigs != 0 {
                continue 's_39;
            }
            if newaction != ActionType::NoAction {
                action = newaction;
                newaction = ActionType::NoAction;
            } else {
                /*
                 * If we are in a multicharacter command, call mca_char.
                 * Otherwise we call fcmd_decode to determine the
                 * action to be performed.
                 */
                if mca != ActionType::Null {
                    match mca_char(ungot, c as u8 as char) {
                        ActionType::McaMore => {
                            /*
                             * Need another character.
                             */
                            c = getcc(ungot);
                            continue;
                        }
                        ActionType::McaDone => {
                            /*
                             * Not a multi-char command
                             * (at least, not anymore).
                             */
                            continue 's_39;
                        }
                        ActionType::NoMca | _ => {
                            /*
                             * Not a multi-char command
                             * (at least, not anymore).
                             */
                        }
                    }
                }
                /*
                 * Decode the command character and decide what to do.
                 */
                if mca != ActionType::Null {
                    /*
                     * We're in a multichar command.
                     * Add the character to the command buffer
                     * and display it on the screen.
                     * If the user backspaces past the start
                     * of the line, abort the command.
                     */
                    if cmd_char(c) == 1 as std::ffi::c_int
                        || cmdbuf_empty()
                    {
                        continue 's_39;
                    }
                    let Some(cbuf_cs) = get_cmdbuf() else {
                        c = getcc(ungot);
                        continue;
                    };
                    cbuf = cbuf_cs.as_ptr();
                    let mut extra_str: Option<String> = None;
                    if let Some(tables) = get_tables_mut() {
                        let cbuf = CStr::from_ptr(cbuf).to_bytes();
                        action = fcmd_decode(&tables, cbuf, &mut extra);
                        if let Some((spi, t_idx)) = extra {
                            extra_str = Some(tables.get_fcmd_extra_str(spi, t_idx).to_string());
                        }
                    }
                    if let Some(ref s) = extra_str {
                        ungot.ungetsc(s);
                    }
                } else {
                    /*
                     * Don't use cmd_char if we're starting fresh
                     * at the beginning of a command, because we
                     * don't want to echo the command until we know
                     * it is a multichar command.  We also don't
                     * want erase_char/kill_char to be treated
                     * as line editing characters.
                     */
                    let tbuf = b"c\0";
                    let mut extra_str: Option<String> = None;
                    if let Some(tables) = get_tables_mut() {
                        action = fcmd_decode(&tables, tbuf, &mut extra);
                        if let Some((spi, t_idx)) = extra {
                            extra_str = Some(tables.get_fcmd_extra_str(spi, t_idx).to_string());
                        }
                    }
                    if let Some(ref s) = extra_str {
                        ungot.ungetsc(s);
                    }
                }
            }
            if action != ActionType::Prefix {
                cmd_reset();
            }
            if is_ignoring_input(action) as u64 != 0 {
                continue 's_39;
            }
            let opts = get_options();
            match action {
                ActionType::StartPaste => {
                    if opts.no_paste != 0 {
                        start_ignoring_input();
                    }
                    continue 's_39;
                }
                ActionType::Digit => {
                    /*
                     * First digit of a number.
                     */
                    start_mca(
                        ActionType::Digit,
                        b":\0" as *const u8 as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void,
                        (1 as std::ffi::c_int) << 0 as std::ffi::c_int,
                    );
                }
                ActionType::FWindow => {
                    /*
                     * Forward one window (and set the window size).
                     */
                    if number > 0 as std::ffi::c_int as LINENUM {
                        opts.swindow = number as std::ffi::c_int;
                    }
                    current_block = 3507267478320338004;
                    break;
                }
                ActionType::FScreen => {
                    /*
                     * Forward one screen.
                     */
                    current_block = 3507267478320338004;
                    break;
                }
                ActionType::BWindow => {
                    /*
                     * Backward one window (and set the window size).
                     */
                    if number > 0 as std::ffi::c_int as LINENUM {
                        opts.swindow = number as std::ffi::c_int;
                    }
                    current_block = 2194593563755971021;
                    break;
                }
                ActionType::BScreen => {
                    /*
                     * Backward one screen.
                     */
                    current_block = 2194593563755971021;
                    break;
                }
                ActionType::FLine | ActionType::FNewline => {
                    /*
                     * Forward N (default 1) line.
                     */
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    cmd_exec();
                    if opts.show_attn == 2 as std::ffi::c_int
                        && number > 1 as std::ffi::c_int as LINENUM
                    {
                        set_attnpos(bottompos);
                    }
                    forward(
                        number as std::ffi::c_int,
                        LFALSE,
                        LFALSE,
                        (action == ActionType::FNewline && opts.chopline == 0) as std::ffi::c_int
                            as lbool,
                    );
                    continue 's_39;
                }
                ActionType::BLine | ActionType::BNewline => {
                    /*
                     * Backward N (default 1) line.
                     */
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    cmd_exec();
                    backward(
                        number as std::ffi::c_int,
                        LFALSE,
                        LFALSE,
                        (action == ActionType::BNewline && opts.chopline == 0) as std::ffi::c_int
                            as lbool,
                    );
                    continue 's_39;
                }
                ActionType::FMouse => {
                    /*
                     * Forward wheel_lines lines.
                     */
                    cmd_exec();
                    forward(opts.wheel_lines, LFALSE, LFALSE, LFALSE);
                    continue 's_39;
                }
                ActionType::BMouse => {
                    /*
                     * Backward wheel_lines lines.
                     */
                    cmd_exec();
                    backward(opts.wheel_lines, LFALSE, LFALSE, LFALSE);
                    continue 's_39;
                }
                ActionType::FFLine => {
                    /*
                     * Force forward N (default 1) line.
                     */
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    cmd_exec();
                    if opts.show_attn == 2 as std::ffi::c_int
                        && number > 1 as std::ffi::c_int as LINENUM
                    {
                        set_attnpos(bottompos);
                    }
                    forward(number as std::ffi::c_int, LTRUE, LFALSE, LFALSE);
                    continue 's_39;
                }
                ActionType::BFLine => {
                    /*
                     * Force backward N (default 1) line.
                     */
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    cmd_exec();
                    backward(number as std::ffi::c_int, LTRUE, LFALSE, LFALSE);
                    continue 's_39;
                }
                ActionType::FFScreen => {
                    /*
                     * Force forward one screen.
                     */
                    let opts = get_options();
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = get_swindow() as LINENUM;
                    }
                    cmd_exec();
                    if opts.show_attn == 2 as std::ffi::c_int {
                        set_attnpos(bottompos);
                    }
                    forward(number as std::ffi::c_int, LTRUE, LFALSE, LFALSE);
                    continue 's_39;
                }
                ActionType::BFScreen => {
                    /*
                     * Force backward one screen.
                     */
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = get_swindow() as LINENUM;
                    }
                    cmd_exec();
                    backward(number as std::ffi::c_int, LTRUE, LFALSE, LFALSE);
                    continue 's_39;
                }
                ActionType::FForever => {
                    /*
                     * Forward forever, ignoring EOF.
                     */
                    let opts = get_options();
                    if !(get_altfilename(curr_ifile)).is_null() {
                        error(
                            b"Warning: command may not work correctly when file is viewed via LESSOPEN\0"
                                as *const u8 as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void as *mut PARG,
                        );
                    }
                    if opts.show_attn != 0 {
                        set_attnpos(bottompos);
                    }
                    newaction = forw_loop(0 as std::ffi::c_int);
                    continue 's_39;
                }
                ActionType::FUntilHilite => {
                    newaction = forw_loop(1 as std::ffi::c_int);
                    continue 's_39;
                }
                ActionType::FScroll => {
                    /*
                     * Forward N lines
                     * (default same as last 'd' or 'u' command).
                     */
                    let opts = get_options();
                    if number > 0 as std::ffi::c_int as LINENUM {
                        wscroll = number as std::ffi::c_int;
                    }
                    cmd_exec();
                    if opts.show_attn == 2 as std::ffi::c_int {
                        set_attnpos(bottompos);
                    }
                    forward(wscroll, LFALSE, LFALSE, LFALSE);
                    continue 's_39;
                }
                ActionType::BScroll => {
                    /*
                     * Forward N lines
                     * (default same as last 'd' or 'u' command).
                     */
                    if number > 0 as std::ffi::c_int as LINENUM {
                        wscroll = number as std::ffi::c_int;
                    }
                    cmd_exec();
                    backward(wscroll, LFALSE, LFALSE, LFALSE);
                    continue 's_39;
                }
                ActionType::FRepaint => {
                    /*
                     * Flush buffers, then repaint screen.
                     * Don't flush the buffers on a pipe!
                     */
                    clear_buffers();
                    current_block = 12373568287479140350;
                    break;
                }
                ActionType::Repaint => {
                    /*
                     * Repaint screen.
                     */
                    current_block = 12373568287479140350;
                    break;
                }
                ActionType::GoLine => {
                    /*
                     * Go to line N, default beginning of file.
                     * If N <= 0, ignore jump_sline in order to avoid
                     * empty lines before the beginning of the file.
                     */
                    let opts = get_options();
                    save_jump_sline = opts.jump_sline;
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                        opts.jump_sline = 0 as std::ffi::c_int;
                    }
                    cmd_exec();
                    jump_back(number);
                    opts.jump_sline = save_jump_sline;
                    continue 's_39;
                }
                ActionType::Percent => {
                    /*
                     * Go to a specified percentage into the file.
                     */
                    if number < 0 as std::ffi::c_int as LINENUM {
                        number = 0 as std::ffi::c_int as LINENUM;
                        fraction = 0 as std::ffi::c_int as std::ffi::c_long;
                    }
                    if number > 100 as std::ffi::c_int as LINENUM
                        || number == 100 as std::ffi::c_int as LINENUM
                            && fraction != 0 as std::ffi::c_int as std::ffi::c_long
                    {
                        number = 100 as std::ffi::c_int as LINENUM;
                        fraction = 0 as std::ffi::c_int as std::ffi::c_long;
                    }
                    cmd_exec();
                    jump_percent(number as std::ffi::c_int, fraction);
                    continue 's_39;
                }
                ActionType::GoEnd => {
                    /*
                     * Go to line N, default end of file.
                     */
                    cmd_exec();
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        jump_forw();
                    } else {
                        jump_back(number);
                    }
                    continue 's_39;
                }
                ActionType::GoEndBuf => {
                    /*
                     * Go to line N, default last buffered byte.
                     */
                    cmd_exec();
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        jump_forw_buffered();
                    } else {
                        jump_back(number);
                    }
                    continue 's_39;
                }
                ActionType::GoPos => {
                    /*
                     * Go to a specified byte position in the file.
                     */
                    let opts = get_options();
                    cmd_exec();
                    if number < 0 as std::ffi::c_int as LINENUM {
                        number = 0 as std::ffi::c_int as LINENUM;
                    }
                    jump_line_loc(number, opts.jump_sline);
                    continue 's_39;
                }
                ActionType::Stat => {
                    /*
                     * Print file name, etc.
                     */
                    if ch_getflags() & 0o10 as std::ffi::c_int != 0 {
                        continue 's_39;
                    }
                    cmd_exec();
                    parg.p_string = eq_message();
                    error(b"%s\0" as *const u8 as *const std::ffi::c_char, &mut parg);
                    continue 's_39;
                }
                ActionType::Version => {
                    /*
                     * Print version number.
                     */
                    cmd_exec();
                    dispversion();
                    continue 's_39;
                }
                ActionType::Quit => {
                    /*
                     * Exit.
                     */
                    if curr_ifile != 0 as *mut std::ffi::c_void
                        && ch_getflags() & 0o10 as std::ffi::c_int != 0
                    {
                        /*
                         * Quit while viewing the help file
                         * just means return to viewing the
                         * previous file.
                         */
                        current_block = 5431927413890720344;
                        break;
                    } else {
                        current_block = 16974974966130203269;
                        break;
                    }
                }
                ActionType::FSearch => {
                    let opts = get_options();
                    search_type =
                        (1 as std::ffi::c_int) << 0 as std::ffi::c_int | opts.def_search_type;
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    literal_char = false;
                    mca_search();
                    c = getcc(ungot);
                }
                ActionType::BSearch => {
                    let opts = get_options();
                    search_type =
                        (1 as std::ffi::c_int) << 1 as std::ffi::c_int | opts.def_search_type;
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    literal_char = false;
                    mca_search();
                    c = getcc(ungot);
                }
                ActionType::Osc8FSearch => {
                    cmd_exec();
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    osc8_search(
                        (1 as std::ffi::c_int) << 0 as std::ffi::c_int,
                        0 as *const std::ffi::c_char,
                        number as std::ffi::c_int,
                    );
                    continue 's_39;
                }
                ActionType::Osc8BSearch => {
                    cmd_exec();
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    osc8_search(
                        (1 as std::ffi::c_int) << 1 as std::ffi::c_int,
                        0 as *const std::ffi::c_char,
                        number as std::ffi::c_int,
                    );
                    continue 's_39;
                }
                ActionType::Osc8Open => {
                    if secure_allow((1 as std::ffi::c_int) << 12 as std::ffi::c_int) != 0 {
                        current_block = 6662862405959679103;
                        break;
                    } else {
                        current_block = 2089914658669629659;
                        break;
                    }
                }
                ActionType::Osc8Jump => {
                    cmd_exec();
                    osc8_jump();
                    continue 's_39;
                }
                ActionType::Filter => {
                    search_type = (1 as std::ffi::c_int) << 0 as std::ffi::c_int
                        | (1 as std::ffi::c_int) << 13 as std::ffi::c_int;
                    literal_char = false;
                    mca_search();
                    c = getcc(ungot);
                }
                ActionType::AgainSearch => {
                    search_type = last_search_type;
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    mca_search();
                    cmd_exec();
                    multi_search(
                        0 as *const std::ffi::c_char,
                        number as std::ffi::c_int,
                        0 as std::ffi::c_int,
                    );
                    continue 's_39;
                }
                ActionType::TAgainSearch => {
                    search_type = last_search_type | (1 as std::ffi::c_int) << 9 as std::ffi::c_int;
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    mca_search();
                    cmd_exec();
                    multi_search(
                        0 as *const std::ffi::c_char,
                        number as std::ffi::c_int,
                        0 as std::ffi::c_int,
                    );
                    continue 's_39;
                }
                ActionType::ReverseSearch => {
                    search_type = last_search_type;
                    save_search_type = search_type;
                    search_type =
                        if search_type & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
                            search_type & !((1 as std::ffi::c_int) << 0 as std::ffi::c_int)
                                | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                        } else {
                            search_type & !((1 as std::ffi::c_int) << 1 as std::ffi::c_int)
                                | (1 as std::ffi::c_int) << 0 as std::ffi::c_int
                        };
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    mca_search();
                    cmd_exec();
                    multi_search(
                        0 as *const std::ffi::c_char,
                        number as std::ffi::c_int,
                        0 as std::ffi::c_int,
                    );
                    last_search_type = save_search_type;
                    continue 's_39;
                }
                ActionType::TReverseSearch => {
                    search_type = last_search_type;
                    save_search_type = search_type;
                    search_type =
                        (if search_type & (1 as std::ffi::c_int) << 0 as std::ffi::c_int != 0 {
                            search_type & !((1 as std::ffi::c_int) << 0 as std::ffi::c_int)
                                | (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                        } else {
                            search_type & !((1 as std::ffi::c_int) << 1 as std::ffi::c_int)
                                | (1 as std::ffi::c_int) << 0 as std::ffi::c_int
                        }) | (1 as std::ffi::c_int) << 9 as std::ffi::c_int;
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    mca_search();
                    cmd_exec();
                    multi_search(
                        0 as *const std::ffi::c_char,
                        number as std::ffi::c_int,
                        0 as std::ffi::c_int,
                    );
                    last_search_type = save_search_type;
                    continue 's_39;
                }
                ActionType::UndoSearch | ActionType::ClrSearch => {
                    undo_search((action == ActionType::ClrSearch) as std::ffi::c_int as lbool);
                    continue 's_39;
                }
                ActionType::Help => {
                    let opts = get_options();
                    if ch_getflags() & 0o10 as std::ffi::c_int != 0 {
                        continue 's_39;
                    }
                    cmd_exec();
                    save_hshift = hshift;
                    hshift = 0 as std::ffi::c_int;
                    save_bs_mode = opts.bs_mode;
                    opts.bs_mode = 0 as std::ffi::c_int;
                    save_proc_backspace = opts.proc_backspace;
                    opts.proc_backspace = 0 as std::ffi::c_int;
                    edit(b"@/\\less/\\help/\\file/\\@\0" as *const u8 as *const std::ffi::c_char);
                    continue 's_39;
                }
                ActionType::Examine => {
                    if secure_allow((1 as std::ffi::c_int) << 2 as std::ffi::c_int) != 0 {
                        start_mca(
                            ActionType::Examine,
                            b"Examine: \0" as *const u8 as *const std::ffi::c_char,
                            ml_examine,
                            0 as std::ffi::c_int,
                        );
                        c = getcc(ungot);
                    } else {
                        error(
                            b"Command not available\0" as *const u8 as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void as *mut PARG,
                        );
                        continue 's_39;
                    }
                }
                ActionType::Visual => {
                    if secure_allow((1 as std::ffi::c_int) << 1 as std::ffi::c_int) != 0 {
                        current_block = 16718638665978159145;
                        break;
                    } else {
                        current_block = 8120009455218959897;
                        break;
                    }
                }
                ActionType::NextFile => {
                    if ntags() != 0 {
                        current_block = 18001984906674336099;
                        break;
                    } else {
                        current_block = 14148461183130080616;
                        break;
                    }
                }
                ActionType::PrevFile => {
                    if ntags() != 0 {
                        current_block = 8193737063574930042;
                        break;
                    } else {
                        current_block = 3818209998506676277;
                        break;
                    }
                }
                ActionType::NextTag => {
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    tagfile = nexttag(number as std::ffi::c_int);
                    if tagfile.is_null() {
                        current_block = 8075351136037156718;
                        break;
                    } else {
                        current_block = 5798072534372498777;
                        break;
                    }
                }
                ActionType::PrevTag => {
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    tagfile = prevtag(number as std::ffi::c_int);
                    if tagfile.is_null() {
                        current_block = 10029375464402185584;
                        break;
                    } else {
                        current_block = 13267105165099174640;
                        break;
                    }
                }
                ActionType::IndexFile => {
                    if number <= 0 as std::ffi::c_int as LINENUM {
                        number = 1 as std::ffi::c_int as LINENUM;
                    }
                    cmd_exec();
                    if edit_index(number as std::ffi::c_int) != 0 {
                        error(
                            b"No such file\0" as *const u8 as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void as *mut PARG,
                        );
                    }
                    continue 's_39;
                }
                ActionType::RemoveFile => {
                    if ch_getflags() & 0o10 as std::ffi::c_int != 0 {
                        continue 's_39;
                    }
                    old_ifile = curr_ifile;
                    new_ifile = getoff_ifile(curr_ifile);
                    cmd_exec();
                    if new_ifile == 0 as *mut std::ffi::c_void {
                        current_block = 18330534242458572360;
                        break;
                    } else {
                        current_block = 18361011714114715771;
                        break;
                    }
                }
                ActionType::OptToggle => {
                    optflag = 1 as std::ffi::c_int;
                    optgetname = false;
                    mca_opt_toggle();
                    c = getcc(ungot);
                    msg = opt_toggle_disallowed(c as std::ffi::c_int);
                    if msg.is_null() {
                        continue;
                    }
                    error(msg, 0 as *mut std::ffi::c_void as *mut PARG);
                    continue 's_39;
                }
                ActionType::DispOption => {
                    optflag = 0 as std::ffi::c_int;
                    optgetname = false;
                    mca_opt_toggle();
                    c = getcc(ungot);
                }
                ActionType::FirstCmd => {
                    start_mca(
                        ActionType::FirstCmd,
                        b"+\0" as *const u8 as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void,
                        0 as std::ffi::c_int,
                    );
                    c = getcc(ungot);
                }
                ActionType::Shell | ActionType::PShell => {
                    if secure_allow((1 as std::ffi::c_int) << 9 as std::ffi::c_int) != 0 {
                        start_mca(
                            action,
                            if action == ActionType::Shell {
                                b"!\0" as *const u8 as *const std::ffi::c_char
                            } else {
                                b"#\0" as *const u8 as *const std::ffi::c_char
                            },
                            ml_shell,
                            0 as std::ffi::c_int,
                        );
                        c = getcc(ungot);
                    } else {
                        error(
                            b"Command not available\0" as *const u8 as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void as *mut PARG,
                        );
                        continue 's_39;
                    }
                }
                ActionType::SetMark | ActionType::SetMarkBot => {
                    if ch_getflags() & 0o10 as std::ffi::c_int != 0 {
                        current_block = 7991679940794782184;
                        break;
                    } else {
                        current_block = 2595745308905254098;
                        break;
                    }
                }
                ActionType::ClrMark => {
                    start_mca(
                        ActionType::ClrMark,
                        b"clear mark: \0" as *const u8 as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void,
                        0 as std::ffi::c_int,
                    );
                    c = getcc(ungot);
                    if is_erase_char(c) as std::ffi::c_uint != 0
                        || is_newline_char(c) as std::ffi::c_uint != 0
                    {
                        continue 's_39;
                    }
                    marks.clrmark(c as u8);
                    repaint();
                    continue 's_39;
                }
                ActionType::GoMark => {
                    start_mca(
                        ActionType::GoMark,
                        b"goto mark: \0" as *const u8 as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void,
                        0 as std::ffi::c_int,
                    );
                    c = getcc(ungot);
                    if is_erase_char(c) as std::ffi::c_uint != 0
                        || is_newline_char(c) as std::ffi::c_uint != 0
                    {
                        continue 's_39;
                    }
                    cmd_exec();
                    marks.gomark(ifiles, c as u8);
                    continue 's_39;
                }
                ActionType::Pipe => {
                    if secure_allow((1 as std::ffi::c_int) << 8 as std::ffi::c_int) != 0 {
                        start_mca(
                            ActionType::Pipe,
                            b"|mark: \0" as *const u8 as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void,
                            0 as std::ffi::c_int,
                        );
                        c = getcc(ungot);
                        if is_erase_char(c) as u64 != 0 {
                            continue 's_39;
                        }
                        if is_newline_char(c) as u64 != 0 {
                            c = '.';
                        }
                        if marks.badmark(c as u8) {
                            continue 's_39;
                        }
                        pipec = c;
                        start_mca(
                            ActionType::Pipe,
                            b"!\0" as *const u8 as *const std::ffi::c_char,
                            ml_shell,
                            0 as std::ffi::c_int,
                        );
                        c = getcc(ungot);
                    } else {
                        error(
                            b"Command not available\0" as *const u8 as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void as *mut PARG,
                        );
                        continue 's_39;
                    }
                }
                ActionType::FBracket | ActionType::BBracket => {
                    start_mca(
                        action,
                        b"Brackets: \0" as *const u8 as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void,
                        0 as std::ffi::c_int,
                    );
                    c = getcc(ungot);
                }
                ActionType::LShift => {
                    let opts = get_options();
                    if number > 0 as std::ffi::c_int as LINENUM {
                        opts.shift_count = number as std::ffi::c_int;
                    } else {
                        number = (if opts.shift_count > 0 as std::ffi::c_int {
                            opts.shift_count
                        } else {
                            sc_width / 2 as std::ffi::c_int
                        }) as LINENUM;
                    }
                    if number > hshift as LINENUM {
                        number = hshift as LINENUM;
                    }
                    pos_rehead();
                    hshift -= number as std::ffi::c_int;
                    screen_trashed();
                    continue 's_39;
                }
                ActionType::RShift => {
                    let opts = get_options();
                    if number > 0 as std::ffi::c_int as LINENUM {
                        opts.shift_count = number as std::ffi::c_int;
                    } else {
                        number = (if opts.shift_count > 0 as std::ffi::c_int {
                            opts.shift_count
                        } else {
                            sc_width / 2 as std::ffi::c_int
                        }) as LINENUM;
                    }
                    pos_rehead();
                    hshift += number as std::ffi::c_int;
                    screen_trashed();
                    continue 's_39;
                }
                ActionType::LLShift => {
                    pos_rehead();
                    hshift = 0 as std::ffi::c_int;
                    screen_trashed();
                    continue 's_39;
                }
                ActionType::RRShift => {
                    pos_rehead();
                    hshift = rrshift();
                    screen_trashed();
                    continue 's_39;
                }
                ActionType::Prefix => {
                    if mca != ActionType::Prefix {
                        cmd_reset();
                        start_mca(
                            ActionType::Prefix,
                            b" \0" as *const u8 as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void,
                            (1 as std::ffi::c_int) << 0 as std::ffi::c_int,
                        );
                        cmd_char(c);
                    }
                    c = getcc(ungot);
                }
                ActionType::NoAction => {
                    continue 's_39;
                }
                _ => {
                    bell();
                    continue 's_39;
                }
            }
        }
        match current_block {
            16718638665978159145 => {
                if ch_getflags() & 0o10 as std::ffi::c_int != 0 {
                    continue;
                }
                if strcmp(
                    get_filename(curr_ifile),
                    b"-\0" as *const u8 as *const std::ffi::c_char,
                ) == 0 as std::ffi::c_int
                {
                    error(
                        b"Cannot edit standard input\0" as *const u8 as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void as *mut PARG,
                    );
                    continue;
                } else {
                    let opts = get_options();
                    if opts.no_edit_warn == 0 && !(get_altfilename(curr_ifile)).is_null() {
                        error(
                            b"WARNING: This file was viewed via LESSOPEN\0" as *const u8
                                as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void as *mut PARG,
                        );
                    }
                    start_mca(
                        ActionType::Shell,
                        b"!\0" as *const u8 as *const std::ffi::c_char,
                        ml_shell,
                        0 as std::ffi::c_int,
                    );
                    make_display();
                    cmd_exec();
                    lsystem(pr_expand(editproto), 0 as *const std::ffi::c_char);
                    continue;
                }
            }
            18361011714114715771 => {
                if edit_ifile(new_ifile) != 0 as std::ffi::c_int {
                    reedit_ifile(old_ifile);
                    continue;
                } else {
                    del_ifile(old_ifile);
                    continue;
                }
            }
            2595745308905254098 => {
                start_mca(
                    ActionType::SetMark,
                    b"set mark: \0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void,
                    0 as std::ffi::c_int,
                );
                c = getcc(ungot);
                if is_erase_char(c) as std::ffi::c_uint != 0
                    || is_newline_char(c) as std::ffi::c_uint != 0
                {
                    continue;
                }
                marks.setmark(
                    c as u8,
                    if action == ActionType::SetMarkBot {
                        -1
                    } else {
                        0
                    },
                );
                repaint();
                continue;
            }
            14148461183130080616 => {
                if number <= 0 as std::ffi::c_int as LINENUM {
                    number = 1 as std::ffi::c_int as LINENUM;
                }
                cmd_exec();
                if edit_next(number as std::ffi::c_int) != 0 {
                    if get_quit_at_eof() != 0
                        && eof_displayed(LFALSE) as std::ffi::c_uint != 0
                        && ch_getflags() & 0o10 as std::ffi::c_int == 0
                    {
                        quit(0 as std::ffi::c_int);
                    }
                    parg.p_string = if number > 1 as std::ffi::c_int as LINENUM {
                        b"(N-th) \0" as *const u8 as *const std::ffi::c_char
                    } else {
                        b"\0" as *const u8 as *const std::ffi::c_char
                    };
                    error(
                        b"No %snext file\0" as *const u8 as *const std::ffi::c_char,
                        &mut parg,
                    );
                }
                continue;
            }
            2194593563755971021 => {
                if number <= 0 as std::ffi::c_int as LINENUM {
                    number = get_swindow() as LINENUM;
                }
                cmd_exec();
                backward(number as std::ffi::c_int, LFALSE, LTRUE, LFALSE);
                continue;
            }
            6662862405959679103 => {
                cmd_exec();
                osc8_open();
                continue;
            }
            3507267478320338004 => {
                let opts = get_options();
                if number <= 0 as std::ffi::c_int as LINENUM {
                    number = get_swindow() as LINENUM;
                }
                cmd_exec();
                if opts.show_attn != 0 {
                    set_attnpos(bottompos);
                }
                forward(number as std::ffi::c_int, LFALSE, LTRUE, LFALSE);
                continue;
            }
            3818209998506676277 => {
                if number <= 0 as std::ffi::c_int as LINENUM {
                    number = 1 as std::ffi::c_int as LINENUM;
                }
                cmd_exec();
                if edit_prev(number as std::ffi::c_int) != 0 {
                    parg.p_string = if number > 1 as std::ffi::c_int as LINENUM {
                        b"(N-th) \0" as *const u8 as *const std::ffi::c_char
                    } else {
                        b"\0" as *const u8 as *const std::ffi::c_char
                    };
                    error(
                        b"No %sprevious file\0" as *const u8 as *const std::ffi::c_char,
                        &mut parg,
                    );
                }
                continue;
            }
            5431927413890720344 => {
                let opts = get_options();
                hshift = save_hshift;
                opts.bs_mode = save_bs_mode;
                opts.proc_backspace = save_proc_backspace;
                if edit_prev(1 as std::ffi::c_int) == 0 as std::ffi::c_int {
                    continue;
                }
            }
            12373568287479140350 => {
                cmd_exec();
                repaint();
                continue;
            }
            5798072534372498777 => {
                let opts = get_options();
                cmd_exec();
                if edit(tagfile) == 0 as std::ffi::c_int {
                    let mut pos: POSITION = tagsearch();
                    if pos != -(1 as std::ffi::c_int) as POSITION {
                        jump_loc(pos, opts.jump_sline);
                    }
                }
                continue;
            }
            13267105165099174640 => {
                let opts = get_options();
                cmd_exec();
                if edit(tagfile) == 0 as std::ffi::c_int {
                    let mut pos_0: POSITION = tagsearch();
                    if pos_0 != -(1 as std::ffi::c_int) as POSITION {
                        jump_loc(pos_0, opts.jump_sline);
                    }
                }
                continue;
            }
            2089914658669629659 => {
                error(
                    b"Command not available\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                continue;
            }
            18001984906674336099 => {
                error(
                    b"No next file\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                continue;
            }
            8193737063574930042 => {
                error(
                    b"No previous file\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                continue;
            }
            8075351136037156718 => {
                error(
                    b"No next tag\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                continue;
            }
            18330534242458572360 => {
                bell();
                continue;
            }
            7991679940794782184 => {
                if !ungot.is_empty() {
                    getcc(ungot);
                }
                continue;
            }
            10029375464402185584 => {
                error(
                    b"No previous tag\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                continue;
            }
            8120009455218959897 => {
                error(
                    b"Command not available\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                continue;
            }
            _ => {}
        }
        if !extra.is_none() {
            quit(extra.unwrap().0 as i32);
        }
        quit(0 as std::ffi::c_int);
    }
}
