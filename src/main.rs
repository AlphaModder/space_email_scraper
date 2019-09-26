mod args;
mod args2;
mod filter;

use space_email_api::*;
use args::Arguments;
use std::cell::RefCell;
use std::collections::HashSet;

struct SpaceEmailScraper {
    arguments: Arguments,
    client: SpaceEmailClient,
    emails: HashSet<SpaceEmail>,
    searched: usize,
    tries: u32,
    paused: bool,
}

impl SpaceEmailScraper {

    fn new(arguments: Arguments) -> SpaceEmailScraper {
        SpaceEmailScraper {
            arguments: arguments,
            client: SpaceEmailClient::new(),
            emails: RefCell::new(HashSet::with_capacity(arguments.emails)),
            searched: 0,
            tries: 0,
            paused: false,
        }
    }

    fn run(&mut self) {
        while (self.emails.len() < self.arguments.emails) && (self.searched < self.arguments.search_volume) {
            match self.client.get_random() {
                Ok(email) => self.handle_email(email),
                Err(e) => if !self.handle_error(e) { return },
            };
            thread::sleep(self.arguments.cooldown);
        }

        if(self.searched == self.arguments.search_volume) {
            self.print("Reached maximum search volume.");
        }
    }

    fn handle_email(&mut self, email: SpaceEmail) {
        if(paused) {
            self.print("Reconnected to Space Email.");
            paused = false;
        }

        if(self.arguments.filter.matches(&email) && !self.emails.contains(&email)) {
            self.emails.insert(email);
        }
    }

    fn handle_error(&mut self, error: SpaceEmailError) -> bool {
        if(self.paused) {
            self.print("Failed to reconnect to Space Email.");
        }
        else {
            self.tries += 1;
            if(self.tries < self.arguments.tries) {
                self.print("Failed to reach Space Email. Retrying.");
            }
            else {
                match self.arguments.reconnect {
                    Some(_) => {
                        self.print("Space Email is not responding. Will reconnect when possible.");
                        self.paused = true;
                        self.tries = 0;
                    }
                    None => {
                        self.print("Space Email is not responding. Aborting.");
                        return false
                    }
                }
            }
        }

        if(self.paused) {
            thread::sleep(self.arguments.reconnect.unwrap());
        }

        true
    }

    fn print(text: &str) {

    }

}

fn main() {

}