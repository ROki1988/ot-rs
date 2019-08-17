use crate::api::trace::span_context::{SpanId, TraceId, TraceOption, TraceState};

pub struct TraceContext {
    pub trace_id: TraceId,
    pub span_id: SpanId,
    pub trace_option: TraceOption,
    pub trace_state: TraceState,
}

impl TraceContext {
    pub fn new(
        trace_id: TraceId,
        span_id: SpanId,
        trace_option: TraceOption,
        trace_state: TraceState,
    ) -> Self {
        Self {
            trace_id,
            span_id,
            trace_option,
            trace_state,
        }
    }

    pub fn new_without_trace_state(
        trace_id: TraceId,
        span_id: SpanId,
        trace_option: TraceOption,
    ) -> Self {
        Self::new(trace_id, span_id, trace_option, TraceState::empty())
    }

    pub fn set_trace_state(&mut self, trace_state: TraceState) {
        self.trace_state = trace_state;
    }

    pub fn with_trace_state(&mut self, trace_state: TraceState) -> &mut Self {
        self.set_trace_state(trace_state);
        self
    }
}
