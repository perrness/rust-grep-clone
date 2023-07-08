use std::io::Write;

use anyhow::Context;
use log::info;

pub fn find_matches(content: &str, pattern: &str, mut handle: impl Write) {
    for line in content.lines() {
        info!("reading line {:?}", line);
        if line.contains(pattern) {
            writeln!(handle, "{}", line)
                .with_context(|| format!("Can't read line"))
                .ok();
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n")
}
