pub fn user_agent() -> String {
    let version = if cfg!(debug_assertions) {
        "dev"
    } else {
        env!("CARGO_PKG_VERSION")
    };

    format!("tracer/{}", version)
}
