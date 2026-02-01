use crate::config::{Build, Config, Package};
use std::{
    error::Error,
    fs::{self, create_dir},
};

pub fn new(
    name: String,
    language: String,
    cc: Option<String>,
    cxx: Option<String>,
    linker: String,
) -> Result<(), Box<dyn Error>> {
    let config = Config {
        package: Package {
            name: name.clone(),
            version: "0.1.0".to_string(),
            author: "".to_string(),
            license: "No License".to_string(),
            language: language.clone(),
        },
        build: Build {
            debug: true,
            linker,
            cc,
            cxx,
            cflags: None,
            cxxflags: None,
            lnflags: None,
            includes: None,
        },
        dependencies: None,
    };
    let toml_string = toml::to_string_pretty(&config)?;
    create_dir(name.clone())?;
    create_dir(format!("{}/src", name.clone()))?;
    match language.as_str() {
        "C" => {
            fs::write(
                format!("{}/src/main.c", name),
                "#include <stdio.h>\n\nint main() {\n    printf(\"Hello, World!\\n\");\n    return 0;\n}\n",
            )?;
        }
        "C++" => {
            fs::write(
                format!("{}/src/main.cpp", name),
                "#include <iostream>\n\nint main() {\n    std::cout << \"Hello, World!\" << std::endl;\n    return 0;\n}\n",
            )?;
        }
        _ => {}
    }
    fs::write(format!("{}/Alumake.toml", name), toml_string)?;
    Ok(())
}
