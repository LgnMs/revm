use core::any::Any;

use crate::{Database, Inspector};
/// Dummy [Inspector], helpful as standalone replacement.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoOpInspector;

impl<T: Any, DB: Database> Inspector<T, DB> for NoOpInspector {}
