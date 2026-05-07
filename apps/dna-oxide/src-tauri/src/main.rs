mod commands;
mod services;

fn main() {
    let boundary = services::scaffold_claim_boundary();
    println!(
        "{} ({}) {}",
        boundary.product_name, boundary.app_name, boundary.scaffold_kind
    );
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
    fn native_scaffold_exposes_command_name_shell_only() {
        assert_eq!(commands::COMMAND_REGISTRATION_KIND, "w341-command-name-scaffold-only");
        let commands = commands::all_command_placeholders();
        assert!(commands.len() >= 12);
    }
}
