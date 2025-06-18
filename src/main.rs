mod detection;
mod web;

use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
    #[structopt(short, long)]
    web: bool,
}

#[derive(Error, Debug)]
enum AppError {
    #[error("OpenCV error: {0}")]
    OpenCV(#[from] opencv::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let args = Cli::from_args();

    if args.web {
        web::start_server().await?;
    } else if let Some(input) = args.input {
        detection::process_video(&input)?;
    } else {
        detection::process_webcam()?;
    }

    Ok(())
}
