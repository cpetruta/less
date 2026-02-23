/*
 * Copyright (C) 1984-2025  Mark Nudelman
 *
 * You may distribute under the terms of either the GNU General Public
 * License or the Less License, as specified in the README file.
 *
 * For more information, see the README file.
 */

/*
 * Functions which manipulate the command buffer.
 * Used only by command() and related functions.
 */

use crate::decode::lgetenv;
use crate::defs::*;
use crate::opttbl::get_options;
use ::c2rust_bitfields;
use std::ffi::{c_char, c_void, CString};
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn rename(__old: *const c_char, __new: *const c_char) -> i32;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const c_char, _: *const c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const c_char, _: ...) -> i32;
    fn fgets(
        __s: *mut c_char,
        __n: i32,
        __stream: *mut FILE,
    ) -> *mut c_char;
    fn fileno(__stream: *mut FILE) -> i32;
    fn free(_: *mut c_void);
    fn strcpy(_: *mut c_char, _: *const c_char) -> *mut c_char;
    fn strncpy(
        _: *mut c_char,
        _: *const c_char,
        _: u64,
    ) -> *mut c_char;
    fn strcmp(_: *const c_char, _: *const c_char) -> i32;
    fn strncmp(
        _: *const c_char,
        _: *const c_char,
        _: u64,
    ) -> i32;
    fn strlen(_: *const c_char) -> u64;
    fn save(s: *const c_char) -> *mut c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut c_void;
    fn secure_allow(features: i32) -> i32;
    fn bell();
    fn clear_eol();
    fn putbs();
    fn prchar(c: LWCHAR) -> *const c_char;
    fn prutfchar(ch: LWCHAR) -> *const c_char;
    fn utf_len(ch: c_char) -> i32;
    fn is_utf8_well_formed(ss: *const c_char, slen: i32) -> lbool;
    fn step_charc(
        pp: *mut *const c_char,
        dir: i32,
        limit: *const c_char,
    ) -> LWCHAR;
    fn step_char(
        pp: *mut *mut c_char,
        dir: i32,
        limit: *const c_char,
    ) -> LWCHAR;
    fn is_composing_char(ch: LWCHAR) -> lbool;
    fn is_ubin_char(ch: LWCHAR) -> lbool;
    fn is_wide_char(ch: LWCHAR) -> lbool;
    fn is_combining_char(ch1: LWCHAR, ch2: LWCHAR) -> lbool;
    fn in_mca() -> i32;
    fn stop_ignoring_input();
    fn is_ignoring_input(action: i32) -> lbool;
    fn isnullenv(s: *const c_char) -> lbool;
    fn editchar(c: c_char, flags: i32) -> i32;
    fn init_textlist(tlist: *mut textlist, str: *mut c_char);
    fn forw_textlist(
        tlist: *mut textlist,
        prev: *const c_char,
    ) -> *const c_char;
    fn back_textlist(
        tlist: *mut textlist,
        prev: *const c_char,
    ) -> *const c_char;
    fn get_meta_escape() -> *const c_char;
    fn shell_quote(s: *const c_char) -> *mut c_char;
    fn dirfile(
        dirname: *const c_char,
        filename: *const c_char,
        must_exist: i32,
    ) -> *mut c_char;
    fn fcomplete(s: *const c_char) -> *mut c_char;
    fn is_dir(filename: *const c_char) -> lbool;
    fn save_marks(fout: *mut FILE, hdr: *const c_char);
    fn restore_mark(line: *const c_char);
    fn getfraction(
        sp: *mut *const c_char,
        printopt: *const c_char,
        errp: *mut lbool,
    ) -> i64;
    fn findopts_name(pfx: *const c_char) -> *mut c_char;
    fn putchr(ch: i32) -> i32;
    fn putstr(s: *const c_char);
    fn error(fmt: *const c_char, parg: *mut PARG);
    fn fstat(__fd: i32, __buf: *mut stat) -> i32;
    fn fchmod(__fd: i32, __mode: __mode_t) -> i32;
    static mut sc_width: i32;
    static mut utf_mode: i32;
    static mut marks_modified: i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut c_char,
    pub _IO_read_end: *mut c_char,
    pub _IO_read_base: *mut c_char,
    pub _IO_write_base: *mut c_char,
    pub _IO_write_ptr: *mut c_char,
    pub _IO_write_end: *mut c_char,
    pub _IO_buf_base: *mut c_char,
    pub _IO_buf_end: *mut c_char,
    pub _IO_save_base: *mut c_char,
    pub _IO_backup_base: *mut c_char,
    pub _IO_save_end: *mut c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    #[bitfield(name = "_flags2", ty = "i32", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [c_char; 1],
    pub _lock: *mut c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: i32,
    pub _unused2: [c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union parg {
    pub p_string: *const c_char,
    pub p_int: i32,
    pub p_linenum: LINENUM,
    pub p_char: c_char,
}
pub type PARG = parg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct textlist {
    pub string: *mut c_char,
    pub endstring: *mut c_char,
}
/*
 * A mlist structure represents a command history.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mlist {
    pub next: *mut mlist,
    pub prev: *mut mlist,
    pub curr_mp: *mut mlist,
    pub string: *mut c_char,
    pub modified: lbool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct save_ctx {
    pub mlist: *mut mlist,
    pub fout: *mut FILE,
}
#[no_mangle]
pub static mut pasting: lbool = LFALSE;
static mut cmdbuf: [c_char; 2048] = [0; 2048]; /* Buffer for holding a multi-char command */
static mut cmd_col: i32 = 0;                  /* Current column of the cursor */
static mut prompt_col: i32 = 0;                /* Column of cursor just after prompt */
static mut cp: *mut c_char = 0 as *const c_char as *mut c_char; /* Pointer into cmdbuf */
static mut cmd_offset: i32 = 0;                /* Index into cmdbuf of first displayed char */
static mut literal: lbool = LFALSE;                        /* Next input char should not be interpreted */
static mut updown_match: size_t = 0;                       /* Prefix length in up/down movement */
static mut have_updown_match: lbool = LFALSE;
/*
 * These variables are statics used by cmd_complete.
 */
static mut in_completion: lbool = LFALSE;
static mut tk_text: *mut c_char = 0 as *const c_char as *mut c_char;
static mut tk_original: *mut c_char =
    0 as *const c_char as *mut c_char;
static mut tk_ipoint: *const c_char = 0 as *const c_char;
static mut tk_trial: *const c_char = 0 as *const c_char;
static mut tk_tlist: textlist = textlist {
    string: 0 as *const c_char as *mut c_char,
    endstring: 0 as *const c_char as *mut c_char,
};
#[no_mangle]
pub static mut openquote: c_char = '"' as i32 as c_char;
#[no_mangle]
pub static mut closequote: c_char = '"' as i32 as c_char;
/*
 * These are the various command histories that exist.
 */
#[no_mangle]
pub static mut mlist_search: mlist = unsafe {
    {
        let mut init = mlist {
            next: &mlist_search as *const mlist as *mut mlist,
            prev: &mlist_search as *const mlist as *mut mlist,
            curr_mp: &mlist_search as *const mlist as *mut mlist,
            string: 0 as *const c_char as *mut c_char,
            modified: LFALSE,
        };
        init
    }
};
#[no_mangle]
pub static mut ml_search: *mut c_void =
    unsafe { &mlist_search as *const mlist as *mut mlist as *mut c_void };
#[no_mangle]
pub static mut mlist_examine: mlist = unsafe {
    {
        let mut init = mlist {
            next: &mlist_examine as *const mlist as *mut mlist,
            prev: &mlist_examine as *const mlist as *mut mlist,
            curr_mp: &mlist_examine as *const mlist as *mut mlist,
            string: 0 as *const c_char as *mut c_char,
            modified: LFALSE,
        };
        init
    }
};
#[no_mangle]
pub static mut ml_examine: *mut c_void =
    unsafe { &mlist_examine as *const mlist as *mut mlist as *mut c_void };
#[no_mangle]
pub static mut mlist_shell: mlist = unsafe {
    {
        let mut init = mlist {
            next: &mlist_shell as *const mlist as *mut mlist,
            prev: &mlist_shell as *const mlist as *mut mlist,
            curr_mp: &mlist_shell as *const mlist as *mut mlist,
            string: 0 as *const c_char as *mut c_char,
            modified: LFALSE,
        };
        init
    }
};
#[no_mangle]
pub static mut ml_shell: *mut c_void =
    unsafe { &mlist_shell as *const mlist as *mut mlist as *mut c_void };
/*
 * History for the current command.
 */
static mut curr_mlist: *mut mlist = 0 as *const mlist as *mut mlist;
static mut curr_cmdflags: i32 = 0;
static mut cmd_mbc_buf: [c_char; 6] = [0; 6];
static mut cmd_mbc_buf_len: i32 = 0;
static mut cmd_mbc_buf_index: i32 = 0;

/*
 * Reset command buffer (to empty).
 */
#[no_mangle]
pub unsafe extern "C" fn cmd_reset() {
    cp = cmdbuf.as_mut_ptr();
    *cp = '\0' as i32 as c_char;
    cmd_col = 0 as i32;
    cmd_offset = 0 as i32;
    literal = LFALSE;
    cmd_mbc_buf_len = 0 as i32;
    have_updown_match = LFALSE;
}
/*
 * Clear command line.
 */
#[no_mangle]
pub unsafe extern "C" fn clear_cmd() {
    prompt_col = 0 as i32;
    cmd_col = prompt_col;
    cmd_mbc_buf_len = 0 as i32;
    have_updown_match = LFALSE;
}
/*
 * Display a string, usually as a prompt for input into the command buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn cmd_putstr(mut s: *const c_char) {
    let mut prev_ch: LWCHAR = 0 as i32 as LWCHAR;
    let mut ch: LWCHAR = 0;
    let mut endline: *const c_char = s.offset(strlen(s) as isize);
    while *s as i32 != '\0' as i32 {
        let mut os: *const c_char = s;
        let mut width: i32 = 0;
        ch = step_charc(&mut s, 1 as i32, endline);
        while os < s {
            let fresh0 = os;
            os = os.offset(1);
            putchr(*fresh0 as i32);
        }
        if utf_mode == 0 {
            width = 1 as i32;
        } else if is_composing_char(ch) as u32 != 0
            || is_combining_char(prev_ch, ch) as u32 != 0
        {
            width = 0 as i32;
        } else {
            width = if is_wide_char(ch) as u32 != 0 {
                2 as i32
            } else {
                1 as i32
            };
        }
        cmd_col += width;
        prompt_col += width;
        prev_ch = ch;
    }
}
/*
 * How many characters are in the command buffer?
 */
#[no_mangle]
pub unsafe extern "C" fn len_cmdbuf() -> i32 {
    let mut s: *const c_char = cmdbuf.as_mut_ptr();
    let mut endline: *const c_char = s.offset(strlen(s) as isize);
    let mut len: i32 = 0 as i32;
    while *s as i32 != '\0' as i32 {
        step_charc(&mut s, 1 as i32, endline);
        len += 1;
    }
    return len;
}
/*
 * Is the command buffer empty?
 * It is considered nonempty if there is any text in it,
 * or if a multibyte command is being entered but not yet complete.
 */
#[no_mangle]
pub unsafe extern "C" fn cmdbuf_empty() -> lbool {
    return (cp == cmdbuf.as_mut_ptr() && cmd_mbc_buf_len == 0 as i32) as i32
        as lbool;
}
/*
 * Common part of cmd_step_right() and cmd_step_left().
 * {{ Returning pwidth and bswidth separately is a historical artifact
 *    since they're always the same. Maybe clean this up someday. }}
 */
unsafe extern "C" fn cmd_step_common(
    mut p: *mut c_char,
    mut ch: LWCHAR,
    mut len: size_t,
    mut pwidth: *mut i32,
    mut bswidth: *mut i32,
) -> *const c_char {
    let mut pr: *const c_char = 0 as *const c_char;
    let mut width: i32 = 0;
    if len == 1 as i32 as size_t {
        pr = prchar(ch);
        width = strlen(pr) as i32;
    } else {
        pr = prutfchar(ch);
        if is_composing_char(ch) as u64 != 0 {
            width = 0 as i32;
        } else if is_ubin_char(ch) as u64 != 0 {
            width = strlen(pr) as i32;
        } else {
            let mut prev_ch: LWCHAR =
                step_char(&mut p, -(1 as i32), cmdbuf.as_mut_ptr());
            if is_combining_char(prev_ch, ch) as u64 != 0 {
                width = 0 as i32;
            } else {
                width = if is_wide_char(ch) as u32 != 0 {
                    2 as i32
                } else {
                    1 as i32
                };
            }
        }
    }
    if !pwidth.is_null() {
        *pwidth = width;
    }
    if !bswidth.is_null() {
        *bswidth = width;
    }
    return pr;
}
/*
 * Step a pointer one character right in the command buffer.
 */
unsafe extern "C" fn cmd_step_right(
    mut pp: *mut *mut c_char,
    mut pwidth: *mut i32,
    mut bswidth: *mut i32,
) -> *const c_char {
    let mut p: *mut c_char = *pp;
    let mut ch: LWCHAR = step_char(pp, 1 as i32, p.offset(strlen(p) as isize));
    return cmd_step_common(
        p,
        ch,
        (*pp).offset_from(p) as i64 as size_t,
        pwidth,
        bswidth,
    );
}
/*
 * Step a pointer one character left in the command buffer.
 */
unsafe extern "C" fn cmd_step_left(
    mut pp: *mut *mut c_char,
    mut pwidth: *mut i32,
    mut bswidth: *mut i32,
) -> *const c_char {
    let mut p: *mut c_char = *pp;
    let mut ch: LWCHAR = step_char(pp, -(1 as i32), cmdbuf.as_mut_ptr());
    return cmd_step_common(
        *pp,
        ch,
        p.offset_from(*pp) as i64 as size_t,
        pwidth,
        bswidth,
    );
}
/*
 * Put the cursor at "home" (just after the prompt),
 * and set cp to the corresponding char in cmdbuf.
 */
unsafe extern "C" fn cmd_home() {
    while cmd_col > prompt_col {
        let mut width: i32 = 0;
        let mut bswidth: i32 = 0;
        cmd_step_left(&mut cp, &mut width, &mut bswidth);
        loop {
            let fresh1 = bswidth;
            bswidth = bswidth - 1;
            if !(fresh1 > 0 as i32) {
                break;
            }
            putbs();
        }
        cmd_col -= width;
    }
    cp = &mut *cmdbuf.as_mut_ptr().offset(cmd_offset as isize) as *mut c_char;
}
/*
 * Repaint the line from cp onwards.
 * Then position the cursor just after the char old_cp (a pointer into cmdbuf).
 */
#[no_mangle]
pub unsafe extern "C" fn cmd_repaint(mut old_cp: *const c_char) {
    /*
     * Repaint the line from the current position.
     */
    if old_cp.is_null() {
        old_cp = cp;
        cmd_home();
    }
    clear_eol();
    while *cp as i32 != '\0' as i32 {
        let mut np: *mut c_char = cp;
        let mut width: i32 = 0;
        let mut pr: *const c_char =
            cmd_step_right(&mut np, &mut width, 0 as *mut i32);
        if cmd_col + width >= sc_width {
            break;
        }
        cp = np;
        putstr(pr);
        cmd_col += width;
    }
    while *cp as i32 != '\0' as i32 {
        let mut np_0: *mut c_char = cp;
        let mut width_0: i32 = 0;
        let mut pr_0: *const c_char =
            cmd_step_right(&mut np_0, &mut width_0, 0 as *mut i32);
        if width_0 > 0 as i32 {
            break;
        }
        cp = np_0;
        putstr(pr_0);
    }
    /*
     * Back up the cursor to the correct position.
     */
    while cp > old_cp as *mut c_char {
        cmd_left();
    }
}
/*
 * Repaint the entire line, without moving the cursor.
 */
unsafe extern "C" fn cmd_repaint_curr() {
    let mut save_cp: *mut c_char = cp;
    cmd_home();
    cmd_repaint(save_cp);
}
/*
 * Shift the cmdbuf display left a half-screen.
 */
unsafe extern "C" fn cmd_lshift() {
    let mut s: *mut c_char = 0 as *mut c_char;
    let mut save_cp: *mut c_char = 0 as *mut c_char;
    let mut cols: i32 = 0;
    /*
     * Start at the first displayed char, count how far to the
     * right we'd have to move to reach the center of the screen.
     */
    s = cmdbuf.as_mut_ptr().offset(cmd_offset as isize);
    cols = 0 as i32;
    while cols < (sc_width - prompt_col) / 2 as i32
        && *s as i32 != '\0' as i32
    {
        let mut width: i32 = 0;
        cmd_step_right(&mut s, &mut width, 0 as *mut i32);
        cols += width;
    }
    while *s as i32 != '\0' as i32 {
        let mut width_0: i32 = 0;
        let mut ns: *mut c_char = s;
        cmd_step_right(&mut ns, &mut width_0, 0 as *mut i32);
        if width_0 > 0 as i32 {
            break;
        }
        s = ns;
    }
    cmd_offset = s.offset_from(cmdbuf.as_mut_ptr()) as i64 as i32;
    save_cp = cp;
    cmd_home();
    cmd_repaint(save_cp);
}
/*
 * Shift the cmdbuf display right a half-screen.
 */
unsafe extern "C" fn cmd_rshift() {
    let mut s: *mut c_char = 0 as *mut c_char;
    let mut save_cp: *mut c_char = 0 as *mut c_char;
    let mut cols: i32 = 0;
    /*
     * Start at the first displayed char, count how far to the
     * left we'd have to move to traverse a half-screen width
     * of displayed characters.
     */
    s = cmdbuf.as_mut_ptr().offset(cmd_offset as isize);
    cols = 0 as i32;
    while cols < (sc_width - prompt_col) / 2 as i32 && s > cmdbuf.as_mut_ptr() {
        let mut width: i32 = 0;
        cmd_step_left(&mut s, &mut width, 0 as *mut i32);
        cols += width;
    }
    cmd_offset = s.offset_from(cmdbuf.as_mut_ptr()) as i64 as i32;
    save_cp = cp;
    cmd_home();
    cmd_repaint(save_cp);
}
/*
 * Move cursor right one character.
 */
unsafe extern "C" fn cmd_right() -> i32 {
    let mut pr: *const c_char = 0 as *const c_char;
    let mut ncp: *mut c_char = 0 as *mut c_char;
    let mut width: i32 = 0;
    if *cp as i32 == '\0' as i32 {
        /* Already at the end of the line. */
        return CC_OK;
    }
    ncp = cp;
    pr = cmd_step_right(&mut ncp, &mut width, 0 as *mut i32);
    if cmd_col + width >= sc_width {
        cmd_lshift();
    } else if cmd_col + width == sc_width - 1 as i32
        && *cp.offset(1 as i32 as isize) as i32 != '\0' as i32
    {
        cmd_lshift();
    }
    cp = ncp;
    cmd_col += width;
    putstr(pr);
    while *cp as i32 != '\0' as i32 {
        pr = cmd_step_right(&mut ncp, &mut width, 0 as *mut i32);
        if width > 0 as i32 {
            break;
        }
        putstr(pr);
        cp = ncp;
    }
    return CC_OK;
}
/*
 * Move cursor left one character.
 */
unsafe extern "C" fn cmd_left() -> i32 {
    let mut ncp: *mut c_char = 0 as *mut c_char;
    let mut width: i32 = 0 as i32;
    let mut bswidth: i32 = 0 as i32;
    if cp <= cmdbuf.as_mut_ptr() {
        /* Already at the beginning of the line */
        return CC_OK;
    }
    ncp = cp;
    while ncp > cmdbuf.as_mut_ptr() {
        cmd_step_left(&mut ncp, &mut width, &mut bswidth);
        if width > 0 as i32 {
            break;
        }
    }
    if cmd_col < prompt_col + width {
        cmd_rshift();
    }
    cp = ncp;
    cmd_col -= width;
    loop {
        let fresh2 = bswidth;
        bswidth = bswidth - 1;
        if !(fresh2 > 0 as i32) {
            break;
        }
        putbs();
    }
    return CC_OK;
}

/*
 * Insert a char into the command buffer, at the current position.
 */
unsafe extern "C" fn cmd_ichar(cs: &str, clen: usize) -> i32 {
    let mut s: *mut c_char = 0 as *mut c_char;
    if (strlen(cmdbuf.as_mut_ptr())).wrapping_add(clen)
        >= (::core::mem::size_of::<[c_char; 2048]>() as u64)
            .wrapping_sub(1 as i32 as u64)
    {
        /* No room in the command buffer for another char. */
        bell();
        return CC_ERROR;
    }

    /*
     * Make room for the new character (shift the tail of the buffer right).
     */
    s = &mut *cmdbuf.as_mut_ptr().offset((strlen
        as unsafe extern "C" fn(*const c_char) -> u64)(
        cmdbuf.as_mut_ptr()
    ) as isize) as *mut c_char;

    /*
     * Insert the character into the buffer.
     */
    while s >= cp {
        *s.offset(clen as isize) = *s.offset(0 as i32 as isize);
        s = s.offset(-1);
    }
    s = cp;
    while s < cp.offset(clen as isize) {
        let fresh3 = cs;
        cs = cs.offset(1);
        *s = *fresh3;
        s = s.offset(1);
    }
    /*
     * Reprint the tail of the line from the inserted char.
     */
    have_updown_match = LFALSE;
    cmd_repaint(cp);
    cmd_right();
    return CC_OK;
}

/*
 * Backspace in the command buffer.
 * Delete the char to the left of the cursor.
 */
unsafe extern "C" fn cmd_erase() -> i32 {
    let mut s: *mut c_char = 0 as *mut c_char;
    let mut clen: i32 = 0;
    if cp == cmdbuf.as_mut_ptr() {
        /*
         * Backspace past beginning of the buffer:
         * this usually means abort the command.
         */
        return CC_QUIT;
    }
    /*
     * Move cursor left (to the char being erased).
     */
    s = cp;
    cmd_left();
    clen = s.offset_from(cp) as i64 as i32;
    /*
     * Remove the char from the buffer (shift the buffer left).
     */
    s = cp;
    loop {
        *s.offset(0 as i32 as isize) = *s.offset(clen as isize);
        if *s.offset(0 as i32 as isize) as i32 == '\0' as i32 {
            break;
        }
        s = s.offset(1);
    }
    /*
     * Repaint the buffer after the erased char.
     */
    have_updown_match = LFALSE;
    cmd_repaint(cp);
    /*
     * We say that erasing the entire command string causes us
     * to abort the current command, if CF_QUIT_ON_ERASE is set.
     */
    if curr_cmdflags & CF_QUIT_ON_ERASE != 0
        && cp == cmdbuf.as_mut_ptr()
        && *cp as i32 == '\0' as i32
    {
        return CC_QUIT;
    }
    return CC_OK;
}
/*
 * Delete the char under the cursor.
 */
unsafe extern "C" fn cmd_delete() -> i32 {
    if *cp as i32 == '\0' as i32 {
        /* At end of string; there is no char under the cursor. */
        return CC_OK;
    }
    /*
     * Move right, then use cmd_erase.
     */
    cmd_right();
    cmd_erase();
    return CC_OK;
}
/*
 * Delete the "word" to the left of the cursor.
 */
unsafe extern "C" fn cmd_werase() -> i32 {
    if cp > cmdbuf.as_mut_ptr()
        && *cp.offset(-(1 as i32) as isize) as i32 == ' ' as i32
    {
        /*
         * If the char left of cursor is a space,
         * erase all the spaces left of cursor (to the first non-space).
         */
        while cp > cmdbuf.as_mut_ptr()
            && *cp.offset(-(1 as i32) as isize) as i32 == ' ' as i32
        {
            cmd_erase();
        }
    } else {
        /*
         * If the char left of cursor is not a space,
         * erase all the nonspaces left of cursor (the whole "word").
         */
        while cp > cmdbuf.as_mut_ptr()
            && *cp.offset(-(1 as i32) as isize) as i32 != ' ' as i32
        {
            cmd_erase();
        }
    }
    return CC_OK;
}
/*
 * Delete the "word" under the cursor.
 */
unsafe extern "C" fn cmd_wdelete() -> i32 {
    if *cp as i32 == ' ' as i32 {
        /*
         * If the char under the cursor is a space,
         * delete it and all the spaces right of cursor.
         */
        while *cp as i32 == ' ' as i32 {
            cmd_delete();
        }
    } else {
        /*
         * If the char under the cursor is not a space,
         * delete it and all nonspaces right of cursor (the whole word).
         */
        while *cp as i32 != ' ' as i32 && *cp as i32 != '\0' as i32 {
            cmd_delete();
        }
    }
    return CC_OK;
}
/*
 * Delete all chars in the command buffer.
 */
unsafe extern "C" fn cmd_kill() -> i32 {
    if cmdbuf[0 as i32 as usize] as i32 == '\0' as i32 {
        /* Buffer is already empty; abort the current command. */
        return CC_QUIT;
    }
    cmd_offset = 0 as i32;
    cmd_home();
    *cp = '\0' as i32 as c_char;
    have_updown_match = LFALSE;
    cmd_repaint(cp);
    /*
     * We say that erasing the entire command string causes us
     * to abort the current command, if CF_QUIT_ON_ERASE is set.
     */
    if curr_cmdflags & CF_QUIT_ON_ERASE != 0 {
        return CC_QUIT;
    }
    return CC_OK;
}
/*
 * Select an mlist structure to be the current command history.
 */
#[no_mangle]
pub unsafe extern "C" fn set_mlist(
    mut mlist: *mut c_void,
    mut cmdflags: i32,
) {
    curr_mlist = mlist as *mut mlist;
    curr_cmdflags = cmdflags;
    /* Make sure the next up-arrow moves to the last string in the mlist. */
    if !curr_mlist.is_null() {
        (*curr_mlist).curr_mp = curr_mlist;
    }
}
/*
 * Move up or down in the currently selected command history list.
 * Only consider entries whose first updown_match chars are equal to
 * cmdbuf's corresponding chars.
 */
unsafe extern "C" fn cmd_updown(mut action: i32) -> i32 {
    let mut s: *const c_char = 0 as *const c_char;
    let mut ml: *mut mlist = 0 as *mut mlist;
    if curr_mlist.is_null() {
        /*
         * The current command has no history list.
         */
        bell();
        return CC_OK;
    }
    if have_updown_match as u64 == 0 {
        updown_match = cp.offset_from(cmdbuf.as_mut_ptr()) as i64 as size_t;
        have_updown_match = LTRUE;
    }
    /*
     * Find the next history entry which matches.
     */
    ml = (*curr_mlist).curr_mp;
    loop {
        ml = if action == EC_UP {
            (*ml).prev
        } else {
            (*ml).next
        };
        if ml == curr_mlist {
            /*
             * We reached the end (or beginning) of the list.
             */
            break;
        }
        if strncmp(cmdbuf.as_mut_ptr(), (*ml).string, updown_match) == 0 as i32 {
            /*
             * This entry matches; stop here.
             * Copy the entry into cmdbuf and echo it on the screen.
             */
            (*curr_mlist).curr_mp = ml;
            s = (*ml).string;
            if s.is_null() {
                s = b"\0" as *const u8 as *const c_char;
            }
            cmd_offset = 0 as i32;
            cmd_home();
            clear_eol();
            strcpy(cmdbuf.as_mut_ptr(), s);
            cp = cmdbuf.as_mut_ptr();
            while *cp as i32 != '\0' as i32 {
                cmd_right();
            }
            return CC_OK;
        }
    }
    /*
     * We didn't find a history entry that matches.
     */
    bell();
    return CC_OK;
}
/*
 * Yet another lesson in the evils of global variables.
 */
#[no_mangle]
pub unsafe extern "C" fn save_updown_match() -> ssize_t {
    if have_updown_match as u64 == 0 {
        return -(1 as i32) as ssize_t;
    }
    return updown_match as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn restore_updown_match(mut udm: ssize_t) {
    updown_match = udm as size_t;
    have_updown_match = (udm != -(1 as i32) as ssize_t) as i32 as lbool;
}
unsafe extern "C" fn ml_link(mut mlist: *mut mlist, mut ml: *mut mlist) {
    (*ml).next = mlist;
    (*ml).prev = (*mlist).prev;
    (*(*mlist).prev).next = ml;
    (*mlist).prev = ml;
}
unsafe extern "C" fn ml_unlink(mut ml: *mut mlist) {
    (*(*ml).prev).next = (*ml).next;
    (*(*ml).next).prev = (*ml).prev;
}
/*
 * Add a string to an mlist.
 */
#[no_mangle]
pub unsafe extern "C" fn cmd_addhist(
    mut mlist: *mut mlist,
    mut cmd: *const c_char,
    mut modified: lbool,
) {
    let mut ml: *mut mlist = 0 as *mut mlist;
    /*
     * Don't save a trivial command.
     */
    if strlen(cmd) == 0 as i32 as u64 {
        return;
    }
    let opts = get_options();
    if opts.no_hist_dups != 0 {
        let mut next: *mut mlist = 0 as *mut mlist;
        ml = (*mlist).next;
        while !((*ml).string).is_null() {
            next = (*ml).next;
            if strcmp((*ml).string, cmd) == 0 as i32 {
                ml_unlink(ml);
                free((*ml).string as *mut c_void);
                drop(Box::from_raw(ml));
            }
            ml = next;
        }
    }
    /*
     * Save the command unless it's a duplicate of the
     * last command in the history.
     */
    ml = (*mlist).prev;
    if ml == mlist || strcmp((*ml).string, cmd) != 0 as i32 {
        /*
         * Did not find command in history.
         * Save the command and put it at the end of the history list.
         */
        ml = Box::into_raw(Box::new(mlist {
            next: 0 as *mut mlist,
            prev: 0 as *mut mlist,
            curr_mp: 0 as *mut mlist,
            string: 0 as *mut c_char,
            modified: LFALSE,
        }));
        (*ml).string = save(cmd);
        (*ml).modified = modified;
        ml_link(mlist, ml);
    }
    /*
     * Point to the cmd just after the just-accepted command.
     * Thus, an UPARROW will always retrieve the previous command.
     */
    (*mlist).curr_mp = (*ml).next;
}
/*
 * Accept the command in the command buffer.
 * Add it to the currently selected history list.
 */
#[no_mangle]
pub unsafe extern "C" fn cmd_accept() {
    /*
     * Nothing to do if there is no currently selected history list.
     */
    if curr_mlist.is_null() || curr_mlist == ml_examine as *mut mlist {
        return;
    }
    cmd_addhist(curr_mlist, cmdbuf.as_mut_ptr(), LTRUE);
    (*curr_mlist).modified = LTRUE;
}
/*
 * Try to perform a line-edit function on the command buffer,
 * using a specified char as a line-editing command.
 * Returns:
 *      CC_PASS The char does not invoke a line edit function.
 *      CC_OK   Line edit function done.
 *      CC_QUIT The char requests the current command to be aborted.
 */
unsafe extern "C" fn cmd_edit(
    mut c: c_char,
    mut stay_in_completion: bool,
) -> i32 {
    let mut action: i32 = 0;
    let mut flags: i32 = 0;
    let opts = get_options();
    /*
     * See if the char is indeed a line-editing command.
     */
    flags = 0 as i32;
    if curr_mlist.is_null() {
        /*
         * No current history; don't accept history manipulation cmds.
         */
        flags |= ECF_NOHISTORY;
    }
    /*
     * Don't accept completion cmds in contexts
     * such as search pattern, digits, etc.
     */
    if !(curr_mlist.is_null()
        && curr_cmdflags & CF_OPTION != 0
        || curr_mlist == ml_examine as *mut mlist
        || curr_mlist == ml_shell as *mut mlist)
    {
        flags |= ECF_NOCOMPLETE;
    }
    action = editchar(c, flags);
    if is_ignoring_input(action) as u64 != 0 {
        return CC_OK;
    }
    match action {
        A_NOACTION => return CC_OK,
        EC_START_PASTE => {
            if opts.no_paste != 0 {
                pasting = LTRUE;
            }
            return CC_OK;
        }
        EC_END_PASTE => {
            stop_ignoring_input();
            return CC_OK;
        }
        EC_RIGHT => {
            if !stay_in_completion {
                in_completion = LFALSE;
            }
            return cmd_right();
        }
        EC_LEFT => {
            if !stay_in_completion {
                in_completion = LFALSE;
            }
            return cmd_left();
        }
        EC_W_RIGHT => {
            if !stay_in_completion {
                in_completion = LFALSE;
            }
            while *cp as i32 != '\0' as i32 && *cp as i32 != ' ' as i32 {
                cmd_right();
            }
            while *cp as i32 == ' ' as i32 {
                cmd_right();
            }
            return CC_OK;
        }
        EC_W_LEFT => {
            if !stay_in_completion {
                in_completion = LFALSE;
            }
            while cp > cmdbuf.as_mut_ptr()
                && *cp.offset(-(1 as i32) as isize) as i32 == ' ' as i32
            {
                cmd_left();
            }
            while cp > cmdbuf.as_mut_ptr()
                && *cp.offset(-(1 as i32) as isize) as i32 != ' ' as i32
            {
                cmd_left();
            }
            return CC_OK;
        }
        EC_HOME => {
            if !stay_in_completion {
                in_completion = LFALSE;
            }
            cmd_offset = 0 as i32;
            cmd_home();
            cmd_repaint(cp);
            return CC_OK;
        }
        EC_END => {
            if !stay_in_completion {
                in_completion = LFALSE;
            }
            while *cp as i32 != '\0' as i32 {
                cmd_right();
            }
            return CC_OK;
        }
        EC_INSERT => {
            if !stay_in_completion {
                in_completion = LFALSE;
            }
            return CC_OK;
        }
        EC_BACKSPACE => {
            if !stay_in_completion {
                in_completion = LFALSE;
            }
            return cmd_erase();
        }
        EC_LINEKILL => {
            if !stay_in_completion {
                in_completion = LFALSE;
            }
            return cmd_kill();
        }
        EC_ABORT => {
            if !stay_in_completion {
                in_completion = LFALSE;
            }
            cmd_kill();
            return CC_QUIT;
        }
        EC_W_BACKSPACE => {
            if !stay_in_completion {
                in_completion = LFALSE;
            }
            return cmd_werase();
        }
        EC_DELETE => {
            if !stay_in_completion {
                in_completion = LFALSE;
            }
            return cmd_delete();
        }
        EC_W_DELETE => {
            if !stay_in_completion {
                in_completion = LFALSE;
            }
            return cmd_wdelete();
        }
        EC_LITERAL => {
            literal = LTRUE;
            return CC_OK;
        }
        EC_UP | EC_DOWN => {
            if !stay_in_completion {
                in_completion = LFALSE;
            }
            return cmd_updown(action);
        }
        EC_F_COMPLETE | EC_B_COMPLETE | EC_EXPAND => return cmd_complete(action),
        _ => {
            if !stay_in_completion {
                in_completion = LFALSE;
            }
            return CC_PASS;
        }
    };
}
/*
 * Insert a string into the command buffer, at the current position.
 */
unsafe extern "C" fn cmd_istr(mut str: *const c_char) -> i32 {
    let mut endline: *const c_char = str.offset(strlen(str) as isize);
    let mut s: *const c_char = 0 as *const c_char;
    let mut action: i32 = 0;
    s = str;
    while *s as i32 != '\0' as i32 {
        let mut os: *const c_char = s;
        step_charc(&mut s, 1 as i32, endline);
        action = cmd_ichar(os, s.offset_from(os) as i64 as size_t);
        if action != CC_OK {
            return action;
        }
    }
    return CC_OK;
}
/*
 * Set tk_original to word.
 */
unsafe extern "C" fn set_tk_original(mut word: *const c_char) {
    if !tk_original.is_null() {
        free(tk_original as *mut c_void);
    }
    tk_original = ecalloc(
        (cp.offset_from(word) as i64 as size_t)
            .wrapping_add(1 as i32 as size_t),
        ::core::mem::size_of::<c_char>() as u64,
    ) as *mut c_char;
    strncpy(
        tk_original,
        word,
        cp.offset_from(word) as i64 as size_t,
    );
}
/*
 * Find the beginning and end of the "current" word.
 * This is the word which the cursor (cp) is inside or at the end of.
 * Return pointer to the beginning of the word and put the
 * cursor at the end of the word.
 */
unsafe extern "C" fn delimit_word() -> *mut c_char {
    let mut word: *mut c_char = 0 as *mut c_char;
    let mut p: *mut c_char = 0 as *mut c_char;
    let mut delim_quoted: i32 = LFALSE as i32;
    let mut meta_quoted: i32 = LFALSE as i32;
    let mut esc: *const c_char = get_meta_escape();
    let mut esclen: size_t = strlen(esc);
    /*
     * Move cursor to end of word.
     */
    if *cp as i32 != ' ' as i32 && *cp as i32 != '\0' as i32 {
        /*
         * Cursor is on a nonspace.
         * Move cursor right to the next space.
         */
        while *cp as i32 != ' ' as i32 && *cp as i32 != '\0' as i32 {
            cmd_right();
        }
    } else if cp > cmdbuf.as_mut_ptr()
        && *cp.offset(-(1 as i32) as isize) as i32 != ' ' as i32
    {
        /*
         * Cursor is on a space, and char to the left is a nonspace.
         * We're already at the end of the word.
         */
    }
    /*
     * Find the beginning of the word which the cursor is in.
     */
    if cp == cmdbuf.as_mut_ptr() {
        return 0 as *mut c_char;
    }
    /*
     * If we have an unbalanced quote (that is, an open quote
     * without a corresponding close quote), we return everything
     * from the open quote, including spaces.
     */
    word = cmdbuf.as_mut_ptr();
    while word < cp {
        if *word as i32 != ' ' as i32 {
            break;
        }
        word = word.offset(1);
    }
    if word >= cp {
        return cp;
    }
    p = cmdbuf.as_mut_ptr();
    while p < cp {
        if meta_quoted != 0 {
            meta_quoted = LFALSE as i32;
        } else if esclen > 0 as i32 as size_t
            && p.offset(esclen as isize) < cp
            && strncmp(p, esc, esclen) == 0 as i32
        {
            meta_quoted = LTRUE as i32;
            p = p.offset(esclen.wrapping_sub(1 as i32 as size_t) as isize);
        } else if delim_quoted != 0 {
            if *p as i32 == closequote as i32 {
                delim_quoted = LFALSE as i32;
            }
        } else if *p as i32 == openquote as i32 {
            delim_quoted = LTRUE as i32;
        } else if *p as i32 == ' ' as i32 {
            word = p.offset(1 as i32 as isize);
        }
        p = p.offset(1);
    }
    return word;
}
/*
 * Set things up to enter file completion mode.
 * Expand the word under the cursor into a list of filenames
 * which start with that word, and set tk_text to that list.
 */
unsafe extern "C" fn init_file_compl() {
    let mut word: *mut c_char = 0 as *mut c_char;
    let mut c: c_char = 0;
    /*
     * Find the original (uncompleted) word in the command buffer.
     */
    word = delimit_word();
    if word.is_null() {
        return;
    }
    /*
     * Set the insertion point to the point in the command buffer
     * where the original (uncompleted) word now sits.
     */
    tk_ipoint = word;
    set_tk_original(word);
    /*
     * Get the expanded filename.
     * This may result in a single filename, or
     * a blank-separated list of filenames.
     */
    c = *cp;
    *cp = '\0' as i32 as c_char;
    if *word as i32 != openquote as i32 {
        tk_text = fcomplete(word);
    } else {
        let mut qword: *mut c_char =
            shell_quote(word.offset(1 as i32 as isize));
        if qword.is_null() {
            tk_text = fcomplete(word.offset(1 as i32 as isize));
        } else {
            tk_text = fcomplete(qword);
            free(qword as *mut c_void);
        }
    }
    *cp = c;
}
/*
 * Set things up to enter option completion mode.
 */
unsafe extern "C" fn init_opt_compl() {
    tk_ipoint = cmdbuf.as_mut_ptr();
    set_tk_original(cmdbuf.as_mut_ptr());
    tk_text = findopts_name(cmdbuf.as_mut_ptr());
}
/*
 * Return the next word in the current completion list.
 */
unsafe extern "C" fn next_compl(
    mut action: i32,
    mut prev: *const c_char,
) -> *const c_char {
    match action {
        EC_F_COMPLETE => return forw_textlist(&mut tk_tlist, prev),
        EC_B_COMPLETE => return back_textlist(&mut tk_tlist, prev),
        _ => {}
    }
    return b"?\0" as *const u8 as *const c_char;
}
/*
 * Complete the filename before (or under) the cursor.
 * cmd_complete may be called multiple times.  The global in_completion
 * remembers whether this call is the first time (create the list),
 * or a subsequent time (step thru the list).
 */
unsafe extern "C" fn cmd_complete(mut action: i32) -> i32 {
    let mut current_block: u64;
    let mut s: *const c_char = 0 as *const c_char;
    if in_completion as u64 == 0 || action == EC_EXPAND {
        /*
         * Expand the word under the cursor and
         * use the first word in the expansion
         * (or the entire expansion if we're doing EC_EXPAND).
         */
        if !tk_text.is_null() {
            free(tk_text as *mut c_void);
            tk_text = 0 as *mut c_char;
        }
        if curr_cmdflags & CF_OPTION != 0 {
            init_opt_compl();
        } else {
            init_file_compl();
        }
        if tk_text.is_null() {
            bell();
            return CC_OK;
        }
        if action == EC_EXPAND {
            /*
             * Use the whole list.
             */
            tk_trial = tk_text;
        } else {
            /*
             * Use the first filename in the list.
             */
            in_completion = LTRUE;
            init_textlist(&mut tk_tlist, tk_text);
            tk_trial = next_compl(action, 0 as *mut c_void as *mut c_char);
        }
    } else {
        /*
         * We already have a completion list.
         * Use the next/previous filename from the list.
         */
        tk_trial = next_compl(action, tk_trial);
    }
    /*
     * Remove the original word, or the previous trial completion.
     */
    while cp > tk_ipoint as *mut c_char {
        cmd_erase();
    }
    if tk_trial.is_null() {
        /*
         * There are no more trial completions.
         * Insert the original (uncompleted) filename.
         */
        in_completion = LFALSE;
        if cmd_istr(tk_original) != CC_OK {
            current_block = 16725810106060436304;
        } else {
            current_block = 4488286894823169796;
        }
    } else if cmd_istr(tk_trial) != CC_OK {
        current_block = 16725810106060436304;
    } else if is_dir(tk_trial) as u64 != 0 {
        /*
         * If it is a directory, append a slash.
         */
        if cp > cmdbuf.as_mut_ptr()
            && *cp.offset(-(1 as i32) as isize) as i32
                == closequote as i32
        {
            cmd_erase();
        }
        let ss = lgetenv("LESSSEPARATOR0");
        let ss_cstring;
        if ss.is_err() {
            s = b"/\0" as *const u8 as *const c_char;
        } else {
            ss_cstring = CString::new(ss.unwrap()).unwrap();
            s = ss_cstring.as_ptr();
        }
        if cmd_istr(s) != CC_OK {
            current_block = 16725810106060436304;
        } else {
            current_block = 4488286894823169796;
        }
    } else {
        current_block = 4488286894823169796;
    }
    match current_block {
        4488286894823169796 => return CC_OK,
        _ => {
            in_completion = LFALSE;
            bell();
            return CC_OK;
        }
    };
}
/*
 * Build a UTF-8 char in cmd_mbc_buf.
 * Returns:
 *      CC_OK    Char has been stored but we don't have a complete UTF-8 sequence yet.
 *      CC_ERROR This is an invalid UTF-8 sequence.
 *      CC_PASS  There is a complete UTF-8 sequence in cmd_mbc_buf.
 *               The length of the complete sequence is returned in *plen.
 */
unsafe extern "C" fn cmd_uchar(mut c: c_char, mut plen: *mut size_t) -> i32 {
    if utf_mode == 0 {
        cmd_mbc_buf[0 as i32 as usize] = c;
        *plen = 1 as i32 as size_t;
    } else {
        /* Perform strict validation in all possible cases. */
        let mut current_block_24: u64;
        if cmd_mbc_buf_len == 0 as i32 {
            current_block_24 = 6649913226281796480;
        } else if c as i32 & 0xc0 as i32 == 0x80 as i32 {
            let fresh4 = cmd_mbc_buf_index;
            cmd_mbc_buf_index = cmd_mbc_buf_index + 1;
            cmd_mbc_buf[fresh4 as usize] = c;
            if cmd_mbc_buf_index < cmd_mbc_buf_len {
                return CC_OK;
            }
            if is_utf8_well_formed(cmd_mbc_buf.as_mut_ptr(), cmd_mbc_buf_index) as u64 == 0 {
                /* complete, but not well formed (non-shortest form), sequence */
                cmd_mbc_buf_len = 0 as i32;
                bell();
                return CC_ERROR;
            }
            current_block_24 = 26972500619410423;
        } else {
            /* Flush incomplete (truncated) sequence. */
            cmd_mbc_buf_len = 0 as i32;
            bell();
            /* Handle new char. */
            current_block_24 = 6649913226281796480;
        }
        match current_block_24 {
            6649913226281796480 => {
                cmd_mbc_buf_index = 1 as i32;
                *cmd_mbc_buf.as_mut_ptr() = c;
                if c as i32 & 0x80 as i32 == 0 as i32 {
                    cmd_mbc_buf_len = 1 as i32;
                } else if c as i32 & 0xc0 as i32 == 0xc0 as i32
                    && !(c as i32 & 0xfe as i32 == 0xfe as i32)
                {
                    cmd_mbc_buf_len = utf_len(c);
                    return CC_OK;
                } else {
                    bell();
                    return CC_ERROR;
                }
            }
            _ => {}
        }
        *plen = cmd_mbc_buf_len as size_t;
        cmd_mbc_buf_len = 0 as i32;
    }
    return CC_PASS;
}

/*
 * Process a single character of a multi-character command, such as
 * a number, or the pattern of a search command.
 * Returns:
 *      CC_OK           The char was accepted.
 *      CC_QUIT         The char requests the command to be aborted.
 *      CC_ERROR        The char could not be accepted due to an error.
 */
unsafe extern "C" fn cmd_char2(c: char, stay_in_completion: bool) -> i32 {
    let mut len: size_t = 0;
    let mut action = cmd_uchar(c, &mut len);
    if action != CC_PASS {
        return action;
    }
    if literal as u64 != 0 {
        /*
         * Insert the char, even if it is a line-editing char.
         */
        literal = LFALSE;
        return cmd_ichar(cmd_mbc_buf.as_mut_ptr(), len);
    }

    /*
     * See if it is a line-editing character.
     */
    if in_mca() != 0 && len == 1 as i32 as size_t {
        action = cmd_edit(c, stay_in_completion);
        match action {
            CC_OK | CC_QUIT => return action,
            CC_PASS | _ => {}
        }
    }
    /*
     * Insert the char into the command buffer.
     */
    return cmd_ichar(cmd_mbc_buf.as_mut_ptr(), len) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_char(c: char) -> i32 {
    return cmd_char2(c, false);
}

/*
 * Copy an ASCII string to the command buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn cmd_setstring(s: &str, uc: bool) -> i32 {
    for c in s.chars() {
        let ch = if uc && c.is_ascii_lowercase() {
            c.to_ascii_uppercase()
        } else {
            c
        };
        let action = cmd_char2(ch, true);
        if action != CC_OK {
            return action;
        }
    }
    cmd_repaint_curr();
    return CC_OK;
}

/*
 * Return the number currently in the command buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn cmd_int(mut frac: *mut i64) -> LINENUM {
    let mut p: *const c_char = 0 as *const c_char;
    let mut n: LINENUM = 0 as i32 as LINENUM;
    let mut err: lbool = LFALSE;
    p = cmdbuf.as_mut_ptr();
    while *p as i32 >= '0' as i32 && *p as i32 <= '9' as i32 {
        let (fresh6, fresh7) = n.overflowing_mul(10 as i32 as i64);
        *(&mut n as *mut LINENUM) = fresh6;
        if fresh7 as i32 != 0 || {
            let (fresh8, fresh9) = n.overflowing_add((*p as i32 - '0' as i32) as i64);
            *(&mut n as *mut LINENUM) = fresh8;
            fresh9 as i32 != 0
        } {
            error(
                b"Integer is too big\0" as *const u8 as *const c_char,
                0 as *mut c_void as *mut PARG,
            );
            return 0 as i32 as LINENUM;
        }
        p = p.offset(1);
    }
    *frac = 0 as i32 as i64;
    let fresh10 = p;
    p = p.offset(1);
    if *fresh10 as i32 == '.' as i32 {
        *frac = getfraction(&mut p, 0 as *const c_char, &mut err);
    }
    return n;
}
/*
 * Return a pointer to the command buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn get_cmdbuf() -> *const c_char {
    if cmd_mbc_buf_index < cmd_mbc_buf_len {
        /* Don't return buffer containing an incomplete multibyte char. */
        return 0 as *const c_char;
    }
    return cmdbuf.as_mut_ptr();
}
/*
 * Return the last (most recent) string in the current command history.
 */
#[no_mangle]
pub unsafe extern "C" fn cmd_lastpattern() -> *const c_char {
    if curr_mlist.is_null() {
        return 0 as *const c_char;
    }
    return (*(*(*curr_mlist).curr_mp).prev).string;
}
unsafe extern "C" fn mlist_size(mut ml: *mut mlist) -> i32 {
    let mut size: i32 = 0 as i32;
    ml = (*ml).next;
    while !((*ml).string).is_null() {
        size += 1;
        ml = (*ml).next;
    }
    return size;
}
/*
 * Get the name of the history file.
 */
unsafe extern "C" fn histfile_find(mut must_exist: lbool) -> *mut c_char {
    let home_cstring = CString::new(lgetenv("HOME").unwrap_or_default()).unwrap();
    let mut home: *const c_char = home_cstring.as_ptr();
    let mut name: *mut c_char = 0 as *mut c_char;
    /* Try in $XDG_STATE_HOME, then in $HOME/.local/state, then in $XDG_DATA_HOME, then in $HOME. */
    let xdg_state_cstring = lgetenv("XDG_STATE_HOME").ok().map(|s| CString::new(s).unwrap());
    let xdg_state_ptr = xdg_state_cstring.as_ref().map_or(0 as *const c_char, |c| c.as_ptr());
    name = dirfile(
        xdg_state_ptr,
        &*(b".lesshst\0" as *const u8 as *const c_char)
            .offset(1 as i32 as isize),
        must_exist as i32,
    );
    if name.is_null() {
        let mut dir: *mut c_char = dirfile(
            home,
            b".local/state\0" as *const u8 as *const c_char,
            1 as i32,
        );
        if !dir.is_null() {
            name = dirfile(
                dir,
                &*(b".lesshst\0" as *const u8 as *const c_char)
                    .offset(1 as i32 as isize),
                must_exist as i32,
            );
            free(dir as *mut c_void);
        }
    }
    if name.is_null() {
        let xdg_data_cstring = lgetenv("XDG_DATA_HOME").ok().map(|s| CString::new(s).unwrap());
        let xdg_data_ptr = xdg_data_cstring.as_ref().map_or(0 as *const c_char, |c| c.as_ptr());
        name = dirfile(
            xdg_data_ptr,
            &*(b".lesshst\0" as *const u8 as *const c_char)
                .offset(1 as i32 as isize),
            must_exist as i32,
        );
    }
    if name.is_null() {
        name = dirfile(
            home,
            b".lesshst\0" as *const u8 as *const c_char,
            must_exist as i32,
        );
    }
    return name;
}
unsafe extern "C" fn histfile_name(mut must_exist: lbool) -> *mut c_char {
    let mut wname: *mut c_char = 0 as *mut c_char;
    /* See if filename is explicitly specified by $LESSHISTFILE. */
    if let Ok(name) = lgetenv("LESSHISTFILE") {
        if name == "-" || name == "/dev/null" {
            /* $LESSHISTFILE == "-" means don't use a history file. */
            return 0 as *mut c_char;
        }
        let name_cstring = CString::new(name).unwrap();
        return save(name_cstring.as_ptr());
    }
    /* See if history file is disabled in the build. */
    if ".lesshst" == "" || ".lesshst" == "-" {
        return 0 as *mut c_char;
    }
    wname = 0 as *mut c_char;
    if must_exist as u64 == 0 {
        /* If we're writing the file and the file already exists, use it. */
        wname = histfile_find(LTRUE);
    }
    if wname.is_null() {
        wname = histfile_find(must_exist);
    }
    return wname;
}
/*
 * Read a .lesshst file and call a callback for each line in the file.
 */
unsafe extern "C" fn read_cmdhist2(
    mut action: Option<
        unsafe extern "C" fn(*mut c_void, *mut mlist, *const c_char) -> (),
    >,
    mut uparam: *mut c_void,
    mut skip_search: i32,
    mut skip_shell: i32,
) {
    let mut ml: *mut mlist = 0 as *mut mlist;
    let mut line: [c_char; 2048] = [0; 2048];
    let mut filename: *mut c_char = 0 as *mut c_char;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut skip: *mut i32 = 0 as *mut i32;
    filename = histfile_name(LTRUE);
    if filename.is_null() {
        return;
    }
    f = fopen(filename, b"r\0" as *const u8 as *const c_char);
    free(filename as *mut c_void);
    if f.is_null() {
        return;
    }
    if (fgets(
        line.as_mut_ptr(),
        ::core::mem::size_of::<[c_char; 2048]>() as u64 as i32,
        f,
    ))
    .is_null()
        || strncmp(
            line.as_mut_ptr(),
            b".less-history-file:\0" as *const u8 as *const c_char,
            b".less-history-file:".len() as size_t,
        ) != 0 as i32
    {
        fclose(f);
        return;
    }
    while !(fgets(
        line.as_mut_ptr(),
        ::core::mem::size_of::<[c_char; 2048]>() as u64 as i32,
        f,
    ))
    .is_null()
    {
        let mut p: *mut c_char = 0 as *mut c_char;
        p = line.as_mut_ptr();
        while *p as i32 != '\0' as i32 {
            if *p as i32 == '\n' as i32 || *p as i32 == '\r' as i32 {
                *p = '\0' as i32 as c_char;
                break;
            } else {
                p = p.offset(1);
            }
        }
        if strcmp(
            line.as_mut_ptr(),
            b".search\0" as *const u8 as *const c_char,
        ) == 0 as i32
        {
            ml = &mut mlist_search;
            skip = &mut skip_search;
        } else if strcmp(
            line.as_mut_ptr(),
            b".shell\0" as *const u8 as *const c_char,
        ) == 0 as i32
        {
            ml = &mut mlist_shell;
            skip = &mut skip_shell;
        } else if strcmp(
            line.as_mut_ptr(),
            b".mark\0" as *const u8 as *const c_char,
        ) == 0 as i32
        {
            ml = 0 as *mut mlist;
        } else if *line.as_mut_ptr() as i32 == '"' as i32 {
            if !ml.is_null() {
                if !skip.is_null() && *skip > 0 as i32 {
                    *skip -= 1;
                    *skip;
                } else {
                    (Some(action.expect("non-null function pointer")))
                        .expect("non-null function pointer")(
                        uparam,
                        ml,
                        line.as_mut_ptr().offset(1 as i32 as isize),
                    );
                }
            }
        } else if *line.as_mut_ptr() as i32 == 'm' as i32 {
            (Some(action.expect("non-null function pointer"))).expect("non-null function pointer")(
                uparam,
                0 as *mut mlist,
                line.as_mut_ptr(),
            );
        }
    }
    fclose(f);
}
unsafe extern "C" fn read_cmdhist(
    mut action: Option<
        unsafe extern "C" fn(*mut c_void, *mut mlist, *const c_char) -> (),
    >,
    mut uparam: *mut c_void,
    mut skip_search: lbool,
    mut skip_shell: lbool,
) {
    if secure_allow((1 as i32) << 4 as i32) == 0 {
        return;
    }
    read_cmdhist2(
        action,
        uparam,
        skip_search as i32,
        skip_shell as i32,
    );
    (Some(action.expect("non-null function pointer"))).expect("non-null function pointer")(
        uparam,
        0 as *mut mlist,
        0 as *const c_char,
    ); /* signal end of file */
}
unsafe extern "C" fn addhist_init(
    mut uparam: *mut c_void,
    mut ml: *mut mlist,
    mut string: *const c_char,
) {
    if !ml.is_null() {
        cmd_addhist(ml, string, LFALSE);
    } else if !string.is_null() {
        restore_mark(string);
    }
}
/*
 * Initialize history from a .lesshist file.
 */
#[no_mangle]
pub unsafe extern "C" fn init_cmdhist() {
    read_cmdhist(
        Some(
            addhist_init
                as unsafe extern "C" fn(
                    *mut c_void,
                    *mut mlist,
                    *const c_char,
                ) -> (),
        ),
        0 as *mut c_void,
        LFALSE,
        LFALSE,
    );
}
/*
 * Write the header for a section of the history file.
 */
unsafe extern "C" fn write_mlist_header(mut ml: *mut mlist, mut f: *mut FILE) {
    if ml == &mut mlist_search as *mut mlist {
        fprintf(
            f,
            b"%s\n\0" as *const u8 as *const c_char,
            b".search\0" as *const u8 as *const c_char,
        );
    } else if ml == &mut mlist_shell as *mut mlist {
        fprintf(
            f,
            b"%s\n\0" as *const u8 as *const c_char,
            b".shell\0" as *const u8 as *const c_char,
        );
    }
}
/*
 * Write all modified entries in an mlist to the history file.
 */
unsafe extern "C" fn write_mlist(mut ml: *mut mlist, mut f: *mut FILE) {
    ml = (*ml).next;
    while !((*ml).string).is_null() {
        if !((*ml).modified as u64 == 0) {
            fprintf(
                f,
                b"\"%s\n\0" as *const u8 as *const c_char,
                (*ml).string,
            );
            (*ml).modified = LFALSE;
        }
        ml = (*ml).next;
    }
    (*ml).modified = LFALSE; /* entire mlist is now unmodified */
}
/*
 * Make a temp name in the same directory as filename.
 */
unsafe extern "C" fn make_tempname(mut filename: *const c_char) -> *mut c_char {
    let mut lastch: c_char = 0;
    let mut tempname: *mut c_char = ecalloc(
        1 as i32 as size_t,
        (strlen(filename)).wrapping_add(1 as i32 as u64),
    ) as *mut c_char;
    strcpy(tempname, filename);
    lastch = *tempname.offset(
        (strlen(tempname)).wrapping_sub(1 as i32 as u64) as isize,
    );
    *tempname.offset(
        (strlen(tempname)).wrapping_sub(1 as i32 as u64) as isize,
    ) = (if lastch as i32 == 'Q' as i32 {
        'Z' as i32
    } else {
        'Q' as i32
    }) as c_char;
    return tempname;
}
/*
 * Copy entries from the saved history file to a new file.
 * At the end of each mlist, append any new entries
 * created during this session.
 */
unsafe extern "C" fn copy_hist(
    mut uparam: *mut c_void,
    mut ml: *mut mlist,
    mut string: *const c_char,
) {
    let mut ctx: *mut save_ctx = uparam as *mut save_ctx;
    if !ml.is_null() && ml != (*ctx).mlist {
        /* We're changing mlists. */
        if !((*ctx).mlist).is_null() {
            /* Append any new entries to the end of the current mlist. */
            write_mlist((*ctx).mlist, (*ctx).fout);
        }
        /* Write the header for the new mlist. */
        (*ctx).mlist = ml;
        write_mlist_header((*ctx).mlist, (*ctx).fout);
    }
    if string.is_null() {
        /* End of file */
        /* Write any sections that were not in the original file. */
        if mlist_search.modified as u64 != 0 {
            write_mlist_header(&mut mlist_search, (*ctx).fout);
            write_mlist(&mut mlist_search, (*ctx).fout);
        }
        if mlist_shell.modified as u64 != 0 {
            write_mlist_header(&mut mlist_shell, (*ctx).fout);
            write_mlist(&mut mlist_shell, (*ctx).fout);
        }
    } else if !ml.is_null() {
        /* Copy mlist entry. */
        fprintf(
            (*ctx).fout,
            b"\"%s\n\0" as *const u8 as *const c_char,
            string,
        );
    }
    /* Skip marks */
}
/*
 * Make a file readable only by its owner.
 */
unsafe extern "C" fn make_file_private(mut f: *mut FILE) {
    let mut do_chmod: lbool = LTRUE;
    let mut statbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut r: i32 = fstat(fileno(f), &mut statbuf);
    if r < 0 as i32
        || !(statbuf.st_mode & 0o170000 as i32 as __mode_t
            == 0o100000 as i32 as __mode_t)
    {
        /* Don't chmod if not a regular file. */
        do_chmod = LFALSE;
    }
    if do_chmod as u64 != 0 {
        fchmod(fileno(f), 0o600 as i32 as __mode_t);
    }
}
/*
 * Does the history file need to be updated?
 */
unsafe extern "C" fn histfile_modified() -> lbool {
    if mlist_search.modified as u64 != 0 {
        return LTRUE;
    }
    if mlist_shell.modified as u64 != 0 {
        return LTRUE;
    }
    if marks_modified != 0 {
        return LTRUE;
    }
    return LFALSE;
}
/*
 * Update the .lesshst file.
 */
#[no_mangle]
pub unsafe extern "C" fn save_cmdhist() {
    let mut histname: *mut c_char = 0 as *mut c_char;
    let mut tempname: *mut c_char = 0 as *mut c_char;
    let mut skip_search: i32 = 0;
    let mut skip_shell: i32 = 0;
    let mut ctx: save_ctx = save_ctx {
        mlist: 0 as *mut mlist,
        fout: 0 as *mut FILE,
    };
    let mut s: *const c_char = 0 as *const c_char;
    let mut fout: *mut FILE = 0 as *mut FILE;
    let mut histsize: i32 = 0 as i32;
    if secure_allow((1 as i32) << 4 as i32) == 0
        || histfile_modified() as u64 == 0
    {
        return;
    }
    histname = histfile_name(LFALSE);
    if histname.is_null() {
        return;
    }
    tempname = make_tempname(histname);
    fout = fopen(tempname, b"w\0" as *const u8 as *const c_char);
    if !fout.is_null() {
        make_file_private(fout);
        if let Ok(s) = lgetenv("LESSHISTSIZE") {
            histsize = s.parse::<i32>().unwrap_or(0);
        }
        if histsize <= 0 as i32 {
            histsize = 100 as i32;
        }
        skip_search = mlist_size(&mut mlist_search) - histsize;
        skip_shell = mlist_size(&mut mlist_shell) - histsize;
        fprintf(
            fout,
            b"%s\n\0" as *const u8 as *const c_char,
            b".less-history-file:\0" as *const u8 as *const c_char,
        );
        ctx.fout = fout;
        ctx.mlist = 0 as *mut mlist;
        read_cmdhist(
            Some(
                copy_hist
                    as unsafe extern "C" fn(
                        *mut c_void,
                        *mut mlist,
                        *const c_char,
                    ) -> (),
            ),
            &mut ctx as *mut save_ctx as *mut c_void,
            skip_search as lbool,
            skip_shell as lbool,
        );
        save_marks(fout, b".mark\0" as *const u8 as *const c_char);
        fclose(fout);
        rename(tempname, histname);
    }
    free(tempname as *mut c_void);
    free(histname as *mut c_void);
}
