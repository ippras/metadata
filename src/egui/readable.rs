use crate::{
    Metadata,
    r#const::{AUTHORS, DATES, DESCRIPTION, NAME, PARAMETERS, PREFIX, VERSIONS},
    egui::MetadataOptions,
};
use const_format::formatcp;
use egui::{Grid, Label, Response, Ui, Widget};
use egui_l10n::ContextExt;
use itertools::Itertools;

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
                    ui.label(ui.localize(formatcp!("{PREFIX}_{NAME}")));
                    ui.label(&self.metadata.name);
                    ui.end_row();
                }
                if self.options.description {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{DESCRIPTION}")));
                    Label::new(&self.metadata.description).truncate().ui(ui);
                    ui.end_row();
                }
                if self.options.authors && !self.metadata.authors.is_empty() {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{AUTHORS}")));
                    ui.label(self.metadata.authors.iter().format(", ").to_string());
                    ui.end_row();
                }
                if self.options.parameters && !self.metadata.parameters.is_empty() {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{PARAMETERS}")));
                    ui.label(self.metadata.parameters.iter().format(", ").to_string());
                    ui.end_row();
                }
                if self.options.versions && !self.metadata.versions.is_empty() {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{VERSIONS}")));
                    ui.label(self.metadata.versions.iter().format(", ").to_string());
                    ui.end_row();
                }
                if self.options.dates && !self.metadata.dates.is_empty() {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{DATES}")));
                    ui.label(self.metadata.dates.iter().format(", ").to_string());
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
