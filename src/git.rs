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

pub fn get_git_config(key: &str, scope: GitScope) -> Option<String> {
    let mut cmd = Command::new("git");
    cmd.arg("config");

    let scope_flag = if scope == GitScope::Global {
        "--global"
    } else {
        "--local"
    };
    cmd.arg(scope_flag);

    cmd.arg(key);

    let output = cmd.output().ok()?;
    if output.status.success() {
        let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !value.is_empty() {
            return Some(value);
        }
    }

    None
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
