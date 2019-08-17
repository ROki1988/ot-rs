use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::api::trace::key::Value;
use crate::api::trace::span_context::{SpanContext, SpanId};

pub struct Span<'a, 'b> {
    context: SpanContext<'a>,
    name: String,
    start_time: Option<SystemTime>,
    finish_time: Option<SystemTime>,
    attributes: HashMap<String, Value>,
    parent_span_id: &'b SpanId,
    links: Vec<Link<'b>>,
    events: Vec<TimedEvent>,
}

impl<'a, 'b> Span<'a, 'b> {
    fn context(&self) -> &SpanContext {
        &self.context
    }

    fn span_duration(&self) -> Option<Duration> {
        self.start_time.and_then(|s| {
            self.finish_time.and_then(|fin| {
                fin.duration_since(s).ok()
            })
        })
    }

    fn add_link(&mut self, link: Link<'b>) {
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
