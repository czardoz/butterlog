use butterlog::SearchTerm;

#[test]
fn search_term_normalizes_input() {
    let term = SearchTerm::new("ErRoR");
    assert_eq!(term.raw, "ErRoR");
    assert_eq!(term.normalized, "error");
}

#[test]
fn search_term_matches_normalized_line() {
    let term = SearchTerm::new("Error");
    assert!(term.matches_line("an error occurred"));
    assert!(!term.matches_line("all good"));
}
