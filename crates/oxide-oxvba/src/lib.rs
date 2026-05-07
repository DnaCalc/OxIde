//! OxIde-side adapter boundary for OxVba-owned truth.
//!
//! W210-B01 establishes the crate boundary only. The first real OxVba
//! project load adapter lands in W210-B02.

use oxide_domain::OxideDomainRole;

/// Compile-time marker for the OxVba adapter crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OxideOxVbaRole {
    /// Adapter over authoritative OxVba APIs and types.
    AuthoritativeOxVbaAdapter,
}

impl OxideOxVbaRole {
    pub fn consumes_domain_vocabulary(self) -> OxideDomainRole {
        match self {
            Self::AuthoritativeOxVbaAdapter => OxideDomainRole::HostIndependentIdeVocabulary,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_role_declares_authoritative_oxvba_boundary() {
        assert_eq!(
            OxideOxVbaRole::AuthoritativeOxVbaAdapter.consumes_domain_vocabulary(),
            OxideDomainRole::HostIndependentIdeVocabulary
        );
    }
}
