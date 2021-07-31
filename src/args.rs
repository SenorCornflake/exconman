use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Set {
    // Setting name
    pub name: String,
    // New setting value
    pub value: String
}

#[derive(StructOpt, Debug)]
pub struct Get {
    /// Setting Name
    pub name: String
}

#[derive(StructOpt, Debug)]
pub struct Load {
    /// Path to JSON file
    pub path: String
}

#[derive(StructOpt, Debug)]
pub enum SubCommands {
    Set(Set),
    Get(Get),
    Load(Load),
    Dump
} 

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(long, short)]
    /// Path to a custom registry
    pub registry: Option<String>,
    #[structopt(subcommand)]
    pub sub: SubCommands
}

