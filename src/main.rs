use chrono::format::parse;
use clap::{Parser, Subcommand};
use crate::file::{init_repository};

mod file;
mod parse;

const DEFAULT_NAME: &str = "worklog";

#[derive(Parser)]
#[command(name = "worklog", author = "Benedikt Grande", version = "0.1", about = "reminds you about any domain expiry", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short, long)]
        name: Option<String>,
    },
    Process {
        #[arg(short, long)]
        name: Option<String>,
    }
    // todo: Add -> interactive adding data routine
}

fn create_new(name: String) {
    match init_repository(name.clone()) {
        Ok(()) => println!("Initializing repository {} was successful", name),
        Err(e) => {
            eprintln!("Could not create new worklog because: {}", e); return
        }
    };
}

fn process(name: Option<String>) {
    let new_name = match name {
        Some(nam) => nam,
        None => DEFAULT_NAME.to_string()
    };
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Init {
                 name,
             }) => {
            let named = match name.to_owned() {
                Some(nam) => nam,
                None => DEFAULT_NAME.to_string()
            };
            create_new(named);
        }

        Some(Commands::Process {
            name,
         }) => {
            process(name.to_owned());
        }
        
        None => {
            println!("I really don't know what to do!");
            println!("Please use '--help' to get more command information!")
        }
    }
}
