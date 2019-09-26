use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "space_email_scraper", about = "space_email_scraper usage.")]
pub struct Args {
    #[structopt(value_name = "EMAILS", help = "The number of emails to download. 0 means infinite.")]
    pub emails: usize,

    #[structopt(value_name = "SAVE_PATH", help = "The directory to save downloaded emails to.")]
    pub save_path: Option<PathBuf>,

    #[structopt(short, long = "cooldown-ms", default_value = "250", 
    #[structopt(help = "The minimum time between making requests to space email, in milliseconds.")]
    pub cooldown_ms: usize,

    #[structopt(short, long = "max-search-volume", default_value = "0")]
    #[structopt(help = "The maximum number of emails to search before stopping. 0 means infinite.")]
    pub max_volume: usize,

    #[structopt(short, long, default_value = "10")]
    #[structopt(help = "The number of times to retry a failed request to space email.")]
    pub retries: usize,

    #[structopt(long, default_value = "30000")]
    #[structopt(help = "When the connection is lost, how many milliseconds to wait before attempting to reconnect.")]
    pub reconnect_ms: usize,

    #[structopt(short, long)]
    #[structopt(help = "The filter to apply when searching. Only emails that match it will be saved.")]
    pub filter: Option<Filter>,
}