mod commands;
mod services;

#[tauri::command]
fn dna_oxide_desktop_host_capabilities_probe(
    project_path: Option<String>,
) -> Result<commands::DesktopHostCommandSpinePacket, String> {
    commands::dna_oxide_desktop_host_capabilities_probe(project_path.as_deref())
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn dna_oxide_get_host_capabilities_probe(
    project_path: Option<String>,
) -> Result<commands::DesktopHostCommandSpinePacket, String> {
    commands::dna_oxide_desktop_host_capabilities_probe(project_path.as_deref())
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn dna_oxide_save_active_module(
    project_path: Option<String>,
    new_source: String,
) -> Result<commands::DnaOxideModuleCommandPacket, String> {
    let project_path = match project_path {
        Some(path) => std::path::PathBuf::from(path),
        None => commands::dna_oxide_default_tauri_project_path().map_err(|error| error.to_string())?,
    };
    commands::dna_oxide_save_active_module(project_path, new_source)
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn dna_oxide_reload_active_module(
    project_path: Option<String>,
) -> Result<commands::DnaOxideModuleCommandPacket, String> {
    let project_path = match project_path {
        Some(path) => std::path::PathBuf::from(path),
        None => commands::dna_oxide_default_tauri_project_path().map_err(|error| error.to_string())?,
    };
    commands::dna_oxide_reload_active_module(project_path).map_err(|error| error.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            dna_oxide_desktop_host_capabilities_probe,
            dna_oxide_get_host_capabilities_probe,
            dna_oxide_save_active_module,
            dna_oxide_reload_active_module
        ])
        .run(tauri::generate_context!())
        .expect("error while running DNA OxIde Tauri app");
}

#[cfg(test)]
mod tests {
    use super::{commands, services};

    #[test]
    fn native_scaffold_reports_dna_oxide_branding() {
        let boundary = services::scaffold_claim_boundary();
        assert_eq!(boundary.product_name, "DNA OxIde");
        assert_eq!(boundary.app_name, "DnaOxIde");
        assert_eq!(boundary.scaffold_kind, "tauri-native-scaffold");
    }

    #[test]
    fn native_scaffold_exposes_rust_callable_command_boundary() {
        assert_eq!(
            commands::COMMAND_REGISTRATION_KIND,
            "w352-tauri-linked-native-command-spine"
        );
        let commands = commands::all_command_placeholders();
        assert!(commands.len() >= 30);
        assert!(commands.contains(&"dna_oxide_open_project_path"));
        assert!(commands.contains(&"dna_oxide_build_check"));
        assert!(commands.contains(&"dna_oxide_run_project"));
    }
}
