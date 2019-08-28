use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::{
    BufRead,
    BufReader,
    Write
};
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

use console::{
    Style,
    Term
};
use dialoguer::Confirmation;
use dialoguer::theme::{
    ColorfulTheme,
    Theme
};
use lazy_static::lazy_static;

use crate::main;


lazy_static! {
    static ref THEME: ColorfulTheme = ColorfulTheme {
        defaults_style: Style::new().cyan().dim(),
        error_style: Style::new().cyan().dim(),
        inactive_style: Style::new().white().dim(),
        active_style: Style::new().cyan().dim(),
        values_style: Style::new().cyan().dim(),
        indicator_style: Style::new().cyan().dim(),
        yes_style: Style::new().cyan().dim(),
        no_style: Style::new().cyan().dim(),
        ..ColorfulTheme::default()
    };
}

pub fn get_theme() -> &'static dyn Theme {
    &*THEME
}

pub fn open_undertasker() {
    let mut path = PathBuf::new();
    let directory = env::current_exe();

    match directory {
        Ok(directory) => {
            path.push(directory);
            path.set_file_name("undertasker");
            path.set_extension("exe");
        }
        Err(error) => eprint!("Directory not found. {}", error)
    };

    Command::new(path)
        .creation_flags(0x00000008) // DETACHED_PROCESS
        .spawn()
        .expect("Process could not be spawned.");

    Term::stdout()
        .clear_screen()
        .expect("Terminal could not be cleared.");

    main();
}

pub fn open_file() {
    let path = get_path();

    Command::new("cmd")
        .arg("/C")
        .arg(path)
        .creation_flags(0x00000008) // DETACHED_PROCESS
        .spawn()
        .expect("Process could not be spawned.");

    Term::stdout()
        .clear_screen()
        .expect("Terminal could not be cleared.");

    main();
}

pub fn open_location() {
    let directory = env::current_dir();

    match directory {
        Ok(directory) => {
            Command::new("cmd")
                .arg("/C")
                .arg("start")
                .arg(directory)
                .creation_flags(0x00000008) // DETACHED_PROCESS
                .spawn()
                .expect("Process could not be spawned.");
        }
        Err(error) => eprint!("Directory not found. {}", error)
    };

    Term::stdout()
        .clear_screen()
        .expect("Terminal could not be cleared.");

    main();
}

pub fn clear_file() {
    let path = get_path();

    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .expect("File could not be opened.");

    Term::stdout()
        .clear_screen()
        .expect("Terminal could not be cleared.");

    println!("File is cleared.");

    print!("\n");
    main();
}

pub fn update_file(path: &str) -> std::fs::File {
    let directory = get_path();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(&directory)
        .expect("File could not be opened.");

    let buffer = BufReader::new(&file);

    for line in buffer.lines() {
        match line {
            Ok(line) => {
                if path == &line {
                    let confirmation = format!("{} already exists. Would you like to add it anyway?", path);

                    if Confirmation::with_theme(get_theme())
                        .with_text(&confirmation)
                        .interact()
                        .expect("Confirmation could not be accepted.") {

                        if let Err(error) = writeln!(&file, "{}", path) {
                            eprintln!("Path could not be added. {}.", error);
                        }
                    }
                    return file;
                }
            }
            Err(error) => eprintln!("Line could not be read. {}", error),
        }
    }

    if let Err(error) = writeln!(file, "{}", path) {
        eprintln!("Path could not be added. {}.", error);
    }

    println!("{} was added.", path);
    return file
}

pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn get_windows_path() -> std::path::PathBuf {
    let mut path = PathBuf::new();
    let windows = env::var("windir");

    match windows {
        Ok(ref variable) => path.push(variable),
        Err(ref error) => eprintln!("Windows path could not be found. {}", error),
    }

    return path;
}

pub fn get_path() -> std::path::PathBuf {
    let mut path = PathBuf::new();
    let directory = env::current_exe();

    match directory {
        Ok(directory) => {
            path.push(directory);
            path.set_file_name("command");
            path.set_extension("txt");
        }
        Err(error) => eprint!("Directory not found. {}", error)
    };

    return path;
}
