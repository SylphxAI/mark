//! capsule-render's color tables (MIT, © 2020 Ye-Chan Kang), kept in
//! upstream order: `customColorList=` indexes them, so the order is contract.
//!
//! A color is `"offset:hex,offset:hex"` for a gradient or one hex for a solid.
//! Rows are `(color, text, textBg)`.

/// `color=gradient` / `timeGradient` (upstream `gradient.json`).
pub(super) const GRADIENTS: &[(&str, &str, &str)] = &[
    ("0:F8B195,50:F67280,100:C06C84", "f7f5f5", "282829"),
    ("0:feac5e,50:c779d0,100:4bc0c8", "f7f5f5", "282829"),
    ("0:43cea2,100:185a9d", "f7f5f5", "282829"),
    ("0:c2e59c,100:64b3f4", "f7f5f5", "282829"),
    ("0:00C9FF,100:92FE9D", "f7f5f5", "282829"),
    ("0:e53935,100:e35d5b", "f7f5f5", "282829"),
    ("0:fc00ff,100:00dbde", "f7f5f5", "282829"),
    ("0:ff6a00,100:f720db", "f7f5f5", "282829"),
    ("0:004FF9,100:FFF94C", "f7f5f5", "282829"),
    ("0:7b4397,100:dc2430", "f7f5f5", "282829"),
    ("0:833ab4,50:fd1d1d,100:fcb045", "f7f5f5", "282829"),
    ("0:D38312,100:A83279", "f7f5f5", "282829"),
    ("0:00c6ff,100:0072ff", "f7f5f5", "282829"),
    ("0:780206,100:061161", "f7f5f5", "282829"),
    ("0:B993D6,100:8CA6DB", "f7f5f5", "282829"),
    ("0:ddd6f3,100:faaca8", "f7f5f5", "282829"),
    ("0:C04848,100:480048", "f7f5f5", "282829"),
    ("0:3CA55C,100:B5AC49", "f7f5f5", "282829"),
    ("0:9796f0,100:fbc7d4", "f7f5f5", "282829"),
    ("0:2193b0,100:6dd5ed", "f7f5f5", "282829"),
    ("0:4568dc,100:b06ab3", "f7f5f5", "282829"),
    ("0:13547a,100:80d0c7", "f7f5f5", "282829"),
    ("0:5C258D,100:4389A2", "f7f5f5", "282829"),
    ("0:134E5E,100:71B280", "f7f5f5", "282829"),
    ("0:4776E6,100:8E54E9", "f7f5f5", "282829"),
    ("0:614385,100:516395", "f7f5f5", "282829"),
    ("0:1D976C,100:93F9B9", "f7f5f5", "282829"),
    ("0:E55D87,100:5FC3E4", "f7f5f5", "282829"),
    ("0:EDE574,100:E1F5C4", "282829", "f7f5f5"),
    ("0:e52d27,100:b31217", "282829", "f7f5f5"),
    ("0:5433FF,50:20BDFF,100:A5FECB", "f7f5f5", "282829"),
];

/// `color=auto` / `timeAuto` (upstream `pallete.json`).
pub(super) const PALETTE: &[(&str, &str, &str)] = &[
    ("A3DCBE", "363636", "f7f5f5"),
    ("E3A6AE", "363636", "f7f5f5"),
    ("FD866E", "f7f5f5", "282829"),
    ("F8E2CF", "363636", "f7f5f5"),
    ("6FC7E1", "363636", "f7f5f5"),
    ("FFA883", "363636", "f7f5f5"),
    ("CDE4AD", "363636", "f7f5f5"),
    ("97DBAE", "363636", "f7f5f5"),
    ("7BD1D2", "363636", "f7f5f5"),
    ("75BDE0", "363636", "f7f5f5"),
    ("C3E5AE", "f7f5f5", "363636"),
    ("F1E1A6", "f7f5f5", "363636"),
    ("F4BBBB", "f7f5f5", "363636"),
    ("FCB6D0", "363636", "f7f5f5"),
    ("B6DCB6", "363636", "f0f0f0"),
    ("C7A48B", "f7f5f5", "363636"),
    ("C6BBB7", "363636", "f7f5f5"),
    ("F7EFE9", "363636", "f7f5f5"),
    ("76819C", "363636", "f7f5f5"),
    ("364765", "FAF7F5", "282829"),
    ("B97A63", "f7f5f5", "282829"),
    ("F4D47B", "f7f5f5", "282829"),
    ("1C768F", "032539", "f7f5f5"),
    ("FA991C", "032539", "f7f5f5"),
    ("FFD159", "3E3C3C", "f7f5f5"),
    ("FF6F3C", "FCE2CF", "FF6F3C"),
    ("9ACB34", "41544c", "f7f5f5"),
    ("42564F", "C0EB6A", "f7f5f5"),
    ("3A4A51", "4A675A", "f7f5f5"),
];

/// `theme=` (upstream `pallete_theme.json`): `(id, color, text, textBg)`.
pub(super) const THEMES: &[(&str, &str, &str, &str)] = &[
    ("default", "fffefe", "4f94ef", "434d58"),
    ("dark", "151515", "dadada", "9f9f9f"),
    ("radical", "141321", "d83a7c", "a9fef7"),
    ("merko", "0a0f0b", "90b302", "68b587"),
    ("gruvbox", "282828", "d8a52d", "8ec07c"),
    ("gruvbox_light", "fbf1c7", "bf8930", "427b58"),
    ("tokyonight", "1a1b27", "628fda", "38bdae"),
    ("onedark", "282c34", "c6a76e", "df6d74"),
    ("cobalt", "193549", "c576c1", "75eeb2"),
];
