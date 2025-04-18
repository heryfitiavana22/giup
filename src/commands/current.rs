use crate::git::{GitScope, get_git_config};

pub fn run_current() {
    let name_local = get_git_config("user.name", GitScope::Local);
    let email_local = get_git_config("user.email", GitScope::Local);

    let name_global = get_git_config("user.name", GitScope::Global);
    let email_global = get_git_config("user.email", GitScope::Global);

    if let (Some(name), Some(email)) = (&name_local, &email_local) {
        println!("Current Git profile (scope: local)\n");
        print_profile(name, email);
    } else if let (Some(name), Some(email)) = (&name_global, &email_global) {
        println!("Current Git profile (scope: global)\n");
        print_profile(name, email);
    } else {
        println!("No Git profile is configured.");
    }
}

fn print_profile(name: &str, email: &str) {
    println!("Git Name     : {}", name);
    println!("Git Email    : {}", email);
}
