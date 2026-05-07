pub const COMMAND_REGISTRATION_KIND: &str = "w341-command-name-scaffold-only";

pub const PROVEN_OXIDE_COMMAND_PLACEHOLDERS: &[&str] = &[
    "dna_oxide_get_host_capabilities",
    "dna_oxide_open_project_path",
    "dna_oxide_load_active_module",
    "dna_oxide_save_active_module",
    "dna_oxide_reload_active_module",
    "dna_oxide_save_session_snapshot",
    "dna_oxide_load_session_snapshot",
];

pub const PENDING_OXVBA_COMMAND_PLACEHOLDERS: &[&str] = &[
    "dna_oxide_build_check",
    "dna_oxide_get_compile_options",
    "dna_oxide_apply_compile_options",
    "dna_oxide_find_com_candidates",
    "dna_oxide_apply_reference_plan",
    "dna_oxide_run_project",
    "dna_oxide_stop_runtime",
    "dna_oxide_evaluate_immediate",
    "dna_oxide_debug_attach",
    "dna_oxide_debug_continue",
    "dna_oxide_debug_step_into",
    "dna_oxide_debug_step_over",
    "dna_oxide_debug_step_out",
    "dna_oxide_debug_stop",
    "dna_oxide_watch_upsert",
    "dna_oxide_watch_remove",
    "dna_oxide_breakpoint_set",
    "dna_oxide_breakpoint_clear",
];

pub fn all_command_placeholders() -> Vec<&'static str> {
    PROVEN_OXIDE_COMMAND_PLACEHOLDERS
        .iter()
        .chain(PENDING_OXVBA_COMMAND_PLACEHOLDERS.iter())
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_placeholders_cover_proven_lifecycle_path() {
        let commands = all_command_placeholders();
        assert!(commands.contains(&"dna_oxide_get_host_capabilities"));
        assert!(commands.contains(&"dna_oxide_open_project_path"));
        assert!(commands.contains(&"dna_oxide_save_active_module"));
        assert!(commands.contains(&"dna_oxide_load_session_snapshot"));
    }

    #[test]
    fn command_placeholders_name_pending_full_scope_services() {
        let commands = all_command_placeholders();
        assert!(commands.contains(&"dna_oxide_build_check"));
        assert!(commands.contains(&"dna_oxide_evaluate_immediate"));
        assert!(commands.contains(&"dna_oxide_debug_attach"));
        assert!(commands.contains(&"dna_oxide_watch_upsert"));
        assert!(commands.contains(&"dna_oxide_breakpoint_set"));
        assert!(commands.contains(&"dna_oxide_find_com_candidates"));
    }
}
