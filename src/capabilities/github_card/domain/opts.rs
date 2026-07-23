//! Card rendering options.

#[derive(Debug, Clone)]
pub struct CardOpts {
    pub theme: Option<String>,
    pub color: Option<String>,
    pub credit: bool,
    pub width: u32,
    /// Shell-injected hour-bucket seed for `timeAuto` / `timeGradient`.
    pub clock_seed: Option<String>,
}

impl Default for CardOpts {
    fn default() -> Self {
        Self {
            theme: Some("dark".into()),
            color: None,
            credit: true,
            width: 420,
            clock_seed: None,
        }
    }
}
