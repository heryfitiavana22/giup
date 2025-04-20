use crate::file::read_file_to_string;

#[macro_export]
macro_rules! exit {
    ($msg:expr) => {
        eprintln!("{}", $msg);
        std::process::exit(1);
    };
}

pub fn file_to_clipboard(file_path: &str) {
    let file_content = read_file_to_string(file_path);

    let mut clipboard = arboard::Clipboard::new().unwrap_or_else(|_| {
        exit!("Failed to access the clipboard.");
    });

    clipboard.set_text(file_content).unwrap_or_else(|_| {
        exit!("Failed to copy to clipboard.");
    });

    println!("File content from '{}' copied to clipboard.", file_path);
}
