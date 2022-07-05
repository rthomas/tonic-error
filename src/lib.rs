use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub use tonic_error_impl::*;

pub trait TonicError<'de>: Serialize + Deserialize<'de> + Display {}
