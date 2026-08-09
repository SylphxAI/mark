//! The Mark renderer — one grammar, one dispatch.

use crate::capabilities::mark::domain::MarkForm;
use crate::capabilities::mark::domain::MarkSpec;

/// Render any mark from its spec. Pure and total: every spec renders.
pub fn render(spec: &MarkSpec) -> String {
    match spec.form {
        MarkForm::Hero => super::hero::render(spec),
        MarkForm::Pill => super::pill::render(spec),
        MarkForm::Strip => super::strip::render(spec),
        MarkForm::Profile => super::profile::render(spec),
        MarkForm::Deploy => super::deploy::render(spec),
    }
}
