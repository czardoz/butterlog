use butterlog::{mark_search_matches, LineStore, Partition, SearchTerm};

#[test]
fn marks_matching_partitions_and_bubbles_to_parents() {
    let mut parent = Partition::new("P".to_string(), vec![], 0, 1);
    let child = Partition::new("C".to_string(), vec![0], 1, 1);
    parent.children.push(child);

    let mut partitions = vec![parent];
    let store = LineStore::new(vec!["error here".to_string()]);
    let term = SearchTerm::new("error");

    mark_search_matches(&mut partitions, &store, &term);

    assert!(!partitions[0].matches_self);
    assert!(partitions[0].matches_descendants);
    assert!(partitions[0].children[0].matches_self);
}

#[test]
fn clears_flags_when_no_match() {
    let mut partition = Partition::new("X".to_string(), vec![0], 0, 1);
    partition.matches_self = true;
    partition.matches_descendants = true;

    let mut partitions = vec![partition];
    let store = LineStore::new(vec!["all good".to_string()]);
    let term = SearchTerm::new("error");

    mark_search_matches(&mut partitions, &store, &term);

    assert!(!partitions[0].matches_self);
    assert!(!partitions[0].matches_descendants);
}
