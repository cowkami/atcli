use clap::{arg, command, ArgMatches, Command};

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("login")
                .about("Login with your AtCoder account.")
                .arg(arg!(--user <VALUE>))
                .arg(arg!(--token <VALUE>)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("login", sub_matches)) => login(sub_matches),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}

fn login(matches: &ArgMatches) {
    println!(
        "user:{:?} token:{:?}",
        matches.value_of("user").unwrap(),
        matches.value_of("token").unwrap(),
    )
}
