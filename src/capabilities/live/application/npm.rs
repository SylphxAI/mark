//! npm readers: registry metadata and the public downloads API (no token).

use serde_json::Value;

use super::upstream::{read_json, Call, Resource, Upstream, UpstreamError};
use crate::capabilities::live::domain::model::NpmPackage;

/// A validated package name: `name` or `@scope/name` (npm's URL-safe set).
pub(crate) fn valid_package(name: &str) -> bool {
    let body = match name.strip_prefix('@') {
        Some(scoped) => match scoped.split_once('/') {
            Some((scope, pkg)) if !scope.is_empty() && !pkg.contains('/') => {
                return [scope, pkg].iter().all(|p| valid_part(p)) && name.len() <= 214;
            }
            _ => return false,
        },
        None => name,
    };
    valid_part(body) && name.len() <= 214
}

fn valid_part(p: &str) -> bool {
    !p.is_empty()
        && !p.starts_with('.')
        && p.chars().all(|c| {
            c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '-' | '.' | '_' | '~')
        })
}

async fn read(up: &dyn Upstream, url: String) -> Result<Option<Value>, UpstreamError> {
    read_json(up, Call::read(Resource::Npm, url)).await
}

/// `dist-tag` (default `latest`) version and license.
pub(crate) async fn package(
    up: &dyn Upstream,
    name: &str,
    tag: &str,
) -> Result<Option<NpmPackage>, UpstreamError> {
    let url = format!(
        "https://registry.npmjs.org/{}/{}",
        name.replace('/', "%2F"),
        urlencoding::encode(tag)
    );
    let Some(v) = read(up, url).await? else {
        return Ok(None);
    };
    let Some(version) = v.get("version").and_then(Value::as_str) else {
        return Ok(None);
    };
    let license = match v.get("license") {
        Some(Value::String(s)) => Some(s.clone()),
        Some(Value::Object(o)) => o.get("type").and_then(Value::as_str).map(str::to_string),
        _ => None,
    };
    Ok(Some(NpmPackage {
        version: version.to_string(),
        license,
    }))
}

/// Downloads over `period` (`last-month`, `last-week`, or the all-time range
/// the API allows, which npm caps at 18 months — as shields does).
pub(crate) async fn downloads(
    up: &dyn Upstream,
    name: &str,
    period: &str,
) -> Result<Option<u64>, UpstreamError> {
    let url = format!("https://api.npmjs.org/downloads/point/{period}/{name}");
    Ok(read(up, url)
        .await?
        .and_then(|v| v.get("downloads").and_then(Value::as_u64)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_names_are_validated() {
        for ok in ["react", "@types/node", "lodash.debounce", "a_b-c"] {
            assert!(valid_package(ok), "{ok}");
        }
        for bad in ["", "React", "@types", "@/x", "a/b", "../x", "@a/b/c", "x?y"] {
            assert!(!valid_package(bad), "{bad}");
        }
    }
}
