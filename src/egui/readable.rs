use crate::{
    AUTHORS, DATE, DESCRIPTION, Metadata, NAME, PARAMETERS, VERSION, egui::MetadataOptions,
};
use egui::{Grid, Label, Response, Ui, Widget};

/// Readable metadata widget
pub(super) struct Readable<'a> {
    metadata: &'a Metadata,
    options: MetadataOptions,
}

impl<'a> Readable<'a> {
    pub(super) fn new(metadata: &'a Metadata, options: MetadataOptions) -> Self {
        Self { metadata, options }
    }
}

impl Readable<'_> {
    pub(super) fn show(&self, ui: &mut Ui) -> Response {
        Grid::new(ui.next_auto_id())
            .show(ui, |ui| {
                if self.options.name {
                    ui.label("Name");
                    if let Some(name) = self.metadata.get(NAME) {
                        ui.label(name);
                    }
                    ui.end_row();
                }
                if self.options.description {
                    ui.label("Description");
                    if let Some(description) = self.metadata.get(DESCRIPTION) {
                        Label::new(description).truncate().ui(ui);
                    }
                    ui.end_row();
                }
                if self.options.authors {
                    ui.label("Authors");
                    if let Some(authors) = self.metadata.get(AUTHORS) {
                        ui.label(authors);
                    }
                    ui.end_row();
                }
                if self.options.parameters {
                    ui.label("Parameters");
                    if let Some(parameters) = self.metadata.get(PARAMETERS) {
                        ui.label(parameters);
                    }
                    ui.end_row();
                }
                if self.options.version {
                    ui.label("Version");
                    if let Some(version) = self.metadata.get(VERSION) {
                        ui.label(version);
                    }
                    ui.end_row();
                }
                if self.options.date {
                    ui.label("Date");
                    if let Some(date) = self.metadata.get(DATE) {
                        ui.label(date);
                    }
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
