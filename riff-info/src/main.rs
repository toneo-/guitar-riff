use std::env;

use guitar_riff::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    // Check for 2 args as the first one is the exe location
    if args_len != 2 {
        panic!("Expected exactly one argument, got {args_len}");
    }

    let file_path = &args[1];

    let mut file = match RiffFile::open(file_path) {
        Ok(x) => x,
        Err(_) => panic!("Could not open file"),
    };

    println!("File size: {} bytes", file.chunks_size);
    println!("Form type: {}", file.form_type);

    print_chunks(file.chunks(), 0);
}

fn get_indent(level: i32) -> String {
    let mut string = String::new();

    for i in 0..level {
        string += "  ";
    }

    return string;
}

fn print_chunks<I: Iterator<Item = Chunk>>(chunks: I, indent_level: i32) {
    let indent = get_indent(indent_level);

    for chunk in chunks {
        let size = chunk.inner_size();
        match chunk {
            Chunk::Simple {
                identifier,
                data: _,
            } => {
                println!("{indent}{identifier}: {}", size);
            }
            Chunk::List {
                data_type,
                sub_chunks,
            } => {
                println!("{indent}LIST {} items: {}", size, data_type);

                print_chunks(sub_chunks.iter().cloned(), indent_level + 1);
            }
        }
    }
}
