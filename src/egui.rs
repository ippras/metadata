use crate::{AUTHORS, DATE, DESCRIPTION, Metadata, NAME, VERSION};
use chrono::NaiveDate;
use egui::{
    DragValue, Grid, Label, Response, TextEdit, Ui,
    cache::{ComputerMut, FrameCache},
};
use egui_extras::{Column, DatePickerButton, TableBuilder};
use egui_phosphor::regular::{MINUS, PLUS};
use semver::Version;
use std::borrow::{Borrow, BorrowMut};
use tracing::error;

pub const DATE_FORMAT: &str = "%Y-%m-%d";

/// Metadata widget
pub struct MetadataWidget<T> {
    metadata: T,
    writable: bool,
    name: bool,
    description: bool,
    authors: bool,
    version: bool,
    date: bool,
}

impl<T> MetadataWidget<T> {
    pub fn new(metadata: T) -> Self {
        Self {
            metadata,
            writable: false,
            name: true,
            description: true,
            authors: true,
            version: true,
            date: true,
        }
    }
}

impl MetadataWidget<&mut Metadata> {
    pub fn with_writable(self, writable: bool) -> Self {
        Self { writable, ..self }
    }

    pub fn with_name(self, name: bool) -> Self {
        Self { name, ..self }
    }

    pub fn with_description(self, description: bool) -> Self {
        Self {
            description,
            ..self
        }
    }

    pub fn with_authors(self, authors: bool) -> Self {
        Self { authors, ..self }
    }

    pub fn with_version(self, version: bool) -> Self {
        Self { version, ..self }
    }

    pub fn with_date(self, date: bool) -> Self {
        Self { date, ..self }
    }

    pub fn show(mut self, ui: &mut Ui) {
        if self.writable {
            self.writable(ui);
        } else {
            self.readable(ui);
        }
    }
}

impl MetadataWidget<&Metadata> {
    pub fn show(self, ui: &mut Ui) {
        self.readable(ui);
    }
}

impl<T: Borrow<Metadata>> MetadataWidget<T> {
    /// Readable
    fn readable(&self, ui: &mut Ui) -> Response {
        Grid::new(ui.next_auto_id())
            .show(ui, |ui| {
                let metadata = self.metadata.borrow();
                if self.name {
                    ui.label("Name");
                    if let Some(name) = metadata.get(NAME) {
                        ui.label(name);
                    }
                    ui.end_row();
                }
                if self.description {
                    ui.label("Description");
                    if let Some(description) = metadata.get(DESCRIPTION) {
                        ui.add(Label::new(description).truncate());
                    }
                    ui.end_row();
                }
                if self.authors {
                    ui.label("Authors");
                    if let Some(authors) = metadata.get(AUTHORS) {
                        ui.label(authors);
                    }
                    ui.end_row();
                }
                if self.version {
                    ui.label("Version");
                    if let Some(version) = metadata.get(VERSION) {
                        ui.label(version);
                    }
                    ui.end_row();
                }
                if self.date {
                    ui.label("Date");
                    if let Some(date) = metadata.get(DATE) {
                        ui.label(date);
                    }
                    ui.end_row();
                }
            })
            .response
    }
}

impl<T: BorrowMut<Metadata>> MetadataWidget<T> {
    /// Writable
    fn writable(&mut self, ui: &mut Ui) {
        ui.style_mut().visuals.collapsing_header_frame = true;
        let height = ui.spacing().interact_size.y;
        TableBuilder::new(ui)
            .resizable(false)
            .column(Column::auto())
            .column(Column::remainder())
            .body(|mut body| {
                let metadata = self.metadata.borrow_mut();
                // Name
                if self.name {
                    body.row(height, |mut row| {
                        row.col(|ui| {
                            ui.label("Name");
                        });
                        row.col(|ui| {
                            let name = metadata.entry(NAME.to_owned()).or_default();
                            if ui
                                .add(TextEdit::singleline(name).desired_width(f32::INFINITY))
                                .lost_focus()
                            {
                                *name = name.trim().to_owned();
                            }
                        });
                    });
                }
                // Description
                if self.description {
                    body.row(height, |mut row| {
                        row.col(|ui| {
                            ui.label("Description");
                        });
                        row.col(|ui| {
                            let description = metadata.entry(DESCRIPTION.to_owned()).or_default();
                            if ui
                                .add(TextEdit::multiline(description).desired_width(f32::INFINITY))
                                .lost_focus()
                            {
                                *description = description.trim().to_owned();
                            }
                        });
                    });
                }
                // Authors
                if self.authors {
                    body.row(height, |mut row| {
                        row.col(|ui| {
                            ui.label("Authors");
                        });
                        row.col(|ui| {
                            let value = metadata.entry(AUTHORS.to_owned()).or_default();
                            let mut authors = ui.memory_mut(|memory| {
                                memory.caches.cache::<AuthorsComputed>().get(value)
                            });
                            let mut changed = false;
                            authors.retain_mut(|author| {
                                let mut keep = true;
                                ui.horizontal(|ui| {
                                    keep = !ui.button(MINUS).clicked();
                                    changed |= !keep;
                                    let response = ui.add(
                                        TextEdit::singleline(author).desired_width(f32::INFINITY),
                                    );
                                    changed |= response.changed();
                                    if response.lost_focus() {
                                        *author = author.trim().to_owned();
                                        changed = true;
                                    }
                                });
                                keep
                            });
                            if changed {
                                *value = authors.join(",");
                            }
                            if ui.button(PLUS).clicked() {
                                value.push(',');
                            }
                        });
                    });
                }
                // Version
                if self.version {
                    body.row(height, |mut row| {
                        row.col(|ui| {
                            ui.label("Version");
                        });
                        row.col(|ui| {
                            ui.horizontal(|ui| {
                                let value = metadata.entry(VERSION.to_owned()).or_default();
                                let mut version = Version::parse(value).unwrap_or_else(|error| {
                                    error!(%error);
                                    let version = Version::new(0, 0, 0);
                                    *value = version.to_string();
                                    version
                                });
                                let mut changed = false;
                                ui.menu_button(version.to_string(), |ui| {
                                    ui.visuals_mut().widgets.inactive = ui.visuals().widgets.active;
                                    ui.horizontal(|ui| {
                                        changed |=
                                            ui.add(DragValue::new(&mut version.major)).changed();
                                        changed |=
                                            ui.add(DragValue::new(&mut version.minor)).changed();
                                        changed |=
                                            ui.add(DragValue::new(&mut version.patch)).changed();
                                    });
                                });
                                if changed {
                                    *value = version.to_string();
                                }
                            });
                        });
                    });
                }
                // Date
                if self.date {
                    body.row(height, |mut row| {
                        row.col(|ui| {
                            ui.label("Date");
                        });
                        row.col(|ui| {
                            ui.horizontal(|ui| {
                                let value = metadata.entry(DATE.to_owned()).or_default();
                                let mut date = NaiveDate::parse_from_str(value, DATE_FORMAT)
                                    .unwrap_or_else(|error| {
                                        error!(%error);
                                        let date = NaiveDate::default();
                                        *value = date.to_string();
                                        date
                                    });
                                if ui
                                    .add(DatePickerButton::new(&mut date).show_icon(false))
                                    .changed()
                                {
                                    *value = date.to_string();
                                }
                            });
                        });
                    });
                }
            });
    }
}

/// Authors computed
type AuthorsComputed = FrameCache<Vec<String>, AuthorsComputer>;

/// Authors computer
#[derive(Default)]
struct AuthorsComputer;

impl ComputerMut<&str, Vec<String>> for AuthorsComputer {
    fn compute(&mut self, key: &str) -> Vec<String> {
        key.split(',').map(ToOwned::to_owned).collect()
    }
}

// /// Writable
// fn writable(metadata: &mut Metadata, ui: &mut Ui) {
//     ui.style_mut().visuals.collapsing_header_frame = true;
//     let height = ui.spacing().interact_size.y;
//     TableBuilder::new(ui)
//         .resizable(true)
//         .column(Column::auto())
//         .column(Column::remainder())
//         .body(|mut body| {
//             // Name
//             body.row(height, |mut row| {
//                 row.col(|ui| {
//                     ui.label("Name");
//                 });
//                 row.col(|ui| {
//                     if ui
//                         .add(TextEdit::singleline(&mut metadata.name).desired_width(f32::INFINITY))
//                         .lost_focus()
//                     {
//                         metadata.name = metadata.name.trim().to_owned();
//                     }
//                 });
//             });
//             // Description
//             body.row(height, |mut row| {
//                 row.col(|ui| {
//                     ui.label("Description");
//                 });
//                 row.col(|ui| {
//                     if ui
//                         .add(
//                             TextEdit::multiline(&mut metadata.description)
//                                 .desired_width(f32::INFINITY),
//                         )
//                         .lost_focus()
//                     {
//                         metadata.description = metadata.description.trim().to_owned();
//                     }
//                 });
//             });
//             // Authors
//             body.row(height, |mut row| {
//                 row.col(|ui| {
//                     ui.label("Authors");
//                 });
//                 row.col(|ui| {
//                     metadata.authors.retain_mut(|author| {
//                         let mut keep = true;
//                         ui.horizontal(|ui| {
//                             keep = !ui.button(MINUS).clicked();
//                             if ui
//                                 .add(TextEdit::singleline(author).desired_width(f32::INFINITY))
//                                 .lost_focus()
//                             {
//                                 *author = author.trim().to_owned();
//                             }
//                         });
//                         keep
//                     });
//                     if ui.button(PLUS).clicked() {
//                         metadata.authors.push(String::new());
//                     }
//                 });
//             });
//             // Version
//             body.row(height, |mut row| {
//                 row.col(|ui| {
//                     ui.label("Version");
//                 });
//                 row.col(|ui| {
//                     ui.horizontal(|ui| {
//                         let mut checked = metadata.version.is_some();
//                         if ui.checkbox(&mut checked, "").changed() {
//                             metadata.version = checked.then_some(Version::new(0, 0, 0));
//                         }
//                         if let Some(version) = &mut metadata.version {
//                             ui.menu_button(version.to_string(), |ui| {
//                                 ui.visuals_mut().widgets.inactive = ui.visuals().widgets.active;
//                                 ui.horizontal(|ui| {
//                                     ui.add(DragValue::new(&mut version.major));
//                                     ui.add(DragValue::new(&mut version.minor));
//                                     ui.add(DragValue::new(&mut version.patch));
//                                 });
//                             });
//                         }
//                     });
//                 });
//             });
//             // Date
//             body.row(height, |mut row| {
//                 row.col(|ui| {
//                     ui.label("Date");
//                 });
//                 row.col(|ui| {
//                     ui.horizontal(|ui| {
//                         let mut checked = metadata.date.is_some();
//                         if ui.checkbox(&mut checked, "").changed() {
//                             metadata.date = checked.then_some(Local::now().date_naive());
//                         }
//                         if let Some(date) = &mut metadata.date {
//                             ui.add(DatePickerButton::new(date).show_icon(false));
//                         }
//                     });
//                 });
//             });
//         });
// }

// /// Writable
// fn writable(metadata: &mut Metadata, ui: &mut Ui) {
//     ui.style_mut().visuals.collapsing_header_frame = true;
//     let height = ui.spacing().interact_size.y;
//     TableBuilder::new(ui)
//         .resizable(true)
//         .column(Column::auto())
//         .column(Column::remainder())
//         .body(|mut body| {
//             // Name
//             body.row(height, |mut row| {
//                 row.col(|ui| {
//                     ui.label("Name");
//                 });
//                 row.col(|ui| {
//                     if let Some(name) = metadata.get_mut(NAME) {
//                         if ui
//                             .add(TextEdit::singleline(name).desired_width(f32::INFINITY))
//                             .lost_focus()
//                         {
//                             *name = name.trim().to_owned();
//                         }
//                     } else {
//                         if ui.checkbox(&mut false, "").changed() {
//                             metadata.insert(NAME.to_owned(), String::new());
//                         }
//                     }
//                 });
//             });
//             // Description
//             if let Some(description) = metadata.get_mut(DESCRIPTION) {
//                 body.row(height, |mut row| {
//                     row.col(|ui| {
//                         ui.label("Description");
//                     });
//                     row.col(|ui| {
//                         if ui
//                             .add(TextEdit::multiline(description).desired_width(f32::INFINITY))
//                             .lost_focus()
//                         {
//                             *description = description.trim().to_owned();
//                         }
//                     });
//                 });
//             }
//             // Authors
//             if let Some(authors) = metadata.get_mut(AUTHORS) {
//                 body.row(height, |mut row| {
//                     row.col(|ui| {
//                         ui.label("Authors");
//                     });
//                     let mut authors: Vec<_> = authors.split(",").map(str::trim).collect();
//                     row.col(|ui| {
//                         authors.retain_mut(|author| {
//                             let mut keep = true;
//                             ui.horizontal(|ui| {
//                                 keep = !ui.button(MINUS).clicked();
//                                 if ui
//                                     .add(TextEdit::singleline(author).desired_width(f32::INFINITY))
//                                     .lost_focus()
//                                 {
//                                     *author = author.trim().to_owned();
//                                 }
//                             });
//                             keep
//                         });
//                         if ui.button(PLUS).clicked() {
//                             authors.push(String::new());
//                         }
//                     });
//                 });
//             }
//             // Version
//             if let Some(version) = metadata.get_mut(VERSION) {
//                 body.row(height, |mut row| {
//                     row.col(|ui| {
//                         ui.label("Version");
//                     });
//                     row.col(|ui| {
//                         if ui
//                             .add(TextEdit::singleline(version).desired_width(f32::INFINITY))
//                             .lost_focus()
//                         {
//                             *version = version.trim().to_owned();
//                         }
//                     });
//                 });
//             }
//             // Date
//             body.row(height, |mut row| {
//                 row.col(|ui| {
//                     ui.label("Date");
//                 });
//                 // if let Ok(mut parsed) = NaiveDate::parse_from_str(date, DATE_FORMAT) {
//                 //     if ui
//                 //         .add(DatePickerButton::new(&mut parsed).show_icon(false))
//                 //         .changed()
//                 //     {
//                 //         *date = parsed.format(DATE_FORMAT).to_string();
//                 //     }
//                 // } else {
//                 //     ui.label(&*date);
//                 // }
//                 row.col(|ui| {
//                     ui.horizontal(|ui| {
//                         let value = metadata.get_mut(DATE);
//                         let mut checked = value.is_some();
//                         let value = value.unwrap_or(&mut String::new());
//                         let t = NaiveDate::parse_from_str(value, DATE_FORMAT);
//                         if ui.checkbox(&mut checked, "").changed() {
//                             metadata.date = checked.then_some(Local::now().date_naive());
//                         }
//                         if let Some(date) = &mut metadata.date {}
//                     });
//                 });
//                 // row.col(|ui| {
//                 //     ui.horizontal(|ui| {
//                 //         let mut checked = metadata.date.is_some();
//                 //         if ui.checkbox(&mut checked, "").changed() {
//                 //             metadata.date = checked.then_some(Local::now().date_naive());
//                 //         }
//                 //         if let Some(date) = &mut metadata.date {}
//                 //     });
//                 // });
//             });
//         });
// }
