//! Per-glyph advance tables for the shields badge faces.
//!
//! Shields sizes every badge with precomputed Verdana/Helvetica advances
//! (badge-maker's `anafanafo` tables). Mark embeds the same advances for
//! U+0020–U+00FF (hundredths of a pixel, generated from anafanafo 2.0.0) so a
//! badge is exactly as wide as the shields one; other characters fall back to
//! a class estimate (wide CJK/emoji → one em, letters → `n`/`N`, rest → `m`).
//! The painted `<text>` carries `textLength`, so the box and the glyph run can
//! never disagree whatever font the viewer has.

/// A measured face: the font and size shields measures a badge part with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Face {
    /// `11px Verdana` — flat, flat-square, plastic, pill.
    Verdana11,
    /// `10px Verdana` — for-the-badge label.
    Verdana10,
    /// `bold 10px Verdana` — for-the-badge message.
    Verdana10Bold,
    /// `bold 11px Helvetica` — social.
    Helvetica11Bold,
    /// Monospace at the given pixel size (`font=mono`): 0.6 em per glyph.
    Mono(u8),
}

impl Face {
    fn table(self) -> Option<&'static [u16; 224]> {
        match self {
            Self::Verdana11 => Some(&VERDANA_11),
            Self::Verdana10 => Some(&VERDANA_10),
            Self::Verdana10Bold => Some(&VERDANA_10_BOLD),
            Self::Helvetica11Bold => Some(&HELVETICA_11_BOLD),
            Self::Mono(_) => None,
        }
    }

    fn px(self) -> f64 {
        match self {
            Self::Verdana10 | Self::Verdana10Bold => 10.0,
            Self::Mono(px) => px as f64,
            _ => 11.0,
        }
    }

    /// Advance of one glyph in px.
    pub(crate) fn advance(self, ch: char) -> f64 {
        let cp = ch as u32;
        if cp < 0x20 || (0x7F..=0x9F).contains(&cp) || (0x300..=0x36F).contains(&cp) {
            return 0.0;
        }
        let Some(table) = self.table() else {
            return if is_wide(cp) {
                self.px()
            } else {
                self.px() * 0.6
            };
        };
        let at = |c: char| table[c as usize - 0x20] as f64 / 100.0;
        match cp {
            0x20..=0xFF => table[cp as usize - 0x20] as f64 / 100.0,
            _ if is_wide(cp) => self.px(),
            _ if ch.is_uppercase() => at('N'),
            _ if ch.is_alphabetic() => at('n'),
            _ => at('m'),
        }
    }

    /// Advance of a run in px (unrounded). Summed in `f64` in text order,
    /// exactly as anafanafo does, so `floor` lands on the same pixel.
    pub(crate) fn width(self, text: &str) -> f64 {
        text.chars().fold(0.0, |acc, c| acc + self.advance(c))
    }
}

/// East Asian wide and emoji blocks: one em each.
fn is_wide(cp: u32) -> bool {
    matches!(cp,
        0x1100..=0x115F
        | 0x2E80..=0xA4CF
        | 0xAC00..=0xD7A3
        | 0xF900..=0xFAFF
        | 0xFE30..=0xFE4F
        | 0xFF00..=0xFF60
        | 0xFFE0..=0xFFE6
        | 0x1F300..=0x1FAFF
        | 0x20000..=0x3FFFD)
}

const VERDANA_11: [u16; 224] = [
    387, 433, 505, 900, 699, 1184, 799, 295, 500, 500, 699, 900, 400, 500, 400, 500, 699, 699, 699,
    699, 699, 699, 699, 699, 699, 699, 500, 500, 900, 900, 900, 600, 1100, 752, 754, 768, 848, 696,
    632, 853, 827, 463, 500, 762, 612, 927, 823, 866, 663, 866, 765, 752, 678, 805, 752, 1088, 754,
    677, 754, 500, 500, 500, 900, 699, 699, 661, 685, 573, 685, 655, 387, 685, 696, 302, 379, 651,
    302, 1070, 696, 668, 685, 685, 469, 573, 433, 696, 651, 900, 651, 651, 578, 698, 500, 698, 900,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 387, 433, 699, 699, 699, 699, 500, 699, 699, 1100, 600, 709, 900, 0, 1100, 699, 596, 900,
    596, 596, 699, 706, 699, 400, 699, 596, 600, 709, 1100, 1100, 1100, 600, 752, 752, 752, 752,
    752, 752, 1083, 768, 696, 696, 696, 696, 463, 463, 463, 463, 853, 823, 866, 866, 866, 866, 866,
    900, 866, 805, 805, 805, 805, 677, 666, 682, 661, 661, 661, 661, 661, 661, 1051, 573, 655, 655,
    655, 655, 302, 302, 302, 302, 673, 696, 668, 668, 668, 668, 668, 900, 668, 696, 696, 696, 696,
    651, 685, 1070,
];
const VERDANA_10: [u16; 224] = [
    352, 394, 459, 818, 636, 1076, 727, 269, 454, 454, 636, 818, 364, 454, 364, 454, 636, 636, 636,
    636, 636, 636, 636, 636, 636, 636, 454, 454, 818, 818, 818, 545, 1000, 684, 686, 698, 771, 632,
    575, 775, 751, 421, 455, 693, 557, 843, 748, 787, 603, 787, 695, 684, 616, 732, 684, 989, 685,
    615, 685, 454, 454, 454, 818, 636, 636, 601, 623, 521, 623, 596, 352, 623, 633, 274, 344, 592,
    274, 973, 633, 607, 623, 623, 427, 521, 394, 633, 592, 818, 592, 592, 525, 635, 454, 635, 818,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 352, 394, 636, 636, 636, 636, 454, 636, 636, 1000, 545, 645, 818, 0, 1000, 636, 542, 818,
    542, 542, 636, 642, 636, 364, 636, 542, 545, 645, 1000, 1000, 1000, 545, 684, 684, 684, 684,
    684, 684, 984, 698, 632, 632, 632, 632, 421, 421, 421, 421, 775, 748, 787, 787, 787, 787, 787,
    818, 787, 732, 732, 732, 732, 615, 605, 620, 601, 601, 601, 601, 601, 601, 955, 521, 596, 596,
    596, 596, 274, 274, 274, 274, 612, 633, 607, 607, 607, 607, 607, 818, 607, 633, 633, 633, 633,
    592, 623, 973,
];
const VERDANA_10_BOLD: [u16; 224] = [
    342, 402, 587, 867, 711, 1272, 862, 332, 543, 543, 711, 867, 361, 480, 361, 689, 711, 711, 711,
    711, 711, 711, 711, 711, 711, 711, 402, 402, 867, 867, 867, 617, 964, 776, 762, 724, 830, 683,
    650, 811, 837, 546, 555, 771, 637, 948, 847, 850, 733, 850, 782, 710, 682, 812, 764, 1128, 764,
    737, 692, 543, 689, 543, 867, 711, 711, 668, 699, 588, 699, 664, 422, 699, 712, 342, 403, 671,
    342, 1058, 712, 687, 699, 699, 497, 593, 456, 712, 650, 979, 669, 651, 597, 711, 543, 711, 867,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 342, 402, 711, 711, 711, 711, 543, 711, 711, 964, 598, 850, 867, 0, 964, 711, 587, 867, 598,
    598, 711, 721, 711, 361, 711, 598, 598, 850, 1182, 1182, 1182, 617, 776, 776, 776, 776, 776,
    776, 1094, 724, 683, 683, 683, 683, 546, 546, 546, 546, 830, 847, 850, 850, 850, 850, 850, 867,
    850, 812, 812, 812, 812, 737, 735, 713, 668, 668, 668, 668, 668, 668, 1018, 588, 664, 664, 664,
    664, 342, 342, 342, 342, 679, 712, 687, 687, 687, 687, 687, 867, 687, 712, 712, 712, 712, 651,
    699, 1058,
];
const HELVETICA_11_BOLD: [u16; 224] = [
    306, 306, 509, 612, 612, 1100, 753, 306, 326, 326, 448, 660, 306, 448, 306, 408, 612, 612, 612,
    612, 612, 612, 612, 612, 612, 612, 306, 306, 660, 660, 660, 612, 880, 753, 774, 815, 815, 713,
    652, 835, 815, 324, 612, 794, 652, 998, 815, 856, 734, 856, 794, 714, 672, 815, 693, 1038, 734,
    734, 713, 366, 408, 366, 660, 550, 285, 631, 672, 631, 672, 631, 366, 672, 652, 284, 306, 631,
    284, 997, 652, 672, 672, 672, 428, 591, 387, 652, 572, 895, 591, 571, 571, 366, 245, 366, 660,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 306, 306, 612, 612, 612, 612, 245, 612, 285, 880, 378, 488, 660, 0, 880, 285, 440, 660, 431,
    431, 285, 652, 682, 306, 285, 431, 404, 488, 981, 978, 978, 612, 753, 753, 753, 753, 753, 753,
    1079, 815, 713, 713, 713, 713, 324, 324, 324, 324, 815, 815, 856, 856, 856, 856, 856, 660, 856,
    815, 815, 815, 815, 734, 734, 672, 631, 631, 631, 631, 631, 631, 998, 631, 631, 631, 631, 631,
    284, 284, 284, 284, 672, 652, 672, 672, 672, 672, 672, 660, 672, 652, 652, 652, 652, 571, 672,
    997,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_matches_the_shields_tables() {
        // anafanafo: 'build' = 26.7, 'passing' = 41.75 at 11px Verdana.
        assert!((Face::Verdana11.width("build") - 26.7).abs() < 1e-9);
        assert!((Face::Verdana11.width("passing") - 41.75).abs() < 1e-9);
        assert!((Face::Verdana11.advance('m') - 10.7).abs() < 0.001);
        assert!((Face::Verdana10Bold.advance('m') - 10.58).abs() < 0.001);
        assert!((Face::Helvetica11Bold.advance('m') - 9.97).abs() < 0.001);
    }

    #[test]
    fn fallbacks_are_sane() {
        assert_eq!(Face::Verdana11.advance('中'), 11.0);
        assert_eq!(Face::Verdana10.advance('語'), 10.0);
        assert_eq!(Face::Verdana11.advance('\u{7}'), 0.0);
        assert_eq!(Face::Verdana11.advance('ł'), Face::Verdana11.advance('n'));
        assert_eq!(Face::Verdana11.advance('Ж'), Face::Verdana11.advance('N'));
        assert!((Face::Mono(11).width("abcd") - 26.4).abs() < 0.01);
        assert!(Face::Verdana11.advance('é') > 6.0);
    }
}
