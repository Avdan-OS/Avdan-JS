use std::{path::{PathBuf, Path}, fmt::Display};

use colored::Colorize;

pub struct SourceFile {
    path: String,
}

impl SourceFile {
    fn new(path: String) -> SourceFile {
        SourceFile {
            path
        }
    }

    pub fn to_path(&self, root_path: String) -> Result<PathBuf, String> {
        let mut p = Path::new(root_path.as_str()).join(self.path.clone());

        if !p.exists() {
            p.set_extension("js");
            if !p.exists() {
                return Err(format!("{}\nPath does not exist !", p.to_str().unwrap()));
            }
        }
        if p.is_dir() {
            return Err("Path is a directory !".to_string());
        }

        return Ok(p);
    } 

    pub fn extension(&self) -> String {
        Path::new(&self.path).extension().unwrap().to_str().unwrap_or_else(|| "js").to_string()
    }

    pub fn is_js_file(&self) -> bool {
        self.extension() == "js" || self.extension() == "mjs"
    }
}

impl Display for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", Colorize::blue("SourceFile"), self.path.yellow())
    }
}

pub struct ExternalModule {
    identifier: String,
}

impl ExternalModule {
    pub fn new(identifier: String) -> ExternalModule {
        ExternalModule {
            identifier
        }
    }
}

impl Display for ExternalModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]: {}", Colorize::blue("Module"), self.identifier)
    }
}

pub enum Resource {
    File(SourceFile),
    Module(ExternalModule),
    Internal(String)
}

impl TryFrom<String> for Resource {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value {
            v if v.is_empty() => Err("Empty resource path!".to_string()),
            v if v.starts_with("@avdan") => Ok(Self::Internal(v[6..].to_string())),
            v if v.starts_with(".") || v.starts_with("/") => Ok(Self::File(SourceFile::new(v))),
            v => Ok(Self::Module(ExternalModule::new(v)))
        }
    }
}

impl Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(file) => write!(f, "{}({}) -→ {}", "Resource".blue(), "File".purple(), file),
            Self::Internal(identifier) => write!(f, "{}({}) -→ {}", "Resource".blue(), "Internal".purple(), identifier),
            Self::Module(module) => write!(f, "{}({}) -→ {}", "Resource".blue(), "Internal".purple(), module),
        }
    }
}