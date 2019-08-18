use std::collections::HashMap;
use crate::api::trace::key::Value;
use crate::api::trace::span_context::{SpanContext, SpanId};
use crate::api::trace::status::Status;
use crate::api::trace::{Timestamp, Link, TimedEvent, Event, Span};

/// [Span spec](https://github.com/open-telemetry/opentelemetry-specification/blob/master/specification/api-tracing.md#span)
pub struct InMemorySpan<'a, 'b> {
    context: SpanContext<'a>,
    name: String,
    start_time: Timestamp,
    finish_time: Option<Timestamp>,
    attributes: HashMap<String, Value>,
    parent_span_id: Option<&'b SpanId>,
    links: Vec<Link<'a>>,
    events: Vec<TimedEvent>,
    status: Status,
}

impl<'a, 'b> InMemorySpan<'a, 'b> {
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
}

impl<'a, 'b>  Span<'a> for InMemorySpan<'a, 'b>  {
    fn context(&self) -> &SpanContext {
        &self.context
    }

    fn is_recording_events(&self) -> bool {
        true
    }


    fn add_link(&mut self, link: Link<'a>) {
        self.links.push(link);
    }

    fn add_event(&mut self, event: Event) {
        self.events.push(TimedEvent::new(event));
    }

    fn set_attribute(&mut self, key: String, value: Value) {
        self.attributes.insert(key, value);
    }

    fn update_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn set_status(&mut self, next: Status) {
        self.status = next;
    }

    fn end(&mut self) {
        self.finish_time = Some(Timestamp::now());
    }

}
