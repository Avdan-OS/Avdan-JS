/*
    Permissions system for extension.

    Allows access to a component of the Avdan API.

    Permissions are in the format:
        avdan.file.* == [
            avdan.file.read
            avdan.file.write
            avdan.file.move
            avdan.file.delete
        ]
*/
/*
    Permissions system for extension.

    Allows access to a component of the Avdan API.

    Permissions are in the format:
        avdan.file.* == [
            avdan.file.read
            avdan.file.write
            avdan.file.move
            avdan.file.delete
        ]
*/

pub mod security;
pub mod loader;
mod runtime;
pub mod api;

pub mod utils;

pub struct Permission {
    contents: String,
}

impl Permission {
    pub fn new(contents : &str) -> Permission {
        Permission { contents : String::from(contents) }
    }
    pub fn has(&self, permission: &str) -> bool {
        let to_check_arr : Vec<&str> = self.contents.split(".").collect();
        let permission_arr : Vec<&str> = permission.split(".").collect();

        for (i, el) in to_check_arr.iter().enumerate() {
            match *el {
                "*" => return true,
                n => {
                    match permission_arr.get(i) {
                        Some(v) => if *v != n { return false; },
                        None => return false,
                    }
                } 
            }
        };

        return true;
    }
}

pub use runtime::{Runtime, PromIndex};
