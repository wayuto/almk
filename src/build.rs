use crate::config::Config;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    error::Error,
    fs::{self, metadata, read_to_string},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;
pub enum Target {
    CSRC,
    CXXSRC,
    OBJ,
    DEPS,
}

fn get_files(root: &str, target: Target) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|p| match target {
            Target::CSRC => matches!(p.extension().and_then(|s| s.to_str()), Some("c")),
            Target::CXXSRC => matches!(
                p.extension().and_then(|s| s.to_str()),
                Some("cpp" | "cc" | "cxx")
            ),
            Target::OBJ => matches!(p.extension().and_then(|s| s.to_str()), Some("o")),
            Target::DEPS => matches!(p.extension().and_then(|s| s.to_str()), Some("d")),
        })
        .collect()
}

fn get_hash(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let content = fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

fn compile(
    log: bool,
    compiler: String,
    debug: bool,
    target: Target,
    flags: Option<String>,
    includes: Option<Vec<String>>,
) -> Result<(), Box<dyn Error>> {
    let debug_flag = if debug {
        "-g -O0 -DDEBUG"
    } else {
        "-O2 -DNDEBUG"
    };
    let source_files = get_files("./src", target);
    let mut flags = if let Some(flags) = flags {
        flags
    } else {
        String::new()
    };
    if let Some(includes) = includes {
        for include in includes {
            flags.push_str(&format!(" -I{}", include));
        }
    }
    for file in source_files {
        let output = PathBuf::from("target/objects")
            .join(file.strip_prefix("./src").unwrap().with_extension("o"));
        if fs::metadata(&output).is_ok() {
            continue;
        }
        let cmd = format!(
            "{} {} {} -c {} -o {} -MMD",
            compiler,
            debug_flag,
            flags,
            file.to_str().unwrap(),
            output.to_str().unwrap(),
        );
        if log {
            println!("{}", cmd);
        }
        let status = std::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .status()?;
        if !status.success() {
            return Err(format!("Failed to compile file: {:?}", file).into());
        }
    }
    Ok(())
}

pub fn get_deps(path: impl AsRef<Path>) -> Result<Vec<String>, std::io::Error> {
    let content = fs::read_to_string(path)?;
    let mut deps = Vec::new();

    let mut full_line = String::new();
    for line in content.lines() {
        if line.ends_with('\\') {
            full_line.push_str(&line[..line.len() - 1]);
        } else {
            full_line.push_str(line);

            if let Some(colon_pos) = full_line.find(':') {
                let after_colon = &full_line[colon_pos + 1..];
                for token in after_colon.split_whitespace() {
                    if !token.is_empty() {
                        deps.push(token.to_string());
                    }
                }
            }
            full_line.clear();
        }
    }

    Ok(deps)
}

pub fn build(log: bool) -> Result<(), Box<dyn Error>> {
    let toml_string = fs::read_to_string(&"./Alumake.toml")?;
    let config: Config = toml::from_str(&toml_string)?;
    if metadata(&"target").is_err() {
        fs::create_dir(&"target")?;
    }
    if metadata(&"target/objects").is_err() {
        fs::create_dir(&"target/objects")?;
    } else {
        let deps = get_files("target/objects", Target::DEPS);
        let mut hash: HashMap<String, String> = match read_to_string("target/deps.json") {
            Ok(content) => serde_json::from_str(&content)?,
            Err(_) => HashMap::new(),
        };

        for dep in deps {
            let deps = get_deps(&dep)?;
            for dep in deps {
                let curr_hash = get_hash(&PathBuf::from(&dep))?;
                if curr_hash != hash.get(&dep).unwrap_or(&String::new()).as_str() {
                    let obj_file = format!(
                        "target/objects/{}.o",
                        Path::new(&dep).file_stem().unwrap().to_str().unwrap()
                    );
                    if fs::metadata(&obj_file).is_ok() {
                        fs::remove_file(&obj_file)?;
                    }
                    hash.remove(&dep);
                    hash.insert(dep, curr_hash);
                }
            }
        }

        serde_json::to_writer_pretty(fs::File::create("target/deps.json")?, &hash)?;
    }

    if let Some(cc) = config.build.cc {
        compile(
            log,
            cc,
            config.build.debug,
            Target::CSRC,
            config.build.cflags.clone(),
            config.build.includes.clone(),
        )?;
    }

    if let Some(cxx) = config.build.cxx {
        compile(
            log,
            cxx,
            config.build.debug,
            Target::CXXSRC,
            config.build.cxxflags,
            config.build.includes,
        )?;
    }

    let name = config.package.name;
    let object_files = get_files("target/objects", Target::OBJ);
    let mut link_cmd = config.build.linker.clone();
    if let Some(lnflags) = config.build.lnflags.clone() {
        link_cmd.push(' ');
        link_cmd.push_str(&lnflags);
        link_cmd.push(' ');
    } else {
        link_cmd.push(' ');
    }
    for file in &object_files {
        link_cmd.push_str(file.to_str().unwrap());
        link_cmd.push(' ');
    }
    link_cmd.push_str(&format!("-o target/{}", name));
    if log {
        println!("{}", link_cmd);
    }
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(link_cmd)
        .status()?;
    if !status.success() {
        return Err(format!("Failed to link file: {:?}", name).into());
    }
    Ok(())
}
