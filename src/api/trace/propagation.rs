use crate::api::context::{
    HttpTextExtract, HttpTextFormat, HttpTextInject, ToHttpText, TryFromHttpText,
};
use crate::api::trace::span_context::{
    Entry, SpanContext, SpanId, TraceId, TraceOption, TraceState,
};
use crate::api::trace::trace_context::TraceContext;

const TRACEPARENT: &str = "traceparent";
const TRACESTATE: &str = "tracestate";
const FIELDS: [&str; 2] = [TRACEPARENT, TRACESTATE];
const VERSION: &str = "00";
const TRACEPARENT_DELIMITER: &str = "-";
const TRACESTATE_KEY_VALUE_DELIMITER: &str = "=";
const TRACESTATE_ENTRY_DELIMITER: &str = ",";

impl TryFromHttpText for Entry {
    type Err = ();

    fn try_from_http_text(s: &str) -> Result<Self, Self::Err> {
        let xs: Vec<&str> = s.splitn(2, TRACESTATE_KEY_VALUE_DELIMITER).collect();
        if xs.len() == 2 {
            xs.get(0)
                .and_then(|k| {
                    xs.get(1)
                        .and_then(|v| Entry::try_from(k.to_string(), v.to_string()))
                })
                .ok_or(())
        } else {
            Err(())
        }
    }
}

impl<'a> HttpTextFormat for SpanContext<'a> {
    fn fields(&self) -> &[&str] {
        &FIELDS
    }
}

impl<'a> ToHttpText for SpanContext<'a> {
    fn to_http_text(&self) -> String {
        [
            VERSION.to_string(),
            self.trace_id.to_base16(),
            self.span_id.to_base16(),
            self.trace_option.to_base16(),
        ]
        .join(TRACEPARENT_DELIMITER)
    }
}

impl ToHttpText for TraceState {
    fn to_http_text(&self) -> String {
        self.iter()
            .map(Entry::to_string)
            .collect::<Vec<String>>()
            .join(TRACESTATE_ENTRY_DELIMITER)
    }
}

impl<'a> HttpTextInject for SpanContext<'a> {
    fn inject<C, R>(&self, carrier: &mut C, setter: fn(&mut C, String, String) -> R) {
        setter(carrier, TRACEPARENT.to_owned(), self.to_http_text());
        if self.trace_state.has_entry() {
            setter(
                carrier,
                TRACESTATE.to_owned(),
                self.trace_state.to_http_text(),
            );
        }
    }
}

impl HttpTextFormat for TraceContext {
    fn fields(&self) -> &[&str] {
        &FIELDS
    }
}

impl TryFromHttpText for TraceState {
    type Err = ();

    fn try_from_http_text(s: &str) -> Result<Self, Self::Err> {
        s.split(TRACESTATE_ENTRY_DELIMITER)
            .map(Entry::try_from_http_text)
            .take(Self::max_entry_size())
            .collect::<Result<Vec<_>, _>>()
            .map(Self)
    }
}

impl TryFromHttpText for TraceContext {
    type Err = ();

    fn try_from_http_text(s: &str) -> Result<Self, Self::Err> {
        let xs: Vec<_> = s.splitn(4, TRACEPARENT_DELIMITER).collect();
        if xs.get(0) != Some(&VERSION) {
            return Err(());
        }

        xs.get(1)
            .and_then(|x| TraceId::try_from_base16(x))
            .and_then(|t| {
                xs.get(2)
                    .and_then(|y| SpanId::try_from_base16(y))
                    .and_then(|s| {
                        xs.get(3)
                            .and_then(|z| TraceOption::try_from_base16(z))
                            .map(|o| TraceContext::new_without_trace_state(t, s, o))
                    })
            })
            .ok_or(())
    }
}

impl HttpTextExtract for TraceContext {
    fn extract<C>(
        carrier: &C,
        getter: for<'r> fn(&'r C, &str) -> Option<&'r String>,
    ) -> Option<Self> {
        getter(carrier, &TRACEPARENT)
            .and_then(|v| Self::try_from_http_text(v).ok())
            .map(|mut x| {
                let o = getter(carrier, &TRACESTATE)
                    .and_then(|y| TraceState::try_from_http_text(y).ok())
                    .unwrap_or_else(TraceState::empty);
                x.with_trace_state(o);
                x
            })
    }
}

#[test]
fn http_trace_context_inject() {
    use std::collections::HashMap;
    use std::num::{NonZeroU128, NonZeroU64};

    let t = TraceId::new(NonZeroU128::new(42).unwrap());

    let i = SpanContext::new(
        &t,
        SpanId::new(NonZeroU64::new(42).unwrap()),
        TraceOption::MASK_SAMPLE,
        TraceState::empty(),
    );
    let mut m: HashMap<String, String> = HashMap::new();
    i.inject(&mut m, HashMap::insert);
    assert_eq!(m.keys().next().map(String::as_str), Some("traceparent"));
    assert_eq!(
        m.values().next().map(String::as_str),
        Some("00-2a000000000000000000000000000000-2a00000000000000-01")
    )
}

#[test]
fn http_trace_context_extract() {
    use std::collections::HashMap;
    use std::num::{NonZeroU128, NonZeroU64};

    let t = TraceId::new(NonZeroU128::new(42).unwrap());

    let e = SpanContext::new(
        &t,
        SpanId::new(NonZeroU64::new(42).unwrap()),
        TraceOption::MASK_SAMPLE,
        TraceState::empty(),
    );
    let mut m: HashMap<String, String> = HashMap::new();
    m.insert(
        "traceparent".to_owned(),
        "00-2a000000000000000000000000000000-2a00000000000000-01".to_owned(),
    );
    let a = TraceContext::extract(&m, HashMap::get);
    assert!(a.is_some());
    let aa = a.unwrap();
    assert_eq!(e.trace_id, &aa.trace_id);
    assert_eq!(e.span_id, aa.span_id);
    assert_eq!(e.trace_option, aa.trace_option);
    assert_eq!(e.trace_state, aa.trace_state);
}

#[test]
fn span_context_convert_base16() {
    let t = TraceId::generate_random();
    let e = SpanContext::new(
        &t,
        SpanId::generate_random(),
        TraceOption::MASK_SAMPLE,
        TraceState::empty(),
    );
    let a = TraceContext::try_from_http_text(&e.to_http_text());

    assert!(a.is_ok());
    let aa = a.unwrap();
    assert_eq!(e.trace_id, &aa.trace_id);
    assert_eq!(e.span_id, aa.span_id);
    assert_eq!(e.trace_option, aa.trace_option);
    assert_eq!(e.trace_state, aa.trace_state);
}
