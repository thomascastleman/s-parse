
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
pub struct ParseResult<'a> {
    parsed: SExpr<'a>,
    rest: &'a str
}

// parse an s-expression from a string
pub fn s_parse(s : &str) -> SExpr {
    SExpr::SInt(1)  // temp debug
}

// parse a number from the given string slice
pub fn num_parse(s : &str) -> ParseResult {
    let mut num_end = 0;    // index at which number chars stop

    let chars = s.chars();
    let l = chars.clone().count();

    // scan to find end of number
    for (i, c) in chars.enumerate() {
        // if "numeric" char, read over
        if c.is_digit(10) || c == '.' || c == '-' {
            // if read to end of string, num ends at end
            if (i == l - 1) { num_end = l; }
            continue;
        } else {
            num_end = i;
            break;
        }
    }

    let num_slice = &s[..num_end];  // assume full string is number
    let rest_slice = &s[num_end..]; // rest of string past number

    // println!("num_end: {}", num_end);
    // println!("num_slice: \"{}\"", num_slice);
    // println!("rest_slice: \"{}\"", rest_slice);

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
                            if num_end == 0 { rest_slice } else { num_slice }),
            }
        }
    }
}

#[cfg(test)]
mod parse_tests {
    use super::*;
    use SExpr::*;

    #[test]
    fn parses_int() {
        assert_eq!(s_parse("3"), SInt(3));
        assert_eq!(s_parse("193755"), SInt(193755));
        assert_eq!(s_parse("0"), SInt(0));
        assert_eq!(s_parse("-1728"), SInt(-1728));
    }

    #[test]
    fn parses_float() {
        assert_eq!(s_parse("0.5"), SFloat(0.5));
        assert_eq!(s_parse("-11.28"), SFloat(-11.28));
        assert_eq!(s_parse("99.9"), SFloat(99.9));
        assert_eq!(s_parse("34587.23424"), SFloat(34587.23424));
    }

    #[test]
    fn parses_symbol() {
        assert_eq!(s_parse("my-symbol"), SSym("my-symbol"));
        assert_eq!(s_parse("x"), SSym("x"));
        assert_eq!(s_parse("NAME"), SSym("NAME"));
        assert_eq!(s_parse("e^2*x/y"), SSym("e^2*x/y"));
    }

    #[test]
    fn parses_str() {
        assert_eq!(s_parse("\"test\""), SStr("test"));
        assert_eq!(s_parse("\"this is a string\""), SStr("this is a string"));
        assert_eq!(s_parse("\"23847\""), SStr("23847"));
        assert_eq!(s_parse("\"(parens)\""), SStr("(parens)"));
    }

    #[test]
    fn parses_list() {
        assert_eq!(
            s_parse("(1 2 3)"), 
            SList(vec![SInt(1), SInt(2), SInt(3)]));
        assert_eq!(
            s_parse("(name)"), 
            SList(vec![SSym("name")]));
        assert_eq!(
            s_parse("(f \"arg\" 2 5)"), 
            SList(vec![SSym("f"), SStr("arg"), SInt(2), SInt(5)]));
    }
}