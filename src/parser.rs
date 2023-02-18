pub enum parser {
    BEGIN(String),
    END(String),

}
const BEGIN_STR:String = String::from("BEGIN");
const END_STR:String = String::from("END.");


impl parser {
    fn start_func(lexan: &mut Lexer) {
        insert(BEGIN_STR, BEGIN);
        insert(END_STR, END);

        let mut lookahead = lexan.lexan();
        match_token(BEGIN);
        while lookahead != END && lookahead != DONE && lookahead != ERROR {
            assignment_stmt(&mut lookahead, lexan);
            if lookahead == END {
                match_token(END);
            }
        }
        if lookahead != ERROR {
            print_table();
        }
    }

    fn match_token(token: i32, lookahead: &mut i32) {
        if *lookahead == token {
            *lookahead = lexan.lexan();
        } else {
            if token == BEGIN {
                println!("ERROR in {} at Line:{} expected: '{}' found: '{}'", __func__, get_lineno(), BEGIN_STR, (*lookahead as u8) as char);
            } else if token == END {
                println!("ERROR in {} at Line:{} expected: '{}' found: '{}'", __func__, get_lineno(), BEGIN_STR, (*lookahead as u8) as char);
            } else {
                println!("ERROR in {} at Line:{} expected: '{}' found: '{}'", __func__, get_lineno(), (token as u8) as char, (*lookahead as u8) as char);
                error();
            }
            *lookahead = ERROR;
        }
    }

    fn assignment_stmt(lookahead: &mut i32, lexan: &mut Lexer) {
        match_token(ID, lookahead);
        if *lookahead != '=' as i32 {
            println!("ERROR in {} Line:{} Expected assignment op '=', got '{}'", __func__,  get_lineno(), (*lookahead as u8) as char);
            *lookahead = ERROR;
        } else {
            match_token(*lookahead, lookahead);
            expression(lookahead, lexan);
            match_token(';' as i32, lookahead);
        }
    }

    fn expression(lookahead: &mut i32, lexan: &mut Lexer) {
        term(lookahead, lexan);
        while *lookahead == '+' as i32 || *lookahead == '-' as i32 {
            match_token(*lookahead, lookahead);
            term(lookahead, lexan);
        }
    }

    fn term(lookahead: &mut i32, lexan: &mut Lexer) {
        factor(lookahead, lexan);
        while *lookahead == '*' as i32 || *lookahead == '/' as i32 {
            match_token(*lookahead, lookahead);
            factor(lookahead, lexan);
        }
    }

    fn factor(lookahead: &mut i32, lexan: &mut Lexer) {
        if *lookahead == ID {
            match_token(ID, lookahead);
        } else if *lookahead == NUM {
            match_token(NUM, lookahead);
        } else if *lookahead == '(' as i32 {
            match_token('(' as i32, lookahead);
            expression(lookahead, lexan);
            match_token(')' as i32, lookahead);
        } else {
            println!("ERROR in {} at Line:{}", __func__, get_lineno());
            *lookahead = ERROR;
        }
    }
}