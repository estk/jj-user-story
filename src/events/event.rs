pub enum EventInner {
    String(String),
    Float(f64),
}

pub struct Event {
    pub origin: crate::EventOrigin,
    pub inner: EventInner,
}

pub fn str_event(origin: crate::EventOrigin, s: impl ToString) -> Event {
    let inner = EventInner::String(s.to_string());
    Event { origin, inner }
}
pub fn float_event(origin: crate::EventOrigin, f: f64) -> Event {
    Event {
        origin,
        inner: EventInner::Float(f),
    }
}
