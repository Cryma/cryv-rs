use std::{collections::HashMap, fmt::Display};

use nom::character::{is_alphabetic, is_digit, is_hex_digit};
use nom::{alt, many_till, named, opt, peek, tag, take, take_till, take_while, take_while1, tuple};

macro_rules! byte_array_to_utf8_string {
    ($value:expr) => {
        String::from_utf8($value.to_vec()).unwrap()
    };
}

// Parts for the natives parser
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

// Completed parser type for natives
named!(parser<&[u8], (&[u8], &[u8], Option<&[u8]>, &[u8], &[u8], &[u8], &[u8], (Vec<(Vec<&[u8]>, &[u8])>, &[u8]), &[u8], &[u8], &[u8], Option<&[u8]>, Option<&[u8]>, &[u8], &[u8], &[u8], &[u8], &[u8], &[u8])>,
    tuple!(word, space, const_optional, word, space, native_name, skip_one, parameters, skip_one, skip_one, space, return_statement, space_optional, invoke_statement, skip_one, invoke_return_type, skip_one, skip_one, hash));

#[derive(Debug)]
pub struct Natives {
    pub data: HashMap<String, Vec<NativeEntry>>,
}

#[derive(Debug)]
pub struct NativeEntry {
    pub name: String,
    pub return_type: String,
    pub parameters: Vec<NativeParameter>,
    pub hash: String,
    pub does_return: bool,
}

#[derive(Debug)]
pub struct NativeParameter {
    pub name: String,
    pub return_type: String,
}

impl Display for NativeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We use this method to convert a native entry to rust code
        let mut parameter_parts: Vec<String> = vec![];
        let mut parameter_invoke_names: Vec<String> = vec![];

        for parameter in &self.parameters {
            // Check if the parameter return type is some kind of char-pointer
            // They are special types and need to be handled in a specific way
            let parameter_invoke_name = match parameter.return_type.as_str() {
                "const char*" | "char*" => format!("{}.as_ptr()", parameter.name),
                _ => format!("{}", parameter.name),
            };

            // Build parameter signatures ("parameter_name: parameter_type")
            parameter_parts.push(format!(
                "{}: {}",
                parameter.name,
                get_translated_type(&parameter.return_type)
            ));

            parameter_invoke_names.push(format!("{}", parameter_invoke_name));
        }

        let mut return_value_modifications: Vec<String> = vec![];

        // Check if the return type for the native is some kind of char-pointer
        // They are special types and need to be handled in a specific way
        match self.return_type.as_str() {
            "const char*" | "char*" => {
                return_value_modifications
                    .push("let cstr = unsafe { std::ffi::CStr::from_ptr(value) };".to_owned());
                return_value_modifications
                    .push("let value = cstr.to_str().unwrap().to_string();".to_owned());
            }
            _ => (),
        };

        // Build the rust function for the native
        write!(
            f,
            r"
pub fn {}({}) -> {} {{
    let value = native!({}, {}, native_parameters!({}));
    {}
    value
}}
",
            self.name.to_lowercase(),
            parameter_parts.join(", "),
            get_translated_native_return_type(&self.return_type),
            // additional_cstring_lines.join("\r\n"),
            get_translated_native_invoke_type(&self.return_type),
            self.hash,
            format!("{}", parameter_invoke_names.join(", ")),
            return_value_modifications.join("\r\n"),
        )
    }
}

/// Translates the native type to the corresponding rust type
///
/// # Arguments
///
/// * `native_type` - The native type that will get translated
fn get_translated_type(native_type: &String) -> String {
    match native_type.as_str() {
        "void" => "()".to_owned(),
        "int" => "i32".to_owned(),
        "int*" => "*mut i32".to_owned(),
        "float" => "f32".to_owned(),
        "float*" => "*mut f32".to_owned(),
        "const char*" => "&std::ffi::CString".to_owned(),
        "char*" => "&std::ffi::CString".to_owned(),
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
        _ => panic!("Could not find translation for type: {}", native_type),
    }
}

/// Translates the native type to the corresponding rust type
/// This function treats special cases for native call return types
///
/// # Arguments
///
/// * `native_type` - The native type that will get translated
fn get_translated_native_invoke_type(native_type: &String) -> String {
    match native_type.as_str() {
        "const char*" => "*const i8".to_owned(),
        "char*" => "*const i8".to_owned(),
        _ => get_translated_type(native_type),
    }
}

/// Translates the native type to the corresponding rust type
/// This function treats special cases for native function return types
///
/// # Arguments
///
/// * `native_type` - The native type that will get translated
fn get_translated_native_return_type(native_type: &String) -> String {
    match native_type.as_str() {
        "const char*" | "char*" => "String".to_owned(),
        _ => get_translated_type(native_type),
    }
}

/// Converts the lines from the natives.h file to a HashMap with
/// the namespaces as the key and a vector of string slices as the value
///
///# Arguments
///
/// * `lines` - Content from the natives.h file in form of a vector of string slices
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
                    nom::Err::Error(err) => {
                        println!(
                            "ERR::ERROR - {:#?} -  {:#?}",
                            err,
                            byte_array_to_utf8_string!(err.input)
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
