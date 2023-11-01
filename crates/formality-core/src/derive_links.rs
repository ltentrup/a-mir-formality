//! This module is a total hack. The Fold procedural macro references it.
//! It's the only way I can find to have the procedural macro generate
//! references to the Fold trait that work both in this crate and others.
//! Other crates that wish to use the Fold macro must re-export this module.

pub use crate::cast::DowncastTo;
pub use crate::cast::UpcastFrom;
pub use crate::fixed_point;
pub use crate::fold::Fold;
pub use crate::fold::SubstitutionFn;
pub use crate::parse;
pub use crate::term::Term;
pub use crate::variable::Variable;
pub use crate::visit::Visit;
