use discord_rich_presence::activity;
use moc_rs::{MocInfo, MocState};
use std::time::SystemTime;

pub fn make_activity<'a>(
    info: MocInfo,
    details: &'a str,
    custom_state: Option<&'a str>,
) -> activity::Activity<'a> {
    let mut state;
    if let Some(unwrapped_state) = custom_state {
        state = unwrapped_state;
    } else {
        state = match info.state() {
            MocState::Playing => "Playing...",
            MocState::Paused => "Playback paused!",
            MocState::Stopped => "Playback stopped!",
        };
    }

    if (info.state() == MocState::Paused) || (info.state() == MocState::Stopped) {
        state = match info.state() {
            MocState::Paused => "Playback paused!",
            MocState::Stopped => "Playback stopped!",
            _ => unreachable!(),
        };
    }
    let state = state;

    let smol_image: &'a str = match info.state() {
        MocState::Playing => "playbutton",
        MocState::Paused => "pausebutton",
        MocState::Stopped => "stopbutton",
    };

    let mut assets = activity::Assets::new()
        .large_image("moclogo")
        .large_text("Music on Console")
        .small_image(smol_image)
        .small_text(state);

    if !details.is_empty() {
        assets = assets.large_text(details);
    }

    let rn_unix = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let start_time = rn_unix - info.current_time();

    let mut timestamp = activity::Timestamps::new().start(start_time.as_secs() as i64);

    // a.k.a if its not a stream
    if !info.total_time().is_zero() {
        timestamp =
            timestamp.end((rn_unix + (info.total_time() - info.current_time())).as_secs() as i64)
        // this is a whole bunch of non-sense at first glance, pay attention to the parentheses!
        //
        // 1. get the current time of the track and subtract it from the total time
        // 2. add that to the current unix time
        // 3. get the Duration as seconds and cast it to an i64
    }

    let mut activity = activity::Activity::new().assets(assets).state(state);

    if !details.is_empty() {
        activity = activity.details(details);
    }

    if (info.state() != MocState::Paused) && (info.state() != MocState::Stopped) {
        activity = activity.timestamps(timestamp);
    }

    activity
}
