use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about="USB phone file browser")]
pub struct Cli 
{
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command 
{
    Scan 
    {
        #[arg(long)]
        xiaomi: bool,
    },
    List
    {
        #[arg(long)]
        xiaomi: bool,
    },

}
