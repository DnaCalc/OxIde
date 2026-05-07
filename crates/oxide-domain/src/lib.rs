//! Host-independent OxIde GUI domain vocabulary.
//!
//! This crate is intentionally small in W210-B01. Concrete project and
//! editor view models land in later W210 beads once their contracts are
//! backed by the thin-slice fixture.

use serde::{Deserialize, Serialize};

/// Compile-time marker for the GUI pivot domain crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OxideDomainRole {
    /// Pure, host-independent vocabulary owned by OxIde.
    HostIndependentIdeVocabulary,
}

impl OxideDomainRole {
    pub const fn label(self) -> &'static str {
        match self {
            Self::HostIndependentIdeVocabulary => "host-independent IDE vocabulary",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_role_names_host_independent_vocabulary() {
        assert_eq!(
            OxideDomainRole::HostIndependentIdeVocabulary.label(),
            "host-independent IDE vocabulary"
        );
    }
}
