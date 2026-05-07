//! GUI-neutral OxIde application orchestration.
//!
//! This crate owns GUI-native state transitions above OxIde domain
//! vocabulary. It must not import parked TUI session/editor code.

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentLifecycleState {
    persisted_source: String,
    working_source: String,
    capabilities: LifecycleCapabilities,
}

impl DocumentLifecycleState {
    pub fn open_clean(source: impl Into<String>, capabilities: LifecycleCapabilities) -> Self {
        let source = source.into();
        Self {
            persisted_source: source.clone(),
            working_source: source,
            capabilities,
        }
    }

    pub fn persisted_source(&self) -> &str {
        &self.persisted_source
    }

    pub fn working_source(&self) -> &str {
        &self.working_source
    }

    pub fn is_dirty(&self) -> bool {
        self.working_source != self.persisted_source
    }

    pub fn edit_working_source(&mut self, source: impl Into<String>) {
        self.working_source = source.into();
    }

    pub fn acknowledge_saved(&mut self) -> Result<(), LifecycleCommandDisabled> {
        self.ensure_enabled(LifecycleCommand::Save)?;
        self.persisted_source = self.working_source.clone();
        Ok(())
    }

    pub fn reload_from_persisted(
        &mut self,
        source: impl Into<String>,
    ) -> Result<(), LifecycleCommandDisabled> {
        self.ensure_enabled(LifecycleCommand::Reload)?;
        let source = source.into();
        self.persisted_source = source.clone();
        self.working_source = source;
        Ok(())
    }

    pub fn revert_to_persisted(&mut self) -> Result<(), LifecycleCommandDisabled> {
        self.ensure_enabled(LifecycleCommand::Revert)?;
        self.working_source = self.persisted_source.clone();
        Ok(())
    }

    pub fn command_status(&self, command: LifecycleCommand) -> LifecycleCommandStatus {
        match command {
            LifecycleCommand::Save if !self.capabilities.can_save => {
                LifecycleCommandStatus::disabled(
                    self.capabilities
                        .save_disabled_reason
                        .clone()
                        .unwrap_or_else(|| {
                            String::from("save is unavailable in this host profile")
                        }),
                )
            }
            LifecycleCommand::Reload if !self.capabilities.can_reload => {
                LifecycleCommandStatus::disabled(
                    self.capabilities
                        .reload_disabled_reason
                        .clone()
                        .unwrap_or_else(|| {
                            String::from("reload is unavailable in this host profile")
                        }),
                )
            }
            LifecycleCommand::Revert if !self.capabilities.can_revert => {
                LifecycleCommandStatus::disabled(
                    self.capabilities
                        .revert_disabled_reason
                        .clone()
                        .unwrap_or_else(|| {
                            String::from("revert is unavailable in this host profile")
                        }),
                )
            }
            LifecycleCommand::Save => LifecycleCommandStatus::enabled(self.is_dirty()),
            LifecycleCommand::Reload => LifecycleCommandStatus::enabled(true),
            LifecycleCommand::Revert => LifecycleCommandStatus::enabled(self.is_dirty()),
        }
    }

    fn ensure_enabled(&self, command: LifecycleCommand) -> Result<(), LifecycleCommandDisabled> {
        let status = self.command_status(command);
        if status.is_enabled {
            Ok(())
        } else {
            Err(LifecycleCommandDisabled {
                command,
                reason: status
                    .reason
                    .unwrap_or_else(|| String::from("command disabled")),
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleCapabilities {
    pub can_save: bool,
    pub can_reload: bool,
    pub can_revert: bool,
    pub save_disabled_reason: Option<String>,
    pub reload_disabled_reason: Option<String>,
    pub revert_disabled_reason: Option<String>,
}

impl LifecycleCapabilities {
    pub fn all_supported() -> Self {
        Self {
            can_save: true,
            can_reload: true,
            can_revert: true,
            save_disabled_reason: None,
            reload_disabled_reason: None,
            revert_disabled_reason: None,
        }
    }

    pub fn browser_limited() -> Self {
        let reason = String::from("browser-safe profile has no direct filesystem persistence");
        Self {
            can_save: false,
            can_reload: false,
            can_revert: true,
            save_disabled_reason: Some(reason.clone()),
            reload_disabled_reason: Some(reason),
            revert_disabled_reason: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleCommand {
    Save,
    Reload,
    Revert,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleCommandStatus {
    pub is_enabled: bool,
    pub reason: Option<String>,
}

impl LifecycleCommandStatus {
    pub fn enabled(is_enabled: bool) -> Self {
        Self {
            is_enabled,
            reason: None,
        }
    }

    pub fn disabled(reason: impl Into<String>) -> Self {
        Self {
            is_enabled: false,
            reason: Some(reason.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleCommandDisabled {
    pub command: LifecycleCommand,
    pub reason: String,
}

impl std::fmt::Display for LifecycleCommandDisabled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} disabled: {}", self.command, self.reason)
    }
}

impl std::error::Error for LifecycleCommandDisabled {}

pub trait DocumentPersistence {
    fn load(&self) -> Result<String, PersistenceError>;
    fn save(&mut self, source: &str) -> Result<(), PersistenceError>;
    fn capabilities(&self) -> LifecycleCapabilities;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InMemoryDocumentPersistence {
    persisted_source: String,
}

impl InMemoryDocumentPersistence {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            persisted_source: source.into(),
        }
    }

    pub fn persisted_source(&self) -> &str {
        &self.persisted_source
    }
}

impl DocumentPersistence for InMemoryDocumentPersistence {
    fn load(&self) -> Result<String, PersistenceError> {
        Ok(self.persisted_source.clone())
    }

    fn save(&mut self, source: &str) -> Result<(), PersistenceError> {
        self.persisted_source = source.to_string();
        Ok(())
    }

    fn capabilities(&self) -> LifecycleCapabilities {
        LifecycleCapabilities::all_supported()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserLimitedPersistence {
    source_snapshot: String,
    disabled_reason: String,
}

impl BrowserLimitedPersistence {
    pub fn new(source_snapshot: impl Into<String>) -> Self {
        Self {
            source_snapshot: source_snapshot.into(),
            disabled_reason: String::from(
                "browser-safe profile has no direct filesystem persistence",
            ),
        }
    }

    pub fn source_snapshot(&self) -> &str {
        &self.source_snapshot
    }
}

impl DocumentPersistence for BrowserLimitedPersistence {
    fn load(&self) -> Result<String, PersistenceError> {
        Err(PersistenceError::Disabled {
            operation: PersistenceOperation::Load,
            reason: self.disabled_reason.clone(),
        })
    }

    fn save(&mut self, _source: &str) -> Result<(), PersistenceError> {
        Err(PersistenceError::Disabled {
            operation: PersistenceOperation::Save,
            reason: self.disabled_reason.clone(),
        })
    }

    fn capabilities(&self) -> LifecycleCapabilities {
        LifecycleCapabilities::browser_limited()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistenceOperation {
    Load,
    Save,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersistenceError {
    Disabled {
        operation: PersistenceOperation,
        reason: String,
    },
}

impl std::fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disabled { operation, reason } => write!(f, "{:?} disabled: {reason}", operation),
        }
    }
}

impl std::error::Error for PersistenceError {}

pub fn open_lifecycle_from_persistence(
    persistence: &impl DocumentPersistence,
) -> Result<DocumentLifecycleState, PersistenceError> {
    Ok(DocumentLifecycleState::open_clean(
        persistence.load()?,
        persistence.capabilities(),
    ))
}

pub fn save_lifecycle_to_persistence(
    state: &mut DocumentLifecycleState,
    persistence: &mut impl DocumentPersistence,
) -> Result<(), PersistenceError> {
    persistence.save(state.working_source())?;
    state
        .acknowledge_saved()
        .map_err(|disabled| PersistenceError::Disabled {
            operation: PersistenceOperation::Save,
            reason: disabled.reason,
        })
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

    #[test]
    fn open_clean_document_starts_not_dirty() {
        let state = DocumentLifecycleState::open_clean(
            "Option Explicit",
            LifecycleCapabilities::all_supported(),
        );

        assert_eq!(state.persisted_source(), "Option Explicit");
        assert_eq!(state.working_source(), "Option Explicit");
        assert!(!state.is_dirty());
    }

    #[test]
    fn edit_makes_document_dirty() {
        let mut state =
            DocumentLifecycleState::open_clean("before", LifecycleCapabilities::all_supported());

        state.edit_working_source("after");

        assert_eq!(state.persisted_source(), "before");
        assert_eq!(state.working_source(), "after");
        assert!(state.is_dirty());
        assert!(state.command_status(LifecycleCommand::Save).is_enabled);
        assert!(state.command_status(LifecycleCommand::Revert).is_enabled);
    }

    #[test]
    fn save_acknowledgement_makes_working_source_clean() {
        let mut state =
            DocumentLifecycleState::open_clean("before", LifecycleCapabilities::all_supported());
        state.edit_working_source("after");

        state.acknowledge_saved().expect("save supported");

        assert_eq!(state.persisted_source(), "after");
        assert_eq!(state.working_source(), "after");
        assert!(!state.is_dirty());
        assert!(!state.command_status(LifecycleCommand::Save).is_enabled);
    }

    #[test]
    fn reload_replaces_persisted_and_working_source() {
        let mut state =
            DocumentLifecycleState::open_clean("before", LifecycleCapabilities::all_supported());
        state.edit_working_source("dirty");

        state
            .reload_from_persisted("from host")
            .expect("reload supported");

        assert_eq!(state.persisted_source(), "from host");
        assert_eq!(state.working_source(), "from host");
        assert!(!state.is_dirty());
    }

    #[test]
    fn revert_restores_persisted_source_without_saving() {
        let mut state =
            DocumentLifecycleState::open_clean("before", LifecycleCapabilities::all_supported());
        state.edit_working_source("dirty");

        state.revert_to_persisted().expect("revert supported");

        assert_eq!(state.persisted_source(), "before");
        assert_eq!(state.working_source(), "before");
        assert!(!state.is_dirty());
    }

    #[test]
    fn unsupported_commands_report_disabled_reasons() {
        let mut state =
            DocumentLifecycleState::open_clean("before", LifecycleCapabilities::browser_limited());
        state.edit_working_source("dirty");

        let save = state
            .acknowledge_saved()
            .expect_err("browser save disabled");
        assert_eq!(save.command, LifecycleCommand::Save);
        assert!(save.reason.contains("browser-safe"));
        assert!(
            state
                .command_status(LifecycleCommand::Reload)
                .reason
                .expect("reload disabled reason")
                .contains("filesystem")
        );
        state
            .revert_to_persisted()
            .expect("local revert remains pure");
        assert_eq!(state.working_source(), "before");
    }

    #[test]
    fn in_memory_persistence_loads_and_saves_without_disk() {
        let mut persistence = InMemoryDocumentPersistence::new("before");
        let mut state = open_lifecycle_from_persistence(&persistence).expect("memory load");

        state.edit_working_source("after");
        save_lifecycle_to_persistence(&mut state, &mut persistence).expect("memory save");

        assert_eq!(persistence.persisted_source(), "after");
        assert_eq!(persistence.load().as_deref(), Ok("after"));
        assert!(!state.is_dirty());
    }

    #[test]
    fn browser_limited_persistence_reports_disabled_reasons() {
        let mut persistence = BrowserLimitedPersistence::new("snapshot");

        assert_eq!(persistence.source_snapshot(), "snapshot");
        let load = persistence.load().expect_err("browser load disabled");
        assert_eq!(
            load,
            PersistenceError::Disabled {
                operation: PersistenceOperation::Load,
                reason: String::from("browser-safe profile has no direct filesystem persistence")
            }
        );
        let save = persistence
            .save("after")
            .expect_err("browser save disabled");
        assert!(save.to_string().contains("filesystem persistence"));
        assert!(!persistence.capabilities().can_save);
    }

    #[test]
    fn lifecycle_and_persistence_keep_working_source_distinct_until_save() {
        let mut persistence = InMemoryDocumentPersistence::new("persisted");
        let mut state = open_lifecycle_from_persistence(&persistence).expect("memory load");

        state.edit_working_source("working");

        assert_eq!(state.persisted_source(), "persisted");
        assert_eq!(state.working_source(), "working");
        assert_eq!(persistence.persisted_source(), "persisted");

        save_lifecycle_to_persistence(&mut state, &mut persistence).expect("memory save");

        assert_eq!(state.persisted_source(), "working");
        assert_eq!(persistence.persisted_source(), "working");
    }
}
