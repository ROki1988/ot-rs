use std::str::FromStr;

use crate::api::trace::HttpTextFormat;
use crate::api::trace::span_context::{
    Entry, SpanContext, SpanId, TraceId, TraceOption, TraceState,
};

const TRACEPARENT: &str = "traceparent";
const TRACESTATE: &str = "tracestate";
const FIELDS: [&str; 2] = [TRACEPARENT, TRACESTATE];
const VERSION: &str = "00";
const TRACEPARENT_DELIMITER: &str = "-";
const TRACESTATE_KEY_VALUE_DELIMITER: &str = "=";
const TRACESTATE_ENTRY_DELIMITER: &str = ",";

impl FromStr for Entry {
    // TODO: define error
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

impl TraceState {
    fn try_from_http_text(trace_state_str: &str) -> Self {
        let x: Vec<_> = trace_state_str
            .split(TRACESTATE_ENTRY_DELIMITER)
            .collect::<Vec<&str>>()
            .into_iter()
            .map(Entry::from_str)
            .filter_map(Result::ok)
            .take(Self::max_entry_size())
            .collect();
        Self(x)
    }
}

impl SpanContext {
    fn generate_trace_state(&self) -> String {
        self.trace_state
            .iter()
            .map(Entry::to_string)
            .collect::<Vec<String>>()
            .join(TRACESTATE_ENTRY_DELIMITER)
    }

    fn generate_trace_parent(&self) -> String {
        [
            VERSION.to_string(),
            self.trace_id.to_base16(),
            self.span_id.to_base16(),
            self.trace_option.to_base16(),
        ]
        .join(TRACEPARENT_DELIMITER)
    }

    pub fn try_from_http_text(trace_parent_str: &str, trace_state: TraceState) -> Option<Self> {
        let xs: Vec<_> = trace_parent_str.splitn(4, TRACEPARENT_DELIMITER).collect();
        xs.get(1)
            .and_then(|x| TraceId::try_from_base16(x))
            .and_then(|t| {
                xs.get(2)
                    .and_then(|y| SpanId::try_from_base16(y))
                    .and_then(|s| {
                        xs.get(3)
                            .and_then(|z| TraceOption::try_from_base16(z))
                            .map(|o| SpanContext::new(t, s, o, trace_state))
                    })
            })
    }
}

impl HttpTextFormat for SpanContext {
    type Item = Self;

    fn fields(&self) -> &[&str] {
        &FIELDS
    }

    fn inject<C, R>(&self, carrier: &mut C, setter: fn(&mut C, String, String) -> R) {
        setter(
            carrier,
            TRACEPARENT.to_owned(),
            self.generate_trace_parent(),
        );
        if self.trace_state.has_entry() {
            setter(carrier, TRACESTATE.to_owned(), self.generate_trace_state());
        }
    }

    fn extract<C>(
        carrier: &C,
        getter: for<'r> fn(&'r C, &str) -> Option<&'r String>,
    ) -> Option<Self::Item> {
        let o = getter(carrier, &TRACESTATE)
            .map(|v| TraceState::try_from_http_text(v))
            .unwrap_or_else(TraceState::empty);
        getter(carrier, &TRACEPARENT).and_then(|v| Self::try_from_http_text(v, o))
    }
}

#[test]
fn http_trace_context_inject() {
    use std::collections::HashMap;
    use std::num::{NonZeroU128, NonZeroU64};

    let i = SpanContext::new(
        TraceId::new(NonZeroU128::new(42).unwrap()),
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

    let e = SpanContext::new(
        TraceId::new(NonZeroU128::new(42).unwrap()),
        SpanId::new(NonZeroU64::new(42).unwrap()),
        TraceOption::MASK_SAMPLE,
        TraceState::empty(),
    );
    let mut m: HashMap<String, String> = HashMap::new();
    m.insert(
        "traceparent".to_owned(),
        "00-2a000000000000000000000000000000-2a00000000000000-01".to_owned(),
    );
    let a = SpanContext::extract(&m, HashMap::get);
    assert_eq!(e, a.unwrap());
}

#[test]
fn span_context_convert_base16() {
    let e = SpanContext::new(
        TraceId::generate_random(),
        SpanId::generate_random(),
        TraceOption::MASK_SAMPLE,
        TraceState::empty(),
    );
    let a = SpanContext::try_from_http_text(&e.generate_trace_parent(), TraceState::empty());

    assert!(a.is_some());
    assert_eq!(e, a.unwrap());
}
