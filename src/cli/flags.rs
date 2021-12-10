use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    Overview,
    TeamHistory,
    OffHours,
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
