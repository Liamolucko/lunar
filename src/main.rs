mod dev;
mod transform;

use clap::{App, Arg, SubCommand};
use dev::dev;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = App::new("Lunar")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            SubCommand::with_name("dev").arg(
                Arg::with_name("port")
                    .short("p")
                    .help("The port at which to start the dev server"),
            ),
        )
        .get_matches();

    if let Some(m) = matches.subcommand_matches("dev") {
        dev(m).await?;
    }

    Ok(())
}
