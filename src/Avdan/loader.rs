use std::{path::Path, };
use serde::{Serialize, Deserialize};
use super::security::Constraints;

const MANIFEST_FILE : &str = "manifest.avdan.json";

#[derive(Serialize, Deserialize)]
pub struct Extension {
    name        : String,
    version     : String,
    description : String,
    author      : String,
    main        : String,
    security    : super::security::Constraints,
}

impl Extension {
    fn is_folder(path : &str) -> bool {
        let m = std::fs::metadata(path);

        m.expect("Folder does not exist!").is_dir()
    }

    fn load_manifest(path : &str) -> String {
        let dir = Path::new(path);
        let manif_file = std::fs::read(dir.join(MANIFEST_FILE));

        String::from_utf8(manif_file.expect("Mainifest file does not exist!")).expect("Could not decode file!")
    }

    fn parse_manifest<'s>(content: String) -> Result<Extension, serde_json::Error> {
        serde_json::from_str(content.as_str())
    }

    ///
    /// Parse a `manifest.avdan.json` file into an [`Extension`] struct.
    /// * `path` - Extension's root directory
    /// 
    /// ## Example
    /// 
    /// ```
    /// let extension = Extension::from_manifest("./test");
    /// todo!() // Do stuff with the Extension struct
    /// ```
    ///
    
    pub fn from_manifest(path: &str) -> Extension {
        if !Self::is_folder(path) {
            panic!("Must provide folder!");
        }

        let text = Self::load_manifest(path);
        let mut e = Self::parse_manifest(text).expect("Failed to parse manifest file!");

        e.main = Path::new(path).join(e.main).to_str().unwrap().to_string();
        
        return e;
    }

    ///
    /// Returns the extension's main file 
    /// ```
    /// let extension = Extension::from_manifest("./test");
    /// extension.main()
    /// ```
    /// 
    
    pub fn main(&self) -> &String {
        return &self.main;
    }

    pub fn security(&self) -> &Constraints {
        return &self.security;
    }
}
