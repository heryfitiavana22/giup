use clap::Args;

use crate::{profile_repo::get_or_select_profile_unwrap, ssh::test_ssh_host};

#[derive(Args, Debug)]
pub struct TestArgs {
    /// Git username to test
    pub username: Option<String>,
}

pub fn run_test(args: TestArgs) {
    let profile =
        get_or_select_profile_unwrap(args.username, "Which profile would you like to test?");

    test_ssh_host(&profile.host_alias);
}
