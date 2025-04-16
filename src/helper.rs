#[macro_export]
macro_rules! exit {
    ($msg:expr) => {
        eprintln!("{}", $msg);
        std::process::exit(1);
    };
}
