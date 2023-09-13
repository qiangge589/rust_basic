use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

// ANCHOR: token
#[derive(Debug, PartialEq)]
pub enum Token {
    // keywords
    And,    Break,  Do,     Else,   Elseif, End,
    False,  For,    Function,   Goto,   If, In,
    Local,  Nil,    Not,    Or,     Repeat, Return,
    Then,   True,   Until,  While,

// +        -       *       /       %       ^       #
    Add,    Sub,    Mul,    Div,    Mod,    Pow,    Len,
// &        ~       |       <<      >>      //
    BitAnd, BitXor, BitOr,  ShiftL, ShiftR, Idiv,
// ==           ~=         <=          >=         <       >             =
    Equal,     NotEq,     LesEq,     GreEq,     Less,     Greater,     Assign,
// (        )       {       }       [       ]       ::
    ParL,  ParR,   CurlyL, CurlyR, SqurL,  SqurR,  DoubColon,
// ;        :       ,       .       ..      ...
    SemiColon, Colon, Comma, Dot, Concat, Dots,
    // constant values
    Integer(i64),
    Float(f64),
    String(String),

    // Name of variables, functions, etc.
    Name(String),

    // End of stream
    Eos,
}
// ANCHOR_END: token

// ANCHOR: lex
#[derive(Debug)]
pub struct Lex {
    input: File,
}
// ANCHOR_END: lex

impl Lex {
    pub fn new(input: File) -> Self {
        Lex { input }
    }

    pub fn next(&mut self) -> Token {
        let ch = self.read_char();
        match ch {
            ' ' | '\r' | '\n' | '\t' => self.next(),
            '\0' => Token::Eos,

            '"' => { // literal String
                let mut s = String::new();
                loop {
                    match self.read_char() {
                        '\0' => panic!("unfinished literal string"),
                        '"' => break,
                        ch => s.push(ch),
                    }
                }
                Token::String(s)
            }

            'A'..='Z' | 'a'..='z' | '_' => { // Name
                let mut name = String::new();
                name.push(ch);
                loop {
                    match self.read_char() {
                        '\0' => break,
                        '_' => name.push('_'),
                        ch if ch.is_alphanumeric() => name.push(ch),
                        _ => {
                            self.input.seek(SeekFrom::Current(-1)).unwrap();
                            break;
                        }
                    }
                }
                Token::Name(name)
            }

            _ => panic!("unexpected char: {ch}"),
        }
    }

    fn read_char(&mut self) -> char {
        let mut buf: [u8; 1] = [0];
        if self.input.read(&mut buf).unwrap() == 1 {
            buf[0] as char
        } else {
            '\0'
        }
    }
}
