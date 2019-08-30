use crate::api::trace::Span;

struct Scope<F, S> {
    f: F,
    span: S,
}

impl<'a, F, U, S> Scope<F, S>
where
    F: Fn() -> U,
    S: Span<'a>,
{
    fn new(span: S, f: F) -> Self {
        Self { f, span }
    }

    fn run(&mut self) -> U {
        let f = &self.f;
        let rt = f();
        self.span.end();

        rt
    }
}

#[test]
fn scope_test() {
    use crate::api::trace::in_memory::InMemorySpan;
    use crate::api::trace::span_context::SpanContext;
    use crate::api::trace::span_context::{SpanId, TraceId, TraceOption, TraceState};
    use crate::api::trace::status::Status;
    use crate::api::trace::trace_context::TraceContext;
    use crate::api::trace::Timestamp;
    use std::collections::HashMap;

    let t = TraceContext::new(
        TraceId::generate_random(),
        SpanId::generate_random(),
        TraceOption::MASK_SAMPLE,
        TraceState::empty(),
    );

    let s = SpanContext::new(
        &t.trace_id,
        SpanId::generate_random(),
        TraceOption::MASK_SAMPLE,
        TraceState::empty(),
    );

    let span = InMemorySpan {
        context: s,
        name: "test".to_owned(),
        start_time: Timestamp::now(),
        finish_time: None,
        attributes: HashMap::new(),
        parent_span_id: None,
        links: Vec::new(),
        events: Vec::new(),
        status: Status::ok(),
    };
    let mut s = Scope::new(span, || 1);
    let a = s.run();

    assert_eq!(a, 1);
    assert!(s.span.finish_time.is_some());
}
