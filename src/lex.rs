use std::fs::File;
use std::io::{BufReader, Read, Seek};

pub enum ReservedWords {
    ID,
    NUM,
    END,
    ERROR,

}

const VAL_LEN: usize = 25;
const DEFAULT_MAP_SIZE: usize = 1024;
pub struct Lex {
    file_name: String,
    line_no: u32,
    br: BufReader<File>,
    id_lexeme: String,
    num_lexeme: String,

}
impl Lex {
    fn lexical(&self) -> i32 {
        for ch in self.br.bytes() {
            match ch {
                Ok(b'\0') => {
                    return ReservedWords::END as i32;
                },
                Ok(byte) => {
                    match byte {
                        b' ' | b'\t' => {
    
                        },
                        b'\n' => {
                            self.line_no += 1;
                        },
                        b'~' => {
                            loop {
                                let next_char = self.br.bytes().next();
                                if next_char.unwrap().unwrap() == b'\n' {
                                    break;
                                }
                            }
                        },
                        ch if ch.is_ascii_digit() => {
                            // read num into numLexeme
                            return ReservedWords::NUM as i32;
                        },
                        ch if ch.is_ascii_alphabetic() => {
                            // read id into lexeme
                        },
                        _ => panic!("ERROR: CANNOT MATCH CH"),
                    }
                },
                Err(err) => panic!("ERROR: {}", err),
            }
        }
    
        ReservedWords::END as i32
    }
    
    fn is_valid_token(id_lexeme: &str) -> i32 {
        if id_lexeme.starts_with('_') || id_lexeme.ends_with('_'){
            return ReservedWords::ERROR as i32;
        }

        let mut prev_char: Option<char> = None;
        for c in id_lexeme.chars() {
            if let Some(prev) = prev_char  {
                if prev == '_' && c == '_' {
                    return ReservedWords::ERROR as i32;
                }
            }
            prev_char = Some(c);
        }

        0
    }

    fn read_token_helper(ch: char) -> bool {
        ![' ', '\n', ';', '+', '-', '*', '/', '(', ')'].contains(&ch)
    }

    fn read_string(br: &mut BufReader<File>, id_lexeme: &mut Vec<u8>) {
        let mut buffer = vec![0u8; 1];
        while let Some(Ok(ch)) = br.bytes().next() {
            if (ch as char).is_alphabetic() || Self::read_token_helper(ch as char) {
                buffer[0] = ch;
                id_lexeme.extend_from_slice(&buffer);
            } else {
                break;
            }
        }
        if let Some(Ok(ch)) = br.bytes().next() {
            br.seek(std::io::SeekFrom::Current(-1)).unwrap();
        }
    }
    
    
}