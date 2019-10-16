use std::collections::HashSet;
use std::thread;
use std::time::Duration;

use space_email_api::*;
use structopt::StructOpt;

mod args;

use args::Args;

struct SpaceEmailScraper {
    args: Args,
    client: SpaceEmailClient,
    emails: HashSet<SpaceEmail>,
    searched: usize,
    connected: bool,
    retries: usize,
}

impl SpaceEmailScraper {
    fn scrape_with(args: Args) {
        SpaceEmailScraper {
            retries: args.retries,
            args: args,
            client: SpaceEmailClient::new(),
            emails: HashSet::new(),
            searched: 0,
            connected: true,
        }.scrape();
    }
}

impl SpaceEmailScraper {
    fn scrape(&mut self) {
        loop {
            match self.client.get_random() {
                Ok(email) => {
                    if !self.connected { println!("Reconnected to Space Email."); }
                    self.connected = true;
                    self.searched += 1;
                    self.retries = self.args.retries;
                    if self.args.filter(&email) { self.save_email(email); }
                    match self.args.should_continue(self.searched, self.emails.len()) {
                        true => SpaceEmailScraper::wait_ms(self.args.cooldown_ms),
                        false => self.exit(),
                    }
                }
                Err(_) => self.handle_error(),
            }
        }
    }

    fn save_email(&mut self, email: SpaceEmail) {
        self.emails.insert(email);
    }

    fn wait_ms(ms: u64) {
        if ms != 0 { thread::sleep(Duration::from_millis(ms)); }
    }

    fn handle_error(&mut self) {
        if self.retries > 0 {
            self.retries -= 1;
            println!("Failed to reach Space Email. Retrying.");
            SpaceEmailScraper::wait_ms(self.args.cooldown_ms);
        } else { // disconnected
            let msg = match (self.args.no_reconnect, self.connected) {
                (true, _) => ("Space Email is not responding. Aborting."),
                (false, true) => ("Space Email is not responding. Will reconnect when possible."),
                (false, false) => ("Failed to reconnect to Space Email."),
            };

            println!("{}", msg);
            if self.args.no_reconnect { self.exit(); }
            SpaceEmailScraper::wait_ms(self.args.reconnect_ms);
        }
    }

    fn exit(&mut self) -> ! {
        let mut code = -1;
        if self.emails.len() == self.args.emails {
            println!("Finished downloading emails.");
            code = 0;
        }
        else if self.searched == self.args.max_volume {
            println!("Reached maximum search volume.");
            code = 0;
        }
        std::process::exit(code)
    }
}

fn main() {
    SpaceEmailScraper::scrape_with(Args::from_args());
}