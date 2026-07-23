//! Card rendering options.

#[derive(Debug, Clone)]
pub struct CardOpts {
    pub theme: Option<String>,
    pub color: Option<String>,
    pub credit: bool,
    pub width: u32,
}

impl Default for CardOpts {
    fn default() -> Self {
        Self {
            theme: Some("dark".into()),
            color: None,
            credit: true,
            width: 420,
        }
    }
}
