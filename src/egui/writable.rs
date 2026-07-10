// TODO: TextEdit::singleline lost_focus: https://github.com/emilk/egui/issues/2142

use crate::{
    Metadata,
    r#const::{AUTHORS, DATE, DESCRIPTION, NAME, PARAMETERS, PREFIX, VERSION},
    egui::{EQUAL, MetadataOptions, SEMICOLON},
};
use egui::{
    Button, DragValue, Event, Grid, Popup, PopupCloseBehavior, TextEdit, TextWrapMode, Ui, Widget,
    cache::{ComputerMut, FrameCache},
    containers::menu::{MenuButton, MenuConfig},
};
use egui_extras::{Column, DatePickerButton, TableBody, TableBuilder};
use egui_l10n::ContextExt;
use egui_phosphor::regular::{CARET_DOWN, CARET_UP, MINUS, PLUS, SORT_ASCENDING};
use itertools::Itertools;
use jiff::{Zoned, civil::Date};
use semver::Version;
use std::collections::btree_map::Entry;
use tracing::{error, instrument};

/// Writable metadata widget
pub(super) struct Writable<'a> {
    metadata: &'a mut Metadata,
    options: MetadataOptions,
}

impl<'a> Writable<'a> {
    pub(super) fn new(metadata: &'a mut Metadata, options: MetadataOptions) -> Self {
        Self { metadata, options }
    }
}

impl Writable<'_> {
    pub(super) fn show(&mut self, ui: &mut Ui) {
        ui.style_mut().visuals.collapsing_header_frame = true;
        // let height = ui.spacing().interact_size.y;
        Grid::new("WritableGrid").num_columns(2).show(ui, |ui| {
            // Name
            if self.options.name {
                key_value(ui, NAME, |ui| name(self.metadata, ui));
            }
            // Description
            if self.options.description {
                key_value(ui, DESCRIPTION, |ui| description(self.metadata, ui));
            }
            ui.separator();
            ui.separator();
            ui.end_row();
            // Authors
            if self.options.authors {
                key_value(ui, AUTHORS, |ui| authors(self.metadata, ui));
            }
            ui.separator();
            ui.separator();
            ui.end_row();
            // Parameters
            if self.options.parameters {
                key_value(ui, PARAMETERS, |ui| parameters(self.metadata, ui));
            }
            ui.separator();
            ui.separator();
            ui.end_row();
            // Version
            if self.options.version {
                key_value(ui, VERSION, |ui| version(self.metadata, ui));
            }
            ui.separator();
            ui.separator();
            ui.end_row();
            // Date
            if self.options.date {
                key_value(ui, DATE, |ui| dates(self.metadata, ui));
            }
            ui.separator();
            ui.separator();
            ui.end_row();
        });
        // let height = ui.spacing().interact_size.y;
        // TableBuilder::new(ui)
        //     .resizable(false)
        //     .column(Column::auto().clip(false))
        //     .column(Column::remainder())
        //     .body(|mut body| {
        //         // Name
        //         if self.options.name {
        //             body.key_value(height, NAME, |ui| name(self.metadata, ui));
        //         }
        //         // Description
        //         if self.options.description {
        //             body.key_value(height, DESCRIPTION, |ui| description(self.metadata, ui));
        //         }
        //         // Authors
        //         if self.options.authors {
        //             body.key_value(height, AUTHORS, |ui| authors(self.metadata, ui));
        //         }
        //         // Parameters
        //         if self.options.parameters {
        //             body.key_value(height, PARAMETERS, |ui| parameters(self.metadata, ui));
        //         }
        //         // Version
        //         if self.options.version {
        //             body.key_value(height, VERSION, |ui| version(self.metadata, ui));
        //         }
        //         // Date
        //         if self.options.date {
        //             body.key_value(height, DATE, |ui| dates(self.metadata, ui));
        //         }
        //     });
    }
}

fn key_value(ui: &mut Ui, key: &str, value: impl FnOnce(&mut Ui)) {
    ui.vertical(|ui| {
        // ui.add_space(4.0);
        ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
        ui.label(ui.localize(&format!("{PREFIX}_{key}")));
    });
    ui.vertical(|ui| {
        ui.set_min_width(ui.available_width());
        value(ui);
    });
    ui.end_row();
}

fn authors(metadata: &mut Metadata, ui: &mut Ui) {
    let mut contains = false;
    if let Entry::Occupied(mut occupied) = metadata.entry(AUTHORS.to_owned()) {
        contains = true;
        let value = occupied.get_mut();
        let mut authors =
            ui.memory_mut(|memory| memory.caches.cache::<AuthorsComputed>().get(value).clone());
        let mut changed = false;
        authors.retain_mut(|author| {
            let mut keep = true;
            ui.horizontal(|ui| {
                keep = !ui.button(MINUS).clicked();
                changed |= !keep;
                let response = TextEdit::singleline(author)
                    .desired_width(f32::INFINITY)
                    .ui(ui);
                changed |= response.changed();
                if response.lost_focus() || response.clicked_elsewhere() {
                    *author = author.trim().to_owned();
                    changed = true;
                }
            });
            keep
        });
        if changed {
            if authors.is_empty() {
                metadata.remove(AUTHORS);
            } else {
                *value = authors.join(SEMICOLON);
            }
        }
    }
    ui.horizontal(|ui| {
        if ui.button(PLUS).clicked() {
            if !contains {
                metadata.insert(AUTHORS.to_owned(), String::new());
            } else {
                metadata
                    .entry(AUTHORS.to_owned())
                    .and_modify(|value| value.push_str(SEMICOLON))
                    .or_insert(SEMICOLON.to_owned());
            }
        }
        if !contains {
            ui.disable();
        }
        if ui.button(SORT_ASCENDING).clicked() {
            metadata
                .entry(AUTHORS.to_owned())
                .and_modify(|value| *value = value.split(SEMICOLON).sorted().join(SEMICOLON));
        }
    });
}

fn dates(metadata: &mut Metadata, ui: &mut Ui) {
    let mut contains = false;
    if let Entry::Occupied(mut occupied) = metadata.entry(DATE.to_owned()) {
        contains = true;
        let value = occupied.get_mut();
        let mut dates =
            ui.memory_mut(|memory| memory.caches.cache::<DatesComputed>().get(value).clone());
        let mut changed = false;
        let mut index = 0;
        dates.retain_mut(|date| {
            index += 1;
            let mut keep = true;
            ui.horizontal(|ui| {
                keep = !ui.button(MINUS).clicked();
                changed |= !keep;
                match parse_date(date) {
                    Ok(mut parsed) => {
                        let response = DatePickerButton::new(&mut parsed)
                            .id_salt(&index.to_string())
                            .show_icon(false)
                            .ui(ui);
                        if response.changed() {
                            *date = parsed.to_string();
                            changed = true;
                        }
                    }
                    Err(error) => {
                        error!(?error);
                        let response = TextEdit::singleline(date).ui(ui);
                        if response.changed() {
                            changed = true;
                        }
                    }
                }
            });
            keep
        });
        if changed {
            if dates.is_empty() {
                metadata.remove(DATE);
            } else {
                *value = dates.join(SEMICOLON);
            }
        }
    }
    ui.horizontal(|ui| {
        if ui.button(PLUS).clicked() {
            let today = Zoned::now().date();
            if !contains {
                metadata.insert(DATE.to_owned(), today.to_string());
            } else {
                metadata
                    .entry(DATE.to_owned())
                    .or_default()
                    .push_str(&format!("{SEMICOLON}{today}"));
            }
        }
        if !contains {
            ui.disable();
        }
        if ui.button(SORT_ASCENDING).clicked() {
            metadata
                .entry(DATE.to_owned())
                .and_modify(|value| *value = value.split(SEMICOLON).sorted().join(SEMICOLON));
        }
    });
}

// fn description(metadata: &mut Metadata, ui: &mut Ui) {
//     let entry = metadata.entry(DESCRIPTION.to_owned()).or_default();
//     let response = TextEdit::multiline(entry).ui(ui);
//     response.context_menu(|ui| {
//         if ui.button("Disable").clicked() {
//         }
//     });
//     if response.lost_focus() || response.clicked_elsewhere() {
//         *entry = entry.trim().to_owned();
//     }
// }
fn description(metadata: &mut Metadata, ui: &mut Ui) {
    let mut remove = None;
    if let Entry::Occupied(mut occupied) = metadata.entry(DESCRIPTION.to_owned()) {
        let value = occupied.get_mut();
        let response = TextEdit::multiline(value)
            .desired_width(f32::INFINITY)
            .ui(ui);
        response.context_menu(|ui| {
            if ui.button("Disable").clicked() {
                remove = Some(true);
            }
        });
        if response.lost_focus() || response.clicked_elsewhere() {
            *value = value.trim().to_owned();
        }
    } else {
        let mut text = String::new();
        let rect = ui
            .add_enabled(
                false,
                TextEdit::multiline(&mut text).desired_width(f32::INFINITY),
            )
            .rect;
        let response = ui.interact(
            rect,
            ui.auto_id_with("DescriptionDisableInteract"),
            egui::Sense::click(),
        );
        response.context_menu(|ui| {
            if ui.button("Enable").clicked() {
                remove = Some(false);
            }
        });
        // ui.disable();
        // let response = TextEdit::multiline(&mut text).ui(ui);
    }
    if let Some(remove) = remove {
        if remove {
            metadata.remove(DESCRIPTION);
        } else {
            metadata.insert(DESCRIPTION.to_owned(), String::new());
        }
    }
}

fn name(metadata: &mut Metadata, ui: &mut Ui) {
    let entry = metadata.entry(NAME.to_owned()).or_default();
    let response = TextEdit::singleline(entry)
        .desired_width(f32::INFINITY)
        .ui(ui);
    if response.lost_focus() || response.clicked_elsewhere() {
        *entry = entry.trim().to_owned();
    }
}

fn parameters(metadata: &mut Metadata, ui: &mut Ui) {
    let mut contains = false;
    if let Entry::Occupied(mut occupied) = metadata.entry(PARAMETERS.to_owned()) {
        contains = true;
        let value = occupied.get_mut();
        let mut parameters = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<ParametersComputed>()
                .get(value)
                .clone()
        });
        let mut changed = false;
        parameters.retain_mut(|(name, value)| {
            let mut keep = true;
            // let desired_width = ui.spacing().text_edit_width / 2.0;
            let desired_width = ui.available_width() / 2.0;
            ui.horizontal(|ui| {
                keep = !ui.button(MINUS).clicked();
                changed |= !keep;
                let response = TextEdit::singleline(name)
                    .desired_width(desired_width)
                    .ui(ui);
                changed |= response.changed();
                if response.lost_focus() || response.clicked_elsewhere() {
                    *name = name.trim().to_owned();
                    changed = true;
                }
                let checked = value.is_some();
                let response = ui.selectable_label(checked, EQUAL);
                if response.clicked() {
                    *value = if checked { None } else { Some(String::new()) };
                }
                if let Some(value) = value {
                    let response = TextEdit::singleline(value)
                        .desired_width(desired_width)
                        .ui(ui);
                    changed |= response.changed();
                    if response.lost_focus() || response.clicked_elsewhere() {
                        *value = value.trim().to_owned();
                        changed = true;
                    }
                } else {
                    ui.disable();
                    let mut text = String::new();
                    TextEdit::singleline(&mut text)
                        .desired_width(desired_width)
                        .ui(ui);
                }
            });
            keep
        });

        if changed {
            if parameters.is_empty() {
                metadata.remove(PARAMETERS);
            } else {
                *value = parameters
                    .into_iter()
                    .format_with(SEMICOLON, |(name, value), f| {
                        if let Some(value) = value {
                            f(&format_args!("{name}{EQUAL}{value}"))
                        } else {
                            f(&name)
                        }
                    })
                    .to_string();
            }
        }
    }
    ui.horizontal(|ui: &mut Ui| {
        if ui.button(PLUS).clicked() {
            if !contains {
                // Если параметров вообще нет, создаем ключ и добавляем первый параметр
                metadata.insert(PARAMETERS.to_owned(), String::new());
            } else {
                // Если параметры уже есть, просто добавляем еще один
                metadata
                    .entry(PARAMETERS.to_owned())
                    .and_modify(|value| value.push_str(SEMICOLON))
                    .or_insert(SEMICOLON.to_owned());
            }
        }
        if !contains {
            ui.disable();
        }
        if ui.button(SORT_ASCENDING).clicked() {
            metadata
                .entry(PARAMETERS.to_owned())
                .and_modify(|value| *value = value.split(SEMICOLON).sorted().join(SEMICOLON));
        }
    });
}

fn version(metadata: &mut Metadata, ui: &mut Ui) {
    let mut remove = None;
    if let Entry::Occupied(mut occupied) = metadata.entry(VERSION.to_owned()) {
        let value = occupied.get_mut();
        let mut version = Version::parse(value).unwrap_or_else(|error| {
            error!(%error);
            let version = Version::new(0, 0, 0);
            *value = version.to_string();
            version
        });
        let mut changed = false;
        let response = ui.button(version.to_string());
        Popup::menu(&response)
            .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
            .id(ui.id().with("VersionPopup"))
            .show(|ui| {
                ui.visuals_mut().widgets.inactive = ui.visuals().widgets.active;
                Grid::new(ui.auto_id_with("VersionGrid")).show(ui, |ui| {
                    let size = ui.spacing().interact_size;
                    if Button::new(CARET_UP)
                        .frame(false)
                        .min_size(size)
                        .ui(ui)
                        .clicked()
                    {
                        version.major = version.major.saturating_add(1);
                        changed = true;
                    }
                    if Button::new(CARET_UP)
                        .frame(false)
                        .min_size(size)
                        .ui(ui)
                        .clicked()
                    {
                        version.minor = version.minor.saturating_add(1);
                        changed = true;
                    }
                    if Button::new(CARET_UP)
                        .frame(false)
                        .min_size(size)
                        .ui(ui)
                        .clicked()
                    {
                        version.patch = version.patch.saturating_add(1);
                        changed = true;
                    }
                    ui.end_row();
                    changed |= DragValue::new(&mut version.major).ui(ui).changed();
                    changed |= DragValue::new(&mut version.minor).ui(ui).changed();
                    changed |= DragValue::new(&mut version.patch).ui(ui).changed();
                    ui.end_row();
                    if Button::new(CARET_DOWN)
                        .frame(false)
                        .min_size(size)
                        .ui(ui)
                        .clicked()
                    {
                        version.major = version.major.saturating_sub(1);
                        changed = true;
                    }
                    if Button::new(CARET_DOWN)
                        .frame(false)
                        .min_size(size)
                        .ui(ui)
                        .clicked()
                    {
                        version.minor = version.minor.saturating_sub(1);
                        changed = true;
                    }
                    if Button::new(CARET_DOWN)
                        .frame(false)
                        .min_size(size)
                        .ui(ui)
                        .clicked()
                    {
                        version.patch = version.patch.saturating_sub(1);
                        changed = true;
                    }
                });
            });
        // let response = MenuButton::new(version.to_string())
        //     .config(MenuConfig::new().close_behavior(PopupCloseBehavior::CloseOnClickOutside))
        //     .ui(ui, |ui| {
        //         ui.visuals_mut().widgets.inactive = ui.visuals().widgets.active;
        //         Grid::new(ui.auto_id_with("VersionGrid")).show(ui, |ui| {
        //             let size = ui.spacing().interact_size;
        //             if Button::new(CARET_UP)
        //                 .frame(false)
        //                 .min_size(size)
        //                 .ui(ui)
        //                 .clicked()
        //             {
        //                 version.major = version.major.saturating_add(1);
        //                 changed = true;
        //             }
        //             if Button::new(CARET_UP)
        //                 .frame(false)
        //                 .min_size(size)
        //                 .ui(ui)
        //                 .clicked()
        //             {
        //                 version.minor = version.minor.saturating_add(1);
        //                 changed = true;
        //             }
        //             if Button::new(CARET_UP)
        //                 .frame(false)
        //                 .min_size(size)
        //                 .ui(ui)
        //                 .clicked()
        //             {
        //                 version.patch = version.patch.saturating_add(1);
        //                 changed = true;
        //             }
        //             ui.end_row();
        //             changed |= DragValue::new(&mut version.major).ui(ui).changed();
        //             changed |= DragValue::new(&mut version.minor).ui(ui).changed();
        //             changed |= DragValue::new(&mut version.patch).ui(ui).changed();
        //             ui.end_row();
        //             if Button::new(CARET_DOWN)
        //                 .frame(false)
        //                 .min_size(size)
        //                 .ui(ui)
        //                 .clicked()
        //             {
        //                 version.major = version.major.saturating_sub(1);
        //                 changed = true;
        //             }
        //             if Button::new(CARET_DOWN)
        //                 .frame(false)
        //                 .min_size(size)
        //                 .ui(ui)
        //                 .clicked()
        //             {
        //                 version.minor = version.minor.saturating_sub(1);
        //                 changed = true;
        //             }
        //             if Button::new(CARET_DOWN)
        //                 .frame(false)
        //                 .min_size(size)
        //                 .ui(ui)
        //                 .clicked()
        //             {
        //                 version.patch = version.patch.saturating_sub(1);
        //                 changed = true;
        //             }
        //         });
        //     })
        //     .0;
        response.context_menu(|ui| {
            if ui.button("Disable").clicked() {
                remove = Some(true);
            }
        });
        if changed {
            *value = version.to_string();
        }
    } else {
        let version = Version::new(0, 0, 0);
        let rect = ui.add_enabled(false, Button::new(version.to_string())).rect;
        let response = ui.interact(
            rect,
            ui.auto_id_with("VersionDisableInteract"),
            egui::Sense::click(),
        );
        response.context_menu(|ui| {
            if ui.button("Enable").clicked() {
                remove = Some(false);
            }
        });
    }
    if let Some(remove) = remove {
        if remove {
            metadata.remove(VERSION);
        } else {
            metadata.insert(VERSION.to_owned(), String::new());
        }
    }
}

// fn parse_parameters(key: &str) -> impl Iterator<Item = (&str, Option<&str>)> {
//     key.split(SEMICOLON)
//         .map(|parameter| match parameter.split_once(EQUAL) {
//             Some((name, value)) => (name, Some(value)),
//             None => (parameter, None),
//         })
// }

#[instrument(err)]
fn parse_date(key: &str) -> Result<Date, jiff::Error> {
    key.parse()
}

/// Extension methods for [`TableBody`]
trait TableBodyExt {
    fn key_value(&mut self, height: f32, key: &str, value: impl FnOnce(&mut Ui));
}

impl<'a> TableBodyExt for TableBody<'a> {
    fn key_value(&mut self, height: f32, key: &str, value: impl FnOnce(&mut Ui)) {
        self.row(height, |mut row| {
            row.col(|ui| {
                ui.label(ui.localize(&format!("{PREFIX}_{key}")));
            });
            row.col(value);
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
        key.split(SEMICOLON).map(ToOwned::to_owned).collect()
    }
}

/// Dates computed
type DatesComputed = FrameCache<Vec<String>, DatesComputer>;

/// Dates computer
#[derive(Default)]
struct DatesComputer;

impl ComputerMut<&str, Vec<String>> for DatesComputer {
    fn compute(&mut self, key: &str) -> Vec<String> {
        key.split(SEMICOLON).map(ToOwned::to_owned).collect()
    }
}

/// Parameters computed
type ParametersComputed = FrameCache<Vec<(String, Option<String>)>, ParametersComputer>;

/// Parameters computer
#[derive(Default)]
struct ParametersComputer;

impl ComputerMut<&str, Vec<(String, Option<String>)>> for ParametersComputer {
    fn compute(&mut self, key: &str) -> Vec<(String, Option<String>)> {
        key.split(SEMICOLON)
            .map(|parameter| match parameter.split_once(EQUAL) {
                Some((name, value)) => (name.to_owned(), Some(value.to_owned())),
                None => (parameter.to_owned(), None),
            })
            .collect()
    }
}
