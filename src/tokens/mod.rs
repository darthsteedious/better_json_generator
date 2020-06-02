use std::ops::Deref;
use crate::tokens::Token::Whitespace;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Token {
    OpenParenthesis,
    CloseParenthesis,
    OpenCurlyBrace,
    CloseCurlyBrace,
    OpenSquareBrace,
    CloseSquareBrace,
    Character(char),
    Digit(u8),
    Quote,
    Comma,
    Colon,
    NegativeSign,
    Whitespace(char),
    Unknown(char),
}

impl Token {
    pub fn newline() -> Self {
        Token::Whitespace('\n')
    }

    pub fn space() -> Self {
        Token::Whitespace(' ')
    }

    pub fn tab() -> Self {
        Token::Whitespace('\t')
    }
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '(' => Token::OpenParenthesis,
            ')' => Token::CloseParenthesis,
            '{' => Token::OpenCurlyBrace,
            '}' => Token::CloseCurlyBrace,
            '[' => Token::OpenSquareBrace,
            ']' => Token::CloseSquareBrace,
            w @ ' ' => Token::Whitespace(w),
            n @ '\n' => Token::Whitespace(n),
            t @ '\t' => Token::Whitespace(t),
            d @ '0'..='9' => Token::Digit(d.to_digit(10).expect("should be a digit") as u8),
            ',' => Token::Comma,
            ':' => Token::Colon,
            '\"' => Token::Quote,
            '-' => Token::NegativeSign,
            l @ 'a'..='z' => Token::Character(l),
            u @ 'A'..='Z' => Token::Character(u),
            c => Token::Unknown(c)
        }
    }
}

impl From<&Token> for char {
    fn from(t: &Token) -> Self {
        match t {
            Token::OpenParenthesis => '(',
            Token::CloseParenthesis => ')',
            Token::OpenCurlyBrace => '{',
            Token::CloseCurlyBrace => '}',
            Token::OpenSquareBrace => '[',
            Token::CloseSquareBrace => ']',
            Token::Whitespace(ws) => *ws,
            Token::Digit(d) => *d as char,
            Token::Comma => ',',
            Token::Colon => ':',
            Token::Quote => '"',
            Token::NegativeSign => '-',
            Token::Character(c) => *c,
            Token::Unknown(u) => *u
        }
    }
}



pub fn process_str(s: &str) -> Vec<Token> {
    s.chars().into_iter().fold(Vec::new(), |mut v, c| {
        v.push(c.into());
        v
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    macro_rules! create_test {
        ($func:ident, $c:literal, $t:pat) => {
            #[test]
            fn $func() {
                let t: Token = $c.into();
                match t {
                    $t => assert!(true),
                    _ => assert!(false)
                }
            }
        }
    }

    create_test!(test_open_parens, '(', Token::OpenParenthesis);
    create_test!(test_close_parens, ')', Token::CloseParenthesis);
    create_test!(test_open_curly_brace, '{', Token::OpenCurlyBrace);
    create_test!(test_close_curly_brace, '}', Token::CloseCurlyBrace);
    create_test!(test_open_square_brace, '[', Token::OpenSquareBrace);
    create_test!(test_close_square_brace, ']', Token::CloseSquareBrace);
    create_test!(test_white_space, ' ', Token::Whitespace(' '));
    create_test!(test_newline, '\n', Token::Whitespace('\n'));
    create_test!(test_tab, '\t', Token::Whitespace('\t'));
    create_test!(test_colon, ':', Token::Colon);
    create_test!(test_comma, ',', Token::Comma);
    create_test!(test_neg_sign, '-', Token::NegativeSign);
    create_test!(test_digit_0, '0', Token::Digit(0));
    create_test!(test_digit_1, '1', Token::Digit(1));
    create_test!(test_digit_2, '2', Token::Digit(2));
    create_test!(test_digit_3, '3', Token::Digit(3));
    create_test!(test_digit_4, '4', Token::Digit(4));
    create_test!(test_digit_5, '5', Token::Digit(5));
    create_test!(test_digit_6, '6', Token::Digit(6));
    create_test!(test_digit_7, '7', Token::Digit(7));
    create_test!(test_digit_8, '8', Token::Digit(8));
    create_test!(test_digit_9, '9', Token::Digit(9));

    create_test!(test_char_a, 'a', Token::Character('a'));
    create_test!(test_char_b, 'b', Token::Character('b'));
    create_test!(test_char_c, 'c', Token::Character('c'));
    create_test!(test_char_d, 'd', Token::Character('d'));
    create_test!(test_char_e, 'e', Token::Character('e'));
    create_test!(test_char_f, 'f', Token::Character('f'));
    create_test!(test_char_g, 'g', Token::Character('g'));
    create_test!(test_char_h, 'h', Token::Character('h'));
    create_test!(test_char_i, 'i', Token::Character('i'));
    create_test!(test_char_j, 'j', Token::Character('j'));
    create_test!(test_char_k, 'k', Token::Character('k'));
    create_test!(test_char_l, 'l', Token::Character('l'));
    create_test!(test_char_m, 'm', Token::Character('m'));
    create_test!(test_char_n, 'n', Token::Character('n'));
    create_test!(test_char_o, 'o', Token::Character('o'));
    create_test!(test_char_p, 'p', Token::Character('p'));
    create_test!(test_char_q, 'q', Token::Character('q'));
    create_test!(test_char_r, 'r', Token::Character('r'));
    create_test!(test_char_s, 's', Token::Character('s'));
    create_test!(test_char_t, 't', Token::Character('t'));
    create_test!(test_char_u, 'u', Token::Character('u'));
    create_test!(test_char_v, 'v', Token::Character('v'));
    create_test!(test_char_w, 'w', Token::Character('w'));
    create_test!(test_char_x, 'x', Token::Character('x'));
    create_test!(test_char_y, 'y', Token::Character('y'));
    create_test!(test_char_z, 'z', Token::Character('z'));

    create_test!(test_char_upper_a, 'A', Token::Character('A'));
    create_test!(test_char_upper_b, 'B', Token::Character('B'));
    create_test!(test_char_upper_c, 'C', Token::Character('C'));
    create_test!(test_char_upper_d, 'D', Token::Character('D'));
    create_test!(test_char_upper_e, 'E', Token::Character('E'));
    create_test!(test_char_upper_f, 'F', Token::Character('F'));
    create_test!(test_char_upper_g, 'G', Token::Character('G'));
    create_test!(test_char_upper_h, 'H', Token::Character('H'));
    create_test!(test_char_upper_i, 'I', Token::Character('I'));
    create_test!(test_char_upper_j, 'J', Token::Character('J'));
    create_test!(test_char_upper_k, 'K', Token::Character('K'));
    create_test!(test_char_upper_l, 'L', Token::Character('L'));
    create_test!(test_char_upper_m, 'M', Token::Character('M'));
    create_test!(test_char_upper_n, 'N', Token::Character('N'));
    create_test!(test_char_upper_o, 'O', Token::Character('O'));
    create_test!(test_char_upper_p, 'P', Token::Character('P'));
    create_test!(test_char_upper_q, 'Q', Token::Character('Q'));
    create_test!(test_char_upper_r, 'R', Token::Character('R'));
    create_test!(test_char_upper_s, 'S', Token::Character('S'));
    create_test!(test_char_upper_t, 'T', Token::Character('T'));
    create_test!(test_char_upper_u, 'U', Token::Character('U'));
    create_test!(test_char_upper_v, 'V', Token::Character('V'));
    create_test!(test_char_upper_w, 'W', Token::Character('W'));
    create_test!(test_char_upper_x, 'X', Token::Character('X'));
    create_test!(test_char_upper_y, 'Y', Token::Character('Y'));
    create_test!(test_char_upper_z, 'Z', Token::Character('Z'));

    create_test!(test_ampersand_unk, '&', Token::Unknown('&'));
    create_test!(test_caret_unk, '^', Token::Unknown('^'));

    macro_rules! process_str_theory {
        ($name:ident, $input:expr, $($t:expr),+) => {
            #[test]
            fn $name() {
                assert_eq!(process_str($input).iter().cmp(vec![$($t),+].iter()), Ordering::Equal);
            }
        }
    }

    process_str_theory!(first_name_theory, "{\"firstName\": \"{{firstName()}}\"}", Token::OpenCurlyBrace,
            Token::Quote,
            Token::Character('f'),
            Token::Character('i'),
            Token::Character('r'),
            Token::Character('s'),
            Token::Character('t'),
            Token::Character('N'),
            Token::Character('a'),
            Token::Character('m'),
            Token::Character('e'),
            Token::Quote,
            Token::Colon,
            Token::Whitespace(' '),
            Token::Quote,
            Token::OpenCurlyBrace,
            Token::OpenCurlyBrace,
            Token::Character('f'),
            Token::Character('i'),
            Token::Character('r'),
            Token::Character('s'),
            Token::Character('t'),
            Token::Character('N'),
            Token::Character('a'),
            Token::Character('m'),
            Token::Character('e'),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::CloseCurlyBrace,
            Token::CloseCurlyBrace,
            Token::Quote,
            Token::CloseCurlyBrace);

    process_str_theory!(hello_world_theory, "{\"hello\": \"world\"}",
        Token::OpenCurlyBrace,
        Token::Quote,
        Token::Character('h'),
        Token::Character('e'),
        Token::Character('l'),
        Token::Character('l'),
        Token::Character('o'),
        Token::Quote,
        Token::Colon,
        Token::Whitespace(' '),
        Token::Quote,
        Token::Character('w'),
        Token::Character('o'),
        Token::Character('r'),
        Token::Character('l'),
        Token::Character('d'),
        Token::Quote,
        Token::CloseCurlyBrace);

    process_str_theory!(complex_tag_theory, "{\"x\": \"{{foo(true, -123, \"hello\")}}\"}",
        Token::OpenCurlyBrace,
        Token::Quote,
        Token::Character('x'),
        Token::Quote,
        Token::Colon,
        Token::Whitespace(' '),
        Token::Quote,
        Token::OpenCurlyBrace,
        Token::OpenCurlyBrace,
        Token::Character('f'),
        Token::Character('o'),
        Token::Character('o'),
        Token::OpenParenthesis,
        Token::Character('t'),
        Token::Character('r'),
        Token::Character('u'),
        Token::Character('e'),
        Token::Comma,
        Token::Whitespace(' '),
        Token::NegativeSign,
        Token::Digit(1),
        Token::Digit(2),
        Token::Digit(3),
        Token::Comma,
        Token::Whitespace(' '),
        Token::Quote,
        Token::Character('h'),
        Token::Character('e'),
        Token::Character('l'),
        Token::Character('l'),
        Token::Character('o'),
        Token::Quote,
        Token::CloseParenthesis,
        Token::CloseCurlyBrace,
        Token::CloseCurlyBrace,
        Token::Quote,
        Token::CloseCurlyBrace);
}