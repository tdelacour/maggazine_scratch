#[macro_use]
extern crate nom;

use nom::{IResult, Needed};

named!(parens, delimited!(char!('('), is_not!(")"), char!(')')));
named!(abcd_parser, tag!("abcd"));
named!(a_parser, take_while!(is_a));

fn is_a(i: u8) -> bool {
    i == b'a'
}

pub fn take4(i: &[u8]) -> IResult<&[u8], &[u8]> {
    if i.len() < 4 {
        IResult::Incomplete(Needed::Size(4))
    } else {
        IResult::Done(&i[4..], &i[0..4])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::*;
    use nom::Needed;

    #[test]
    fn paren_test() {
        assert_eq!(parens(&b"(hello)"[..]), Done(&b""[..], &b"hello"[..]));
        assert_eq!(parens(&b"(hello)123"[..]), Done(&b"123"[..], &b"hello"[..]));
    }

    #[test]
    fn take4_test() {
        assert_eq!(take4(&b"1234"[..]), Done(&b""[..], &b"1234"[..]));
        assert_eq!(take4(&b"12345"[..]), Done(&b"5"[..], &b"1234"[..]));
        assert_eq!(take4(&b"123"[..]), Incomplete(Needed::Size(4)));
    }

    #[test]
    fn abcd_parser_test() {
        assert_eq!(abcd_parser(&b"abcde"[..]), Done(&b"e"[..], &b"abcd"[..]));
        assert_eq!(abcd_parser(&b"abc"[..]), Incomplete(Needed::Size(4)));
    }

    #[test]
    fn a_parser_test() {
        assert_eq!(a_parser(&b"aaaaabcd"[..]), Done(&b"bcd"[..], &b"aaaaa"[..]));
        assert_eq!(a_parser(&b"baaaabcd"[..]), Done(&b"baaaabcd"[..], &b""[..]));
    }
}
