use crate::api::trace::key::Value;
use crate::api::trace::span_context::SpanContext;
use crate::api::trace::span_data::SpanData;
use crate::api::trace::status::Status;
use std::collections::HashMap;
use std::convert::TryInto;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub mod in_memory;
pub mod key;
pub mod propagation;
pub mod scope;
pub mod span_context;
pub mod span_data;
pub mod status;
pub mod trace_context;

trait Tracer<'a, S>
where
    S: Span<'a> + TryInto<SpanData>,
{
    type Error;
    fn current_span(&self) -> Option<&S>;
    fn record_span_data(&self, value: S) -> Result<SpanData, Self::Error>;
}

enum SpanKind {
    INTERNAL,
    SERVER,
    CLIENT,
    PRODUCER,
    CONSUMER,
}

trait Span<'a>: Sized + Sync {
    fn start(&mut self);

    fn context(&self) -> &SpanContext;

    fn is_recording_events(&self) -> bool;

    fn add_link(&mut self, link: Link<'a>);

    fn add_event(&mut self, event: Event);

    fn set_attribute(&mut self, key: String, value: Value);

    fn update_name(&mut self, name: &str);

    fn set_status(&mut self, next: Status);

    fn end(&mut self);
}

pub struct Link<'a> {
    span_context: SpanContext<'a>,
    attributes: HashMap<String, Value>,
}

pub struct Event {
    name: String,
    attributes: HashMap<String, Value>,
}

impl Event {
    fn new(name: &str) -> Self {
        Self::new_with_attributes(name, HashMap::new())
    }

    fn new_with_attributes(name: &str, attributes: HashMap<String, Value>) -> Self {
        Self {
            name: name.to_string(),
            attributes,
        }
    }
}

#[derive(Clone)]
pub struct TimedEvent {
    timestamp: Timestamp,
    name: String,
    attributes: HashMap<String, Value>,
}

impl TimedEvent {
    pub fn new(event: Event) -> Self {
        Self::new_with_timestamp(Timestamp::now(), event)
    }

    fn new_with_timestamp(timestamp: Timestamp, event: Event) -> Self {
        Self {
            timestamp,
            name: event.name,
            attributes: event.attributes,
        }
    }
}

/// [Timestamp spec](https://github.com/open-telemetry/opentelemetry-specification/blob/master/specification/api-tracing.md#timestamp)
#[derive(Clone)]
pub(crate) struct Timestamp(SystemTime);

impl Timestamp {
    fn now() -> Self {
        Self(SystemTime::now())
    }
    fn as_millis(&self) -> u128 {
        self.0
            .duration_since(UNIX_EPOCH)
            .as_ref()
            .map(Duration::as_millis)
            .unwrap()
    }
    fn as_micros(&self) -> u128 {
        self.0
            .duration_since(UNIX_EPOCH)
            .as_ref()
            .map(Duration::as_micros)
            .unwrap()
    }
    fn as_nanos(&self) -> u128 {
        self.0
            .duration_since(UNIX_EPOCH)
            .as_ref()
            .map(Duration::as_nanos)
            .unwrap()
    }

    fn duration_since_as_millis(&self, other: &Self) -> Option<u128> {
        self.0
            .duration_since(other.0)
            .as_ref()
            .map(Duration::as_millis)
            .ok()
    }
    fn duration_since_as_micros(&self, other: &Self) -> Option<u128> {
        self.0
            .duration_since(other.0)
            .as_ref()
            .map(Duration::as_micros)
            .ok()
    }
    fn duration_since_as_nanos(&self, other: &Self) -> Option<u128> {
        self.0
            .duration_since(other.0)
            .as_ref()
            .map(Duration::as_nanos)
            .ok()
    }
}

#[test]
fn timestamp_to_primitive() {
    let a = Timestamp(UNIX_EPOCH.checked_add(Duration::from_secs(1)).unwrap());
    assert_eq!(a.as_millis(), 1000);
    assert_eq!(a.as_micros(), 1000_000);
    assert_eq!(a.as_nanos(), 1000_000_000);
}

#[test]
fn timestamp_to_duration() {
    let a1 = Timestamp(UNIX_EPOCH.checked_add(Duration::from_secs(1)).unwrap());
    let a2 = Timestamp(UNIX_EPOCH);

    assert_eq!(a1.duration_since_as_millis(&a2), Some(1000));
    assert_eq!(a1.duration_since_as_micros(&a2), Some(1000_000));
    assert_eq!(a1.duration_since_as_nanos(&a2), Some(1000_000_000));
}
