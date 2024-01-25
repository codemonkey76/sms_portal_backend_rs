mod error;
pub use self::error::{Error, Result};

#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: i64
}