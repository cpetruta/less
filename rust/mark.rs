use crate::defs::*;
use crate::edit::edit_ifile;
use crate::ifile::{IFileHandle, IFileManager};
use ::c2rust_bitfields;
use std::io::{Read, Write};
use std::path::Path;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn free(_: *mut std::ffi::c_void);
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn postoa(_: POSITION, _: *mut std::ffi::c_char, _: std::ffi::c_int);
    fn get_ifile(
        filename: *const std::ffi::c_char,
        prev: *mut std::ffi::c_void,
    ) -> *mut std::ffi::c_void;
    fn lstrtoic(
        _: *const std::ffi::c_char,
        _: *mut *const std::ffi::c_char,
        _: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn lstrtoposc(
        _: *const std::ffi::c_char,
        _: *mut *const std::ffi::c_char,
        _: std::ffi::c_int,
    ) -> POSITION;
    fn save(s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn bell();
    fn ch_end_seek() -> std::ffi::c_int;
    fn ch_tell() -> POSITION;
    fn ch_getflags() -> std::ffi::c_int;
    fn lrealpath(path: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn prev_ifile(h: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn jump_loc(pos: POSITION, sline: std::ffi::c_int);
    fn error(fmt: *const std::ffi::c_char, parg: *mut PARG);
    fn get_scrpos(scrpos: *mut scrpos, where_0: std::ffi::c_int);
    static mut curr_ifile: Option<IFileHandle>;
    static mut sc_height: std::ffi::c_int;
    static mut jump_sline: std::ffi::c_int;
    static mut perma_marks: std::ffi::c_int;
}
pub type __off_t = std::ffi::c_long;
pub type __off64_t = std::ffi::c_long;
pub type off_t = __off_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::ffi::c_int,
    pub _IO_read_ptr: *mut std::ffi::c_char,
    pub _IO_read_end: *mut std::ffi::c_char,
    pub _IO_read_base: *mut std::ffi::c_char,
    pub _IO_write_base: *mut std::ffi::c_char,
    pub _IO_write_ptr: *mut std::ffi::c_char,
    pub _IO_write_end: *mut std::ffi::c_char,
    pub _IO_buf_base: *mut std::ffi::c_char,
    pub _IO_buf_end: *mut std::ffi::c_char,
    pub _IO_save_base: *mut std::ffi::c_char,
    pub _IO_backup_base: *mut std::ffi::c_char,
    pub _IO_save_end: *mut std::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "std::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [std::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: std::ffi::c_ushort,
    pub _vtable_offset: std::ffi::c_schar,
    pub _shortbuf: [std::ffi::c_char; 1],
    pub _lock: *mut std::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut std::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: std::ffi::c_int,
    pub _unused2: [std::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub struct scrpos {
    pub pos: POSITION,
    pub ln: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union parg {
    pub p_string: *const std::ffi::c_char,
    pub p_int: i32,
    pub p_linenum: LINENUM,
    pub p_char: u8,
}
pub type PARG = parg;

/*
 * The table of marks.
 * Each Mark is identified by a lowercase or uppercase letter.
 * The final one is lmark, for the "last Mark"; addressed by the apostrophe.
 */
const NMARKS: u32 = (2 * 26) + 2; /* a-z, A-Z, mousemark, lastmark */
const NUMARKS: u32 = (2 * 26) + 1; /* user marks (not lastmark) */
const MOUSEMARK: u32 = NMARKS - 2;
const LASTMARK: u32 = NMARKS - 1;

const CH_HELPFILE: i32 = 0o10;
#[no_mangle]
pub static mut marks_modified: bool = false;

/*
 * A mark is an ifile (input file) plus a position within the file.
 */
#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct Mark {
    /*
     * Normally m_ifile != IFILE_NULL and m_filename == NULL.
     * For restored marks we set m_filename instead of m_ifile
     * because we don't want to create an ifile until the
     * user explicitly requests the file (by name or mark).
     */
    pub m_letter: u8,                 /* Associated character */
    pub m_ifile: Option<IFileHandle>, /* Input file being marked */
    pub m_filename: Option<String>,   /* Name of the input file */
    pub m_scrpos: scrpos,             /* Position of the mark */
}

impl Mark {
    /*
     * Set m_ifile and clear m_filename.
     */
    unsafe extern "C" fn mark_set_ifile(&mut self, mut ifile: Option<IFileHandle>) {
        self.m_ifile = ifile;
        /* With m_ifile set, m_filename is no longer needed. */
        self.m_filename = None;
    }

    /*
     * Populate the m_ifile member of a Mark struct from m_filename.
     */
    unsafe extern "C" fn mark_get_ifile(&mut self, ifiles: &mut IFileManager) {
        if self.m_ifile.is_some() {
            return;
        }
        let prev_ifile = ifiles.prev_ifile(None);
        let ifile = ifiles.get_ifile(self.m_filename.clone().unwrap(), prev_ifile);
        self.mark_set_ifile(Some(ifile));
    }

    /*
     * Initialize a Mark struct.
     */
    #[no_mangle]
    unsafe extern "C" fn cmark(
        &mut self,
        mut ifile: Option<IFileHandle>,
        mut pos: POSITION,
        mut ln: i32,
    ) {
        self.m_ifile = ifile;
        self.m_scrpos.pos = pos;
        self.m_scrpos.ln = ln;
        self.m_filename = None;
    }
}

impl Default for Mark {
    fn default() -> Self {
        Mark {
            m_letter: 0,
            m_ifile: None,
            m_filename: None,
            m_scrpos: scrpos { pos: 0, ln: 0 },
        }
    }
}

pub struct Marks {
    marks: Vec<Mark>,
}

impl Marks {
    pub fn new() -> Self {
        let mut marks: Vec<Mark> = Vec::with_capacity(NMARKS as usize);
        for _ in 0..NMARKS {
            marks.push(Mark::default());
        }
        Marks { marks: marks }
    }

    /*
     * Initialize the Mark table to show no marks are set.
     */
    pub unsafe fn init(&mut self) {
        for i in 0..NMARKS {
            let mut letter: u8;
            match i {
                MOUSEMARK => {
                    letter = b'#';
                }
                LASTMARK => {
                    letter = b'\'';
                }
                _ => {
                    letter = if i < 26 {
                        b'a' + i as u8
                    } else {
                        b'A' + i as u8 - 26
                    }
                }
            }
            self.marks[i as usize].m_letter = letter;
            self.marks[i as usize].cmark(None, -1, -1);
        }
    }

    /*
     * Return the user Mark struct identified by a character.
     */
    unsafe extern "C" fn getumark<'a>(&'a mut self, c: u8) -> Option<&'a mut Mark> {
        let mut parg: PARG = parg {
            p_string: 0 as *const std::ffi::c_char,
        };
        match c {
            b'a'..=b'z' => return Some(&mut self.marks[(c - b'a') as usize]),
            b'A'..=b'Z' => return Some(&mut self.marks[(c - b'A' + 26) as usize]),
            b'\'' => return Some(&mut self.marks[LASTMARK as usize]),
            b'#' => return Some(&mut self.marks[MOUSEMARK as usize]),
            _ => return None,
        }
        parg.p_char = c;
        error(
            b"Invalid Mark letter %c\0" as *const u8 as *const std::ffi::c_char,
            &mut parg,
        );
        None
    }

    /*
     * Get the Mark structure identified by a character.
     * The Mark struct may either be in the Mark table (user Mark)
     * or may be constructed on the fly for certain characters like ^, $.
     */
    unsafe extern "C" fn getmark<'a>(&'a mut self, mut c: u8) -> Option<&'a mut Mark> {
        static mut sm: Mark = Mark {
            m_letter: 0,
            m_ifile: None,
            m_filename: None,
            m_scrpos: scrpos { pos: 0, ln: 0 },
        };
        let mut m = &mut sm;
        match c {
            b'^' => {
                /*
                 * Beginning of the current file.
                 */
                m = &mut sm;
                m.cmark(curr_ifile, 0, 0);
            }
            b'$' => {
                /*
                 * End of the current file.
                 */
                if ch_end_seek() != 0 {
                    error(
                        b"Cannot seek to end of file\0" as *const u8 as *const std::ffi::c_char,
                        0 as *mut std::ffi::c_void as *mut PARG,
                    );
                    return None;
                }
                m = &mut sm;
                m.cmark(curr_ifile, ch_tell(), sc_height);
            }
            b'.' => {
                /*
                 * Current position in the current file.
                 */
                m = &mut sm;
                get_scrpos(&mut m.m_scrpos, 0);
                m.cmark(curr_ifile, m.m_scrpos.pos, m.m_scrpos.ln);
            }
            b'\'' => {
                /*
                 * The "last Mark".
                 */
                m = &mut self.marks[LASTMARK as usize];
            }
            _ => {
                /*
                 * Must be a user-defined Mark.
                 */
                if let Some(m) = self.getumark(c) {
                    if m.m_scrpos.pos == -1 {
                        error(
                            b"Mark not set\0" as *const u8 as *const std::ffi::c_char,
                            0 as *mut std::ffi::c_void as *mut PARG,
                        );
                        return None;
                    }
                }
            }
        }
        Some(m)
    }

    /*
     * Is a Mark letter invalid?
     */
    #[no_mangle]
    pub unsafe extern "C" fn badmark(&mut self, mut c: u8) -> bool {
        self.getmark(c).is_none()
    }

    /*
     * Set a user-defined Mark.
     */
    #[no_mangle]
    pub unsafe extern "C" fn setmark(&mut self, mut c: u8, mut where_0: i32) {
        let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
        if let Some(m) = self.getumark(c) {
            get_scrpos(&mut scrpos, where_0);
            if scrpos.pos == -1 {
                bell();
                return;
            }
            m.cmark(curr_ifile, scrpos.pos, scrpos.ln);
            marks_modified = true;
        }
    }

    /*
     * Clear a user-defined Mark.
     */
    #[no_mangle]
    pub unsafe extern "C" fn clrmark(&mut self, mut c: u8) {
        if let Some(m) = self.getumark(c) {
            if m.m_scrpos.pos == -1 {
                bell();
                return;
            }
            m.m_scrpos.pos = -1;
            marks_modified = true;
        }
    }

    /*
     * Set lmark (the Mark named by the apostrophe).
     */
    #[no_mangle]
    pub unsafe extern "C" fn lastmark(&mut self) {
        let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
        if ch_getflags() & CH_HELPFILE != 0 {
            return;
        }
        get_scrpos(&mut scrpos, 0);
        if scrpos.pos == -1 {
            return;
        }
        self.marks[LASTMARK as usize].cmark(curr_ifile, scrpos.pos, scrpos.ln);
        marks_modified = true;
    }

    /*
     * Go to a Mark.
     */
    #[no_mangle]
    pub unsafe extern "C" fn gomark(&mut self, ifiles: &mut IFileManager, mut c: u8) {
        let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
        if let Some(m) = self.getmark(c) {
            /*
             * If we're trying to go to the lastmark and
             * it has not been set to anything yet,
             * set it to the beginning of the current file.
             * {{ Couldn't we instead set marks[LASTMARK] in edit()? }}
             */
            if c == b'\'' && m.m_scrpos.pos == -1 {
                m.cmark(curr_ifile, 0, jump_sline);
            }
            m.mark_get_ifile(ifiles);

            /* Save scrpos; if it's LASTMARK it could change in edit_ifile. */
            scrpos = m.m_scrpos;
            if m.m_ifile != curr_ifile {
                /*
                 * Not in the current file; edit the correct file.
                 */
                if edit_ifile(ifiles, m.m_ifile) != 0 {
                    return;
                }
            }
            jump_loc(scrpos.pos, scrpos.ln);
        }
    }

    /*
     * Return the position associated with a given Mark letter.
     *
     * We don't return which screen line the position
     * is associated with, but this doesn't matter much,
     * because it's always the first non-blank line on the screen.
     */
    #[no_mangle]
    pub unsafe extern "C" fn markpos(&mut self, mut c: u8) -> POSITION {
        if let Some(ref mut m) = self.getmark(c) {
            if m.m_ifile != curr_ifile {
                error(
                    b"Mark not in current file\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void as *mut PARG,
                );
                return -1;
            }
            return m.m_scrpos.pos;
        } else {
            return -1;
        }
    }

    /*
     * Return the Mark associated with a given position, if any.
     */
    #[no_mangle]
    pub unsafe extern "C" fn posmark(&self, mut pos: POSITION) -> u8 {
        /* Only user marks */
        for i in 0..NUMARKS {
            if self.marks[i as usize].m_ifile == curr_ifile
                && self.marks[i as usize].m_scrpos.pos == pos
            {
                if i < 26 {
                    return b'a' + i as u8;
                }
                if i < 26 * 2 {
                    return b'A' + (i as u8 - 26);
                }
                return b'#';
            }
        }
        0
    }

    /*
     * Clear the marks associated with a specified ifile.
     */
    #[no_mangle]
    pub unsafe extern "C" fn unmark(&mut self, ifile: IFileHandle) {
        for i in 0..NMARKS {
            if self.marks[i as usize].m_ifile == Some(ifile) {
                self.marks[i as usize].m_scrpos.pos = NULL_POSITION;
            }
        }
    }

    /*
     * Check if any marks refer to a specified ifile vi m_filename
     * rather than m_ifile.
     */
    #[no_mangle]
    pub unsafe extern "C" fn mark_check_ifile(
        &mut self,
        ifiles: &mut IFileManager,
        ifile: Option<IFileHandle>,
    ) {
        let mut filename = ifiles.get_real_filename(ifile);
        for i in 0..NMARKS {
            let mut m = &mut self.marks[i as usize];
            let mut mark_filename = m.m_filename.clone();
            if let Some(mut mark_filename) = mark_filename {
                mark_filename = std::fs::canonicalize(&mark_filename)
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                if filename == Some(Path::new(&mark_filename)) {
                    m.mark_set_ifile(ifile);
                }
            }
        }
    }

    /*
     * Save marks to history file.
     */
    #[no_mangle]
    pub unsafe extern "C" fn save_marks(
        &mut self,
        ifiles: &mut IFileManager,
        fout: &mut std::fs::File,
        hdr: &str,
    ) {
        let mut i = 0;
        if perma_marks == 0 {
            return;
        }
        writeln!(fout, "{}", hdr);
        for i in 0..NMARKS {
            let mut filename;
            let mut m = &self.marks[i as usize];
            let mut pos_str: [i8; 23] = [0; 23];
            if !m.m_scrpos.pos == NULL_POSITION {
                let pos_str = m.m_scrpos.pos.to_string();
                filename = m.m_filename.clone();
                if filename.is_none() {
                    // FIXME ugly!!!
                    filename = Some(
                        ifiles
                            .get_real_filename(m.m_ifile)
                            .unwrap()
                            .as_os_str()
                            .to_string_lossy()
                            .to_string(),
                    );
                }
                if filename != Some("-".to_string()) {
                    writeln!(
                        fout,
                        "m {} {} {} {}",
                        m.m_letter,
                        m.m_scrpos.ln,
                        pos_str,
                        filename.unwrap(),
                    );
                }
            }
        }
    }

    fn to_int<'a>(s: &'a [u8]) -> (Option<i64>, &'a [u8]) {
        let mut i = 0;
        if s[i] == b'-' {
            i += 1;
        }

        let start_digits = i;
        while i < s.len() && s[i].is_ascii_digit() {
            i += 1;
        }

        if i == start_digits {
            return (None, s);
        }

        let (num, rest) = s.split_at(i);
        (str::from_utf8(num).ok().unwrap().parse().ok(), rest)
    }

    /*
     * Restore one Mark from the history file.
     */
    #[no_mangle]
    pub unsafe extern "C" fn restore_mark<'a>(&mut self, mut line: &'a [u8]) -> &'a [u8] {
        let mut m: Option<&mut Mark>;
        let mut ln = 0;
        let mut pos: POSITION = 0;
        let mut curr_pos = 0;

        if line[curr_pos] != b'm' {
            return &line[curr_pos + 1..];
        }
        // Skip whitespaces
        while line[curr_pos] == b' ' {
            curr_pos += 1;
        }
        m = self.getumark(line[curr_pos]);
        curr_pos += 1;
        if m.is_none() {
            return &line[curr_pos..];
        }
        // Skip whitespaces
        while line[curr_pos] == b' ' {
            curr_pos += 1;
        }
        let (l, mut line) = Self::to_int(&line[curr_pos..]);
        match l {
            Some(l) => ln = l,
            None => ln = -1,
        }
        curr_pos = 0;
        if ln < 0 {
            return &line;
        }
        if ln < 1 {
            ln = 1;
        }
        if ln > sc_height.into() {
            ln = sc_height as i64;
        }
        // Skip whitespaces
        while line[curr_pos] == b' ' {
            curr_pos += 1;
        }
        let (p, mut line) = Self::to_int(&line[curr_pos..]);
        match p {
            Some(p) => pos = p,
            None => pos = -1,
        }
        if pos < 0 {
            return &line;
        }
        curr_pos = 0;
        // Skip whitespaces
        while line[curr_pos] == b' ' {
            curr_pos += 1;
        }
        if let Some(m) = m {
            m.cmark(None, pos, ln as i32);
            let mut s = String::new();
            line.read_to_string(&mut s);
            m.m_filename = Some(s);
        }
        &line[curr_pos..]
    }
}
