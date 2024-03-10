use tokio::select;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;
use tokio::task::JoinHandle;

pub struct Config {
    pub round_floats: bool,
}
pub enum Event {
    String(String),
    Float(f64),
}
pub struct EventDispatcher {
    out_rx: broadcast::Receiver<String>,
    in_tx: mpsc::Sender<Event>,
    _worker: JoinHandle<()>,
}

const IN_BUF_SIZE: usize = 10;
const OUT_BUF_SIZE: usize = 10;

impl EventDispatcher {
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
        out_tx: broadcast::Sender<String>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
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
                        eprintln!("{msg}");
                        if out_tx.send(msg).is_err() {
                            // no listeners
                            return
                        }
                    }
                }
            }
        })
    }
}
