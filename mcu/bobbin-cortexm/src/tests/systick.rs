use hal::systick::*;

use bobbin_common::rw::*;

#[test]
fn test_clock_source() {
    add_region(0xe000e000, 0x1000);
    assert_eq!(clock_source(), ClockSource::External);
}