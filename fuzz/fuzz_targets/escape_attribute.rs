#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let escaped = winged_rust::core::escape::escape_attribute(data);
    assert!(
        !escaped.contains('"'),
        "escape_attribute emitted a raw '\"': {escaped}"
    );
});
