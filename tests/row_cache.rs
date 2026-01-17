use butterlog::{
    AppModel, BuildOutput, LineLoader, LineStore, LoadConfig, LoadState, LoadStatus, Partition,
    PartitionPlan,
};
use tempfile::NamedTempFile;

#[test]
fn app_model_tracks_row_dirty_state() {
    let store = LineStore::new(vec!["line".to_string()]);
    let partitions = vec![Partition::new("A".to_string(), vec![0], 0, 1)];
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

    let mut model = AppModel::new(output, LoadStatus::complete());

    assert!(model.rows_dirty);
    model.mark_rows_clean();
    assert!(!model.rows_dirty);
}
