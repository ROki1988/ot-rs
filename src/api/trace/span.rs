use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::api::trace::key::Value;
use crate::api::trace::span_context::{SpanContext, SpanId};

pub struct Span {
    context: SpanContext,
    name: String,
    start_time: SystemTime,
    finish_time: Option<SystemTime>,
    attributes: HashMap<String, Value>,
    parent_span_id: SpanId,
    links: Vec<Link>,
    events: Vec<TimedEvent>,
}

impl Span {
    pub fn context(&self) -> &SpanContext {
        &self.context
    }

    pub fn span_duration(&self) -> Option<Duration> {
        self.finish_time
            .and_then(|f| f.duration_since(self.start_time).ok())
    }

    pub fn add_link(&mut self, link: Link) {
        self.links.push(link);
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(TimedEvent::new(event));
    }

    pub fn get_links_ter(&self) -> impl Iterator<Item = &Link> {
        self.links.iter()
    }

    pub fn get_events_ter(&self) -> impl Iterator<Item = &TimedEvent> {
        self.events.iter()
    }
}

pub struct Link {
    span_context: SpanContext,
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
