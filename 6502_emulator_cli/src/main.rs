mod opts;
mod error;

fn main() {
    log_init();
}

fn log_init() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "INFO");
    }

    env_logger::init();
}