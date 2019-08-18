use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::api::trace::key::Value;
use crate::api::trace::span_context::{SpanContext, SpanId};
use crate::api::trace::status::Status;

/// [Timestamp spec](https://github.com/open-telemetry/opentelemetry-specification/blob/master/specification/api-tracing.md#timestamp)
struct Timestamp(SystemTime);

impl Timestamp {
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

/// [Span spec](https://github.com/open-telemetry/opentelemetry-specification/blob/master/specification/api-tracing.md#span)
pub struct Span<'a, 'b, 'c> {
    context: SpanContext<'a>,
    name: String,
    start_time: Timestamp,
    finish_time: Option<Timestamp>,
    attributes: HashMap<String, Value>,
    parent_span_id: &'b SpanId,
    links: Vec<Link<'c>>,
    events: Vec<TimedEvent>,
    status: Status,
}

impl<'a, 'b, 'c> Span<'a, 'b, 'c> {
    fn context(&self) -> &SpanContext {
        &self.context
    }

    const fn is_recording_events(&self) -> bool {
        true
    }

    fn span_duration_as_millis(&self) -> Option<u128> {
        self.finish_time
            .as_ref()
            .and_then(|fin| fin.duration_since_as_millis(&self.start_time))
    }
    fn span_duration_as_micros(&self) -> Option<u128> {
        self.finish_time
            .as_ref()
            .and_then(|fin| fin.duration_since_as_micros(&self.start_time))
    }
    fn span_duration_as_nanos(&self) -> Option<u128> {
        self.finish_time
            .as_ref()
            .and_then(|fin| fin.duration_since_as_nanos(&self.start_time))
    }

    fn add_link(&mut self, link: Link<'c>) {
        self.links.push(link);
    }

    fn add_event(&mut self, event: Event) {
        self.events.push(TimedEvent::new(event));
    }

    fn get_links_iter(&self) -> impl Iterator<Item = &Link> {
        self.links.iter()
    }

    fn get_events_iter(&self) -> impl Iterator<Item = &TimedEvent> {
        self.events.iter()
    }

    fn get_attributes_iter(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.attributes.iter()
    }

    fn get_start_epoch_time_as_millis(&self) -> u128 {
        self.start_time.as_millis()
    }

    fn get_finish_epoch_time_as_millis(&self) -> Option<u128> {
        self.finish_time.as_ref().map(Timestamp::as_millis)
    }

    fn get_start_epoch_time_as_micros(&self) -> u128 {
        self.start_time.as_micros()
    }

    fn get_finish_epoch_time_as_micros(&self) -> Option<u128> {
        self.finish_time.as_ref().map(Timestamp::as_micros)
    }
    fn get_start_epoch_time_as_nanos(&self) -> u128 {
        self.start_time.as_nanos()
    }

    fn get_finish_epoch_time_as_nanos(&self) -> Option<u128> {
        self.finish_time.as_ref().map(Timestamp::as_nanos)
    }

    fn set_status(&mut self, next: Status) {
        self.status = next;
    }
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

pub struct TimedEvent {
    timestamp: SystemTime,
    name: String,
    attributes: HashMap<String, Value>,
}

impl TimedEvent {
    pub fn new(event: Event) -> Self {
        Self::new_with_timestamp(UNIX_EPOCH, event)
    }

    pub(crate) fn new_with_timestamp(timestamp: SystemTime, event: Event) -> Self {
        Self {
            timestamp,
            name: event.name,
            attributes: event.attributes,
        }
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