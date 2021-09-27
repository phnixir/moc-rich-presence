#![allow(unreachable_code)]
#![forbid(unsafe_code)]

use discord_rich_presence::{activity, new_client, DiscordIpc};
use moc_rs::{Moc, MocInterface, MocSource, MocState};
use std::time::Duration;

pub mod make_activity;
use make_activity::make_activity;

static INTERVAL: Duration = Duration::from_secs(5);
// interval to re-query the MoC server for new information

fn main() {
    let mut client = new_client("891580272875819038").unwrap();

    client.connect().unwrap();
    let mut last_ftitle = String::with_capacity(32);

    loop {
        let moc = Moc::new("mocp".into());

        let mut ftitle = String::from("");

        if moc.info().full_title().is_empty() {
            if let MocSource::File(path) = moc.info().file() {
                if moc.info().state() != MocState::Stopped {
                    ftitle = Box::new(path.iter().last().unwrap())
                        .to_os_string()
                        .into_string()
                        .unwrap()
                }
            }
        } else {
            ftitle = moc.info().full_title();
        }

        let url = match moc.info().file() {
            MocSource::Url(url) => url,
            _ => String::from(""),
        };

        let mut button = vec![];
        let mut streaming = None;

        if !url.is_empty() {
            button.push(activity::Button::new(
                "Listen along to the stream!",
                url.as_str(),
            ));
            streaming = Some("Streaming...");
        }

        if ftitle != last_ftitle {
            last_ftitle = ftitle.clone();
            let mut activity = make_activity(moc.info(), &ftitle, streaming);
            if !button.is_empty() {
                activity = activity.buttons(button);
            }

            client.set_activity(activity.clone()).unwrap();
        } else {
            let mut activity = make_activity(moc.info(), &last_ftitle, streaming);
            if !button.is_empty() {
                activity = activity.buttons(button);
            }

            client.set_activity(activity.clone()).unwrap();
        }
        std::thread::sleep(INTERVAL);
    }
    client.close().unwrap();
}
