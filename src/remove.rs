use std::fs::OpenOptions;
use std::io::{
    BufRead,
    BufReader,
    Write
};

use console::Term;
use dialoguer::Checkboxes;

use crate::main;
use crate::utils::{
    get_path,
    get_theme
};


fn get_content() -> Vec<String> {
    let path = get_path();

    let file = OpenOptions::new()
        .read(true)
        .open(&path)
        .expect("File could not be opened.");

    let buffer = BufReader::new(&file);
    let mut results = Vec::new();

    for line in buffer.lines() {
        match line {
            Ok(line) => {
                results.push(line);
            }
            Err(error) => eprintln!("Line could not be read. {}", error),
        }
    }

    return results;
}

pub fn remove_path() {
    let results = get_content();

    if results.is_empty() {
        Term::stdout()
            .clear_screen()
            .expect("Line could not be cleared.");

        println!("File is empty.");
    } else {
        Term::stdout()
            .clear_screen()
            .expect("Line could not be cleared.");

        let mut items: Vec<&str> = Vec::new();

        for result in &results {
            let path = result.as_ref();
            items.push(path);
        }

        let selections = Checkboxes::with_theme(get_theme())
            .with_prompt("Please select a path")
            .paged(true)
            .items(&items[..])
            .interact()
            .expect("Selection could not be made.");

        if selections.is_empty() {
            Term::stdout()
                .clear_screen()
                .expect("Line could not be cleared.");

            print!("No path was selected.");
            print!("\n");
        } else {
            Term::stdout()
                .clear_screen()
                .expect("Line could not be cleared.");

            let path = get_path();

            let mut file = OpenOptions::new()
                .truncate(true)
                .write(true)
                .open(&path)
                .expect("File could not be opened.");

            let mut paths = items.clone();

            for selection in selections {
                paths.retain(|p| p != &items[selection]);
                println!("{} was removed.", &items[selection]);
            }

            for path in &paths {
                if let Err(error) = writeln!(file, "{}", path) {
                    eprintln!("Path could not be added. {}.", error);
                }
            }
        }
    }

    print!("\n");
    main();
}
