use std::io::prelude::*;

mod parser;

fn main() {
    let text = std::fs::read_to_string(r"E:\Prototypes\CryV\natives.h") // TODO: Get path via launch arguments
        .unwrap();

    let data = parser::parse_native_file(text);

    let natives = data.expect("yep");

    let all_namespaces = natives.data.keys().collect::<Vec<&String>>();
    let mut file = std::fs::File::create("E:\\Prototypes\\CryV\\tmp\\mod.rs").unwrap();
    file.write_all(
        all_namespaces
            .into_iter()
            .map(|x| format!("pub mod {};\n", x.to_lowercase()))
            .collect::<String>()
            .as_bytes(),
    )
    .unwrap();

    for (namespace, natives) in &natives.data {
        let natives = natives
            .into_iter()
            .map(|x| format!("{}\n", x))
            .collect::<String>();

        let mut file = std::fs::File::create(format!(
            "E:\\Prototypes\\CryV\\tmp\\{}.rs",
            namespace.to_lowercase()
        ))
        .unwrap();
        file.write_all(format!("use crate::natives::NativeVector3;\n{}", natives).as_bytes())
            .unwrap();
    }
}
