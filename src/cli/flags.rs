use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    /// View basic statistics about the git repository
    Overview,
    /// View a quarterly breakdown of contributors and contributor changes
    TeamHistory {
        /// Include current author names
        #[structopt(long, short)]
        verbose: bool,
    },
    /// View quarterly percentage of commits during off-hours
    OffHours {
        /// Include author names
        #[structopt(long, short)]
        verbose: bool,
    },
    /// Print a mailmap file to STDOUT based on repository contributors
    GenerateMailmap,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "git-analyze",
    about = "Analyze git repositories",
    setting = structopt::clap::AppSettings::ColoredHelp
)]
pub struct Flags {
    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}
