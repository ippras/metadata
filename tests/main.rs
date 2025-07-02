// use egui_kittest::{
//     Harness, Node, SnapshotResult, SnapshotResults,
//     kittest::{Queryable as _, by},
// };

// #[test]
// fn widget_tests() {
//     let mut results = SnapshotResults::new();
//     test_widget("radio", |ui| ui.radio(false, "Radio"), &mut results);
// }

// fn test_widget(name: &str, mut w: impl FnMut(&mut Ui) -> Response, results: &mut SnapshotResults) {
//     results.add(test_widget_layout(name, &mut w));
//     results.add(VisualTests::test(name, &mut w));
// }
