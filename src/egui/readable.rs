use crate::Metadata;
use egui::{Grid, Response, Ui, Widget};

/// Readable parameters widget
pub struct Readable<'a> {
    metadata: &'a Metadata,
}

impl<'a> Readable<'a> {
    pub fn new(metadata: &'a Metadata) -> Self {
        Self { metadata }
    }
}

impl Readable<'_> {
    fn show(&self, ui: &mut Ui) -> Response {
        Grid::new(ui.next_auto_id())
            .show(ui, |ui| {
                for parameter in self.metadata {
                    ui.label(&parameter.name);
                    ui.label(&parameter.value);
                    ui.end_row();
                }
            })
            .response
    }
}

impl Widget for Readable<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui)
    }
}
