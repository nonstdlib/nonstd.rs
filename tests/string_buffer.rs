extern crate nonstd;

use std::io::Write;

#[test]
fn string_buffer_is_a_writer() {
    let mut w = nonstd::string::Buffer::new();
    assert_eq!(w.write("hi".as_bytes()).unwrap(), 2);
    assert_eq!(w.as_str(), "hi");
}

