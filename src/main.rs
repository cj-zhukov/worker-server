use std::env;

use color_eyre::Result;
use worker_server::Application;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "Usage: {} <name1,address1> <name2,address2> <name3,address3>",
            args[0]
        );
    }

    let mut servers = Vec::new();
    for arg in args.iter().skip(1) {
        if let Some((name, address)) = arg.split_once(',') {
            servers.push((name.to_string(), address.to_string()));
        }
    }

    let mut tasks = vec![];
    for (name, address) in servers {
        tasks.push(tokio::spawn(run(address, name)));
    }
    for task in tasks {
        task.await??;
    }

    Ok(())
}

async fn run(address: String, name: String) -> Result<()> {
    let app = Application::build(address, name).await?;
    app.run().await?;
    Ok(())
}