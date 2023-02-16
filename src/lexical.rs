use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::str::{from_utf8};
use crate::syntax_error::SyntaxError;

const VAL_LEN: usize = 25;
const DEFAULT_MAP_SIZE: usize = 1024;
static mut FP: Option<BufReader<File>> = None;
static mut LINE_NO: i32 = 0;
static mut ID_LEXEME: [u8; VAL_LEN] = [0; VAL_LEN];
static mut NUM_LEXEME: [u8; VAL_LEN] = [0; VAL_LEN];

pub fn get_lineno() -> i32 {
    unsafe { LINE_NO }
}

fn open_file(file_name: &str) -> Result<BufReader<File>, std::io::Error> {
    let mut file = File::open(file_name)?; // ? operator immediately returns the error to the caller
    let mut temp = BufReader::new(file);
    unsafe { LINE_NO = 1 };
    Ok(temp)
}

fn read_token_helper(ch: char) -> bool {
    ![' ', '\n', ';', '+', '-', '*', '/', '(', ')'].contains(&ch)
}

unsafe fn read_string() {
    let mut i = 0;
    let mut buffer = [0u8; DEFAULT_MAP_SIZE];
    loop {
        let ch = FP.as_mut().unwrap().bytes().next().unwrap().unwrap(); //this smells bad.. think of a rust way to fix this
        if (ch as char).is_alphabetic() || read_token_helper(ch as char) && i < DEFAULT_MAP_SIZE {
            buffer[i] = ch;
            i += 1;
        } else {
            break;
        }
    }
    if i > 0 {
        i -= 1;
        let ch = FP.as_mut().unwrap().bytes().next().unwrap().unwrap();
        FP.as_mut().unwrap().seek(std::io::SeekFrom::Current(-1)).unwrap();
    }
    let buffer = &buffer[0..i];
    unsafe {
        ID_LEXEME.copy_from_slice(from_utf8(buffer).unwrap().as_bytes());
    }
}

fn is_valid_token(id_lexeme: &str) -> Result<(), SyntaxError> {
    if id_lexeme.starts_with('_') || id_lexeme.ends_with('_'){
        return Err(SyntaxError {
            message: format!("found: '{}'", id_lexeme),
            line_no: get_lineno(),
            function: "is_valid_token".to_string(),
        });
    }

    let mut prev_char: Option<char> = None;
    for c in id_lexeme.chars() {
        if let Some(prev) = prev_char  {
            if prev == '_' && c == '_' {
                return Err(SyntaxError {
                    message: format!("found: '{}'", id_lexeme),
                    line_no: get_lineno(),
                    function: "is_valid_token".to_string(),
                });
            }
        }
        prev_char = Some(c);
    }

    Ok(())
}

unsafe fn lexan() -> i32 {

    for ch in BufReader::new(FP).bytes() {
        let ch = ch.unwrap();
        if ch == b' ' || ch == b'\t' {
            // ignore whitespaces
        } else if ch == b'\n' {
            unsafe {
                LINE_NO += 1;
            }
        } else if ch == b'~' {
            // ignore comments - remove the comment
            loop {
                let next_ch = FP.bytes().next().unwrap().unwrap();
                if next_ch == b'\n' {
                    break;
                }
            }
        } else if ch.is_ascii_digit() {
            // read num into numLexeme
            // read_num();
            return NUM;
        } else if ch.is_ascii_alphabetic() {
            // read id into idLexeme
            read_string();
            if !is_valid_token(idLexeme) {
                return ERROR;
            }
            let idtype = lookup(idLexeme); // ensure null term
            if idtype == NOT_FOUND {
                insert(idLexeme, ID);
                return ID;
            } else {
                return idtype;
            }
        } else if ch == EOF {
            return DONE;
        } else {
            return ch as i32;
        }
    }
    0
}
