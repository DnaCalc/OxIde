//! Browser-oriented GUI scenario lab boundary for OxIde.
//!
//! W210-B01 establishes the crate boundary only. Scenario IDs and render
//! contracts land in W210-B03.

use oxide_domain::OxideDomainRole;

/// Compile-time marker for the GUI lab crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OxideGuiLabRole {
    /// Deterministic scenario review surface for the GUI pivot.
    ScenarioReviewSurface,
}

impl OxideGuiLabRole {
    pub fn consumes_domain_vocabulary(self) -> OxideDomainRole {
        match self {
            Self::ScenarioReviewSurface => OxideDomainRole::HostIndependentIdeVocabulary,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guilab_role_consumes_domain_vocabulary() {
        assert_eq!(
            OxideGuiLabRole::ScenarioReviewSurface.consumes_domain_vocabulary(),
            OxideDomainRole::HostIndependentIdeVocabulary
        );
    }
}
