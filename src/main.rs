use core::panic;
use std::io;

use std::fs;
mod codegen;
mod lang_parser;
use codegen::generate_c_code;
use lang_parser::{JPPParser, Rule};
mod tree_printer;
use pest::Parser; // Import the Parser trait
use std::fs::File;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
fn main() -> io::Result<()> {
    println!("Hello, world!");

    // Define the directory path
    let folder_path = "./examples/";

    // Read the directory
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a file
        if path.is_file() {
            println!("{}", "-".repeat(100));
            println!("\nReading file: {:?}", path);

            // Read the content of the file
            let content = fs::read_to_string(&path)?;

            // Parse the content using the "program" rule
            match JPPParser::parse(Rule::program, &content) {
                Ok(parsed) => {
                    // If parsing is successful, print the parse tree

                    // Clone the iterator to use it for counting and then iterating
                    let mut parsed_clone = parsed.clone();

                    // Check if more than one top-level node is found
                    if parsed_clone.clone().count() != 1 {
                        panic!("Error: More than one TL-Node");
                    }

                    // Get the first parse tree node
                    let tree = parsed_clone.next();

                    // Ensure that the tree is not empty
                    let tree = match tree {
                        Some(tree) => tree,
                        None => panic!("Empty Tree"),
                    };

                    tree_printer::print_parse_tree(tree.clone(), 0);

                    // Generate C code from the tree
                    let c_code = generate_c_code(tree);

                    compile_c_code(c_code.clone(), path.to_str().unwrap().to_string());

                    println!("{}", c_code);
                }
                Err(e) => {
                    // Handle parsing error
                    println!("Error while parsing {:?}: {}", path, e);
                }
            }
        }
    }

    Ok(())
}

fn compile_c_code(c_code: String, file_path: String) {
    // Step 2: Write the C code to a temporary file (e.g., "program.c")

    let file_name = file_path.split("/").last().unwrap();

    let c_file_path = file_name.to_string() + ".c";
    let mut c_file = File::create(c_file_path.clone()).expect("Failed to create C file.");

    c_file
        .write_all(c_code.as_bytes())
        .expect("Failed to write C code to file.");

    // Step 3: Compile the C code using GCC

    let binary_name = file_name.replace(".jpp", "");

    let output = Command::new("gcc")
        .arg(c_file_path)
        .arg("-o") // Specify the output binary file
        .arg(binary_name) // Output binary will be named "program"
        .output() // Run the command and capture output
        .expect("Failed to execute gcc");

    // Check if GCC gave any output (e.g., errors or warnings)
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("GCC failed to compile C code: {}", stderr);
    }

    println!("C code compiled successfully!");
}
