//! CLI module: command-line parsing and subcommands.

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub command: Option<Commands>,

    #[clap(long)]
    pub src: Option<String>,

    #[clap(long)]
    pub dest: Option<String>,

    #[clap(long)]
    pub db: Option<String>,

    #[clap(long)]
    pub linkedin_api_key: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Launch the GUI
    #[clap(name = "gui")]
    Ui,
    /// Mark a job's application status
    Mark {
        /// The ID of the job to mark
        #[clap(long = "id")]
        job_id: String,
        /// Set status to 'applied' (true) or 'not applied' (false)
        #[clap(long, value_parser = clap::value_parser!(bool))]
        status: bool,
    },
}
