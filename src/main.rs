use clap::{Parser, Subcommand};
use crate::file::create_path;

mod file;

#[derive(Parser)]
#[command(name = "worklog", author = "Benedikt Grande", version = "0.1", about = "reminds you about any domain expiry", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    New {
        #[arg(short, long)]
        name: String,
    },
    Process {
        #[arg(short, long)]
        name: Option<String>,
    }
    // todo: Add -> interactive adding data routine
}

fn create_new(name: String) {
    let path = match create_path(name) {
        Ok(path_name) => path_name,
        Err(e) => {
            eprintln!("Could not create new worklog because: {}", e); return
        }
    };


}

fn process(name: Option<String>) {
    let new_name = match name {
        Some(nam) => nam,
        None => ""
    };


}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::New {
                 name,
             }) => {
            create_new(name.to_owned()).await;
        }
        Some(Commands::Process {
            name,
         }) => {
            process(name.to_owned());
        }
        None => {
            print!("I really don't know what to do!");
            print!("Please use '--help' to get more command information!")
        }
    }
}
