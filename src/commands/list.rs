use crate::profile_repo::get_profiles;

pub fn run_list() {
    let profiles = get_profiles();

    if profiles.is_empty() {
        println!("No profiles found.");
        return;
    }

    let mut max_username = 0;
    let mut max_name = 0;
    let mut max_email = 0;

    for (username, profile) in &profiles {
        max_username = max_username.max(username.len());
        max_name = max_name.max(profile.name.len());
        max_email = max_email.max(profile.email.len());
    }

    println!("Available profiles:\n");

    println!(
        "{:username_width$} | {:name_width$} | {}",
        "username",
        "name",
        "email",
        username_width = max_username,
        name_width = max_name
    );
    println!(
        "{:username_width$} | {:name_width$} | {}",
        format!("{:-^username_width$}", "", username_width = max_username),
        format!("{:-^name_width$}", "", name_width = max_name),
        format!("{:-^max_email$}", "", max_email = max_email),
        username_width = max_username,
        name_width = max_name
    );
    for (username, profile) in profiles {
        println!(
            "{:username_width$} | {:name_width$} | {}",
            username,
            profile.name,
            profile.email,
            username_width = max_username,
            name_width = max_name
        );
    }
}
