use std::convert::TryInto;
use std::env;
use std::env::VarError;
use std::error::Error;
use std::fs::{read_dir, DirEntry, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

// To run this file: navigate to the folder that contains the .toml file
// Then execute: 
// cargo run --bin build_parser
fn main() {
    let grammars = vec![
        //"../parser/assembler",
        "../../src/parser/assembler", // this is the path to the assembler.g4 file relative to the current working directory
    ];
    let additional_args = vec![
        Some("-visitor"),
    ];
    let antlr_path = "../../resources/antlr4/antlr4-4.8-2-SNAPSHOT-complete.jar";

    for (grammar, arg) in grammars.into_iter().zip(additional_args) {
        // ignoring error because we do not need to run anything when deploying to crates.io
        let _ = gen_for_grammar(grammar, antlr_path, arg);
    }

    //println!("cargo:rerun-if-changed=build.rs");

    //println!("cargo:rerun-if-changed=../../resources/antlr4/antlr4-4.8-2-SNAPSHOT-complete.jar");
}

fn gen_for_grammar(
    grammar_file_name: &str,
    antlr_path: &str,
    additional_arg: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    // let out_dir = env::var("OUT_DIR").unwrap();
    // let dest_path = Path::new(&out_dir);

    //let input = env::current_dir().unwrap().join("parser");
    let input = env::current_dir().unwrap().join("resources").join("antlr4");

    let file_name = grammar_file_name.to_owned() + ".g4";

    let c = Command::new("java")
        .current_dir(input) // this sets the pwd (current working directory) of the command call
        //.arg("-cp")
        //.arg(antlr_path)
        //.arg("org.antlr.v4.Tool")
        .arg("-jar")
        .arg("antlr4-4.8-2-SNAPSHOT-complete.jar")
        .arg("-Dlanguage=Rust")
        .arg("-o")
        //.arg("../tests/gen")
        .arg("../../src/parser") // this is the folder where the generated files are placed
        .arg(&file_name)
        .args(additional_arg)
        .spawn()
        .expect("antlr tool failed to start")
        .wait_with_output()?;
    // .unwrap()
    // .stdout;
    // eprintln!("xx{}",String::from_utf8(x).unwrap());

    //println!("cargo:rerun-if-changed=grammars/{}", file_name);

    Ok(())

}
