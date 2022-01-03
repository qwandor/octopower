use eyre::Report;
use octopower::authenticate;

#[tokio::main]
async fn main() -> Result<(), Report> {
    pretty_env_logger::init();

    let token = authenticate("username", "password").await?;

    println!("Auth token: {:?}", token);

    Ok(())
}
