//! FNV-1a hashing — one implementation, two widths.
//!
//! Deterministic across processes and deploys (unlike std's `DefaultHasher`),
//! so the same bytes always produce the same ETag and the same auto-picked
//! palette. Both consumers read from here instead of carrying their own copy.

const OFFSET_32: u32 = 2166136261;
const PRIME_32: u32 = 16777619;
const OFFSET_64: u64 = 0xcbf29ce484222325;
const PRIME_64: u64 = 0x100000001b3;

/// FNV-1a/32 over bytes (palette/gradient seeding).
pub(crate) fn fnv1a_32(bytes: &[u8]) -> u32 {
    let mut h = OFFSET_32;
    for b in bytes {
        h ^= u32::from(*b);
        h = h.wrapping_mul(PRIME_32);
    }
    h
}

/// FNV-1a/64 over bytes (ETag identity).
pub(crate) fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut h = OFFSET_64;
    for b in bytes {
        h ^= u64::from(*b);
        h = h.wrapping_mul(PRIME_64);
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fnv1a_matches_the_reference_vectors() {
        assert_eq!(fnv1a_32(b""), 2166136261);
        assert_eq!(fnv1a_32(b"a"), 0xe40c292c);
        assert_eq!(fnv1a_64(b""), 0xcbf29ce484222325);
        assert_eq!(fnv1a_64(b"a"), 0xaf63dc4c8601ec8c);
    }

    #[test]
    fn larger_width_is_seeded_independently() {
        assert_ne!(u64::from(fnv1a_32(b"mark")), fnv1a_64(b"mark"));
    }
}
