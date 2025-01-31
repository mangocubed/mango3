use clap::{value_parser, Arg, Command};

use mango3_core::config::load_config;
use mango3_core::models::{InvitationCode, User};
use mango3_core::CoreContext;

const ARG_EMAIL: &str = "Email";
const ARG_USERNAME: &str = "Username";

const COMMAND_LOCK_USER: &str = "lock-user";
const COMMAND_NEW_INVITATION_CODE: &str = "new-invitation-code";

#[tokio::main]
async fn main() {
    load_config();

    let core_context = CoreContext::setup().await;

    let arg_email = Arg::new(ARG_EMAIL)
        .short('e')
        .long("email")
        .value_parser(value_parser!(String));
    let arg_username = Arg::new(ARG_USERNAME)
        .short('u')
        .long("username")
        .value_parser(value_parser!(String));
    let version = env!("CARGO_PKG_VERSION");
    let command_matches = Command::new("Mango³ CLI")
        .version(version)
        .subcommand(
            Command::new(COMMAND_LOCK_USER)
                .version(version)
                .arg(arg_username.clone()),
        )
        .subcommand(
            Command::new(COMMAND_NEW_INVITATION_CODE)
                .version(version)
                .arg(arg_email.clone()),
        )
        .get_matches();

    match command_matches.subcommand() {
        Some((COMMAND_LOCK_USER, matches)) => {
            let username = matches
                .get_one::<String>("ARG_USERNAME")
                .expect("argument username is missing");
            let user = User::get_by_username(&core_context, username)
                .await
                .expect("could not get user");
            let result = user.lock(&core_context).await;

            match result {
                Ok(_) => {
                    println!("User locked successfully.")
                }
                _ => println!("Failed to lock user."),
            }
        }
        Some((COMMAND_NEW_INVITATION_CODE, matches)) => {
            let email = matches.get_one::<String>(ARG_EMAIL).expect("argument email is missing");
            let result = InvitationCode::insert(&core_context, email).await;

            match result {
                Ok(_) => {
                    println!("Invitation code created successfully.")
                }
                _ => println!("Failed to create invitation code."),
            }
        }
        _ => {
            println!("Not doing anything");
        }
    }
}
