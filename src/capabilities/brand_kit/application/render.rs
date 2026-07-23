//! Brand kit cards for fleet orgs.

use crate::shared::svg::{credit_mark, ensure_hash, esc, svg_doc};
use crate::shared::theme;

pub fn render(brand: &str, tagline: Option<&str>, credit: bool) -> String {
    let key = brand.to_ascii_lowercase();
    let (name, theme_key, default_tag) = match key.as_str() {
        "sylphx" | "sylphxai" => ("Sylphx", "sylphx", "AI-native platform for developers"),
        "cubeage" => ("Cubeage", "cubeage", "Games & entertainment"),
        "epiow" | "epiowai" => ("Epiow", "epiow", "B2B technology"),
        "ozyrix" | "ozyrixltd" => ("Ozyrix", "ozyrix", "Premium tech accessories"),
        "kyle" | "shtse8" => ("Kyle Tse", "kyle", "Builder · multi-company portfolio"),
        other => (brand, other, "Brand mark"),
    };

    let t = theme::get(theme_key).unwrap_or_else(|| theme::get("dark").unwrap());
    let w = 640u32;
    let h = 200u32;
    let bg = ensure_hash(t.bg);
    let fg = ensure_hash(t.fg);
    let muted = ensure_hash(t.muted);
    let accent = ensure_hash(t.accent);
    let bg2 = ensure_hash(t.bg2);
    let line = tagline.unwrap_or(default_tag);

    let body = format!(
        "<defs>\
           <linearGradient id=\"bg\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"100%\">\
             <stop offset=\"0%\" stop-color=\"{bg}\"/>\
             <stop offset=\"100%\" stop-color=\"{bg2}\"/>\
           </linearGradient>\
         </defs>\
         <rect width=\"{w}\" height=\"{h}\" rx=\"16\" fill=\"url(#bg)\"/>\
         <circle cx=\"72\" cy=\"100\" r=\"40\" fill=\"{accent}\" fill-opacity=\"0.9\"/>\
         <circle cx=\"72\" cy=\"100\" r=\"16\" fill=\"{fg}\" fill-opacity=\"0.95\"/>\
         <text x=\"140\" y=\"88\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"36\" font-weight=\"700\" fill=\"{fg}\">{}</text>\
         <text x=\"140\" y=\"122\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"16\" fill=\"{muted}\">{}</text>\
         <text x=\"140\" y=\"158\" font-family=\"ui-sans-serif,system-ui,sans-serif\" font-size=\"12\" fill=\"{fg}\" fill-opacity=\"0.55\">mark · brand kit</text>\
         {}",
        esc(name),
        esc(line),
        credit_mark(w, h, credit)
    );
    svg_doc(w, h, &body)
}
