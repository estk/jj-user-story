#![cfg(test)]
use super::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_replay() {
    let mut disp = EventDispatcher::new(Config {
        replay_interval: Duration::from_millis(100),
        round_floats: true,
    });
    disp.event(Event::String("foobar".into())).await.unwrap();
    disp.event(Event::Float(3_f64)).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    let mut float_count = 0;
    let mut foo_count = 0;
    while let Ok(evt) = disp.try_recv() {
        if evt.contains("foo") {
            foo_count += 1;
        }
        if evt.contains('3') {
            float_count += 1;
        }
    }
    assert!(foo_count == 1, "Didnt expect replayed foo msg");
    assert!(float_count > 1, "Floats replay failed");
}
