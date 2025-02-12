use clap::{value_parser, Arg, Command};

use mango3_core::config::load_config;
use mango3_core::models::{InvitationCode, User};
use mango3_core::CoreContext;

const ARG_EMAIL: &str = "Email";
const ARG_ROLE: &str = "role";
const ARG_USERNAME: &str = "Username";

const COMMAND_LOCK_USER: &str = "lock-user";
const COMMAND_NEW_INVITATION_CODE: &str = "new-invitation-code";
const COMMAND_UPDATE_USER_ROLE: &str = "update-user-role";

#[tokio::main]
async fn main() {
    load_config();

    let core_context = CoreContext::setup().await;

    let arg_email = Arg::new(ARG_EMAIL)
        .short('e')
        .long("email")
        .value_parser(value_parser!(String));
    let arg_role = Arg::new(ARG_ROLE).value_parser(value_parser!(String));
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
        .subcommand(
            Command::new(COMMAND_UPDATE_USER_ROLE)
                .version(version)
                .arg(arg_username)
                .arg(arg_role),
        )
        .get_matches();

    match command_matches.subcommand() {
        Some((COMMAND_LOCK_USER, matches)) => {
            let username = matches
                .get_one::<String>(ARG_USERNAME)
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
        Some((COMMAND_UPDATE_USER_ROLE, matches)) => {
            let username = matches
                .get_one::<String>(ARG_USERNAME)
                .expect("Argument username is missing");
            let role = matches.get_one::<String>(ARG_ROLE).expect("Role is missing").into();
            let user = User::get_by_username(&core_context, username)
                .await
                .expect("Could not get user");
            let result = user.update_role(&core_context, role).await;

            match result {
                Ok(_) => {
                    println!("User role updated successfully.")
                }
                _ => println!("Failed to update user role."),
            }
        }
        _ => {
            println!("Not doing anything");
        }
    }
}
