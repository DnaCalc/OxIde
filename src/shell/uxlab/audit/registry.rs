use super::fixtures::{firehorse_audit_suite, matrix_rows};
use super::model::{UxAuditMatrixRow, UxAuditSuite};

pub struct UxAuditRegistry {
    suites: Vec<UxAuditSuite>,
}

impl UxAuditRegistry {
    pub fn built_in() -> Self {
        Self {
            suites: vec![firehorse_audit_suite()],
        }
    }

    pub fn suites(&self) -> &[UxAuditSuite] {
        &self.suites
    }

    pub fn suite(&self, id: &str) -> Option<&UxAuditSuite> {
        self.suites.iter().find(|suite| suite.id == id)
    }

    pub fn matrix_rows(&self, suite_id: &str) -> Option<Vec<UxAuditMatrixRow>> {
        self.suite(suite_id).map(matrix_rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn built_in_registry_exposes_firehorse_suite() {
        let registry = UxAuditRegistry::built_in();
        let suite = registry.suite("firehorse").expect("firehorse suite");

        assert_eq!(suite.id, "firehorse");
        assert!(!suite.personas.is_empty());
        assert!(!suite.scenarios.is_empty());
        assert!(!suite.criteria.is_empty());
    }
}
