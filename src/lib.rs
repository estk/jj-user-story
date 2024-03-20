use std::time::Duration;

use clap::Parser;

use crate::events::{float_event, str_event};

pub mod events;

#[derive(clap::Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    /// Should float events be rounded
    #[arg(long, default_value_t = true)]
    round_floats: bool,

    /// Interval at which float events will be replayed.
    /// If absent or null, said events will not be replayed.
    #[arg(long)]
    float_replay_interval_minutes: Option<u64>,
}

impl Cli {
    pub async fn run() -> anyhow::Result<()> {
        let args = Self::parse();
        dbg!(&args);

        let config = args.into();
        // todo: get from config?
        let origin = EventOrigin(0);

        let disp = events::EventDispatcher::new(config);
        disp.event(str_event(origin, "first event body")).await?;
        disp.event(float_event(origin, 10_f64)).await?;
        let _s = disp.subscribe();

        tokio::time::sleep(Duration::from_secs(65)).await;
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct EventOrigin(u64);
