use std::collections::HashMap;
use std::convert::TryFrom;

use crate::api::resources::Resource;
use crate::api::trace::key::Value;
use crate::api::trace::span_context::{SpanContext, SpanId};
use crate::api::trace::span_data::SpanData;
use crate::api::trace::status::Status;
use crate::api::trace::trace_context::TraceContext;
use crate::api::trace::{Event, Link, Span, TimedEvent, Timestamp, Tracer};

/// [Span spec](https://github.com/open-telemetry/opentelemetry-specification/blob/master/specification/api-tracing.md#span)
pub struct InMemorySpan<'a, 'b> {
    pub(crate) context: SpanContext<'a>,
    pub(crate) resource: &'a Resource,
    pub(crate) name: String,
    pub(crate) start_time: Timestamp,
    pub(crate) finish_time: Option<Timestamp>,
    pub(crate) attributes: HashMap<String, Value>,
    pub(crate) parent_span_id: Option<&'b SpanId>,
    pub(crate) links: Vec<Link<'a>>,
    pub(crate) events: Vec<TimedEvent>,
    pub(crate) status: Status,
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

impl<'a, 'b> Span<'a> for InMemorySpan<'a, 'b> {
    fn start(&mut self) {
        self.start_time = Timestamp::now();
    }

    fn context(&self) -> &SpanContext {
        &self.context
    }

    fn resource(&self) -> &Resource {
        self.resource
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

struct InMemoryTracer<'a, 'b> {
    current_trace: Option<TraceContext>,
    current_span: Option<InMemorySpan<'a, 'b>>,
    resource: Resource,
}

impl<'a, 'b> InMemoryTracer<'a, 'b> {
    fn current_trace(&self) -> Option<&TraceContext> {
        self.current_trace.as_ref()
    }
}

impl<'a, 'b> Tracer<'a, InMemorySpan<'a, 'b>> for InMemoryTracer<'a, 'b> {
    type Error = ();
    fn current_span(&self) -> Option<&InMemorySpan<'a, 'b>> {
        self.current_span.as_ref()
    }

    fn record_span_data(&self, value: InMemorySpan<'a, 'b>) -> Result<SpanData, Self::Error> {
        SpanData::try_from(value)
    }
}
