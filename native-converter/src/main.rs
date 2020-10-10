use nom::character::{is_alphabetic, is_digit};
use nom::{alt, many_till, named, opt, peek, tag, take, take_till, take_while, take_while1, tuple};

extern crate nom;

macro_rules! byte_array_to_utf8_string {
    ($value:expr) => {
        String::from_utf8($value.to_vec()).unwrap()
    };
}

fn main() {
    let text =
        std::fs::read_to_string(r"E:\Prototypes\CryV\natives.h") // TODO: Get path via launch arguments
            .unwrap();
    let lines = text
        .split("\r\n")
        .map(|x| {
            if x.trim().starts_with("static") {
                return Some(x.trim());
            }

            return None;
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<&str>>();

    named!(word, take_while1!(is_alphanumeric_with_asterisk));
    named!(space, take_while1!(|x| x == b' '));
    named!(space_optional<&[u8], Option<&[u8]>>, opt!(take_while1!(|x| x == b' ')));
    named!(native_name, take_till!(|x| x == b'('));
    named!(skip_one, take!(1));
    named!(parameters<&[u8], (Vec<(Vec<&[u8]>, &[u8])>, &[u8])>, many_till!(
        many_till!(
            alt!(
                take_while1!(|x| x == b' ') | take_while!(is_alphanumeric_with_asterisk)
            ),
            alt!(
                tag!(", ") | peek!(tag!(")"))
            )
        ),
        tag!(")")
    ));
    named!(return_statement<&[u8], Option<&[u8]>>, opt!(tag!("return")));
    named!(invoke_statement, tag!("invoke"));
    named!(
        invoke_return_type,
        take_while!(is_alphabetic_with_asterisk_and_underscore_and_space)
    );
    named!(hash, take_while!(is_hex_digit_with_prefix));

    named!(parser<&[u8], (&[u8], &[u8], &[u8], &[u8], &[u8], &[u8], (Vec<(Vec<&[u8]>, &[u8])>, &[u8]), &[u8], &[u8], &[u8], Option<&[u8]>, Option<&[u8]>, &[u8], &[u8], &[u8], &[u8], &[u8], &[u8])>,
        tuple!(word, space, word, space, native_name, skip_one, parameters, skip_one, skip_one, space, return_statement, space_optional, invoke_statement, skip_one, invoke_return_type, skip_one, skip_one, hash));

    for line in lines {
        let result = parser(line.as_bytes());

        match result {
            Ok((
                input,
                (
                    space_modifier,
                    _,
                    return_type,
                    _,
                    name,
                    _,
                    (parameters, _),
                    _,
                    _,
                    _,
                    return_statement,
                    _,
                    invoke_statement,
                    _,
                    invoke_return_type,
                    _,
                    _,
                    hash,
                ),
            )) => {
                println!("{:#?}", byte_array_to_utf8_string!(input));
                println!("{:#?}", byte_array_to_utf8_string!(space_modifier));
                println!("{:#?}", byte_array_to_utf8_string!(return_type));
                println!("{:#?}", byte_array_to_utf8_string!(name));

                for (parameter_part, _) in parameters {
                    let value = parameter_part
                        .into_iter()
                        .map(|x| byte_array_to_utf8_string!(x))
                        .collect::<Vec<String>>()
                        .join("");

                    println!("    {:#?}", value);
                }

                match return_statement {
                    Some(value) => println!("{:#?}", byte_array_to_utf8_string!(value)),
                    None => println!("{:#?}", "no return"),
                };

                println!("{:#?}", byte_array_to_utf8_string!(invoke_statement));
                println!("{:#?}", byte_array_to_utf8_string!(invoke_return_type));
                println!("{:#?}", byte_array_to_utf8_string!(hash));
            }
            Err(error) => match error {
                nom::Err::Incomplete(_) => {
                    println!("ERR::INCOMPLETE");

                    return;
                }
                nom::Err::Error((err, kind)) => {
                    println!("ERR::ERROR - {:#?} -  {:#?}", kind, byte_array_to_utf8_string!(err));

                    return;
                }
                nom::Err::Failure(_) => {
                    println!("ERR::FAILURE");

                    return;
                }
            },
        };
    }
}

fn is_alphabetic_with_asterisk_and_underscore(character: u8) -> bool {
    is_alphabetic(character) || character == 0x2A || character == 0x5F
}

fn is_alphabetic_with_asterisk_and_underscore_and_space(character: u8) -> bool {
    is_alphabetic_with_asterisk_and_underscore(character) || character == 0x20
}

fn is_hex_digit_with_prefix(character: u8) -> bool {
    is_digit(character) || character == 0x78
}

fn is_alphanumeric_with_asterisk(character: u8) -> bool {
    is_alphabetic_with_asterisk_and_underscore(character) || is_digit(character)
}
