pub mod delete;
pub mod delete_tests;
pub mod diff;
pub mod diff_tests;
pub mod edit;
pub mod edit_tests;
pub mod env;
pub mod env_tests;
pub mod export;
pub mod export_tests;
pub mod get;
pub mod get_tests;
pub mod import;
pub mod import_tests;
pub mod init;
pub mod init_tests;
pub mod keygen;
pub mod list;
pub mod lock;
pub mod rename;
pub mod rename_tests;
pub mod rotate;
pub mod rotate_tests;
pub mod run;
pub mod run_tests;
pub mod set;
pub mod set_tests;
pub mod status;
pub mod status_tests;
pub mod unlock;
pub mod unset;
pub mod unset_tests;
pub mod copy;
pub mod copy_tests;
pub mod whoami;
pub mod whoami_tests;
pub mod clone;
pub mod clone_tests;

use crate::config::Config;
use anyhow::Result;

#[derive(Debug)]
pub enum Command {
    Init { env: String },
    Lock { env: String },
    Unlock { env: String },
    List,
    Run { env: String, cmd: Vec<String> },
    Edit { env: String },
    Status { env: String },
    Rotate { env: String },
    Whoami,
    Env { env: String },
    Get { env: String, key: String },
    Set { env: String, key: String, value: String },
    Delete { env: String },
    Import { env: String, file: String },
    Export { env: String, file: Option<String> },
    Unset { env: String, key: String },
    Copy { source: String, destination: String },
    Diff { env_a: String, env_b: String },
    Rename { old_name: String, new_name: String },
    Clone { source: String, destination: String },
    Keygen,
}

pub fn dispatch(command: Command, config: &Config) -> Result<()> {
    match command {
        Command::Init { env } => init::run(&env, config),
        Command::Lock { env } => lock::run(&env, config),
        Command::Unlock { env } => unlock::run(&env, config),
        Command::List => list::run(config),
        Command::Run { env, cmd } => run::run(&env, &cmd, config),
        Command::Edit { env } => edit::run(&env, config),
        Command::Status { env } => status::run(&env, config),
        Command::Rotate { env } => rotate::run(&env, config),
        Command::Whoami => whoami::run(config),
        Command::Env { env } => env::run(&env, config),
        Command::Get { env, key } => get::run(&env, &key, config),
        Command::Set { env, key, value } => set::run(&env, &key, &value, config),
        Command::Delete { env } => delete::run(&env, config),
        Command::Import { env, file } => import::run(&env, &file, config),
        Command::Export { env, file } => export::run(&env, file.as_deref(), config),
        Command::Unset { env, key } => unset::run(&env, &key, config),
        Command::Copy { source, destination } => copy::run(&source, &destination, config),
        Command::Diff { env_a, env_b } => diff::run(&env_a, &env_b, config),
        Command::Rename { old_name, new_name } => rename::run(&old_name, &new_name, config),
        Command::Clone { source, destination } => clone::run(&source, &destination, config),
        Command::Keygen => keygen::run(config),
    }
}
