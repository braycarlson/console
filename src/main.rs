mod file;
mod folder;
mod remove;
mod settings;
mod software;
mod utils;
mod view;

use std::fs::OpenOptions;
use std::process;

use crossterm_cursor::TerminalCursor;
use dialoguer::Select;

use file::search_by_file_name;
use folder::search_by_folder_name;
use remove::remove_path;
use settings::microsoft_settings_menu;
use software::microsoft_software_menu;
use utils::{
    clear_file,
    get_path,
    get_theme,
    open_file,
    open_location,
    open_undertasker
};
use view::view_file;


fn main() {
    let path = get_path();

    // Check for command.txt
    if !path.is_file() {
        OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&path)
            .expect("File could not be opened.");
    };

    // Hide cursor
    let cursor = TerminalCursor::new();
    cursor.hide().expect("Cursor could not be hidden.");

    // Main Menu
    let items = &[
        "Start: Undertasker",
        "Search: File",
        "Search: Folder",
        "Search: Microsoft Software",
        "Search: Microsoft Settings",
        "Command: Open",
        "Command: Location",
        "Command: View",
        "Command: Clear",
        "Command: Remove",
        "Exit"
    ];

    let menu = Select::with_theme(get_theme())
        .with_prompt("Main Menu")
        .paged(true)
        .items(&items[..])
        .default(0)
        .interact()
        .expect("Selection could not be made.");

    match menu {
        0 => open_undertasker(),
        1 => search_by_file_name(),
        2 => search_by_folder_name(),
        3 => microsoft_software_menu(),
        4 => microsoft_settings_menu(),
        5 => open_file(),
        6 => open_location(),
        7 => view_file(),
        8 => clear_file(),
        9 => remove_path(),
        10 => process::exit(1),
        _ => ()
    }
}
