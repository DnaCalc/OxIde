//! GUI-neutral OxIde application orchestration.
//!
//! W210-B01 only establishes the crate boundary. Real state and command
//! reducers land when the project-open spine needs them.

use oxide_domain::OxideDomainRole;

/// Compile-time marker for the GUI core crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OxideCoreRole {
    /// State transitions and orchestration above domain vocabulary.
    GuiNeutralOrchestration,
}

impl OxideCoreRole {
    pub fn depends_on_domain(self) -> OxideDomainRole {
        match self {
            Self::GuiNeutralOrchestration => OxideDomainRole::HostIndependentIdeVocabulary,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_role_depends_on_domain_vocabulary() {
        assert_eq!(
            OxideCoreRole::GuiNeutralOrchestration.depends_on_domain(),
            OxideDomainRole::HostIndependentIdeVocabulary
        );
    }
}
