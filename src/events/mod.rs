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

        let worker = Self::spawn(c.round_floats, in_rx, out_tx);
        Self {
            out_rx,
            in_tx,
            _worker: worker,
        }
    }
    fn spawn(
        round_floats: bool,
        mut in_rx: mpsc::Receiver<Event>,
        mut out_tx: broadcast::Sender<String>,
    ) -> JoinHandle<anyhow::Result<()>> {
        tokio::spawn(async move {
            let mut last_float_event = None;
            loop {
                select! {
                    Some(e) = in_rx.recv() => {
                        let msg = match e {
                            Event::String(s) => s,
                            Event::Float(f) => {
                                let f = if round_floats {f.round()} else {f};
                                format!("Float: {}", f)
                            },
                        };
                        dispatch(&msg, &mut out_tx)?;
                    },
                    _ = interval.tick() => {
                        if let Some(msg) = &last_float_event {
                            dispatch(msg, &mut out_tx)?;
                        }
                    }
                }
            }
        })
    }
}

fn dispatch(msg: &str, out_tx: &mut broadcast::Sender<String>) -> anyhow::Result<()> {
    eprintln!("{msg}");
    if out_tx.send(msg.to_string()).is_err() {
        anyhow!("No listeners");
    }
    Ok(())
}
