use butterlog::{LineStore, Partition, SearchTerm};

#[test]
fn incremental_search_flags_new_matches() {
    let lines = vec!["AA one".to_string(), "AB match".to_string()];
    let store = LineStore::new(lines);
    let mut parent = Partition::new("A".to_string(), vec![0], 0, 1);
    parent.children.push(Partition::new("AA".to_string(), vec![0], 1, 2));
    parent.rebuild_child_index();

    let term = SearchTerm::new("match");
    parent.insert_line(1, &store, 10, Some(&term));

    assert!(parent.matches_descendants);
    let ab_child = parent
        .children
        .iter()
        .find(|child| child.prefix == "AB")
        .expect("AB child");
    assert!(ab_child.matches_self);
}
