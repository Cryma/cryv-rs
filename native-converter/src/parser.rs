use std::{collections::HashMap, fmt::Display};

use nom::character::{is_alphabetic, is_digit, is_hex_digit};
use nom::{alt, many_till, named, opt, peek, tag, take, take_till, take_while, take_while1, tuple};

macro_rules! byte_array_to_utf8_string {
    ($value:expr) => {
        String::from_utf8($value.to_vec()).unwrap()
    };
}

named!(word, take_while1!(is_alphanumeric_with_asterisk));
named!(const_optional<&[u8], Option<&[u8]>>, opt!(tag!("const ")));
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
    take_while!(is_alphanumeric_with_asterisk_and_underscore_and_space)
);
named!(hash, take_while!(is_hex_digit_with_prefix));

named!(parser<&[u8], (&[u8], &[u8], Option<&[u8]>, &[u8], &[u8], &[u8], &[u8], (Vec<(Vec<&[u8]>, &[u8])>, &[u8]), &[u8], &[u8], &[u8], Option<&[u8]>, Option<&[u8]>, &[u8], &[u8], &[u8], &[u8], &[u8], &[u8])>,
    tuple!(word, space, const_optional, word, space, native_name, skip_one, parameters, skip_one, skip_one, space, return_statement, space_optional, invoke_statement, skip_one, invoke_return_type, skip_one, skip_one, hash));

#[derive(Debug)]
pub struct Natives {
    pub data: HashMap<String, Vec<NativeEntry>>,
}

#[derive(Clone, Debug)]
pub struct NativeEntry {
    pub name: String,
    pub return_type: String,
    pub parameters: Vec<NativeParameter>,
    pub hash: String,
    pub does_return: bool,
}

#[derive(Clone, Debug)]
pub struct NativeParameter {
    pub name: String,
    pub return_type: String,
}

impl Display for NativeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parameter_parts: Vec<String> = vec![];
        let mut parameter_invoke_names: Vec<String> = vec![];

        let mut additional_cstring_lines: Vec<String> = vec![];

        for parameter in &self.parameters {
            let parameter_invoke_name = match parameter.return_type.as_str() {
                "const char*" | "char*" => {
                    additional_cstring_lines.push(format!(
                        "let {}_cstring = std::ffi::CString::new({}).unwrap();\n    ",
                        parameter.name, parameter.name
                    ));

                    format!("{}_cstring.as_ptr()", parameter.name)
                }
                _ => format!("{}", parameter.name),
            };

            parameter_parts.push(format!(
                "{}: {}",
                parameter.name,
                get_translated_type(&parameter.return_type)
            ));

            parameter_invoke_names.push(format!("{}", parameter_invoke_name));
        }

        let mut return_value_modifications: Vec<String> = vec![];

        match self.return_type.as_str() {
            "const char*" | "char*" => {
                return_value_modifications
                    .push("let cstr = unsafe { std::ffi::CStr::from_ptr(value) };".to_owned());
                return_value_modifications
                    .push("let value = cstr.to_str().unwrap().to_string();".to_owned());
            }
            _ => (),
        };

        write!(
            f,
            r"
pub fn {}({}) -> {} {{
    {}
    let value = native!({}, {}, native_parameters!({}));
    {}
    value
}}
",
            self.name.to_lowercase(),
            parameter_parts.join(", "),
            get_translated_type(&self.return_type),
            additional_cstring_lines.join("\r\n"),
            get_translated_native_invoke_type(&self.return_type),
            self.hash,
            format!("{}", parameter_invoke_names.join(", ")),
            return_value_modifications.join("\r\n"),
        )
    }
}

fn get_translated_type(t: &String) -> String {
    match t.as_str() {
        "void" => "()".to_owned(),
        "int" => "i32".to_owned(),
        "int*" => "*mut i32".to_owned(),
        "float" => "f32".to_owned(),
        "float*" => "*mut f32".to_owned(),
        "const char*" => "String".to_owned(),
        "char*" => "String".to_owned(),
        "BOOL" => "bool".to_owned(),
        "BOOL*" => "*mut bool".to_owned(),
        "Any" => "u32".to_owned(),
        "Any*" => "*mut u32".to_owned(),
        "Hash" => "u32".to_owned(),
        "Hash*" => "*mut u32".to_owned(),
        "ScrHandle" => "u32".to_owned(),
        "ScrHandle*" => "*mut u32".to_owned(),
        "Cam" => "i32".to_owned(),
        "Player" => "i32".to_owned(),
        "Ped" => "i32".to_owned(),
        "Ped*" => "*mut i32".to_owned(),
        "Entity" => "i32".to_owned(),
        "Entity*" => "*mut i32".to_owned(),
        "Pickup" => "i32".to_owned(),
        "FireId" => "i32".to_owned(),
        "Vehicle" => "i32".to_owned(),
        "Vehicle*" => "*mut i32".to_owned(),
        "Blip" => "i32".to_owned(),
        "Blip*" => "*mut i32".to_owned(),
        "Interior" => "i32".to_owned(),
        "Object" => "i32".to_owned(),
        "Object*" => "*mut i32".to_owned(),
        "Vector3" => "NativeVector3".to_owned(),
        "Vector3*" => "*mut NativeVector3".to_owned(),
        _ => panic!("Could not find translation for type: {}", t),
    }
}

fn get_translated_native_invoke_type(t: &String) -> String {
    match t.as_str() {
        "const char*" => "*const i8".to_owned(),
        "char*" => "*const i8".to_owned(),
        _ => get_translated_type(t),
    }
}

fn map_native_lines<'a>(lines: &'a Vec<&str>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut natives: HashMap<&str, Vec<&str>> = HashMap::new();

    let mut current_namespace: Option<&str> = None;
    for line in lines {
        let line = line.trim();

        if line.starts_with("namespace") {
            current_namespace = line.split(" ").last();
        }

        if let Some(namespace) = current_namespace {
            if line.starts_with("static") {
                match natives.get_mut(namespace) {
                    Some(list) => list.push(line),
                    None => {
                        natives.insert(namespace, vec![line]);
                    }
                };
            }
        }
    }

    natives
}

pub fn parse_native_file<'a>(file_content: String) -> Option<Natives> {
    let lines = file_content.split("\r\n").collect();
    let native_map = map_native_lines(&lines);

    let mut native_data: HashMap<String, Vec<NativeEntry>> = HashMap::new();

    for (namespace, native_lines) in native_map {
        let mut native_entries: Vec<NativeEntry> = vec![];

        for native_line in native_lines {
            let result = parser(native_line.as_bytes());

            match result {
                Ok((
                    _input,
                    (
                        _space_modifier,
                        _,
                        optional_const,
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
                        _invoke_statement,
                        _,
                        _,
                        _,
                        _,
                        hash,
                    ),
                )) => {
                    let mut native_parameters: Vec<NativeParameter> = vec![];

                    for (parameter_part, _) in parameters {
                        let mut value = parameter_part
                            .into_iter()
                            .map(|x| byte_array_to_utf8_string!(x))
                            .collect::<Vec<String>>();

                        let name = sanitize_keywords(value.last().unwrap().clone());

                        value.truncate(value.len().saturating_sub(1));
                        let return_type = value.join("").trim().to_owned();

                        native_parameters.push(NativeParameter { name, return_type });
                    }

                    let main_return_type = if let Some(_) = optional_const {
                        format!("const {}", byte_array_to_utf8_string!(return_type))
                    } else {
                        byte_array_to_utf8_string!(return_type)
                    };

                    native_entries.push(NativeEntry {
                        name: byte_array_to_utf8_string!(name),
                        return_type: main_return_type.trim().to_owned(),
                        hash: byte_array_to_utf8_string!(hash),
                        parameters: native_parameters,
                        does_return: return_statement.is_some(),
                    });
                }
                Err(error) => match error {
                    nom::Err::Incomplete(_) => {
                        println!("ERR::INCOMPLETE: {}", native_line);

                        return None;
                    }
                    nom::Err::Error((err, kind)) => {
                        println!(
                            "ERR::ERROR - {:#?} -  {:#?}",
                            kind,
                            byte_array_to_utf8_string!(err)
                        );

                        return None;
                    }
                    nom::Err::Failure(_) => {
                        println!("ERR::FAILURE");

                        return None;
                    }
                },
            };
        }

        native_data.insert(namespace.to_owned(), native_entries);
    }

    Some(Natives { data: native_data })
}

fn sanitize_keywords(value: String) -> String {
    match &value[..] {
        "type" => "type_esc".to_owned(),
        "loop" => "loop_esc".to_owned(),
        _ => value,
    }
}

fn is_alphabetic_with_asterisk_and_underscore(character: u8) -> bool {
    is_alphabetic(character) || character == 0x2A || character == 0x5F
}

fn is_alphabetic_with_asterisk_and_underscore_and_space(character: u8) -> bool {
    is_alphabetic_with_asterisk_and_underscore(character) || character == 0x20
}

fn is_alphanumeric_with_asterisk_and_underscore_and_space(character: u8) -> bool {
    is_alphabetic_with_asterisk_and_underscore_and_space(character) || is_digit(character)
}

fn is_hex_digit_with_prefix(character: u8) -> bool {
    is_hex_digit(character) || character == 0x78
}

fn is_alphanumeric_with_asterisk(character: u8) -> bool {
    is_alphabetic_with_asterisk_and_underscore(character) || is_digit(character)
}
