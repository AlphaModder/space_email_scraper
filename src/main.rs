use std::collections::HashSet;
use std::thread;
use std::time::Duration;

use space_email_api::*;
use structopt::StructOpt;

mod args;

use args::Args;

pub static RECONNECT_FAILURE_MSG: &str = "Failed to reconnect to Space Email.";
pub static DISCONNECT_MSG: &str = "Space Email is not responding. Will reconnect when possible.";

struct Context {
    args: Args,
    client: SpaceEmailClient,
    emails: HashSet<SpaceEmail>,
    searched: usize,
}

#[derive(PartialEq, Eq)]
enum State {
    Scraping(usize, &'static str),
    Waiting(usize),
    Disconnected,
    Exiting,
}

impl State {
    fn advance(self, ctx: &mut Context) -> State {
        match self {
            State::Scraping(retries, dc_msg) => {
                match ctx.client.get_random() {
                    Ok(email) => {
                        ctx.searched += 1;
                        if ctx.args.filter(&email) { ctx.emails.insert(email); }
                        match ctx.args.should_continue(ctx.searched, ctx.emails.len()) {
                            true => State::Waiting(ctx.args.retries),
                            false => State::Exiting,
                        }
                    }
                    Err(_) => match (retries, ctx.args.no_reconnect) {
                        (0, false) => {
                            println!("{}", dc_msg);
                            State::Disconnected
                        }
                        (0, true) => {
                            println!("Space Email is not responding. Aborting.");
                            State::Exiting
                        }
                        _ => {
                            println!("Failed to reach Space Email. Retrying.");
                            State::Waiting(retries - 1)
                        }
                    }
                }
            },
            State::Waiting(tries) => {
                thread::sleep(Duration::from_millis(ctx.args.cooldown_ms));
                State::Scraping(tries, DISCONNECT_MSG)
            }
            State::Disconnected => {
                thread::sleep(Duration::from_millis(ctx.args.reconnect_ms));
                let new_state = State::Scraping(1, RECONNECT_FAILURE_MSG).advance(ctx);
                if new_state != State::Disconnected { println!("Reconnected to Space Email.") }
                new_state
            }
            State::Exiting => {
                let mut code = -1;
                if ctx.emails.len() == ctx.args.emails {
                    println!("Finished downloading emails.");
                    code = 0;
                }
                else if ctx.searched == ctx.args.max_volume {
                    println!("Reached maximum search volume.");
                    code = 0;
                }
                std::process::exit(code)
            }
        }
    }
}

fn main() {
    let mut ctx = Context {
        args: Args::from_args(),
        client: SpaceEmailClient::new(),
        emails: HashSet::new(),
        searched: 0,
    };
    let mut state = State::Scraping(ctx.args.retries, DISCONNECT_MSG);
    loop { state = state.advance(&mut ctx); }
}