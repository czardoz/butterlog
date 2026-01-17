#[test]
fn readme_mentions_streaming_and_partial_load() {
    let readme = std::fs::read_to_string("README.md").expect("readme exists");
    assert!(readme.contains("loads more") || readme.contains("load more"));
    assert!(readme.contains("Partial load"));
    assert!(readme.contains("search") && readme.contains("incomplete"));
}
