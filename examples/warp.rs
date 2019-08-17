use ot_rs::api::trace::span_context::{SpanContext, TraceState};
use std::env;
use warp::Filter;

fn main() {
    env::set_var("RUST_LOG", "info");

    let base = warp::header::optional::<String>("traceparent").map(|context: Option<String>| {
        context
            .and_then(|c| {
                SpanContext::try_from_http_text(c.as_str(), TraceState::empty()).map(|s| {
                    format!(
                        "t: {}, s: {}, o: {}",
                        s.span_id_str(),
                        s.span_id_str(),
                        s.trace_option.bits()
                    )
                })
            })
            .unwrap_or("test".to_string())
    });

    let routes = base;

    warp::serve(routes).run(([127, 0, 0, 1], 3030));
}
