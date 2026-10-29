pub const HAS_ORIGINAL: u8 = 1 << 0; // video & images
pub const HAS_SAMPLE: u8 = 1 << 1; // both video & images
pub const HAS_480P: u8 = 1 << 2; // video only
pub const HAS_720P: u8 = 1 << 3; // video only

pub const IS_NEW: u8 = 1 << 7; // new upload
