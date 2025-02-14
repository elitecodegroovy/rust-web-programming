mod cli;
mod init;
mod common;

use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let cli_opt = cli::Cli::parse();
    init::init_env(&cli_opt);


    Ok(())
}