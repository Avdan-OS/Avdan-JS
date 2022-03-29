/**
 * Helper module for the Search API.
 * 
 * A very simple wrapper around the `xclip` utility,
 * because trying to use X Rust bindings or rust X-Clipboard crates ended in absolute disaster.
 */
use std::process::{Command, Stdio};
use std::io::{Write};

#[derive(Copy, Clone)]
pub enum ClipSource {
    PRIMARY,
    SECONDARY,
    CLIPBOARD,
}

impl ClipSource {
    pub fn name(&self) -> &'static str {
        match self {
            ClipSource::PRIMARY => "primary",
            ClipSource::SECONDARY => "secondary",
            ClipSource::CLIPBOARD => "clipboard",
        }
    }
}

pub fn get_formats(source: Option<ClipSource>) -> Vec<String> {
    let output = Command::new("xclip")
            .arg("-o")
            .arg("-selection")
            .arg(source.unwrap_or(ClipSource::CLIPBOARD).name())
            .arg("-t")
            .arg("TARGETS")
            .output();

    if output.is_err() {
        return vec![];
    }

    
    let out = String::from_utf8(output.unwrap().stdout).expect("Err converting to string!");
    
    
    // Split by newline characters.
    let mut lns = out.lines();

    let index = lns.position(|x| x == "STRING");
    
    // If there are no types...
    if index.is_none() {
        return vec![];
    }
    
    let v  : Vec<&str> = lns.map(|s| s).collect();
    
    let a = v.iter().map(|&x| String::from(x)).collect::<Vec<_>>();

    return a;
}


pub fn get_contents(source: Option<ClipSource>, types : Vec<String>) -> Option<(String, String)> {
    let s = source.unwrap_or(ClipSource::CLIPBOARD).name();
    for t in types {
        let output = Command::new("xclip")
                .arg("-o")
                .arg("-selection")
                .arg(s)
                .arg("-t")
                .arg(t.clone())
                .output();
        
        if output.is_ok() {
            return ( t, String::from_utf8(output.unwrap().stdout).unwrap() ).into();
        }
    }
        
    return None;
}

// TODO: Allow for contents to be set with a custom type.
pub fn set_contents(contents: String, source : Option<ClipSource>) -> () {
    let mut cmd = Command::new("xclip")
            .arg("-i")
            .arg("-selection")
            .arg(source.unwrap_or(ClipSource::CLIPBOARD).name())
            .stdin(Stdio::piped())
            .spawn().expect("Ahhhhhh!");
    
    let child_stdin = cmd.stdin.as_mut().unwrap();

    child_stdin.write_all(contents.as_bytes()).expect("Ahhh! Cannot write into stdin!");

    drop(cmd);
}

pub fn paste_text(contents: String, delay: u64) -> bool {
    // Sleep for certain ms before typing.

    println!("Attempting to pase `{0}`", contents);

    std::thread::sleep(std::time::Duration::from_millis(delay));
    
    let cmd = Command::new("xdotool")
                .arg("getactivewindow")
                .arg("type")
                .arg(contents)
                .spawn();

    return cmd.is_ok();
}