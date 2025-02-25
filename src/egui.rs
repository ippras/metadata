use crate::Metadata;
use chrono::Local;
use egui::{DragValue, Grid, Label, TextEdit, Ui};
use egui_extras::{Column, DatePickerButton, TableBuilder};
use egui_phosphor::regular::{MINUS, PLUS};
use semver::Version;

// Metadata
impl Metadata {
    pub fn widget(&mut self) -> MetadataWidget<'_> {
        MetadataWidget::new(self)
    }
}

/// Metadata widget
pub struct MetadataWidget<'a> {
    metadata: &'a mut Metadata,
    writable: bool,
}

impl<'a> MetadataWidget<'a> {
    pub fn new(metadata: &'a mut Metadata) -> Self {
        Self {
            metadata,
            writable: false,
        }
    }
}

impl MetadataWidget<'_> {
    pub fn writable(self, writable: bool) -> Self {
        Self {
            writable: writable,
            ..self
        }
    }

    pub fn show(self, ui: &mut Ui) {
        if self.writable {
            writable(self.metadata, ui);
        } else {
            readable(self.metadata, ui);
        }
    }
}

/// Readable
fn readable(metadata: &Metadata, ui: &mut Ui) {
    Grid::new(ui.next_auto_id()).show(ui, |ui| {
        ui.label("Name");
        ui.label(&metadata.name);
        ui.end_row();

        if !metadata.description.is_empty() {
            ui.label("Description");
            ui.add(Label::new(&metadata.description).truncate());
            ui.end_row();
        }

        ui.label("Authors");
        ui.label(metadata.authors.join(", "));
        ui.end_row();

        if let Some(version) = &metadata.version {
            ui.label("Version");
            ui.label(version.to_string());
            ui.end_row();
        }

        if let Some(date) = &metadata.date {
            ui.label("Date");
            ui.label(date.to_string());
            ui.end_row();
        }
    });
}

/// Writable
fn writable(metadata: &mut Metadata, ui: &mut Ui) {
    ui.style_mut().visuals.collapsing_header_frame = true;
    let height = ui.spacing().interact_size.y;
    TableBuilder::new(ui)
        .resizable(true)
        .column(Column::auto())
        .column(Column::remainder())
        .body(|mut body| {
            // Name
            body.row(height, |mut row| {
                row.col(|ui| {
                    ui.label("Name");
                });
                row.col(|ui| {
                    if ui
                        .add(TextEdit::singleline(&mut metadata.name).desired_width(f32::INFINITY))
                        .lost_focus()
                    {
                        metadata.name = metadata.name.trim().to_owned();
                    }
                });
            });
            // Description
            body.row(height, |mut row| {
                row.col(|ui| {
                    ui.label("Description");
                });
                row.col(|ui| {
                    if ui
                        .add(
                            TextEdit::multiline(&mut metadata.description)
                                .desired_width(f32::INFINITY),
                        )
                        .lost_focus()
                    {
                        metadata.description = metadata.description.trim().to_owned();
                    }
                });
            });
            // Authors
            body.row(height, |mut row| {
                row.col(|ui| {
                    ui.label("Authors");
                });
                row.col(|ui| {
                    metadata.authors.retain_mut(|author| {
                        let mut keep = true;
                        ui.horizontal(|ui| {
                            keep = !ui.button(MINUS).clicked();
                            if ui
                                .add(TextEdit::singleline(author).desired_width(f32::INFINITY))
                                .lost_focus()
                            {
                                *author = author.trim().to_owned();
                            }
                        });
                        keep
                    });
                    if ui.button(PLUS).clicked() {
                        metadata.authors.push(String::new());
                    }
                });
            });
            // Version
            body.row(height, |mut row| {
                row.col(|ui| {
                    ui.label("Version");
                });
                row.col(|ui| {
                    ui.horizontal(|ui| {
                        let mut checked = metadata.version.is_some();
                        if ui.checkbox(&mut checked, "").changed() {
                            metadata.version = checked.then_some(Version::new(0, 0, 0));
                        }
                        if let Some(version) = &mut metadata.version {
                            ui.menu_button(version.to_string(), |ui| {
                                ui.visuals_mut().widgets.inactive = ui.visuals().widgets.active;
                                ui.horizontal(|ui| {
                                    ui.add(DragValue::new(&mut version.major));
                                    ui.add(DragValue::new(&mut version.minor));
                                    ui.add(DragValue::new(&mut version.patch));
                                });
                            });
                        }
                    });
                });
            });
            // Date
            body.row(height, |mut row| {
                row.col(|ui| {
                    ui.label("Date");
                });
                row.col(|ui| {
                    ui.horizontal(|ui| {
                        let mut checked = metadata.date.is_some();
                        if ui.checkbox(&mut checked, "").changed() {
                            metadata.date = checked.then_some(Local::now().date_naive());
                        }
                        if let Some(date) = &mut metadata.date {
                            ui.add(DatePickerButton::new(date).show_icon(false));
                        }
                    });
                });
            });
        });
}
