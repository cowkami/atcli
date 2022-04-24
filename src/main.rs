use clap::{arg, command, ArgMatches, Command};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("authorize")
                .about("Get your verification code.")
                .arg(arg!([USER])),
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
            let verification_code = authorize(sub_matches).await;
            println!("Your verification code: {:?}", verification_code);
            println!("Register this verification code to your account's Affiliation.");
        }
        Some(("login", sub_matches)) => {
            login(sub_matches);
            println!("Login succeeded!");
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}

#[derive(Deserialize)]
struct IAuthorizeResponse {
    verification_code: String,
}

async fn authorize(matches: &ArgMatches) -> reqwest::Result<String> {
    let user_id = matches.value_of("USER").unwrap();
    let params = [("user_id", user_id)];
    let client = reqwest::Client::new();
    let res = client
        .post("https://atcoder-auth.kenkoooo.com/api/authorize")
        .form(&params)
        .send()
        .await?
        .json::<IAuthorizeResponse>()
        .await?;
    println!("hello");
    Ok(res.verification_code)
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
