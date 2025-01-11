use from_pest::FromPest;
use pest::Span;
use pest_ast::FromPest;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "notation.pest"]
pub struct ENotationParser;

// #[derive(Debug, FromPest)]
// #[pest_ast(rule(Rule::field))]
// pub struct Field {
//     #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
//     pub value: f64,
// }


fn span_into_str(span: Span) -> &str {
    span.as_str()
}

fn parse_bool(input: Span) -> Result<bool, ()> {
    match input.as_str() {
        "#t" => Ok(true),
        "#f" => Ok(false),
        _ => Err(()),
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::boolean))]
pub struct Boolean {
    #[pest_ast(outer(with(parse_bool), with(Result::unwrap)))]
    pub bool: bool,
}


fn parse_char(input: Span) -> Result<char, ()> {
    match input.as_str() {
        "#\\newline" => Ok('\n'),
        "#\\return" => Ok('\r'),
        "#\\space" => Ok(' '),
        "#\\tab" => Ok('\t'),
        _ => {
            let mut chars = input.as_str().chars();
            if chars.next() == Some('#') && chars.next() == Some('\\') {
                match chars.next() {
                    Some(c) => Ok(c),
                    None => Err(()),
                }
            } else {
                Err(())
            }
        }
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::char))]
pub struct Char {
    #[pest_ast(outer(with(parse_char), with(Result::unwrap)))]
    pub value: char,
}


// SIGN = { "+"|"-" }
// dec_int = { ASCII_DIGIT ~ ("_"? ~ ASCII_DIGIT)* }
// int = @{ SIGN? ~ dec_int }

fn parse_int(input: Span) -> Result<i64, ()> {
    let mut chars = input.as_str().chars();
    let sign = match chars.next() {
        Some('+') => 1,
        Some('-') => -1,
        Some(_) => {
            chars = input.as_str().chars();
            1
        }
        None => return Err(()),
    };
    let mut value = 0;
    for c in chars {
        if c == '_' {
            continue;
        }
        value = value * 10 + c.to_digit(10).ok_or(())? as i64;
    }
    Ok(sign * value)
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::int))]
pub struct Int {
    #[pest_ast(outer(with(parse_int), with(Result::unwrap)))]
    pub value: i64,
}


// // --------- rational -----------
// rational = @{ int ~ "/" ~ int }
// // --------- float ---------------
// float = @{
//     int ~ "." ~ dec_int
//   | SIGN? ~ "." ~ dec_int
// }


#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::rational))]
pub struct Rational {
    pub numerator: Int,
    pub denominator: Int,
}


#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::float))]
pub struct Float {
    #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
    pub float: f64,
}




fn main() {
}
