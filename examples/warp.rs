use ot_rs::api::context::TryFromHttpText;
use ot_rs::api::trace::trace_context::TraceContext;
use std::env;
use warp::Filter;

fn main() {
    env::set_var("RUST_LOG", "info");

    let base = warp::header::optional::<String>("traceparent").map(|context: Option<String>| {
        context
            .and_then(|c| {
                TraceContext::try_from_http_text(c.as_str())
                    .map(|s| {
                        format!(
                            "t: {}, s: {}, o: {}",
                            s.trace_id.to_string(),
                            s.trace_id.to_string(),
                            s.trace_option.bits()
                        )
                    })
                    .ok()
            })
            .unwrap_or("test".to_string())
    });

    let routes = base;

    warp::serve(routes).run(([127, 0, 0, 1], 3030));
}
