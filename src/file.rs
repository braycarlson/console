use console::Term;
use dialoguer::{
    Checkboxes,
    Confirmation,
    Input
};
use systemstat::{
    Platform,
    System
};
use walkdir::WalkDir;

use crate::main;
use crate::utils::{get_theme, update_file};


fn restart() {
    if Confirmation::with_theme(get_theme())
        .with_text("Do you want to search for another file?")
        .interact()
        .expect("Confirmation could not be accepted.") {
            search_by_file_name();
    } else {
        Term::stdout()
            .clear_screen()
            .expect("Terminal could not be cleared.");

        main();
    }
}

fn search() -> Vec<String> {
   Term::stdout()
       .clear_screen()
       .expect("Line could not be cleared.");

    let file_name = Input::<String>::with_theme(get_theme())
        .with_prompt("Filename")
        .interact()
        .expect("Input could not be accepted.")
        .to_lowercase();

    let mut extension = Input::<String>::with_theme(get_theme())
        .with_prompt("Extension")
        .default("exe".to_string())
        .interact()
        .expect("Input could not be accepted.")
        .replace(|c: char| !c.is_alphanumeric(), "")
        .to_lowercase();

    extension.insert(0, '.');

    print!("\n");

    let system = System::new();
    let mut results = Vec::new();

    match system.mounts() {
        Ok(mounts) => {
            for mount in mounts.iter() {
                println!("Scanning {:?}...", mount.fs_mounted_from);

                for (index, entry) in WalkDir::new(&mount.fs_mounted_on)
                        .follow_links(true)
                        .into_iter()
                        .filter_map(|f| f.ok())
                        .enumerate() {

                    print!("\r{:?} files searched.", index);

                    let path = entry.path().to_string_lossy();
                    let file = entry.file_name().to_string_lossy().to_lowercase();

                    if file.contains(&file_name) &&
                    path.to_lowercase().ends_with(&extension) {
                        let string = path.into_owned();
                        results.push(string);
                    }
                }
                println!("\n");
            }
        }
        Err(error) => eprintln!("No mount found. {}", error)
    }

    return results;
}

pub fn search_by_file_name() {
    let results = search();

    if results.is_empty() {
        println!("No file found.");
    } else {
        let mut items: Vec<&str> = Vec::new();

        for result in &results {
            let path = result.as_ref();
            items.push(path);
        }

        let selections = Checkboxes::with_theme(get_theme())
            .with_prompt("Please select a path")
            .paged(true)
            .items(&items)
            .interact()
            .expect("Selection could not be made.");

        if selections.is_empty() {
            Term::stdout()
                .clear_screen()
                .expect("Line could not be cleared.");

            println!("No path was selected.");
        } else {
            Term::stdout()
                .clear_screen()
                .expect("Line could not be cleared.");

            for selection in selections {
                update_file(items[selection]);
            }
        }
    }

    print!("\n");
    restart();
}
