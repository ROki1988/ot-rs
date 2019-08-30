use crate::api::trace::in_memory::InMemorySpan;
use crate::api::trace::key::Value;
use crate::api::trace::span_context::{SpanContext, SpanId, TraceId, TraceOption, TraceState};
use crate::api::trace::status::Status;
use crate::api::trace::{Link, SpanKind, TimedEvent, Timestamp};
use std::collections::HashMap;
use std::convert::TryFrom;

struct ImmutableSpanContext {
    trace_id: TraceId,
    span_id: SpanId,
    trace_option: TraceOption,
    trace_state: TraceState,
}

impl<'a> From<&SpanContext<'a>> for ImmutableSpanContext {
    fn from(value: &SpanContext<'a>) -> Self {
        Self {
            trace_id: value.trace_id.clone(),
            span_id: value.span_id.clone(),
            trace_option: value.trace_option,
            trace_state: value.trace_state.clone(),
        }
    }
}

impl<'a> From<SpanContext<'a>> for ImmutableSpanContext {
    fn from(value: SpanContext<'a>) -> Self {
        Self {
            trace_id: value.trace_id.clone(),
            span_id: value.span_id,
            trace_option: value.trace_option,
            trace_state: value.trace_state,
        }
    }
}

struct ImmutableLink {
    span_context: ImmutableSpanContext,
    attributes: HashMap<String, Value>,
}

impl<'a> From<&Link<'a>> for ImmutableLink {
    fn from(value: &Link<'a>) -> Self {
        Self {
            span_context: ImmutableSpanContext::from(&value.span_context),
            attributes: value.attributes.clone(),
        }
    }
}

impl<'a> From<Link<'a>> for ImmutableLink {
    fn from(value: Link<'a>) -> Self {
        Self {
            span_context: ImmutableSpanContext::from(&value.span_context),
            attributes: value.attributes,
        }
    }
}

/// Immutable and Independent Span data
pub struct SpanData {
    // should not has lifetime, maybe
    context: ImmutableSpanContext,
    parent_span_id: Option<SpanId>,
    name: String,
    kind: SpanKind,
    start_time: Timestamp,
    end_time: Timestamp,
    attributes: HashMap<String, Value>,
    events: Vec<TimedEvent>,
    links: Vec<ImmutableLink>,
    status: Status,
}

impl<'a, 'b> TryFrom<&InMemorySpan<'a, 'b>> for SpanData {
    type Error = ();

    fn try_from(value: &InMemorySpan<'a, 'b>) -> Result<Self, Self::Error> {
        let ft = value.finish_time.clone().ok_or(())?;
        Ok(Self {
            context: ImmutableSpanContext::from(&value.context),
            parent_span_id: value.parent_span_id.cloned(),
            name: value.name.clone(),
            kind: SpanKind::INTERNAL,
            start_time: value.start_time.clone(),
            end_time: ft,
            attributes: value.attributes.clone(),
            events: value.events.clone(),
            links: value.links.iter().map(ImmutableLink::from).collect(),
            status: value.status.clone(),
        })
    }
}

impl<'a, 'b> TryFrom<InMemorySpan<'a, 'b>> for SpanData {
    type Error = ();

    fn try_from(value: InMemorySpan<'a, 'b>) -> Result<Self, Self::Error> {
        let ft = value.finish_time.clone().ok_or(())?;
        Ok(Self {
            context: ImmutableSpanContext::from(&value.context),
            parent_span_id: value.parent_span_id.cloned(),
            name: value.name,
            kind: SpanKind::INTERNAL,
            start_time: value.start_time,
            end_time: ft,
            attributes: value.attributes,
            events: value.events,
            links: value.links.into_iter().map(ImmutableLink::from).collect(),
            status: value.status,
        })
    }
}
