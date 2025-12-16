use crate::charset::{prchar, prutfchar};
use crate::defs::*;
use crate::filename::{last_component, shell_quote};
use crate::forwback::eof_displayed;
use crate::ifile::{IFileHandle, IFileManager};
use crate::linenum::currline;
use std::sync::LazyLock;

extern "C" {
    fn free(_: *mut std::ffi::c_void);
    fn strcpy(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn postoa(_: POSITION, _: *mut std::ffi::c_char, _: std::ffi::c_int);
    fn linenumtoa(_: LINENUM, _: *mut std::ffi::c_char, _: std::ffi::c_int);
    fn inttoa(_: std::ffi::c_int, _: *mut std::ffi::c_char, _: std::ffi::c_int);
    fn save(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn ch_length() -> POSITION;
    fn ch_getflags() -> std::ffi::c_int;
    fn put_wchar(pp: *mut *mut std::ffi::c_char, ch: LWCHAR);
    fn step_charc(
        pp: *mut *const std::ffi::c_char,
        dir: std::ffi::c_int,
        limit: *const std::ffi::c_char,
    ) -> LWCHAR;
    fn find_linenum(pos: POSITION) -> LINENUM;
    fn vlinenum(linenum: LINENUM) -> LINENUM;
    fn percentage(num: POSITION, den: POSITION) -> std::ffi::c_int;
    fn position(sindex: std::ffi::c_int) -> POSITION;
    fn sindex_from_sline(sline: std::ffi::c_int) -> std::ffi::c_int;
    fn ntags() -> std::ffi::c_int;
    fn curr_tag() -> std::ffi::c_int;
    static mut pr_type: std::ffi::c_int;
    static mut new_file: bool;
    static mut linenums: std::ffi::c_int;
    static mut hshift: std::ffi::c_int;
    static mut sc_height: std::ffi::c_int;
    static mut jump_sline: std::ffi::c_int;
    static mut less_is_more: std::ffi::c_int;
    static mut header_lines: std::ffi::c_int;
    static mut utf_mode: std::ffi::c_int;
    static mut curr_ifile: Option<IFileHandle>;
    static mut osc8_path: Option<String>;
    static mut editor: &'static str;
}
static mut s_proto: LazyLock<String> = LazyLock::new(|| unsafe {
    String::from("?n?f%f .?m(%T %i of %m) ..?e(END) ?x- Next\\: %x..%t")
});
static mut m_proto: LazyLock<String> = LazyLock::new(|| unsafe {
    String::from("?n?f%f .?m(%T %i of %m) ..?e(END) ?x- Next\\: %x.:?pB%pB\\%:byte %bB?s/%s...%t")
});
static mut M_proto: LazyLock<String> = LazyLock::new(|| unsafe {
    String::from("?f%f .?n?m(%T %i of %m) ..?ltlines %lt-%lb?L/%L. :byte %bB?s/%s. .?e(END) ?x- Next\\: %x.:?pB%pB\\%..%t")
});
static mut e_proto: LazyLock<String> = LazyLock::new(|| unsafe {
    String::from(
        "?f%f .?m(%T %i of %m) .?ltlines %lt-%lb?L/%L. .byte %bB?s/%s. ?e(END) :?pB%pB\\%..%t",
    )
});

static mut h_proto: LazyLock<String> = LazyLock::new(|| unsafe {
    String::from("HELP -- ?eEND -- Press g to see it again:Press RETURN for more., or q when done")
});
static mut w_proto: LazyLock<String> =
    LazyLock::new(|| unsafe { String::from("Waiting for data") });
pub static mut more_proto: LazyLock<String> = LazyLock::new(|| unsafe {
    String::from("--More--(?eEND ?x- Next\\: %x.:?pB%pB\\%:byte %bB?s/%s...%t)")
});

#[no_mangle]
pub static mut prproto: [String; 3] = [String::new(), String::new(), String::new()];
#[no_mangle]
pub static mut eqproto: LazyLock<String> = LazyLock::new(|| unsafe { e_proto.clone() });
#[no_mangle]
pub static mut hproto: LazyLock<String> = LazyLock::new(|| unsafe { h_proto.clone() });
#[no_mangle]
pub static mut wproto: LazyLock<String> = LazyLock::new(|| unsafe { w_proto.clone() });
static mut message: String = String::new();
static mut mp: *mut std::ffi::c_char = 0 as *const std::ffi::c_char as *mut std::ffi::c_char;

/*
 * Initialize the prompt prototype strings.
 */
#[no_mangle]
pub unsafe extern "C" fn init_prompt() {
    prproto[0] = s_proto.clone();
    prproto[1] = if less_is_more != 0 {
        more_proto.clone()
    } else {
        m_proto.clone()
    };
    prproto[2] = M_proto.clone();
    *eqproto = e_proto.clone();
    *hproto = h_proto.clone();
    *wproto = w_proto.clone();
}

/*
 * Append a string to the end of the message.
 * nprt means the character *may* be nonprintable
 * and should be converted to printable form.
 */
unsafe extern "C" fn ap_estr(s: &str, nprt: bool) {
    while let Some(ch) = s.chars().next() {
        if nprt {
            if utf_mode != 0 {
                message.push_str(&prutfchar(ch));
            } else {
                message.push_str(&prchar(ch));
            };
        } else {
            // TODO do we need range check?
            message.push(ch);
        }
    }
}
unsafe extern "C" fn ap_str(s: &str) {
    ap_estr(s, false);
}

/*
 * Append a character to the end of the message.
 */
unsafe extern "C" fn ap_char(c: u8) {
    message.push(c as char);
}

/*
 * Append a POSITION (as a decimal integer) to the end of the message.
 */
unsafe extern "C" fn ap_pos(mut pos: POSITION) {
    ap_str(&pos.to_string());
}

unsafe extern "C" fn ap_linenum(mut linenum: LINENUM) {
    ap_str(&linenum.to_string());
}

/*
 * Append an integer to the end of the message.
 */
unsafe extern "C" fn ap_int(num: i32) {
    ap_str(&num.to_string());
}

unsafe extern "C" fn ap_quest() {
    ap_str("?");
}
unsafe extern "C" fn curr_byte(mut wh: std::ffi::c_int) -> POSITION {
    let mut pos: POSITION = 0;
    pos = position(wh);
    while pos == -(1 as std::ffi::c_int) as POSITION
        && wh >= 0 as std::ffi::c_int
        && wh < sc_height - 1 as std::ffi::c_int
    {
        wh += 1;
        pos = position(wh);
    }
    if pos == NULL_POSITION {
        pos = ch_length();
    }
    return pos;
}

/*
 * Return the value of a prototype conditional.
 * A prototype string may include conditionals which consist of a
 * question mark followed by a single letter.
 * Here we decode that letter and return the appropriate boolean value.
 */
unsafe extern "C" fn cond(ifiles: &mut IFileManager, c: u8, wh: i32) -> bool {
    let mut len: POSITION = 0;
    match c {
        /* Anything in the message yet? */
        //b'a' => return (mp > message.as_mut_ptr())
        b'a' => return message.len() > 0,
        b'b' => {
            /* Current byte offset known? */
            return (curr_byte(wh) != NULL_POSITION)
        }
        b'c' => return hshift != 0,
        /* Current byte offset known? */
        b'e' => return eof_displayed(false),
        b'f' | b'g' => {
            /* Filename known? */
            return ifiles.get_filename(curr_ifile).unwrap().to_str() != Some("-");
        }
          b'l' /* Line number known? */
        | b'd' /* Same as l */
         => {
            if linenums == 0 {
                return false;
            }
            return currline(wh) != 0;
        }
          b'L' /* Final line number known? */
        | b'D' /* Final page number known? */
        => {
            return linenums != 0 && ch_length() != NULL_POSITION;
        }
        b'm' => {
            /* More than one file? */
            return if ntags() != 0 {
                ntags() > 1
            } else {
                ifiles.nifile() > 1
            }
        }
        b'n' => {
            /* First prompt in a new file? */
            return (if ntags() != 0 {
                true
            } else if new_file {
                true
            } else {
                false
            })
        }
        b'p' => {
         /* Percent into file (bytes) known? */
            return curr_byte(wh) != NULL_POSITION
                && ch_length() > 0;
        }
        b'P' => {
            /* Percent into file (lines) known? */
            return currline(wh) != 0
                && {
                    len = ch_length();
                    len > 0
                }
                && find_linenum(len) != 0
        }
        b's' | b'B' => {
            /* Size of file known? */
            return ch_length() != NULL_POSITION;
        }
        b'x' => {
            /* Is there a "next" file? */
            if ntags() != 0 {
                return false;
            }
            return ifiles.next_ifile(curr_ifile).is_some()
        }
        _ => {}
    }
    return false;
}

fn page_num(linenum: LINENUM, height: i32, head_lines: i32) -> i64 {
    (linenum - 1) / (height - head_lines - 1) as i64 + 1
}

/*
 * Decode a "percent" prototype character.
 * A prototype string may include various "percent" escapes;
 * that is, a percent sign followed by a single letter.
 * Here we decode that letter and take the appropriate action,
 * usually by appending something to the message being built.
 */
unsafe extern "C" fn protochar(ifiles: &mut IFileManager, c: u8, wh: i32) {
    let mut pos: POSITION = 0;
    let mut len: POSITION = 0;
    let mut linenum: LINENUM = 0;
    let mut last_linenum: LINENUM = 0;
    let mut h: Option<IFileHandle>;
    let mut s = String::new();
    match c {
        b'b' => {
            /* Current byte offset */
            pos = curr_byte(wh);
            if pos != NULL_POSITION {
                ap_pos(pos);
            } else {
                ap_quest();
            }
        }
        b'c' => {
            ap_int(hshift);
        }
        b'd' => {
            /* Current page number */
            linenum = currline(wh);
            if linenum > 0 && sc_height > header_lines + 1 {
                ap_linenum(page_num(linenum, sc_height, header_lines));
            } else {
                ap_quest();
            }
        }
        b'D' => {
            /* Final page number */
            /* Find the page number of the last byte in the file (len-1). */
            len = ch_length();
            if len == NULL_POSITION {
                ap_quest();
            } else if len == 0 {
                /* An empty file has no pages. */
                ap_linenum(0);
            } else {
                linenum = find_linenum(len - 1);
                if linenum <= 0 {
                    ap_quest();
                } else {
                    ap_linenum(page_num(linenum, sc_height, header_lines));
                }
            }
        }
        b'E' => {
            /* Editor name */
            ap_str(editor);
        }
        b'f' => {
            /* File name */
            ap_estr(
                ifiles.get_filename(curr_ifile).unwrap().to_str().unwrap(),
                true,
            );
        }
        b'F' => {
            /* Last component of file name */
            ap_estr(
                last_component(ifiles.get_filename(curr_ifile).unwrap()),
                true,
            );
        }
        b'g' => {
            /* Shell escaped file name */
            let f_name = ifiles.get_filename(curr_ifile);
            s = shell_quote(f_name.unwrap().to_str().unwrap()).expect("cannot shell quote");
            ap_str(&s);
        }
        b'i' => {
            /* Index into list of files */
            if ntags() != 0 {
                ap_int(curr_tag());
            } else {
                ap_int(ifiles.get_index(curr_ifile).unwrap());
            }
        }
        b'l' => {
            /* Current line number */
            linenum = currline(wh);
            if linenum != 0 {
                ap_linenum(vlinenum(linenum));
            } else {
                ap_quest();
            }
        }
        b'L' => {
            /* Final line number */
            len = ch_length();
            if len == NULL_POSITION || len == 0 || {
                linenum = find_linenum(len);
                linenum <= 0
            } {
                ap_quest();
            } else {
                ap_linenum(vlinenum(linenum - 1));
            }
        }
        b'm' => {
            /* Number of files */
            let mut n = ntags();
            if n != 0 {
                ap_int(n);
            } else {
                ap_int(ifiles.nifile() as i32);
            }
        }
        b'o' => {
            /* path (URI without protocol) of selected OSC8 link */
            if let Some(ref path) = osc8_path {
                ap_str(&path);
            } else {
                ap_quest();
            }
        }
        b'p' => {
            /* Percent into file (bytes) */
            pos = curr_byte(wh);
            len = ch_length();
            if pos != NULL_POSITION && len > 0 {
                ap_int(percentage(pos, len));
            } else {
                ap_quest();
            }
        }
        b'P' => {
            /* Percent into file (lines) */
            linenum = currline(wh);
            if linenum == 0
                || {
                    len = ch_length();
                    len == NULL_POSITION
                }
                || len == 0
                || {
                    last_linenum = find_linenum(len);
                    last_linenum <= 0
                }
            {
                ap_quest();
            } else {
                ap_int(percentage(linenum, last_linenum));
            }
        }
        b's' | b'B' => {
            /* Size of file */
            len = ch_length();
            if len != -(1 as std::ffi::c_int) as POSITION {
                ap_pos(len);
            } else {
                ap_quest();
            }
        }
        b't' => {
            /* Truncate trailing spaces in the message */
            message = message.trim_end().to_string();
        }
        b'T' => {
            /* Type of list */
            if ntags() != 0 {
                ap_str("tag");
            } else {
                ap_str("file");
            }
        }
        b'x' => {
            /* Name of next file */
            h = ifiles.next_ifile(curr_ifile);
            if h.is_some() {
                ap_str(ifiles.get_filename(h).unwrap().to_str().unwrap());
            } else {
                ap_quest();
            }
        }
        _ => {}
    };
}

/*
 * Skip a false conditional.
 * When a false condition is found (either a false IF or the ELSE part
 * of a true IF), this routine scans the prototype string to decide
 * where to resume parsing the string.
 * We must keep track of nested IFs and skip them properly.
 */
unsafe extern "C" fn skipcond(p: &str) -> &str {
    /*
     * We came in here after processing a ? or :,
     * so we start nested one level deep.
     */
    let mut iflevel = 1;
    loop {
        match p.chars().next() {
            Some('?') => {
                /*
                 * Start of a nested IF.
                 */
                iflevel += 1;
            }
            Some(':') => {
                /*
                 * Else.
                 * If this matches the IF we came in here with,
                 * then we're done.
                 */
                if iflevel == 1 {
                    return p;
                }
            }
            Some('.') => {
                /*
                 * Endif.
                 * If this matches the IF we came in here with,
                 * then we're done.
                 */
                iflevel -= 1;
                if iflevel == 0 {
                    return p;
                }
            }
            Some('\\') => {
                /*
                 * Backslash escapes the next character.
                 */
                _ = p.chars().next();
            }

            /*
             * Whoops.  Hit end of string.
             * This is a malformed conditional, but just treat it
             * as if all active conditionals ends here.
             */
            None => return p,
            _ => {}
        }
    }
}

/*
 * Decode a char that represents a position on the screen.
 */
unsafe extern "C" fn wherechar<'a>(p: &'a str, wp: &mut i32) -> &'a str {
    let ret = p;
    match p.chars().next() {
        Some('b') | Some('d') | Some('l') | Some('p') | Some('P') => match p.chars().next() {
            Some('t') => {
                *wp = TOP;
            }
            Some('m') => {
                *wp = MIDDLE;
            }
            Some('b') => {
                *wp = BOTTOM;
            }
            Some('B') => {
                *wp = BOTTOM_PLUS_ONE;
            }
            Some('j') => {
                *wp = sindex_from_sline(jump_sline);
            }
            _ => {
                *wp = 0;
            }
        },
        _ => {}
    }
    ret
}

/*
 * Construct a message based on a prototype string.
 */
#[no_mangle]
pub unsafe extern "C" fn pr_expand(ifiles: &mut IFileManager, proto: &str) -> String {
    let mut p = "";
    let mut c: u8 = 0;
    let mut wh = 0;

    let old_len = message.len();
    if proto.len() == 0 {
        return "".to_string();
    }
    let mut chars = proto.chars();
    while let Some(ch) = chars.next() {
        match ch {
            /* Backslash escapes the next character */
            '\\' => {
                if let Some(c) = chars.next() {
                    ap_char(c as u8);
                }
            }
            /* Conditional (IF) */
            '?' => {
                if let None = proto.chars().next() {
                    wh = 0;
                    p = wherechar(p, &mut wh);
                    if !cond(ifiles, c, wh) {
                        p = skipcond(p);
                    }
                }
            }
            /* ELSE */
            ':' => {
                p = skipcond(p);
            }
            /* ENDIF */
            '.' => {}
            /* Percent escape */
            '%' => {
                wh = 0;
                p = wherechar(p, &mut wh);
                protochar(ifiles, c, wh);
            }
            _ => {
                ap_char(ch as u8);
            }
        }
    }

    let new_len = message.len();
    if new_len == old_len {
        return String::from("");
    }
    message.clone()
}

/*
 * Return a message suitable for printing by the "=" command.
 */
#[no_mangle]
pub unsafe extern "C" fn eq_message(ifiles: &mut IFileManager) -> String {
    pr_expand(ifiles, &eqproto)
}

/*
 * Return a prompt.
 * This depends on the prompt type (SHORT, MEDIUM, LONG), etc.
 * If we can't come up with an appropriate prompt, return NULL
 * and the caller will prompt with a colon.
 */
#[no_mangle]
pub unsafe extern "C" fn pr_string(ifiles: &mut IFileManager) -> String {
    let ty = if less_is_more == 0 {
        pr_type
    } else if pr_type != 0 {
        0
    } else {
        1
    };
    let prompt = pr_expand(
        ifiles,
        if ch_getflags() & CH_HELPFILE != 0 {
            &hproto
        } else {
            &prproto[ty as usize]
        },
    );
    new_file = false;
    prompt
}

/*
 * Return a message suitable for printing while waiting in the F command.
 */
#[no_mangle]
pub unsafe extern "C" fn wait_message(ifiles: &mut IFileManager) -> String {
    pr_expand(ifiles, &wproto)
}
