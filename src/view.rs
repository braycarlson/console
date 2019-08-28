use std::fs::OpenOptions;
use std::io::{
    BufRead,
    BufReader
};

use console::Term;

use crate::main;
use crate::utils::get_path;


pub fn view_file() {
    let directory = get_path();

    let file = OpenOptions::new()
        .read(true)
        .open(&directory)
        .expect("File could not be opened.");

    let mut result = Vec::new();

    let buffer = BufReader::new(&file);

    for line in buffer.lines() {
        match line {
            Ok(line) => {
                result.push(line);
            }
            Err(error) => eprintln!("Line could not be read. {}", error),
        }
    }

    if result.is_empty() {
        Term::stdout()
            .clear_screen()
            .expect("Line could not be cleared.");

        println!("File is empty.");
    } else {
        Term::stdout()
            .clear_screen()
            .expect("Line could not be cleared.");

        for path in result {
            println!("{}", path);
        }
    }

    print!("\n");
    main();
}
