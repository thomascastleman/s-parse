use regex::Regex;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum SExpr<'a> {
    SInt(i32),
    SFloat(f32),
    SSym(&'a str),
    SStr(&'a str),
    SList(Vec<SExpr<'a>>)
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct ParseResult<'a> {
    parsed: SExpr<'a>,
    rest: &'a str
}

// parse 1 or more s-expressions from the input string
pub fn parse(s: &str) -> Vec<SExpr> {
    vec![]
}

// parse an s-expression off the input
fn s_parse(s: &str) -> ParseResult {
    // decide how to parse from first char
    match s.chars().next() {
        Some(c) => {
            if c.is_digit(10) || c == '-' {
                num_parse(s)
            } else if c == '"' {
                str_parse(s)
            } else if c == '(' {
                list_parse(s)
            } else {
                sym_parse(s)
            }
        },
        None => panic!("s_parse: can't parse s-expr from empty input"),
    }
}

// determine slice from front of input string containing a particular token
fn read_until<F>(s: &str, stop_condition: F) -> (&str, &str) 
    where F: Fn(char) -> bool {

    let mut tok_end = 0;
    let len = s.len();

    // scan until stop condition met
    for (i, c) in s.chars().enumerate() {
        if !stop_condition(c) {
            // if read to end of input, end idx is str length
            if i == len - 1 { tok_end = len; }
            continue;
        } else {
            tok_end = i;
            break;
        }
    }

    // return (scanned token, rest of input)
    (&s[..tok_end], &s[tok_end..])
}

// parse a number off the input
fn num_parse(s: &str) -> ParseResult {
    // read until non-(digit/sign/decimal) char encountered
    let (num_slice, rest_slice) = read_until(s, |c| {
        !(c.is_digit(10) || c == '.' || c == '-')
    });

    // attempt to parse int, then try float on fail
    match num_slice.parse::<i32>() {
        Ok(int_val) => 
            ParseResult { 
                parsed: SExpr::SInt(int_val),
                rest: rest_slice
            },
        Err(_) => {
            match num_slice.parse::<f32>() {
                Ok(float_val) => 
                    ParseResult { 
                        parsed: SExpr::SFloat(float_val),
                        rest: rest_slice
                    },
                Err(_) => panic!("num_parse: expected numeric value, got: \"{}\"", 
                            if num_slice.is_empty() { rest_slice } else { num_slice }),
            }
        }
    }
}

// parse a symbol off the input
fn sym_parse(s: &str) -> ParseResult {
    // read chars until space/closing parenthesis encountered
    let (sym_slice, rest_slice) = read_until(s, |c| {
        c == ' ' || c == ')'
    });

    if sym_slice.is_empty() {
        panic!("sym_parse: expected symbol, got: \"{}\"", rest_slice);
    }

    ParseResult { parsed: SExpr::SSym(sym_slice), rest: rest_slice }
}

// parse a string literal within double quotes off the input
// this will NOT handle escaped quotes
fn str_parse(s: &str) -> ParseResult {
    // validate opening quote
    if !s.starts_with('"') {
        panic!("str_parse: expected double quote to start string, got: \"{}\"", s);
    }

    // read from past the beginning quote, up to the ending quote
    let (str_slice, rest_slice) = read_until(&s[1..], |c| {
        c == '"'
    });

    // validate closing quote
    if !rest_slice.starts_with('"') {
        panic!("str_parse: expected double quote to end string, got: \"{}\"", rest_slice);
    }

    // return parsed str and slice rest to ignore closing quote
    ParseResult { parsed: SExpr::SStr(str_slice), rest: &rest_slice[1..] }
}

// parse a list expression off the input
fn list_parse(s: &str) -> ParseResult {
    if !s.starts_with('(') {
        panic!("list_parse: expected opening parenthesis, got: \"{}\"", s);
    }

    let mut els = Vec::new();       // vector for accumulating list elements
    let mut el;                     // holder for each element
    let mut copy = &s[1..];         // eat the opening paren

    // parse elements of list, with arbitrary whitespace in between
    loop {
        copy = eat_whitespace(copy);

        el = s_parse(copy);     // parse S-expression element
        els.push(el.parsed);    // add to element list
        copy = el.rest;         // move to end of parsed input

        copy = eat_whitespace(copy);

        if copy.is_empty() {
            panic!("list_parse: unexpected end of list: no \
            closing parenthesis found at \"{}\"", s);
        }

        // on end of list, break
        if copy.starts_with(')') { break; }
    }

    copy = &copy[1..];  // eat the closing paren

    ParseResult { parsed: SExpr::SList(els), rest: copy }
}

// advance str slice past leading whitespace, return reduced str
fn eat_whitespace(mut s: &str) -> &str {
    lazy_static! {
        static ref WHITESPACE: Regex = Regex::new(r"\s").unwrap();
    }

    // predicate for detecting a whitespace char
    let ws = |c: char| {
        WHITESPACE.is_match(&c.to_string()) 
    };
    
    // eat arbitrary whitespace between elements
    while !s.is_empty() && s.starts_with(ws) {
        s = &s[1..];
    }

    return s;
}


#[cfg(test)]
mod parse_tests {
    use super::*;
    use SExpr::*;

    /*---------------- s_parse tests ----------------*/

    // #[test]
    // fn parses_int() {
    //     assert_eq!(s_parse("3"), SInt(3));
    //     assert_eq!(s_parse("193755"), SInt(193755));
    //     assert_eq!(s_parse("0"), SInt(0));
    //     assert_eq!(s_parse("-1728"), SInt(-1728));
    // }

    // #[test]
    // fn parses_float() {
    //     assert_eq!(s_parse("0.5"), SFloat(0.5));
    //     assert_eq!(s_parse("-11.28"), SFloat(-11.28));
    //     assert_eq!(s_parse("99.9"), SFloat(99.9));
    //     assert_eq!(s_parse("34587.23424"), SFloat(34587.23424));
    // }

    // #[test]
    // fn parses_symbol() {
    //     assert_eq!(s_parse("my-symbol"), SSym("my-symbol"));
    //     assert_eq!(s_parse("x"), SSym("x"));
    //     assert_eq!(s_parse("NAME"), SSym("NAME"));
    //     assert_eq!(s_parse("e^2*x/y"), SSym("e^2*x/y"));
    // }

    // #[test]
    // fn parses_str() {
    //     assert_eq!(s_parse("\"test\""), SStr("test"));
    //     assert_eq!(s_parse("\"this is a string\""), SStr("this is a string"));
    //     assert_eq!(s_parse("\"23847\""), SStr("23847"));
    //     assert_eq!(s_parse("\"(parens)\""), SStr("(parens)"));
    // }

    // #[test]
    // fn parses_list() {
    //     assert_eq!(
    //         s_parse("(1 2 3)"), 
    //         SList(vec![SInt(1), SInt(2), SInt(3)]));
    //     assert_eq!(
    //         s_parse("(name)"), 
    //         SList(vec![SSym("name")]));
    //     assert_eq!(
    //         s_parse("(f \"arg\" 2 5)"), 
    //         SList(vec![SSym("f"), SStr("arg"), SInt(2), SInt(5)]));
    // }

    /*---------------- tests for parsing helpers ----------------*/

    #[test]
    fn test_s_parse() {
        // s_parse can parse an expression of any type
        assert_eq!(
            s_parse("100.05"),
            ParseResult { parsed: SFloat(100.05), rest: "" });
        assert_eq!(
            s_parse("75"),
            ParseResult { parsed: SInt(75), rest: "" });
        assert_eq!(
            s_parse("symbol"),
            ParseResult { parsed: SSym("symbol"), rest: "" });
        assert_eq!(
            s_parse("\"string\""),
            ParseResult { parsed: SStr("string"), rest: "" });
        assert_eq!(
            s_parse("(list of els)"),
            ParseResult {
                parsed: SList(vec![SSym("list"), SSym("of"), SSym("els")]), 
                rest: "" });
    }

    #[test]
    fn test_num_parse() {
        // parsing numeric expressions
        assert_eq!(
            num_parse("-17.182 x y z)"),
            ParseResult { parsed: SFloat(-17.182), rest: " x y z)" });
        assert_eq!(
            num_parse("6)"),
            ParseResult { parsed: SInt(6), rest: ")" });
        assert_eq!(
            num_parse("100"),
            ParseResult { parsed: SInt(100), rest: "" });
    }

    #[test]
    fn test_sym_parse() {
        // parsing symbols
        assert_eq!(
            sym_parse("symbol-name/here next)"),
            ParseResult { parsed: SSym("symbol-name/here"), rest: " next)" });
        assert_eq!(
            sym_parse("name-with-nums1283)"),
            ParseResult { parsed: SSym("name-with-nums1283"), rest: ")" });
        assert_eq!(
            sym_parse("terminal"),
            ParseResult { parsed: SSym("terminal"), rest: "" });
    }

    #[test]
    fn test_str_parse() {
        // parsing strings
        assert_eq!(
            str_parse("\"string value inside!\""),
            ParseResult { parsed: SStr("string value inside!"), rest: "" });
        assert_eq!(
            str_parse("\"first\" next-sym)"),
            ParseResult { parsed: SStr("first"), rest: " next-sym)" });
        assert_eq!(
            str_parse("\"\" 1 5"),
            ParseResult { parsed: SStr(""), rest: " 1 5" });
    }

    #[test]
    fn test_list_parse() {
        // parsing list expressions
        assert_eq!(
            list_parse("(a 1 \"c\")"),
            ParseResult { parsed: SList(vec![SSym("a"), SInt(1), SStr("c")]), rest: "" });
        assert_eq!(
            list_parse("(name (list within list))"),
            ParseResult { 
                parsed: SList(vec![SSym("name"), SList(vec![SSym("list"), SSym("within"), SSym("list")])]), 
                rest: "" });
        assert_eq!(
            list_parse("( spacing     does not    matter  )"),
            ParseResult { 
                parsed: SList(vec![SSym("spacing"), SSym("does"), SSym("not"), SSym("matter")]), 
                rest: "" });
    }

}