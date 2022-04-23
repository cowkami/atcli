use clap::{arg, command, ArgMatches, Command};

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("authorize")
                .about("Get your verification code.")
                .arg(arg!(<USER>)),
        )
        .subcommand(
            Command::new("login")
                .about("Login with your AtCoder account.")
                .arg(arg!(--user <VALUE>))
                .arg(arg!(--token <VALUE>)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("authorize", sub_matches)) => {
            authorize(sub_matches);
            println!("Register this verification code to your account's Affiliation.");
        }
        Some(("login", sub_matches)) => {
            login(sub_matches);
            println!("Login succeeded!");
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}

async fn authorize(matches: &ArgMatches) -> reqwest::Result<()> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()
        .await?;

    Ok(())
}

async fn login(matches: &ArgMatches) -> reqwest::Result<()> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()
        .await?;

    Ok(())
}
