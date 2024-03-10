use crate::events::Event;

mod events;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let config = events::Config { round_floats: true };
    let disp = events::EventDispatcher::new(config);
    disp.event(Event::String("first".into())).await?;
    disp.event(Event::Float(10_f64)).await?;
    let _s = disp.subscribe();
    Ok(())
}
