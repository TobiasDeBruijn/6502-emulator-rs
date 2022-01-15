use thiserror::Error;

#[derive(Debug, Error)]
#[allow(clippy::module_name_repetitions)]
pub enum Error {
    Opts(#[from])
}