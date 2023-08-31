use clap::{Parser, Subcommand};

pub mod api;

/// Interface with zappy.sh API from the terminal
#[derive(Debug, Parser)]
#[command(name = "zappy")]
#[command(about = "Interface with zappy.sh API from the terminal", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a new alias
    #[command(arg_required_else_help = true)]
    Create {
        /// The name of the alias
        #[arg(required = true)]
        alias_name: String,
        /// The url to redirect to
        #[arg(required = true)]
        url: String,
    },
    /// Retreive all requests made to an alias
    #[command(arg_required_else_help = true)]
    Requests {
        /// The name of the alias
        #[arg(required = true)]
        alias_name: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Create { alias_name, url } => {
            api::create_alias(&alias_name, &url);
        }
        Commands::Requests { alias_name } => {
            println!("Fetching requests for alias {}", alias_name);
            if let Err(e) = api::get_requests(&alias_name) {
                println!("Failed to get requests for alias {} because {}", alias_name, e);
            }
        }
    }
}
