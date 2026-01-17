use butterlog::{
    AppModel, BuildOutput, LineLoader, LineStore, LoadConfig, LoadState, LoadStatus, Partition,
    PartitionPlan,
};
use tempfile::NamedTempFile;

#[test]
fn app_model_initializes_rows_and_ui() {
    let store = LineStore::new(vec!["line".to_string()]);
    let partitions = vec![Partition::new("A".to_string(), vec![0], 0)];
    let temp = NamedTempFile::new().expect("temp file");
    let loader = LineLoader::open(temp.path()).expect("loader");
    let load_state = LoadState::new(loader, LoadConfig::default());
    let plan = PartitionPlan::from_sample(&store.lines, 1, 1);
    let output = BuildOutput {
        line_store: store,
        partitions,
        load_state,
        plan,
    };

    let model = AppModel::new(output, LoadStatus::complete());

    assert_eq!(model.rows.len(), 1);
    assert_eq!(model.rows[0].text, "A");
    assert_eq!(model.ui.selected, 0);
}
