use crate::{Parameters, r#const::EM_DASH};
use egui::{Grid, Response, Ui, Widget};

/// Readable parameters widget
pub struct Readable<'a> {
    parameters: &'a Parameters,
}

impl<'a> Readable<'a> {
    pub fn new(parameters: &'a Parameters) -> Self {
        Self { parameters }
    }
}

impl Readable<'_> {
    pub fn show(&self, ui: &mut Ui) -> Response {
        Grid::new(ui.next_auto_id())
            .show(ui, |ui| {
                for parameter in self.parameters.iter() {
                    ui.label(&parameter.name);
                    if let Some(value) = &parameter.value {
                        ui.label(value);
                    } else {
                        ui.label(EM_DASH);
                    }
                    ui.end_row();
                }
                // if self.options.name {
                //     ui.label(ui.localize(formatcp!("{PREFIX}_{NAME}")));
                //     ui.label(&self.parameters.name);
                //     ui.end_row();
                // }
                // if self.options.description && !self.parameters.description.is_empty() {
                //     ui.label(ui.localize(formatcp!("{PREFIX}_{DESCRIPTION}")));
                //     Label::new(&self.parameters.description).truncate().ui(ui);
                //     ui.end_row();
                // }
                // if self.options.authors && !self.parameters.authors.is_empty() {
                //     ui.label(ui.localize(formatcp!("{PREFIX}_{AUTHORS}")));
                //     ui.label(self.parameters.authors.iter().format(", ").to_string());
                //     ui.end_row();
                // }
                // if self.options.parameters && !self.parameters.parameters.is_empty() {
                //     ui.label(ui.localize(formatcp!("{PREFIX}_{PARAMETERS}")));
                //     ui.label(self.parameters.parameters.iter().format(", ").to_string());
                //     ui.end_row();
                // }
                // if self.options.versions && !self.parameters.versions.is_empty() {
                //     ui.label(ui.localize(formatcp!("{PREFIX}_{VERSIONS}")));
                //     ui.label(self.parameters.versions.iter().format(", ").to_string());
                //     ui.end_row();
                // }
                // if self.options.dates && !self.parameters.dates.is_empty() {
                //     ui.label(ui.localize(formatcp!("{PREFIX}_{DATES}")));
                //     ui.label(self.parameters.dates.iter().format(", ").to_string());
                //     ui.end_row();
                // }
            })
            .response
    }
}

impl Widget for Readable<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui)
    }
}

// /// Kind
// pub enum Kind {
//     Multiline,
//     Singleline,
// }
