use crate::Metadata;
use chrono::Local;
use egui::{DragValue, TextEdit, Ui};
use egui_extras::{Column, DatePickerButton, TableBuilder};
use egui_phosphor::regular::{MINUS, PLUS};
use semver::Version;

// TAG
impl Metadata {
    pub fn show(&mut self, ui: &mut Ui) {
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
                            .add(TextEdit::singleline(&mut self.name).desired_width(f32::INFINITY))
                            .lost_focus()
                        {
                            self.name = self.name.trim_end().to_owned();
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
                                TextEdit::multiline(&mut self.description)
                                    .desired_width(f32::INFINITY),
                            )
                            .lost_focus()
                        {
                            self.description = self.description.trim_end().to_owned();
                        }
                    });
                });
                // Authors
                body.row(height, |mut row| {
                    row.col(|ui| {
                        ui.label("Authors");
                    });
                    row.col(|ui| {
                        self.authors.retain_mut(|author| {
                            let mut keep = true;
                            ui.horizontal(|ui| {
                                keep = !ui.button(MINUS).clicked();
                                if ui
                                    .add(TextEdit::singleline(author).desired_width(f32::INFINITY))
                                    .lost_focus()
                                {
                                    *author = author.trim_end().to_owned();
                                }
                            });
                            keep
                        });
                        if ui.button(PLUS).clicked() {
                            self.authors.push(String::new());
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
                            let mut checked = self.version.is_some();
                            if ui.checkbox(&mut checked, "").changed() {
                                self.version = checked.then_some(Version::new(0, 0, 0));
                            }
                            if let Some(version) = &mut self.version {
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
                            let mut checked = self.date.is_some();
                            if ui.checkbox(&mut checked, "").changed() {
                                self.date = checked.then_some(Local::now().date_naive());
                            }
                            if let Some(date) = &mut self.date {
                                ui.add(DatePickerButton::new(date).show_icon(false));
                            }
                        });
                    });
                });
            });
    }
}
