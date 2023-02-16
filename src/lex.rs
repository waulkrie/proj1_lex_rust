use std::fs::File;
use std::io::{BufReader, Read};

const VAL_LEN: usize = 25;
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
                Some(ch) => {

                },
                None => None,

            }
        }


        0
    }
}