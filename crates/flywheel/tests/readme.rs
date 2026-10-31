//! README's example configuration is one the runner reads (task 1.2).

use flywheel::config;
use std::path::Path;

#[test]
fn the_readme_configuration_reads() {
    let readme = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../README.md");
    let text = std::fs::read_to_string(&readme).unwrap();
    let (_, block) = text
        .split_once("```toml\n")
        .expect("README holds a ```toml block");
    let (block, _) = block.split_once("```").expect("the block is closed");
    let config = config::parse(block, &readme, Path::new("/h")).unwrap();
    assert!(!config.repositories.is_empty());
}
