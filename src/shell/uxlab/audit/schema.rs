use serde::Serialize;

use super::model::AUDIT_SCHEMA_VERSION;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AuditJsonEnvelope<T>
where
    T: Serialize,
{
    pub schema_version: u32,
    pub kind: &'static str,
    pub data: T,
}

impl<T> AuditJsonEnvelope<T>
where
    T: Serialize,
{
    pub fn new(kind: &'static str, data: T) -> Self {
        Self {
            schema_version: AUDIT_SCHEMA_VERSION,
            kind,
            data,
        }
    }
}
