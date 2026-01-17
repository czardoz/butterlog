use butterlog::{AppModel, LineStore, LoadStatus, Partition};

#[test]
fn app_model_initializes_rows_and_ui() {
    let store = LineStore::new(vec!["line".to_string()]);
    let partitions = vec![Partition::new("A".to_string(), vec![0], 0)];

    let model = AppModel::new(store, partitions, LoadStatus::complete());

    assert_eq!(model.rows.len(), 1);
    assert_eq!(model.rows[0].text, "A");
    assert_eq!(model.ui.selected, 0);
}
