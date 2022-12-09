use clap::{Parser, Subcommand};
mod new;
use std::process;
#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Add {
        ip: String,
        /// Second number to add
        name: String,

        user: String,
    },
}

fn main() {
    let value = Value::parse();
    if let Err(e) = pulao_cli::connect_db() {
        print!("error connecting to the database {}", e);
        process::exit(1);
    }
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    match &value.command {
        Commands::Add { ip, name, user } => new::add_ssh(ip, name, user), // Commands::Substract {
                                                                          //     number_one,
                                                                          //     number_two,
                                                                          // } => {
                                                                          //     println!(
                                                                          //         "'myapp add' was used, name is: {:?}",
                                                                          //         number_one - number_two
                                                                          //     )
                                                                          // }
                                                                          // Commands::Multiply {
                                                                          //     number_one,
                                                                          //     number_two,
                                                                          // } => {
                                                                          //     println!(
                                                                          //         "'myapp add' was used, name is: {:?}",
                                                                          //         number_one * number_two
                                                                          //     )
                                                                          // }
                                                                          // Commands::Divide {
                                                                          //     number_one,
                                                                          //     number_two,
                                                                          // } => {
                                                                          //     println!(
                                                                          //         "'myapp add' was used, name is: {:?}",
                                                                          //         number_one / number_two
                                                                          //     )
                                                                          // }
    }
}
