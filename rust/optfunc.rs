use crate::charset::{setfmt, step_charc};
use crate::decode::{get_tables_mut, lesskey, lesskey_content, lesskey_src};
use crate::defs::*;
use crate::edit::use_logfile;
use crate::filename::shell_unquote;
use crate::ifile::IFileHandle;
use crate::line::{set_color_map, NUM_SEARCH_COLORS};
use crate::main::secure_allow;
use crate::option::{getfraction, getnumc};
use std::ffi::{CStr, CString};

extern "C" {
    fn sprintf(_: *mut std::ffi::c_char, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn snprintf(
        _: *mut std::ffi::c_char,
        _: std::ffi::c_ulong,
        _: *const std::ffi::c_char,
        _: ...
    ) -> std::ffi::c_int;
    fn strcpy(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strcat(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn save(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn skipspc(s: *const std::ffi::c_char) -> *const std::ffi::c_char;
    fn quit(status: std::ffi::c_int);
    fn init_mouse();
    fn deinit_mouse();
    fn init_bracketed_paste();
    fn deinit_bracketed_paste();
    fn sync_logfile();
    fn ch_length() -> POSITION;
    fn ch_setbufspace(bufspace_0: ssize_t);
    fn ch_getflags() -> std::ffi::c_int;
    fn prchar(c: LWCHAR) -> *const std::ffi::c_char;
    fn norm_search_type(st: std::ffi::c_int) -> std::ffi::c_int;
    fn dispversion();
    fn ungetcc_end_command();
    fn ungetsc(s: *const std::ffi::c_char);
    fn save_curr_ifile() -> *mut std::ffi::c_void;
    fn unsave_ifile(save_ifile: *mut std::ffi::c_void);
    fn reedit_ifile(save_ifile: *mut std::ffi::c_void);
    fn lglob(afilename: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn jump_loc(pos: POSITION, sline: std::ffi::c_int);
    fn pwidth(
        ch: LWCHAR,
        a: std::ffi::c_int,
        prev_ch: LWCHAR,
        prev_a: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn find_linenum(pos: POSITION) -> LINENUM;
    fn find_pos(linenum: LINENUM) -> POSITION;
    fn scan_eof();
    fn umuldiv(val: uintmax, num: uintmax, den: uintmax) -> uintmax;
    fn set_output(fd: std::ffi::c_int);
    fn putstr(s: *const std::ffi::c_char);
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn pattern_lib_name() -> *const std::ffi::c_char;
    fn position(sindex: std::ffi::c_int) -> POSITION;
    fn pos_rehead();
    fn set_header(pos: POSITION);
    fn chg_caseless();
    fn findtag(tag: *const std::ffi::c_char);
    fn tagsearch() -> POSITION;
    fn edit_tagfile() -> std::ffi::c_int;
    fn default_wheel_lines() -> std::ffi::c_int;
    static mut bufspace: std::ffi::c_int;
    static mut pr_type: i32;
    static mut plusoption: bool;
    static mut swindow: std::ffi::c_int;
    static mut sc_width: std::ffi::c_int;
    static mut sc_height: std::ffi::c_int;
    static mut dohelp: std::ffi::c_int;
    static mut openquote: char;
    static mut closequote: char;
    static mut prproto: [String; 3];
    static mut eqproto: String;
    static mut hproto: String;
    static mut wproto: String;
    static mut every_first_cmd: String;
    static mut curr_ifile: Option<IFileHandle>;
    static mut version: [std::ffi::c_char; 0];
    static mut jump_sline: std::ffi::c_int;
    static mut jump_sline_fraction: std::ffi::c_long;
    static mut shift_count: std::ffi::c_int;
    static mut shift_count_fraction: std::ffi::c_long;
    static mut match_shift: std::ffi::c_int;
    static mut match_shift_fraction: std::ffi::c_long;
    static mut rscroll_char: char;
    static mut rscroll_attr: std::ffi::c_int;
    static mut mousecap: std::ffi::c_int;
    static mut wheel_lines: std::ffi::c_int;
    static mut less_is_more: std::ffi::c_int;
    static mut linenum_width: std::ffi::c_int;
    static mut status_col_width: std::ffi::c_int;
    static mut use_color: std::ffi::c_int;
    static mut want_filesize: std::ffi::c_int;
    static mut header_lines: std::ffi::c_int;
    static mut header_cols: std::ffi::c_int;
    static mut def_search_type: std::ffi::c_int;
    static mut chopline: std::ffi::c_int;
    static mut tabstops: [i32; 0];
    static mut ntabstops: std::ffi::c_int;
    static mut tabdefault: std::ffi::c_int;
    static mut no_paste: std::ffi::c_int;
    static mut intr_char: char;
    static mut nosearch_header_lines: std::ffi::c_int;
    static mut nosearch_header_cols: std::ffi::c_int;
    static mut header_start_pos: POSITION;
    static mut init_header: String;
    static mut namelogfile: Option<String>;
    static mut force_logfile: lbool;
    static mut logfile: std::ffi::c_int;
    static mut tags: String;
    static mut ztags: String;
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
#[no_mangle]
pub static mut tagoption: String = String::new();

/*
 * Handler for -o option.
 */
#[no_mangle]
pub unsafe fn opt_o(ty: i32, s: &str) {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    let mut s = s;
    let mut filename: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    if !secure_allow(SF_LOGFILE) {
        error(
            b"log file support is not available\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void as *mut PARG,
        );
        return;
    }
    match ty {
        INIT => {
            namelogfile = Some(s.to_owned());
        }
        TOGGLE => {
            if ch_getflags() & CH_CANSEEK != 0 {
                error(
                    b"Input is not a pipe\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                return;
            }
            if logfile >= 0 {
                error(
                    b"Log file is already in use\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                return;
            }
            s = s.trim_start();
            filename = lglob(CString::new(s).unwrap().as_ptr());
            let filename = CStr::from_ptr(filename);
            let filename_str = filename.to_string_lossy().into_owned();
            namelogfile = Some(shell_unquote(&filename_str));
            use_logfile(&namelogfile.clone().unwrap());
            sync_logfile();
        }
        QUERY => {
            if logfile < 0 {
                error(
                    b"No log file\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
            } else {
                parg.p_string = CString::new(namelogfile.clone().unwrap()).unwrap().as_ptr();
                error(
                    b"Log file \"%s\"\0" as *const u8 as *const std::ffi::c_char,
                    &mut parg,
                );
            }
        }
        _ => {}
    };
}

#[no_mangle]
pub unsafe fn opt__O(ty: i32, s: &str) {
    force_logfile = LTRUE;
    opt_o(ty, s);
}

/*
 * Handler for -O option.
 */
unsafe extern "C" fn toggle_fraction(
    s: &str,
    printopt: Option<&str>,
    calc: Option<unsafe fn() -> ()>,
) -> Option<(i32, i64)> {
    let mut err = Some(&mut false);
    let mut num = 0;
    let mut frac = 0;
    if s == "" {
        if let Some(func) = calc {
            func();
        }
    } else if s == "." {
        let mut tfrac = 0;
        let (tfr, s) = getfraction(&s[1..], printopt, err);
        if let Some(tfrac) = tfr {
            frac = tfrac;
            if let Some(func) = calc {
                func();
            }
        } else {
            error(
                b"Invalid fraction\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
            return None;
        }
    } else {
        let (mut tnum, _) = getnumc(&s, printopt);
        if let Some(n) = tnum {
            frac = -1;
            num = n as i32;
        } else {
            error(
                b"Invalid number\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
            return None;
        }
    }
    Some((num, frac))
}

unsafe extern "C" fn query_fraction(value: i32, fraction: i64, int_msg: &str, frac_msg: &str) {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    if fraction < 0 {
        parg.p_int = value;
        error(CString::new(int_msg).unwrap().as_ptr(), &mut parg);
    } else {
        let mut buf = format!(".{:06}", fraction);
        let mut len = buf.len();
        while len > 2 && buf.chars().last() == Some('0') {
            buf = (&buf[..len - 1]).to_string();
            len = buf.len();
        }
        parg.p_string = CString::new(buf).unwrap().as_ptr();
        error(CString::new(frac_msg).unwrap().as_ptr(), &mut parg);
    };
}

/*
 * Handlers for -j option.
 */
#[no_mangle]
pub unsafe fn opt_j(mut ty: i32, s: &str) {
    match ty {
        INIT | TOGGLE => {
            let res = toggle_fraction(s, Some("j"), Some(calc_jump_sline));
            if let Some((js, jsf)) = res {
                jump_sline = js;
                jump_sline_fraction = jsf;
            }
        }
        QUERY => {
            query_fraction(
                jump_sline,
                jump_sline_fraction,
                "Position target at screen line %d",
                "Position target at screen position %s",
            );
        }
        _ => {}
    };
}

#[no_mangle]
pub unsafe fn calc_jump_sline() {
    if jump_sline_fraction >= 0 {
        jump_sline = umuldiv(
            sc_height as uintmax,
            jump_sline_fraction as uintmax,
            1000000 as std::ffi::c_int as uintmax,
        ) as std::ffi::c_int;
    }
    if jump_sline <= header_lines {
        jump_sline = header_lines + 1;
    }
}

/*
 * Handlers for -# option.
 */
#[no_mangle]
pub unsafe fn opt_shift(ty: i32, s: &str) {
    match ty {
        INIT | TOGGLE => {
            let res = toggle_fraction(s, Some("#"), Some(calc_shift_count));
            if let Some((sc, scf)) = res {
                shift_count = sc;
                shift_count_fraction = scf;
            }
        }
        QUERY => {
            query_fraction(
                shift_count,
                shift_count_fraction,
                "Horizontal shift %d columns",
                "Horizontal shift %s of screen width",
            );
        }
        _ => {}
    };
}

#[no_mangle]
pub unsafe fn calc_shift_count() {
    if shift_count_fraction < 0 {
        return;
    }
    shift_count = umuldiv(
        sc_width as uintmax,
        shift_count_fraction as uintmax,
        1000000 as std::ffi::c_int as uintmax,
    ) as std::ffi::c_int;
}

#[no_mangle]
pub unsafe fn opt_k(ty: i32, s: &str) {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    match ty {
        INIT => {
            if let Some(tables) = get_tables_mut() {
                if lesskey(tables, s.as_bytes(), false) != 0 {
                    parg.p_string = CString::new(s).unwrap().as_ptr();
                    error(
                        b"Cannot use lesskey file \"%s\"\0" as *const u8 as *const std::ffi::c_char,
                        &mut parg,
                    );
                }
            }
        }
        _ => {}
    };
}

#[no_mangle]
pub unsafe fn opt_ks(ty: i32, s: &str) {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    match ty {
        INIT => {
            if let Some(tables) = get_tables_mut() {
                if lesskey_src(tables, s.as_bytes(), false) != 0 {
                    parg.p_string = CString::new(s).unwrap().as_ptr();
                    error(
                        b"Cannot use lesskey source file \"%s\"\0" as *const u8
                            as *const std::ffi::c_char,
                        &mut parg,
                    );
                }
            }
        }
        _ => {}
    };
}

#[no_mangle]
pub unsafe fn opt_kc(mut type_0: std::ffi::c_int, mut s: *const std::ffi::c_char) {
    match type_0 {
        0 => {
            if let Some(tables) = get_tables_mut() {
                let content = if s.is_null() {
                    b""
                } else {
                    CStr::from_ptr(s).to_bytes()
                };
                if lesskey_content(tables, content, LFALSE != 0) != 0 {
                    error(
                        b"Error in lesskey content\0" as *const u8 as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void as *mut PARG,
                    );
                }
            }
        }
        _ => {}
    };
}

/*
 * Handler for -S option.
 */
#[no_mangle]
pub unsafe fn opt__S(ty: i32, s: &str) {
    match ty {
        TOGGLE => {
            pos_rehead();
        }
        _ => {}
    };
}

/*
 * Handler for -t option.
 */
#[no_mangle]
pub unsafe fn opt_t(ty: i32, s: &str) {
    let mut save_ifile: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
    let mut pos: POSITION = 0;
    match ty {
        INIT => {
            tagoption = String::from(s);
        }
        TOGGLE => {
            if !secure_allow(SF_TAGS) {
                error(
                    b"tags support is not available\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
            } else {
                findtag(CString::new(s.trim_start()).unwrap().as_ptr());
                save_ifile = save_curr_ifile();
                /*
                 * Try to open the file containing the tag
                 * and search for the tag in that file.
                 */
                if edit_tagfile() != 0 || {
                    pos = tagsearch();
                    pos == NULL_POSITION
                } {
                    /* Failed: reopen the old file. */
                    reedit_ifile(save_ifile);
                } else {
                    unsave_ifile(save_ifile);
                    jump_loc(pos, jump_sline);
                }
            }
        }
        _ => {}
    };
}

/*
 * Handler for -T option.
 */
#[no_mangle]
pub unsafe fn opt__T(ty: i32, s: &str) {
    let mut s = s;
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    let mut filename: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    match ty {
        INIT => {
            tags = String::from(s);
        }
        TOGGLE => {
            s = s.trim_start();
            filename = lglob(CString::new(s).unwrap().as_ptr());
            let filename = CStr::from_ptr(filename);
            let filename_str = filename.to_string_lossy().into_owned();
            tags = shell_unquote(&filename_str);
        }
        QUERY => {
            parg.p_string = CString::new(tags.clone()).unwrap().as_ptr();
            error(
                b"Tags file \"%s\"\0" as *const u8 as *const std::ffi::c_char,
                &mut parg,
            );
        }
        _ => {}
    };
}

/*
 * Handler for -p option.
 */
#[no_mangle]
pub unsafe fn opt_p(ty: i32, s: &str) {
    match ty {
        INIT => {
            /*
             * Unget a command for the specified string.
             */
            if less_is_more != 0 {
                /*
                 * In "more" mode, the -p argument is a command,
                 * not a search string, so we don't need a slash.
                 */
                every_first_cmd = String::from(s);
            } else {
                plusoption = true;
                /*
                 * {{ This won't work if the "/" command is
                 *    changed or invalidated by a .lesskey file. }}
                 */
                ungetsc(b"/\0" as *const u8 as *const std::ffi::c_char);
                ungetsc(CString::new(s).unwrap().as_ptr());
                ungetcc_end_command();
            }
        }
        _ => {}
    };
}

/*
 * Handler for -P option.
 */
#[no_mangle]
pub unsafe fn opt__P(ty: i32, s: &str) {
    let mut proto: &str;
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    let mut chars = s.chars();
    match ty {
        INIT | TOGGLE => {
            // Figure out which prototype string should be changed.
            match chars.next() {
                Some('s') => {
                    proto = &prproto[PR_SHORT];
                    chars.next();
                }
                Some('m') => {
                    proto = &prproto[PR_MEDIUM];
                    chars.next();
                }
                Some('M') => {
                    proto = &prproto[PR_LONG];
                    chars.next();
                }
                Some('=') => {
                    proto = &eqproto;
                    chars.next();
                }
                Some('h') => {
                    proto = &hproto;
                    chars.next();
                }
                Some('w') => {
                    proto = &wproto;
                    chars.next();
                }
                _ => {
                    proto = &prproto[PR_SHORT];
                }
            }
        }
        QUERY => {
            parg.p_string = CString::new(prproto[pr_type as usize].clone())
                .unwrap()
                .as_ptr();
            error(b"%s\0" as *const u8 as *const std::ffi::c_char, &mut parg);
        }
        _ => {}
    };
}

/*
 * Handler for the -b option.
 */
#[no_mangle]
pub unsafe fn opt_b(ty: i32, s: &str) {
    match ty {
        INIT | TOGGLE => {
            /*
             * Set the new number of buffers.
             */
            ch_setbufspace(bufspace as ssize_t);
        }
        QUERY | _ => {}
    };
}

/*
 * Handler for the -i option.
 */
#[no_mangle]
pub unsafe fn opt_i(ty: i32, s: &str) {
    match ty {
        TOGGLE => {
            chg_caseless();
        }
        QUERY | INIT | _ => {}
    };
}

/*
 * Handler for the -V option.
 */
#[no_mangle]
pub unsafe fn opt__V(ty: i32, s: &str) {
    match ty {
        TOGGLE | QUERY => {
            dispversion();
        }
        INIT => {
            set_output(1); /* Force output to stdout per GNU standard for --version output. */
            putstr(b"less \0" as *const u8 as *const std::ffi::c_char);
            putstr(version.as_mut_ptr());
            putstr(b" (\0" as *const u8 as *const std::ffi::c_char);
            putstr(pattern_lib_name());
            putstr(b" regular expressions)\n\0" as *const u8 as *const std::ffi::c_char);
            let mut copyright: *const std::ffi::c_char =
                b"Copyright (C) 1984-2025  Mark Nudelman\n\n\0" as *const u8
                    as *const std::ffi::c_char;
            putstr(copyright);
            if *version.as_mut_ptr().offset(
                (strlen(version.as_mut_ptr()))
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                    as isize,
            ) as std::ffi::c_int
                == 'x' as i32
            {
                putstr(
                    b"** This is an EXPERIMENTAL build of the 'less' software,\n\0" as *const u8
                        as *const std::ffi::c_char,
                );
                putstr(
                    b"** and may not function correctly.\n\0" as *const u8
                        as *const std::ffi::c_char,
                );
                putstr(
                    b"** Obtain release builds from the web page below.\n\n\0" as *const u8
                        as *const std::ffi::c_char,
                );
            }
            putstr(
                b"less comes with NO WARRANTY, to the extent permitted by law.\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            putstr(
                b"For information about the terms of redistribution,\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            putstr(
                b"see the file named README in the less distribution.\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            putstr(
                b"Home page: https://greenwoodsoftware.com/less\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            quit(0 as std::ffi::c_int);
        }
        _ => {}
    };
}

unsafe extern "C" fn color_from_namechar(namechar: char) -> i32 {
    match namechar {
        'B' => return AT_COLOR_BIN,
        'C' => return AT_COLOR_CTRL,
        'E' => return AT_COLOR_ERROR,
        'H' => return AT_COLOR_HEADER,
        'M' => return AT_COLOR_MARK,
        'N' => return AT_COLOR_LINENUM,
        'P' => return AT_COLOR_PROMPT,
        'R' => return AT_COLOR_RSCROLL,
        'S' => return AT_COLOR_SEARCH,
        'W' | 'A' => return AT_COLOR_ATTN,
        'n' => return AT_NORMAL,
        's' => return AT_STANDOUT,
        'd' => return AT_BOLD,
        'u' => return AT_UNDERLINE,
        'k' => return AT_BLINK,
        _ => {
            if namechar >= '1' && namechar as i32 <= b'0' as i32 + NUM_SEARCH_COLORS {
                return AT_COLOR_SUBSEARCH(namechar as i32 - b'0' as i32);
            }
            return -1;
        }
    }
}

/*
 * Handler for the -D option.
 */
#[no_mangle]
pub unsafe fn opt_D(ty: i32, s: &str) {
    let mut p: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    let mut attr = 0;
    let mut chars = s.chars();
    match ty {
        INIT | TOGGLE => {
            let ch = chars.nth(0).unwrap();
            attr = color_from_namechar(ch);
            if attr < 0 {
                p.p_char = ch as i8;
                error(
                    b"Invalid color specifier '%c'\0" as *const u8 as *const std::ffi::c_char,
                    &mut p,
                );
                return;
            }
            if use_color == 0 && attr & AT_COLOR != 0 {
                error(
                    b"Set --use-color before changing colors\0" as *const u8
                        as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                return;
            }
            chars.next();
            // FIXME set color map takes a &'static str
            /*
            if set_color_map(attr, s) < 0 {
                p.p_string = CString::new(s).unwrap().as_ptr();
                error(
                    b"Invalid color string \"%s\"\0" as *const u8 as *const std::ffi::c_char,
                    &mut p,
                );
                return;
            }
            */
        }
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_tabs(s: &str, len: usize) {
    /* Start at 1 because tabstops[0] is always zero. */
    let mut s = s;
    let i = 0;
    for ref mut i in 1..TABSTOP_MAX {
        let mut n: i32 = 0;
        let mut v = false;
        s = s.trim_matches(|c| c == ' ');
        let mut chars = s.chars();
        let mut ch = ' ';
        while let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                if let Some(val) = n.checked_mul(10) {
                    n = val;
                } else {
                    v = true;
                }
                if let Some(val) = n.checked_add((c as u8 - b'0') as i32) {
                    n = val;
                } else {
                    v = true;
                }
            } else {
                ch = c;
                break;
            }
        }
        if !v && n > tabstops[*i as usize - 1] {
            tabstops[*i as usize] = n;
            *i += 1;
        }
        s = s.trim_matches(|c| c == ' ');
        if ch != ',' {
            break;
        }
    }
    if i < 2 {
        return;
    }
    ntabstops = i;
    tabdefault = tabstops[(ntabstops - 1) as usize] - tabstops[(ntabstops - 2) as usize];
}

/*
 * Handler for the -x option.
 */
#[no_mangle]
pub unsafe fn opt_x(mut ty: i32, s: &str) {
    let mut msg = String::new();
    let mut p: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    match ty {
        INIT | TOGGLE => {
            set_tabs(s, s.len());
        }
        QUERY => {
            msg.push_str("Tab stops ");
            if ntabstops > 2 {
                for i in 1..ntabstops {
                    if i > 1 {
                        msg.push(',');
                        msg.push_str(&format!("{}", tabstops[i as usize]));
                    }
                }
                msg.push_str(" and then ");
            }
            msg.push_str(&format!("every {} spaces", tabdefault));
            p.p_string = CString::new(msg).unwrap().as_ptr();
            error(b"%s\0" as *const u8 as *const std::ffi::c_char, &mut p);
        }
        _ => {}
    };
}

/*
 * Handler for the -" option.
 */
#[no_mangle]
pub unsafe fn opt_quote(mut ty: i32, s: &str) {
    let mut buf: [std::ffi::c_char; 3] = [0; 3];
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    match ty {
        INIT | TOGGLE => {
            if s.len() == 0 {
                openquote = '\0';
                closequote = openquote;
            } else {
                let mut chars = s.chars();
                if chars.nth(0) != Some('\0') && chars.nth(2) != Some('\0') {
                    error(
                        b"-\" must be followed by 1 or 2 chars\0" as *const u8
                            as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void as *mut PARG,
                    );
                    return;
                }
                openquote = chars.nth(0).unwrap();
                if chars.nth(1) == Some('\0') {
                    closequote = openquote;
                } else {
                    closequote = chars.nth(1).unwrap();
                }
            }
        }
        QUERY => {
            buf[0 as usize] = openquote as i8;
            buf[1 as usize] = closequote as i8;
            buf[2 as usize] = '\0' as i32 as std::ffi::c_char;
            parg.p_string = buf.as_mut_ptr();
            error(
                b"quotes %s\0" as *const u8 as *const std::ffi::c_char,
                &mut parg,
            );
        }
        _ => {}
    };
}

/*
 * Handler for the --rscroll option.
 */
#[no_mangle]
pub unsafe fn opt_rscroll(mut ty: i32, s: &str) {
    let mut p: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    match ty {
        INIT | TOGGLE => {
            let (fmt, attr) = setfmt(Some(s.to_owned()), "*s>", false);

            if fmt == "-" {
                rscroll_char = 0 as char;
            } else {
                rscroll_attr = attr | AT_COLOR_RSCROLL;
                if fmt.len() == 0 {
                    rscroll_char = '>';
                } else {
                    let (mut ch, _) = step_charc(&fmt.as_bytes(), 1, 0, fmt.len());
                    if pwidth(ch as i32, rscroll_attr, 0, 0) > 1 {
                        error(
                            b"cannot set rscroll to a wide character\0" as *const u8
                                as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void as *mut PARG,
                        );
                    } else {
                        rscroll_char = ch;
                    }
                }
            }
        }
        QUERY => {
            p.p_string = if rscroll_char != 0 as char {
                prchar(rscroll_char as i32)
            } else {
                b"-\0" as *const u8 as *const std::ffi::c_char
            };
            error(
                b"rscroll character is %s\0" as *const u8 as *const std::ffi::c_char,
                &mut p,
            );
        }
        _ => {}
    };
}

/*
 * "-?" means display a help message.
 * If from the command line, exit immediately.
 */
#[no_mangle]
pub unsafe fn opt_query(mut ty: i32, s: &str) {
    match ty {
        INIT | TOGGLE => {
            error(
                b"Use \"h\" for help\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG,
            );
        }
        QUERY => {
            dohelp = 1;
        }
        _ => {}
    };
}

#[no_mangle]
pub unsafe fn opt_match_shift(ty: i32, s: &str) {
    match ty {
        INIT | TOGGLE => {
            let res = toggle_fraction(s, Some("--match-shift"), Some(calc_match_shift));
            if let Some((ms, msf)) = res {
                match_shift = ms;
                match_shift_fraction = msf;
            }
        }
        QUERY => {
            query_fraction(
                match_shift,
                match_shift_fraction,
                "Search match shift is %d",
                "Search match shift is %s of screen width",
            );
        }
        _ => {}
    };
}

#[no_mangle]
pub unsafe fn calc_match_shift() {
    if match_shift_fraction < 0 as std::ffi::c_int as std::ffi::c_long {
        return;
    }
    match_shift = umuldiv(
        sc_width as uintmax,
        match_shift_fraction as uintmax,
        1000000 as std::ffi::c_int as uintmax,
    ) as std::ffi::c_int;
}

/*
 * Handler for the --mouse option.
 */
#[no_mangle]
pub unsafe fn opt_mousecap(ty: i32, s: &str) {
    match ty {
        TOGGLE => {
            if mousecap == 0 {
                deinit_mouse();
            } else {
                init_mouse();
            }
        }
        INIT | QUERY | _ => {}
    };
}

/*
 * Handler for the --wheel-lines option.
 */
#[no_mangle]
pub unsafe fn opt_wheel_lines(ty: i32, s: &str) {
    match ty {
        INIT | TOGGLE => {
            if wheel_lines <= 0 {
                wheel_lines = default_wheel_lines();
            }
        }
        QUERY | _ => {}
    };
}

/*
 * Handler for the --line-number-width option.
 */
#[no_mangle]
pub unsafe fn opt_linenum_width(ty: i32, s: &str) {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    match ty {
        INIT | TOGGLE => {
            if linenum_width > MAX_LINENUM_WIDTH {
                parg.p_int = 16;
                error(
                    b"Line number width must not be larger than %d\0" as *const u8
                        as *const std::ffi::c_char,
                    &mut parg,
                );
                linenum_width = MIN_LINENUM_WIDTH;
            }
        }
        QUERY | _ => {}
    };
}

/*
 * Handler for the --status-column-width option.
 */
#[no_mangle]
pub unsafe fn opt_status_col_width(ty: i32, s: &str) {
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    match ty {
        INIT | TOGGLE => {
            if status_col_width > MAX_STATUSCOL_WIDTH {
                parg.p_int = MAX_STATUSCOL_WIDTH;
                error(
                    b"Status column width must not be larger than %d\0" as *const u8
                        as *const std::ffi::c_char,
                    &mut parg,
                );
                status_col_width = 2;
            }
        }
        QUERY | _ => {}
    };
}

/*
 * Handler for the --file-size option.
 */
#[no_mangle]
pub unsafe fn opt_filesize(mut ty: i32, s: &str) {
    match ty {
        INIT | TOGGLE => {
            if want_filesize != 0 && !curr_ifile.is_none() && ch_length() == NULL_POSITION {
                scan_eof();
            }
        }
        QUERY | _ => {}
    };
}

/*
 * Handler for the --intr option.
 */
#[no_mangle]
pub unsafe fn opt_intr(mut ty: i32, s: &str) {
    let mut p: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    match ty {
        INIT | TOGGLE => {
            if s.starts_with("^") && s.len() > 1 {
                intr_char = (s.chars().nth(1).unwrap() as u8 & 0o37) as char;
            }
        }
        QUERY => {
            p.p_string = prchar(intr_char as LWCHAR);
            error(
                b"interrupt character is %s\0" as *const u8 as *const std::ffi::c_char,
                &mut p,
            );
        }
        _ => {}
    };
}

/*
 * Return the next number from a comma-separated list.
 * Return -1 if the list entry is missing or empty.
 * Returns a reference pointing to the next number in the list
 */
#[no_mangle]
pub unsafe extern "C" fn next_cnum<'a>(
    s: &'a str,
    printopt: Option<&'a str>,
    errmsg: &'a str,
) -> (Option<i64>, &'a str) {
    let mut n = 0;
    let err = false;
    if s.len() == 0 {
        return (None, s);
    }
    if s.starts_with(",") {
        return (None, &s[1..]);
    }
    let (n, rest) = getnumc(s, printopt);
    if n.is_none() {
        let mut parg: PARG = parg {
            p_string: 0 as *const std::ffi::c_char,
        };
        parg.p_string = CString::new(errmsg).unwrap().as_ptr();
        error(
            b"invalid %s\0" as *const u8 as *const std::ffi::c_char,
            &mut parg,
        );
        return (None, s);
    }
    if rest.starts_with(",") {
        return (n, &rest[1..]);
    }
    (n, rest)
}

/*
 * Parse a parameter to the --header option.
 * Value is "L,C,N", where each field is a decimal number or empty.
 */
unsafe extern "C" fn parse_header(s: &str) -> Option<(i64, i64, i64)> {
    let mut lines = 0;
    let mut cols = 0;
    let mut start_pos = 0;

    let s = if s.starts_with("-") { "0,0" } else { s };
    let (n, rest) = next_cnum(s, Some("header"), "number of lines");
    if let Some(n) = n {
        if n >= 0 {
            lines = n;
        }
    } else {
        return None;
    }
    let (n, rest) = next_cnum(&rest, Some("header"), "number of columns");
    if let Some(n) = n {
        if n >= 0 {
            cols = n;
        }
    } else {
        return None;
    }
    let (n, rest) = next_cnum(&rest, Some("header"), "line number");
    if let Some(n) = n {
        if n > 0 {
            start_pos = find_pos(n);
        }
    } else {
        return None;
    }
    Some((lines, cols, start_pos))
}

/*
 * Handler for the --header option.
 */
#[no_mangle]
pub unsafe fn opt_header(mut ty: i32, s: &str) {
    match ty {
        INIT => {
            /* Can't call parse_header now because input file is not yet opened,
             * so find_pos won't work. */
            init_header = String::from(s);
        }
        TOGGLE => {
            let mut lines = header_lines;
            let mut cols = header_cols;
            let mut start_pos = if ty == 0 { 0 } else { position(TOP) };
            if start_pos == NULL_POSITION {
                start_pos = 0;
            }
            if parse_header(s).is_none() {
                header_lines = lines;
                header_cols = cols;
                set_header(start_pos);
                calc_jump_sline();
            }
        }
        QUERY => {
            let mut buf: [std::ffi::c_char; 66] = [0; 66];
            let mut parg: PARG = parg {
                p_string: 0 as *const std::ffi::c_char,
            };
            snprintf(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[std::ffi::c_char; 66]>() as std::ffi::c_ulong,
                b"%ld,%ld,%ld\0" as *const u8 as *const std::ffi::c_char,
                header_lines as std::ffi::c_long,
                header_cols as std::ffi::c_long,
                find_linenum(header_start_pos),
            );
            parg.p_string = buf.as_mut_ptr();
            error(
                b"Header (lines,columns,line-number) is %s\0" as *const u8
                    as *const std::ffi::c_char,
                &mut parg,
            );
        }
        _ => {}
    };
}

fn srch_subsearch(i: i32) -> i32 {
    1 << (17 + i) /* Search for subpattern */
}

/*
 * Handler for the --search-options option.
 */
#[no_mangle]
pub unsafe fn opt_search_type(ty: i32, s: &str) {
    let mut st = 0;
    let mut parg: PARG = parg {
        p_string: 0 as *const std::ffi::c_char,
    };
    let mut chars = s.chars();
    let mut buf = String::new();
    let mut bp: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut i = 0;
    match ty {
        INIT | TOGGLE => {
            st = 0;
            while let Some(c) = chars.next() {
                if c == 'E' || c == 'e' || c == CONTROL('E') {
                    st |= SRCH_PAST_EOF;
                } else if c == 'F' || c == 'f' || c == CONTROL('F') {
                    st |= SRCH_FIRST_FILE;
                } else if c == 'K' || c == 'k' || c == CONTROL('K') {
                    st |= SRCH_NO_MOVE;
                } else if c == 'N' || c == 'n' || c == CONTROL('N') {
                    st |= SRCH_NO_MATCH;
                } else if c == 'R' || c == 'r' || c == CONTROL('R') {
                    st |= SRCH_NO_REGEX;
                } else if c == 'W' || c == 'w' || c == CONTROL('W') {
                    st |= SRCH_WRAP;
                } else if c == '-' {
                    st = 0;
                } else if c == '^' {
                    // do nothing
                } else {
                    let ch = c as u8;
                    if ch >= b'1' && ch <= b'0' + NUM_SEARCH_COLORS as u8 {
                        st |= srch_subsearch((ch - b'0').into());
                    } else {
                        parg.p_char = ch as i8;
                        error(
                            b"invalid search option '%c'\0" as *const u8 as *const std::ffi::c_char,
                            &mut parg,
                        );
                        return;
                    }
                }
            }
            def_search_type = norm_search_type(st);
        }
        QUERY => {
            if def_search_type & SRCH_PAST_EOF != 0 {
                buf.push('E');
            }
            if def_search_type & SRCH_FIRST_FILE != 0 {
                buf.push('F');
            }
            if def_search_type & SRCH_NO_MOVE != 0 {
                buf.push('K');
            }
            if def_search_type & SRCH_NO_MATCH != 0 {
                buf.push('N');
            }
            if def_search_type & SRCH_NO_REGEX != 0 {
                buf.push('R');
            }
            if def_search_type & SRCH_WRAP != 0 {
                buf.push('W');
            }
            for i in 1..=NUM_SEARCH_COLORS {
                if def_search_type & srch_subsearch(i) != 0 {
                    buf.push(('0' as u8 + i as u8) as char);
                }
            }
            if buf.len() == 0 {
                buf.push('-');
            }
            parg.p_string = CString::new(buf).unwrap().as_ptr();
            error(
                b"search options: %s\0" as *const u8 as *const std::ffi::c_char,
                &mut parg,
            );
        }
        _ => {}
    };
}

unsafe extern "C" fn do_nosearch_headers(
    mut type_0: std::ffi::c_int,
    mut no_header_lines: std::ffi::c_int,
    mut no_header_cols: std::ffi::c_int,
) {
    let mut current_block_8: u64;
    match type_0 {
        0 | 2 => {
            nosearch_header_lines = no_header_lines;
            nosearch_header_cols = no_header_cols;
            if type_0 != 2 as std::ffi::c_int {
                current_block_8 = 13109137661213826276;
            } else {
                current_block_8 = 4311149068773253642;
            }
        }
        1 => {
            current_block_8 = 4311149068773253642;
        }
        _ => {
            current_block_8 = 13109137661213826276;
        }
    }
    match current_block_8 {
        4311149068773253642 => {
            if nosearch_header_lines != 0 && nosearch_header_cols != 0 {
                error(
                    b"Search does not include header lines or columns\0" as *const u8
                        as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
            } else if nosearch_header_lines != 0 {
                error(
                    b"Search includes header columns but not header lines\0" as *const u8
                        as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
            } else if nosearch_header_cols != 0 {
                error(
                    b"Search includes header lines but not header columns\0" as *const u8
                        as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
            } else {
                error(
                    b"Search includes header lines and columns\0" as *const u8
                        as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
            }
        }
        _ => {}
    };
}

#[no_mangle]
pub unsafe extern "C" fn opt_nosearch_headers(ty: i32, s: &str) {
    do_nosearch_headers(ty, 1, 1);
}

#[no_mangle]
pub unsafe extern "C" fn opt_nosearch_header_lines(ty: i32, s: &str) {
    do_nosearch_headers(ty, 1, 0);
}

#[no_mangle]
pub unsafe extern "C" fn opt_nosearch_header_cols(ty: i32, s: &str) {
    do_nosearch_headers(ty, 0, 1);
}

#[no_mangle]
pub unsafe fn opt_no_paste(ty: i32, s: &str) {
    match ty {
        TOGGLE => {
            if no_paste != 0 {
                init_bracketed_paste();
            } else {
                deinit_bracketed_paste();
            }
        }
        INIT | QUERY | _ => {}
    };
}

#[no_mangle]
pub unsafe extern "C" fn chop_line() -> bool {
    chopline != 0 || header_cols > 0 || header_lines > 0
}

/*
 * Get the "screen window" size.
 */
#[no_mangle]
pub unsafe extern "C" fn get_swindow() -> i32 {
    if swindow > 0 {
        return swindow;
    }
    return sc_height - header_lines + swindow;
}
