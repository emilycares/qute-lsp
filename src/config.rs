use clap::Parser;

/// A build log analysis tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// prints routes as json
    #[clap(long)]
    pub get_routes: bool,
}
