// build.rs
use std::fs;

fn main() { 
    let source_ext = if cfg!(windows) { "dll" } else { "so" };
    let target_ext = if cfg!(windows) { "pyd" } else { "so" };
    let _ = fs::copy(format!("target/release/myrustlib.{ext}", ext= source_ext), 
                     format!("../myrustlib.{}", target_ext));
    println!("created python lib");
}