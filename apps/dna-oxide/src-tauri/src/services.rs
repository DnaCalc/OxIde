#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeServiceClaimBoundary {
    pub product_name: &'static str,
    pub app_name: &'static str,
    pub scaffold_kind: &'static str,
    pub real_execution_claimed: bool,
    pub native_runtime_claimed: bool,
    pub com_runtime_claimed: bool,
    pub immediate_fake_responses: bool,
    pub debug_fake_data: bool,
}

impl NativeServiceClaimBoundary {
    pub const fn w341_scaffold() -> Self {
        Self {
            product_name: "DNA OxIde",
            app_name: "DnaOxIde",
            scaffold_kind: "tauri-native-scaffold",
            real_execution_claimed: false,
            native_runtime_claimed: false,
            com_runtime_claimed: false,
            immediate_fake_responses: false,
            debug_fake_data: false,
        }
    }

    pub fn all_runtime_claims_false(self) -> bool {
        !self.real_execution_claimed
            && !self.native_runtime_claimed
            && !self.com_runtime_claimed
            && !self.immediate_fake_responses
            && !self.debug_fake_data
    }
}

pub const PENDING_NATIVE_SERVICES: &[&str] = &[
    "runtime-service-pending-oxvba-evidence",
    "immediate-service-pending-oxvba-evidence",
    "debug-service-pending-oxvba-evidence",
    "com-runtime-pending-oxvba-evidence",
];

pub fn scaffold_claim_boundary() -> NativeServiceClaimBoundary {
    NativeServiceClaimBoundary::w341_scaffold()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn w341_scaffold_keeps_runtime_claims_false() {
        let boundary = scaffold_claim_boundary();
        assert_eq!(boundary.product_name, "DNA OxIde");
        assert_eq!(boundary.app_name, "DnaOxIde");
        assert!(boundary.all_runtime_claims_false());
    }

    #[test]
    fn pending_native_services_are_named_without_claiming_support() {
        assert!(PENDING_NATIVE_SERVICES.contains(&"runtime-service-pending-oxvba-evidence"));
        assert!(PENDING_NATIVE_SERVICES.contains(&"immediate-service-pending-oxvba-evidence"));
        assert!(PENDING_NATIVE_SERVICES.contains(&"debug-service-pending-oxvba-evidence"));
        assert!(PENDING_NATIVE_SERVICES.contains(&"com-runtime-pending-oxvba-evidence"));
    }
}
