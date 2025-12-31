use crate::decode::lgetenv;
use bitflags::bitflags;
use crate::defs::*;
use crate::util::str_to_int;
use std::ffi::CString;

extern "C" {
    fn sprintf(_: *mut std::ffi::c_char, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn snprintf(
        _: *mut std::ffi::c_char,
        _: std::ffi::c_ulong,
        _: *const std::ffi::c_char,
        _: ...
    ) -> std::ffi::c_int;
    fn free(_: *mut std::ffi::c_void);
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn lstrtoic(
        _: *const std::ffi::c_char,
        _: *mut *const std::ffi::c_char,
        _: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn save(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut std::ffi::c_void;
    fn skipspc(s: *const std::ffi::c_char) -> *const std::ffi::c_char;
    fn ch_getflags() -> std::ffi::c_int;
    fn prchar(c: LWCHAR) -> *const std::ffi::c_char;
    fn screen_trashed();
    fn ungetcc_end_command();
    fn ungetsc(s: *const std::ffi::c_char);
    fn isnullenv(s: *const std::ffi::c_char) -> lbool;
    fn findopt(c: std::ffi::c_int) -> *mut loption;
    fn findopt_name(
        p_optname: *mut *const std::ffi::c_char,
        p_oname: *mut *const std::ffi::c_char,
        p_ambig: *mut lbool,
    ) -> *mut loption;
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn repaint_hilite(on: lbool);
    fn chg_hilite();
    static mut less_is_more: std::ffi::c_int;
    static mut quit_at_eof: std::ffi::c_int;
    static mut every_first_cmd: *mut std::ffi::c_char;
    static mut opt_use_backslash: std::ffi::c_int;
    static mut ctldisp: std::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union parg {
    pub p_string: Option<String>,
    pub p_int: i32,
    pub p_linenum: LINENUM,
    pub p_char: u8,
}
pub type PARG = parg;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct optname {
    /// List of synonymus long (GNU-style) option names
    oname: Vec<String>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LOption {
    /// The controlling letter (a-z)
    opt_char: char,
    /// Long (GNU-style) option name
    name: Option<&'static str>,
    /// Type of the option
    opt_type: OptType,
    flags: OptFlags,
    /// Default value
    default: i32,
    /// Associated variable
    ovar: Option<&'static mut i32>,
    /// Special handling function
    pub handler: Option<fn(OptAction, &str)>,
    /// Description fore each value
    pub odesc: [&'static str; 3],
    kind: OptType,
}

bitflags! {
    struct OptFlags: u32 {
        const UNSUPPORTED = 0x01;
        const NO_TOGGLE   = 0x02;
        const NO_QUERY    = 0x04;
        const REPAINT     = 0x08;
        const HL_REPAINT  = 0x10;
    }
}

#[derive(Copy, Clone, PartialEq)]
enum OptType {
    Bool,
    Triple,
    String,
    Number,
}

#[derive(Copy, Clone)]
enum OptAction {
    Query,
    Toggle,
}

#[derive(Copy, Clone)]
enum InitState {
    Init,
}

#[derive(Copy, Clone, PartialEq)]
enum ToggleHow {
    NoToggle,
    Toggle,
    Unset,
    Set,
}

/// Global state
struct ScanContext {
    pending: Option<&'static LOption>,
    plus_option: bool,
    less_is_more: bool,
}

struct ConvertError;

static mut pendopt: Option<String> = None;
pub static mut plusoption: bool = false;
/// Max length of a long option name
const OPTNAME_MAX: usize = 32;
/// Invalid option letter
const OLETTER_NONE: u8 = b'\1';

/// Return a printable description of an option
#[no_mangle]
unsafe extern "C" fn opt_desc(o: loption) -> &str {
    if o.oletter == OLETTER_NONE {
        format!("--{}", o.onames[0].oname);
    } else {
        format!("-{} (--{})", o.oletter, o.onames[0].oname)
    }
}

/// Return a string suitable for printing as the "name" of an option.
/// For example, if the option letter is 'x', just return "-x".
#[no_mangle]
pub unsafe extern "C" fn propt(c: u8) -> &str {
    format!("-{}", c)
}

/// Scan an argument (either from the command line or from the
/// LESS environment variable) and process it.
#[no_mangle]
unsafe fn scan_option(ctx: &mut ScanContext, mut s: &str, is_env: bool) {
    if s.is_empty() {
        return;
    }

    /* Handle pending option argument */
    if let Some(opt) = ctx.pending.take() {
        if !opt.flags.contains(OptFlags::UNSUPPORTED) {
            match opt.opt_type {
                OptType::String => {
                    if let Some(handler) = opt.handler {
                        opt.handler(InitState::Init, Some(s.to_string()));
                    }
                }
                OptType::Number => {
                    if let Some(val) = opt.value {
                        *val = getnumc(s, printopt, None);
                    }
                }
                _ => {}
            }
        }
        return;
    }

    let mut set_default = false;
    let mut opt_name: Option<&str> = None;

    while !s.is_empty() {
        let (c, rest) = s.split_at(1);
        let mut optc = c.chars().next().unwrap();
        s = rest;

        match optc {
            ' ' | '\t' => continue,

            '-' => {
                if s.starts_with('-') {
                    opt_name = Some(&s[1..]);
                }
                set_default = s.starts_with('+');
                if set_default {
                    s = &s[1..];
                }
                if opt_name.is_some() {
                    continue;
                }
            }

            '+' => {
                ctx.plus_option = true;
                let (cmd, rest) = extract_string(s);
                s = rest;

                if cmd.starts_with('+') {
                    every_first_cmd = &cmd[1..].to_string();
                }
                continue;
            }

            '0'..='9' => {
                s = c.to_owned().as_str();
                optc = 'z';
            }

            'n' if ctx.less_is_more => {
                optc = 'z';
            }

            _ => {}
        }

        /* Lookup option */
        let (opt, print_name) = if let Some(name) = opt_name.take() {
            match find_option_by_name(name) {
                Some(o) => (o, name),
                None => {
                    error(b"No such option" as *const u8 as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void as *mut PARG);
                    return;
                }
            }
        } else {
            match find_option_by_char(optc) {
                Some(o) => (o, optc.to_string().as_str()),
                None => {
                    error(b"No such option" as *const u8 as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void as *mut PARG);
                    return;
                }
            }
        };

        let mut arg: Option<String> = None;

        match opt.opt_type {
            OptType::Bool => {
                if let Some(v) = opt.value {
                    *v = if set_default { opt.default } else { !opt.default };
                }
            }

            OptType::Triple => {
                if let Some(v) = opt.value {
                    if set_default {
                        *v = opt.default;
                    } else if is_env && opt.opt_char == 'r' {
                        *v = 2;
                    } else {
                        *v = flip_triple(opt.default, optc.is_lowercase());
                    }
                }
            }

            OptType::String => {
                if s.is_empty() {
                    ctx.pending = Some(opt);
                    return;
                }
                let (val, rest) = optstring(s, printopt, o.odesc[1]);
                arg = Some(val);
                s = rest;
            }

            OptType::Number => {
                if s.is_empty() {
                    ctx.pending = Some(opt);
                    return;
                }
                if let Some(v) = opt.value {
                    *v = parse_number(s);
                }
            }
        }

        if let Some(handler) = opt.handler {
            handler(InitState::Init, arg);
        }
    }
}

/// Toggle command line flags from within the program.
/// Used by the "-" and "_" commands.
/// how_toggle may be:
///      OPT_NO_TOGGLE   just report the current setting, without changing it.
///      OPT_TOGGLE      invert the current setting
///      OPT_UNSET       set to the default value
///      OPT_SET         set to the inverse of the default value
#[no_mangle]
fn toggle_option(
    o: Option<&LOption>,
    lower: bool,
    mut s: &str,
    mut how_toggle: ToggleHow,
    no_prompt: bool,
) {
    let o = match o {
        Some(opt) => opt,
        None => {
            error(
                b"No such option\0"
                as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG
            );
            return;
        }
    };

    if how_toggle == ToggleHow::Toggle && o.flags.contains(OptFlags::NO_TOGGLE) {
        error("Cannot change the %s option", Some(opt_desc(o)));
        return;
    }

    if how_toggle == ToggleHow::NoToggle && o.flags.contains(OptFlags::NO_QUERY) {
        error("Cannot query the %s option", Some(opt_desc(o)));
        return;
    }

    /*
     * Detect fake toggle for string/number options
     *
	 * Check for something which appears to be a do_toggle
	 * (because the "-" command was used), but really is not.
	 * This could be a string option with no string, or
	 * a number option with no number.
     */
    match o.kind {
        OptType::String | OptType::Number => {
            if how_toggle == ToggleHow::Toggle && s.is_empty() {
                how_toggle = ToggleHow::NoToggle;
            }
        }
        _ => {}
    }

    /*
     * Actually change the option
     */
    if how_toggle != ToggleHow::NoToggle {
        match o.kind {
            OptType::Bool => {
                if let Some(v) = o.ovar {
                    match how_toggle {
                        ToggleHow::Toggle => *v = !*v,
                        ToggleHow::Unset  => *v = o.default,
                        ToggleHow::Set    => *v = !o.default,
                        _ => {}
                    }
                }
            }

            OptType::Triple => {
                if let Some(v) = o.ovar {
                    match how_toggle {
                        ToggleHow::Toggle => *v = flip_triple(*v, lower),
                        ToggleHow::Unset  => *v = o.default,
                        ToggleHow::Set    => *v = flip_triple(o.default, lower),
                        _ => {}
                    }
                }
            }

            OptType::String => {
                match how_toggle {
                    ToggleHow::Set | ToggleHow::Unset => {
                        error(
                            b"Cannot use \"-+\" or \"-!\" for a string option\0"
                            as *const u8 as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void as *mut PARG
                        );
                        return;
                    }
                    _ => {}
                }
            }

            OptType::Number => {
                if let Some(v) = o.ovar {
                    match how_toggle {
                        ToggleHow::Toggle => {
                            let mut err = false;
                            if let Some(num) = getnumc(&mut s, None, Some(&mut err)) {
                                *v = num as i32;
                            }
                        }
                        ToggleHow::Unset => *v = o.default,
                        ToggleHow::Set => {
                            error(b"Can't use \"-!\" for a numeric option\0"
                                as *const u8 as *const std::ffi::c_char,
                                0 as *mut std::ffi::c_void as *mut PARG);
                            return;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /*
     * Call option handler
     */
    if let Some(handler) = o.handler {
        let action = if how_toggle == ToggleHow::NoToggle {
            OptAction::Query
        } else {
            OptAction::Toggle
        };
        handler(action, s);
    }

    /*
     * Print result
     */
    if !no_prompt {
        match o.kind {
            OptType::Bool | OptType::Triple => {
                if let Some(v) = o.ovar {
                    let s = CString::new(o.odesc[*v as usize]).unwrap().as_ptr() as *const u8 as *const std::ffi::c_char;
                    error(s, 0 as *mut std::ffi::c_void as *mut PARG);
                }
            }

            OptType::Number => {
                if let Some(v) = o.ovar {
                    let s = CString::new(o.odesc[1]).unwrap().as_ptr() as *const u8 as *const std::ffi::c_char;
                    error(s, 0 as *mut std::ffi::c_void as *mut PARG);
                }
            }

            OptType::String => {
                /* already printed */
            }
        }
    }

    if how_toggle != ToggleHow::NoToggle && o.flags.contains(OptFlags::REPAINT) {
        screen_trashed();
    }
}

/// "Toggle" a triple-valued option.
unsafe extern "C" fn flip_triple(val: i32, mut lc: bool) -> i32 {
    if lc {
        return if val == OPT_ON {
            OPT_OFF
        } else {
            OPT_ON
        };
    } else {
        return if val == OPT_ONPLUS {
            OPT_OFF
        } else {
            OPT_ONPLUS
        };
    };
}

///  Determine if an option takes a parameter.
#[no_mangle]
pub unsafe extern "C" fn opt_has_param(o: Option<&loption>) -> bool {
    if let Some(opt) = o {
        if o.otype & (O_BOOL | O_TRIPLE | O_NOVAR | O_NO_TOGGLE) != 0 {
            return false;
        }
    } else {
        return false;
    }
    return true;
}

/// Return the prompt to be used for a given option letter.
/// Only string and number valued options have prompts.
#[no_mangle]
pub unsafe extern "C" fn opt_prompt(o: Option<&loption>) -> Option<String> {
    if let Some(opt) = o {
        if opt.otype & (O_STRING | O_NUMBER) == 0 {
            return Some(String::from("?"));
        }
    } else {
        return Some(String::from("?"));
    }
    None
}

/// If the specified option can be toggled, return NULL.
/// Otherwise return an appropriate error message.
#[no_mangle]
pub unsafe extern "C" fn opt_toggle_disallowed(c: char) -> Option<&'static str> {
    if c == 'o' {
        if ch_getflags() & CH_CANSEEK != 0 {
            return Some("Input is not a pipe");
        }
    }
    None
}

///  Return whether or not there is a string option pending;
///  that is, if the previous option was a string-valued option letter
///  (like -P) without a following string.
///  In that case, the current option is taken to be the string for
///  the previous option.
#[no_mangle]
pub unsafe extern "C" fn isoptpending() -> bool {
    pendopt.is_some()
}

// Print error message about missing string.
unsafe extern "C" fn nostring(printopt: &str) {
    let mut parg: PARG = parg {
        p_string: None,
    };
    parg.p_string = Some(printopt.to_string());
    error(
        b"Value is required after %s\0" as *const u8 as *const std::ffi::c_char,
        &mut parg,
    );
}

// Printe error message if a STRING type option is not followed by a string
#[no_mangle]
pub unsafe extern "C" fn nopendopt() {
    nostring(opt_desc(pendopt));
}

/// Scan to end of string or to an END_OPTION_STRING character.
/// In the latter case, stop before the character.
/// Returns the remainder of the string (starting at the stop position),
/// or None on error.
///
/// `validchars` grammar:
///   "-" optional leading '-'
///   "." optional leading '.'
///   "d" one or more digits
///   "," comma-separated digit strings allowed
///   "s" space terminates the argument
unsafe fn optstring(
    s: &str,
    printopt: &str,
    validchars: Option<&str>,
) -> Option<(String, &str)> {
    if s.is_empty() {
        nostring(printopt);
        return None;
    }

    let mut out = String::with_capacity(s.len());
    let mut valid = validchars.unwrap_or("");
    let mut chars = s.char_indices().peekable();

    let mut end_index = s.len();

    while let Some((i, ch)) = chars.next() {
        /* Handle backslash escaping */
        let ch = if opt_use_backslash != 0 && ch == '\\' {
            if let Some((_, next)) = chars.next() {
                next
            } else {
                break;
            }
        } else {
            ch
        };

        /* Validate character */
        if !valid.is_empty() {
            if valid.starts_with('s') {
                if ch == ' ' {
                    end_index = i;
                    break;
                }
            } else if ch == '-' {
                if !valid.starts_with('-') {
                    end_index = i;
                    break;
                }
                valid = &valid[1..];
            } else if ch == '.' {
                if valid.starts_with('-') {
                    valid = &valid[1..];
                }
                if !valid.starts_with('.') {
                    end_index = i;
                    break;
                }
                valid = &valid[1..];
            } else if ch == ',' {
                if valid.len() < 2 || !valid.ends_with(',') {
                    end_index = i;
                    break;
                }
            } else if ch.is_ascii_digit() {
                while valid.starts_with('-') || valid.starts_with('.') {
                    valid = &valid[1..];
                }
                if !valid.starts_with('d') {
                    end_index = i;
                    break;
                }
            } else {
                end_index = i;
                break;
            }
        }

        /* END_OPTION_STRING handling */
        if ch == END_OPTION_STRING {
            end_index = i;
            break;
        }

        out.push(ch);
    }

    let remainder = &s[end_index..];
    Some((out, remainder))
}

unsafe fn num_error(printopt: Option<&str>, errp: Option<&mut bool>, overflow: bool) -> i32 {
    if let Some(err) = errp {
        *err = true;
        return -1;
    }

    if let Some(opt) = printopt {
        if overflow {
            error(b"Number too large in '%s'" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG);
        } else {
            error(b"Number number is required after %s" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void as *mut PARG);
        }
    }

    -1
}

/// Translate a string into a number.
/// Like atoi(), but takes a pointer to a char *, and updates
/// the char * to point after the translated number.
#[no_mangle]
fn getnumc(
    sp: &mut &str,
    printopt: Option<&str>,
    errp: Option<&mut bool>,
) -> Option<i64> {
    let mut s = sp.trim_start();

    let (num, rest) = str_to_int(s);
    // TODO check for overflow
    //if overflow {
    //    return num_error(printopt, errp, true);
    //}

    *sp = rest;

    if let Some(err) = errp {
        *err = false;
    }
    num
}

#[no_mangle]
fn getnum(
    sp: &mut &str,
    printopt: Option<&str>,
    errp: Option<&mut bool>,
) -> Option<i64> {
    getnumc(sp, printopt, errp)
}


/// Translate a string into a fraction, represented by the part of a
/// number which would follow a decimal point.
/// The value of the fraction is returned as parts per NUM_FRAC_DENOM.
/// That is, if "n" is returned, the fraction intended is n/NUM_FRAC_DENOM.
#[no_mangle]
unsafe fn getfraction(
    sp: &mut &str,
    printopt: Option<&str>,
    errp: Option<&mut bool>,
) -> i64 {
    let mut s = sp.trim_start();
    let mut frac: i64 = 0;
    let mut fraclen: usize = 0;

    let first = s.chars().next();
    if first.map_or(true, |c: char| !c.is_ascii_digit()) {
        return num_error(printopt, errp, false) as i64;
    }

    while let Some(c) = s.chars().next() {
        if !c.is_ascii_digit() {
            break;
        }

        if fraclen < NUM_LOG_FRAC_DENOM as usize {
            frac = frac * 10 + (c as i64 - '0' as i64);
            fraclen += 1;
        }

        s = &s[c.len_utf8()..];
    }

    while fraclen < NUM_LOG_FRAC_DENOM as usize {
        frac *= 10;
        fraclen += 1;
    }

    *sp = s;

    if let Some(err) = errp {
        *err = false;
    }

    frac
}

#[no_mangle]
pub unsafe extern "C" fn init_unsupport() {
    let mut ss = lgetenv("LESS_UNSUPPORT");
    if ss.is_err() {
        return;
    }
    let mut s = ss.unwrap().as_str();

    let mut chars = s.chars();
    loop {
        let mut opt: Option<LOption> = None;
        s = s.trim_start();
        if s.is_empty() {
            break;
        }
        if s == "-" {
            break;
        }
        if s starts_with("-") {
            s = s[1..];
            opt = findopt_name(&mut s, 0 as *mut *const std::ffi::c_char, 0 as *mut lbool);
        } else {
            opt = findopt(s.get(0));
            if !opt.is_null() {
                s = s[1..];
            }
        }
        if !opt.is_null() {
            opt.otype |= O_UNSUPPORTED;
        }
    }
}

// Get the value of the -e flag.
#[no_mangle]
pub unsafe extern "C" fn get_quit_at_eof() -> std::ffi::c_int {
    if less_is_more == 0 {
        return quit_at_eof;
    }
    // When less_is_more is set, the -e flag semantics are different.
    return if quit_at_eof != 0 {
        OPT_ONPLUS
    } else {
        OPT_ON
    };
}
