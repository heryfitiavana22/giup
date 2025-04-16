use std::fmt;
use std::process::Command;

#[derive(PartialEq, Clone)]
pub enum GitScope {
    Local,
    Global,
}

pub fn set_git_name(name: &str, scope: GitScope) {
    let scope = if scope == GitScope::Global {
        "--global"
    } else {
        "--local"
    };

    Command::new("git")
        .args(["config", scope, "user.name", name])
        .status()
        .expect("Failed to configure user.name");
}

pub fn set_git_email(email: &str, scope: GitScope) {
    let scope = if scope == GitScope::Global {
        "--global"
    } else {
        "--local"
    };

    Command::new("git")
        .args(["config", scope, "user.email", email])
        .status()
        .expect("Failed to configure user.email");
}

impl fmt::Display for GitScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scope_str = match self {
            GitScope::Local => "local",
            GitScope::Global => "global",
        };
        write!(f, "{}", scope_str)
    }
}
