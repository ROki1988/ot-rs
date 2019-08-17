use std::num::{NonZeroU128, NonZeroU64};

use ot_rs::api::trace::span_context::{SpanContext, SpanId, TraceId, TraceOption, TraceState};

#[test]
fn span_context_valid() {
    let sid = NonZeroU64::new(42).unwrap();
    let tid = NonZeroU128::new(42).unwrap();
    let t = TraceId::new(tid);
    let a = SpanContext::new(
        &t,
        SpanId::new(sid),
        TraceOption::MASK_SAMPLE,
        TraceState::empty(),
    );

    assert_eq!(a.span_id_str(), "000000000000002a".to_owned());
    assert_eq!(
        a.trace_id_str(),
        "0000000000000000000000000000002a".to_owned()
    );
    assert_eq!(a.span_id.to_base16(), "2a00000000000000".to_owned());
    assert_eq!(
        a.trace_id.to_base16(),
        "2a000000000000000000000000000000".to_owned()
    )
}

#[test]
fn trace_id_compare() {
    let a = TraceId::generate_random();
    let b = TraceId::generate_random();

    assert_ne!(a, b);
}

#[test]
fn span_id_compare() {
    let a = SpanId::generate_random();
    let b = SpanId::generate_random();

    assert_ne!(a, b);
}

#[test]
fn trace_id_convert_base16() {
    let e = TraceId::generate_random();
    let a = TraceId::try_from_base16(e.to_base16().as_str());
    assert!(a.is_some());
    assert_eq!(e, a.unwrap())
}

#[test]
fn span_id_convert_base16() {
    let e = SpanId::generate_random();
    let a = SpanId::try_from_base16(e.to_base16().as_str());
    assert!(a.is_some());
    assert_eq!(e, a.unwrap())
}
