use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub package: Package,
    pub build: Build,
    pub dependencies: Option<Dependency>,
}

#[derive(Deserialize, Serialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub author: String,
    pub license: String,
    pub language: String,
}

#[derive(Deserialize, Serialize)]
pub struct Build {
    pub debug: bool,
    pub linker: String,
    pub cc: Option<String>,
    pub cxx: Option<String>,
    pub cflags: Option<String>,
    pub cxxflags: Option<String>,
    pub lnflags: Option<String>,
    pub includes: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
}
