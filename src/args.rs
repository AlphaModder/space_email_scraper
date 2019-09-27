use structopt::StructOpt;
use std::path::PathBuf;
use chrono::NaiveDateTime;
use space_email_api::SpaceEmail;
use regex::Regex;

#[derive(Debug, StructOpt)]
#[structopt(name = "space_email_scraper", about = "space_email_scraper usage.")]
pub struct Args {
    #[structopt(value_name = "EMAILS", help = "The number of emails to download. 0 means infinite.")]
    pub emails: usize,

    #[structopt(value_name = "SAVE_PATH", help = "The directory to save downloaded emails to.")]
    pub save_path: Option<PathBuf>,

    #[structopt(short, long = "cooldown-ms", default_value = "250")]
    #[structopt(help = "The minimum time between making requests to space email, in milliseconds.")]
    pub cooldown_ms: u64,

    #[structopt(short = "v", long = "max-search-volume", default_value = "0")]
    #[structopt(help = "The maximum number of emails to search before stopping. 0 means infinite.")]
    pub max_volume: usize,

    #[structopt(short, long, default_value = "10")]
    #[structopt(help = "The number of times to retry a failed request to space email.")]
    pub retries: usize,

    #[structopt(long = "no-reconnect")]
    pub no_reconnect: bool,

    #[structopt(long = "reconnect-ms", default_value = "30000")]
    #[structopt(help = "When the connection is lost, how many milliseconds to wait before attempting to reconnect.")]
    pub reconnect_ms: u64,

    #[structopt(short, long)]
    #[structopt(help = "Only download emails that were sent before this date.")]
    pub before: Option<NaiveDateTime>,

    #[structopt(short, long)]
    #[structopt(help = "Only download emails that were sent before this date.")]
    pub after: Option<NaiveDateTime>,

    #[structopt(long, alias = "sender-matches")]
    #[structopt(help = "Match emails whose sender matches the provided regex.")]
    pub sender: Option<Regex>,

    #[structopt(long, alias = "subject-matches")]
    #[structopt(help = "Match emails whose subject matches the provided regex.")]
    pub subject: Option<Regex>,

    #[structopt(long, alias = "body-matches")]
    #[structopt(help = "Match emails whose subject matches the provided regex.")]
    pub body: Option<Regex>,

    #[structopt(short = "m", long, alias = "any-matches")]
    #[structopt(help = "Match emails whose subject, sender, or body matches the provided regex.")]
    pub any: Option<Regex>,

    #[structopt(long)]
    #[structopt(help = "Only downloads emails that match all content filters, rather than at least one.")]
    pub all: bool,
}

impl Args {
    pub fn should_continue(&self, searched: usize, downloaded: usize) -> bool {
        (self.max_volume == 0 || searched < self.max_volume) && 
        (self.emails == 0 || downloaded < self.emails)
    }

    pub fn filter(&self, email: &SpaceEmail) -> bool {
        if !self.before.map(|before| email.timestamp() <= before).unwrap_or(true) {
            return false;
        }

        if !self.after.map(|after| email.timestamp() >= after).unwrap_or(true) {
            return false;
        }

        let (sender_matches, subject_matches, body_matches) = (
            self.sender.as_ref().map(|sender| sender.is_match(&email.contents().sender)),
            self.subject.as_ref().map(|subject| subject.is_match(&email.contents().subject)),
            self.body.as_ref().map(|body| body.is_match(&email.contents().body))
        );

        let any_matches = self.any.as_ref().map(|any| {
            any.is_match(&email.contents().sender) || 
            any.is_match(&email.contents().subject) ||
            any.is_match(&email.contents().body)
        });

        let matches = [sender_matches, subject_matches, body_matches, any_matches];
        match self.all {
            true => !matches.iter().any(|x| *x == Some(false)),
            false => matches.iter().all(|x| *x == None) || matches.iter().any(|x| *x == Some(true)),
        }
    }
}