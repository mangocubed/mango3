use clap::{value_parser, Arg, Command};

use mango3_core::config::load_config;
use mango3_core::models::InvitationCode;
use mango3_core::CoreContext;

const ARG_EMAIL: &str = "Email";

const COMMAND_NEW_INVITATION_CODE: &str = "new-invitation-code";

#[tokio::main]
async fn main() {
    load_config();

    let core_context = CoreContext::setup().await;

    let arg_email = Arg::new(ARG_EMAIL)
        .short('e')
        .long("email")
        .value_parser(value_parser!(String));
    let version = env!("CARGO_PKG_VERSION");
    let command_matches = Command::new("Mango³ CLI")
        .version(version)
        .subcommand(
            Command::new(COMMAND_NEW_INVITATION_CODE)
                .version(version)
                .arg(arg_email.clone()),
        )
        .get_matches();

    match command_matches.subcommand() {
        Some((COMMAND_NEW_INVITATION_CODE, matches)) => {
            let email = matches.get_one::<String>(ARG_EMAIL).cloned().unwrap_or_default();
            let result = InvitationCode::insert(&core_context, &email).await;

            match result {
                Ok(_) => {
                    println!("Invitation code created successfully.")
                }
                _ => println!("Failed to create invitation code"),
            }
        }
        _ => {
            println!("Not doing anything");
        }
    }
}
