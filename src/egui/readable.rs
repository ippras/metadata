use crate::{
    Metadata,
    r#const::{AUTHORS, DATE, DESCRIPTION, NAME, PARAMETERS, PREFIX, VERSION},
    egui::MetadataOptions,
};
use const_format::formatcp;
use egui::{Grid, Label, Response, Ui, Widget};
use egui_l10n::ContextExt;

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
                    if let Some(value) = self.metadata.get(NAME) {
                        ui.label(ui.localize(formatcp!("{PREFIX}_{NAME}")));
                        ui.label(value);
                        ui.end_row();
                    }
                }
                if self.options.description {
                    if let Some(value) = self.metadata.get(DESCRIPTION) {
                        ui.label(ui.localize(formatcp!("{PREFIX}_{DESCRIPTION}")));
                        Label::new(value).truncate().ui(ui);
                        ui.end_row();
                    }
                }
                if self.options.authors {
                    if let Some(value) = self.metadata.get(AUTHORS) {
                        ui.label(ui.localize(formatcp!("{PREFIX}_{AUTHORS}")));
                        ui.label(value);
                        ui.end_row();
                    }
                }
                if self.options.parameters {
                    if let Some(value) = self.metadata.get(PARAMETERS) {
                        ui.label(ui.localize(formatcp!("{PREFIX}_{PARAMETERS}")));
                        ui.label(value);
                        ui.end_row();
                    }
                }
                if self.options.version {
                    if let Some(value) = self.metadata.get(VERSION) {
                        ui.label(ui.localize(formatcp!("{PREFIX}_{VERSION}")));
                        ui.label(value);
                        ui.end_row();
                    }
                }
                if self.options.date {
                    if let Some(value) = self.metadata.get(DATE) {
                        ui.label(ui.localize(formatcp!("{PREFIX}_{DATE}")));
                        ui.label(value);
                        ui.end_row();
                    }
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
