use std::future;
use std::time::Duration;

use anyhow::anyhow;
use tokio::select;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;
use tokio::task::JoinHandle;

mod config;
mod event;

pub use config::Config;
pub use event::Event;
pub use event::{float_event, str_event};
use tokio::time::Interval;

use crate::events::event::EventInner;

pub struct EventDispatcher {
    out_rx: broadcast::Receiver<String>,
    in_tx: mpsc::Sender<Event>,
    _worker: JoinHandle<anyhow::Result<()>>,
}

const IN_BUF_SIZE: usize = 10;
const OUT_BUF_SIZE: usize = 10;

impl EventDispatcher {
    pub fn try_recv(&mut self) -> Result<String, broadcast::error::TryRecvError> {
        self.out_rx.try_recv()
    }
    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.out_rx.resubscribe()
    }
    pub async fn event(&self, e: Event) -> Result<(), SendError<Event>> {
        self.in_tx.send(e).await
    }
    pub fn new(c: Config) -> Self {
        let (in_tx, in_rx) = mpsc::channel(IN_BUF_SIZE);
        let (out_tx, out_rx) = broadcast::channel(OUT_BUF_SIZE);

        let worker = Self::spawn(c.replay_interval, c.round_floats, in_rx, out_tx);
        Self {
            out_rx,
            in_tx,
            _worker: worker,
        }
    }
    fn spawn(
        replay_interval: Option<Duration>,
        round_floats: bool,
        mut in_rx: mpsc::Receiver<Event>,
        mut out_tx: broadcast::Sender<String>,
    ) -> JoinHandle<anyhow::Result<()>> {
        tokio::spawn(async move {
            let mut last_float_event = None;
            let mut interval = replay_interval.map(tokio::time::interval);
            loop {
                select! {
                    Some(Event{origin, inner}) = in_rx.recv() => {
                        let msg = match inner {
                            EventInner::String(s) => s,
                            EventInner::Float(f) => {
                                let f = if round_floats {f.round()} else {f};
                                last_float_event.replace(f);

                                f.to_string()
                            },
                        };
                        dispatch(&msg, &mut out_tx)?;
                    },
                    _ = tick(&mut interval) => {
                        if let Some(msg) = &last_float_event {
                            dispatch(msg, &mut out_tx)?;
                        }
                    }
                }
            }
        })
    }
}

async fn tick(interval: &mut Option<Interval>) {
    if let Some(x) = interval {
        x.tick().await;
    } else {
        future::pending::<()>().await;
    }
}

fn dispatch(msg: impl ToString, out_tx: &mut broadcast::Sender<String>) -> anyhow::Result<()> {
    let msg = msg.to_string();

    eprintln!("{msg}");
    if out_tx.send(msg.to_string()).is_err() {
        return Err(anyhow!("No listeners"));
    }
    Ok(())
}
