use std::fmt::Debug;

use crate::error::*;

pub trait Printer: Send + Sync + Clone + Debug + 'static {
    fn print(&self, filename: String) -> Result<()>;
}