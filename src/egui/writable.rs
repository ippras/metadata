// TODO: TextEdit::singleline lost_focus: https://github.com/emilk/egui/issues/2142

use crate::{
    Parameter, Parameters,
    r#const::{DATE, DESCRIPTION, IDENTIFIER, VERSION},
    egui::EQUAL,
    rule::SPECIAL_ALPHABETICAL_RULES,
};
use egui::{
    Button, DragValue, Event, Grid, Popup, PopupCloseBehavior, TextEdit, TextStyle, TextWrapMode,
    Ui, Widget,
    cache::{ComputerMut, FrameCache},
    containers::menu::{MenuButton, MenuConfig},
};
use egui_extras::{Column, DatePickerButton, TableBody, TableBuilder};
use egui_l10n::ContextExt;
use egui_phosphor::regular::{MINUS, PLUS, SORT_ASCENDING};

/// Writable parameters widget
pub struct Writable<'a> {
    parameters: &'a mut Parameters,
}

impl<'a> Writable<'a> {
    pub fn new(parameters: &'a mut Parameters) -> Self {
        Self { parameters }
    }
}

impl Writable<'_> {
    pub fn show(&mut self, ui: &mut Ui) {
        let mut sort_clicked = false;
        let mut to_remove = Vec::new();

        let header_height = ui.text_style_height(&TextStyle::Heading);
        let body_height = ui.text_style_height(&TextStyle::Body);
        let width = ui.style().spacing.text_edit_width;
        // Отрисовываем таблицу параметров
        TableBuilder::new(ui)
            .striped(true) // Полосатый фон для лучшей читаемости строк
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto().resizable(false)) // Кнопка удаления (MINUS)
            .column(Column::auto_with_initial_suggestion(width)) // Поле ключа (Name)
            .column(Column::auto().resizable(false)) // Переключатель наличия значения (EQUAL)
            .column(Column::remainder()) // Поле значения (Value)
            // .column(Column::auto_with_initial_suggestion(width)) // Поле значения (Value)
            .header(header_height, |mut header| {
                header.col(|ui| {
                    ui.add_enabled_ui(!self.parameters.is_empty(), |ui| {
                        if ui.button(SORT_ASCENDING).clicked() {
                            sort_clicked = true;
                        }
                    });
                });
                header.col(|ui| {
                    ui.heading("Name");
                });
                header.col(|_ui| {
                    // Пустой заголовок для столбца с кнопкой EQUAL
                });
                header.col(|ui| {
                    ui.heading("Value");
                });
            })
            .body(|body| {
                body.rows(body_height, self.parameters.len(), |mut row| {
                    let index = row.index();
                    let parameter = &mut self.parameters[index];

                    // Столбец 1: Кнопка удаления
                    row.col(|ui| {
                        if ui.button(MINUS).clicked() {
                            to_remove.push(index);
                        }
                    });

                    // Столбец 2: Поле ключа (имени)
                    row.col(|ui| {
                        let response = TextEdit::singleline(&mut parameter.name)
                            .desired_width(f32::INFINITY) // Заполняем всю ширину колонки
                            .ui(ui);
                        if response.lost_focus() || response.clicked_elsewhere() {
                            parameter.name = parameter.name.trim().to_owned();
                        }
                    });

                    // Столбец 3: Переключатель наличия значения
                    row.col(|ui| {
                        let checked = parameter.value.is_some();
                        if ui.selectable_label(checked, EQUAL).clicked() {
                            parameter.value = if checked { None } else { Some(String::new()) };
                        }
                    });

                    // Столбец 4: Поле значения со специальными эдиторами
                    row.col(|ui| {
                        if let Some(value) = &mut parameter.value {
                            match &*parameter.name {
                                DATE => {
                                    let mut date = value.parse().unwrap_or_default();
                                    if DatePickerButton::new(&mut date)
                                        .id_salt(&ui.next_auto_id().value().to_string())
                                        .ui(ui)
                                        .changed()
                                    {
                                        *value = date.to_string();
                                    }
                                }
                                IDENTIFIER => {
                                    let mut number = value.parse::<u64>().unwrap_or(0);
                                    if DragValue::new(&mut number).speed(0.1).ui(ui).changed() {
                                        *value = number.to_string();
                                    }
                                }
                                DESCRIPTION => {
                                    // Многострочный редактор для описания
                                    let response = TextEdit::multiline(value)
                                        // .desired_width(f32::INFINITY)
                                        .ui(ui);
                                    if response.lost_focus() || response.clicked_elsewhere() {
                                        *value = value.trim().to_owned();
                                    }
                                }
                                _ => {
                                    // Стандартный однострочный редактор для всех остальных
                                    let response = TextEdit::singleline(value)
                                        // .desired_width(f32::INFINITY)
                                        .ui(ui);
                                    if response.lost_focus() || response.clicked_elsewhere() {
                                        *value = value.trim().to_owned();
                                    }
                                }
                            }
                        } else {
                            ui.disable();
                            let mut text = String::new();
                            TextEdit::singleline(&mut text)
                                // .desired_width(f32::INFINITY)
                                .ui(ui);
                        }
                    });
                });
            });

        // Обработка отложенных действий (удаление и сортировка)
        // Выполняем после отрисовки таблицы, чтобы избежать проблем с заимствованием (borrow checker)

        if !to_remove.is_empty() {
            let mut i = 0;
            self.parameters.retain_mut(|_| {
                let keep = !to_remove.contains(&i);
                i += 1;
                keep
            });
        }

        if sort_clicked {
            self.parameters.retain_mut(|parameter| {
                !parameter.name.is_empty()
                    || parameter
                        .value
                        .as_ref()
                        .is_some_and(|value| !value.is_empty())
            });
            self.parameters
                .filter_and_sort(&*SPECIAL_ALPHABETICAL_RULES);
        }

        // Кнопка добавления
        if ui.button(PLUS).clicked() {
            self.parameters.push(Parameter::default());
        }
    }
}

// impl Writable<'_> {
//     pub(super) fn show(&mut self, ui: &mut Ui) {
//         ui.style_mut().visuals.collapsing_header_frame = true;
//         // let height = ui.spacing().interact_size.y;
//         Grid::new("WritableGrid").num_columns(2).show(ui, |ui| {
//             // Name
//             if self.options.name {
//                 key_value(ui, NAME, |ui| name(&mut self.metadata.name, ui));
//             }
//             // Description
//             if self.options.description {
//                 key_value(ui, DESCRIPTION, |ui| {
//                     description(&mut self.metadata.description, ui)
//                 });
//             }
//             ui.separator();
//             ui.separator();
//             ui.end_row();
//             // Authors
//             if self.options.authors {
//                 key_value(ui, AUTHORS, |ui| authors(&mut self.metadata.authors, ui));
//             }
//             ui.separator();
//             ui.separator();
//             ui.end_row();
//             // Parameters
//             if self.options.parameters {
//                 key_value(ui, PARAMETERS, |ui| {
//                     parameters(&mut self.metadata.parameters, ui)
//                 });
//             }
//             ui.separator();
//             ui.separator();
//             ui.end_row();
//             // Versions
//             if self.options.versions {
//                 key_value(ui, VERSIONS, |ui| versions(&mut self.metadata.versions, ui));
//             }
//             ui.separator();
//             ui.separator();
//             ui.end_row();
//             // Date
//             if self.options.dates {
//                 key_value(ui, DATES, |ui| dates(&mut self.metadata.dates, ui));
//             }
//         });
//         // let height = ui.spacing().interact_size.y;
//         // TableBuilder::new(ui)
//         //     .resizable(false)
//         //     .column(Column::auto().clip(false))
//         //     .column(Column::remainder())
//         //     .body(|mut body| {
//         //         // Name
//         //         if self.options.name {
//         //             body.key_value(height, NAME, |ui| name(self.metadata, ui));
//         //         }
//         //         // Description
//         //         if self.options.description {
//         //             body.key_value(height, DESCRIPTION, |ui| description(self.metadata, ui));
//         //         }
//         //         // Authors
//         //         if self.options.authors {
//         //             body.key_value(height, AUTHORS, |ui| authors(self.metadata, ui));
//         //         }
//         //         // Parameters
//         //         if self.options.parameters {
//         //             body.key_value(height, PARAMETERS, |ui| parameters(self.metadata, ui));
//         //         }
//         //         // Version
//         //         if self.options.version {
//         //             body.key_value(height, VERSION, |ui| version(self.metadata, ui));
//         //         }
//         //         // Date
//         //         if self.options.date {
//         //             body.key_value(height, DATE, |ui| dates(self.metadata, ui));
//         //         }
//         //     });
//     }
// }

// fn key_value(ui: &mut Ui, key: &str, value: impl FnOnce(&mut Ui)) {
//     ui.vertical(|ui| {
//         ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
//         ui.label(ui.localize(&format!("{PREFIX}_{key}")));
//     });
//     ui.vertical(|ui| {
//         // ui.set_min_width(ui.available_width() / 2.0);
//         value(ui);
//     });
//     ui.end_row();
// }

// fn authors(authors: &mut Vec<String>, ui: &mut Ui) {
//     let mut changed = false;
//     authors.retain_mut(|author| {
//         let mut keep = true;
//         ui.horizontal(|ui| {
//             keep = !ui.button(MINUS).clicked();
//             changed |= !keep;
//             let response = TextEdit::singleline(author)
//                 .desired_width(f32::INFINITY)
//                 .ui(ui);
//             changed |= response.changed();
//             if response.lost_focus() || response.clicked_elsewhere() {
//                 *author = author.trim().to_owned();
//                 changed = true;
//             }
//         });
//         keep
//     });
//     ui.horizontal(|ui| {
//         if ui.button(PLUS).clicked() {
//             authors.push(String::new());
//         }
//         if authors.is_empty() {
//             ui.disable();
//         }
//         if ui.button(SORT_ASCENDING).clicked() {
//             authors.retain_mut(|author| !author.is_empty());
//             authors.sort();
//         }
//     });
// }

// fn dates(dates: &mut Vec<Date>, ui: &mut Ui) {
//     dates.retain_mut(|date| {
//         let mut keep = true;
//         let id_salt = format!("{date}{:?}", ui.next_auto_id());
//         ui.horizontal(|ui| {
//             keep = !ui.button(MINUS).clicked();
//             DatePickerButton::new(date)
//                 .id_salt(&id_salt)
//                 .show_icon(false)
//                 .ui(ui);
//         });
//         keep
//     });
//     ui.horizontal(|ui| {
//         if ui.button(PLUS).clicked() {
//             dates.push(Zoned::now().date());
//         }
//         if dates.is_empty() {
//             ui.disable();
//         }
//         if ui.button(SORT_ASCENDING).clicked() {
//             dates.sort();
//         }
//     });
// }

// fn description(description: &mut String, ui: &mut Ui) {
//     let response = TextEdit::multiline(description)
//         .desired_width(f32::INFINITY)
//         .ui(ui);
//     if response.lost_focus() || response.clicked_elsewhere() {
//         *description = description.trim().to_owned();
//     }
// }

// fn name(name: &mut String, ui: &mut Ui) {
//     let response = TextEdit::singleline(name)
//         .desired_width(f32::INFINITY)
//         .ui(ui);
//     if response.lost_focus() || response.clicked_elsewhere() {
//         *name = name.trim().to_owned();
//     }
// }

// fn parameters(parameters: &mut Vec<Parameter>, ui: &mut Ui) {
//     parameters.retain_mut(|Parameter { name: key, value }| {
//         let mut keep = true;
//         // let desired_width = ui.spacing().text_edit_width / 2.0;
//         // let desired_width = ui.available_width() / 2.0;
//         let available_width = ui.available_width();
//         ui.horizontal(|ui| {
//             keep = !ui.button(MINUS).clicked();
//             let response = TextEdit::singleline(key)
//                 .desired_width(ui.spacing().text_edit_width.min(available_width / 2.0))
//                 .ui(ui);
//             if response.lost_focus() || response.clicked_elsewhere() {
//                 *key = key.trim().to_owned();
//             }
//             let checked = value.is_some();
//             let response = ui.selectable_label(checked, EQUAL);
//             if response.clicked() {
//                 *value = if checked { None } else { Some(String::new()) };
//             }
//             if let Some(value) = value {
//                 let response = TextEdit::singleline(value)
//                     .desired_width(available_width)
//                     .ui(ui);
//                 if response.lost_focus() || response.clicked_elsewhere() {
//                     *value = value.trim().to_owned();
//                 }
//             } else {
//                 ui.disable();
//                 let mut text = String::new();
//                 TextEdit::singleline(&mut text)
//                     .desired_width(available_width)
//                     .ui(ui);
//             }
//         });
//         keep
//     });
//     ui.horizontal(|ui| {
//         if ui.button(PLUS).clicked() {
//             parameters.push(Parameter::new());
//         }
//         if parameters.is_empty() {
//             ui.disable();
//         }
//         if ui.button(SORT_ASCENDING).clicked() {
//             parameters.retain_mut(|parameter| {
//                 !parameter.name.is_empty()
//                     || parameter
//                         .value
//                         .as_ref()
//                         .is_some_and(|value| !value.is_empty())
//             });
//             parameters.sort();
//         }
//     });
// }

// fn versions(versions: &mut Vec<Version>, ui: &mut Ui) {
//     versions.retain_mut(|version| {
//         let mut keep = true;
//         ui.horizontal(|ui| {
//             keep = !ui.button(MINUS).clicked();
//             let response = ui.button(format!("{}.{}.{}", version.0, version.1, version.2));
//             Popup::menu(&response)
//                 .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
//                 .id(ui.next_auto_id())
//                 .show(|ui| {
//                     ui.visuals_mut().widgets.inactive = ui.visuals().widgets.active;
//                     Grid::new(ui.auto_id_with("VersionGrid")).show(ui, |ui| {
//                         let size = ui.spacing().interact_size;
//                         if Button::new(CARET_UP)
//                             .frame(false)
//                             .min_size(size)
//                             .ui(ui)
//                             .clicked()
//                         {
//                             version.0 = version.0.saturating_add(1);
//                         }
//                         if Button::new(CARET_UP)
//                             .frame(false)
//                             .min_size(size)
//                             .ui(ui)
//                             .clicked()
//                         {
//                             version.1 = version.1.saturating_add(1);
//                         }
//                         if Button::new(CARET_UP)
//                             .frame(false)
//                             .min_size(size)
//                             .ui(ui)
//                             .clicked()
//                         {
//                             version.2 = version.2.saturating_add(1);
//                         }
//                         ui.end_row();
//                         DragValue::new(&mut version.0).ui(ui);
//                         DragValue::new(&mut version.1).ui(ui);
//                         DragValue::new(&mut version.2).ui(ui);
//                         ui.end_row();
//                         if Button::new(CARET_DOWN)
//                             .frame(false)
//                             .min_size(size)
//                             .ui(ui)
//                             .clicked()
//                         {
//                             version.0 = version.0.saturating_sub(1);
//                         }
//                         if Button::new(CARET_DOWN)
//                             .frame(false)
//                             .min_size(size)
//                             .ui(ui)
//                             .clicked()
//                         {
//                             version.1 = version.1.saturating_sub(1);
//                         }
//                         if Button::new(CARET_DOWN)
//                             .frame(false)
//                             .min_size(size)
//                             .ui(ui)
//                             .clicked()
//                         {
//                             version.2 = version.2.saturating_sub(1);
//                         }
//                     });
//                 });
//         });
//         keep
//     });
//     ui.horizontal(|ui| {
//         if ui.button(PLUS).clicked() {
//             versions.push(Version::new());
//         }
//         if versions.is_empty() {
//             ui.disable();
//         }
//         if ui.button(SORT_ASCENDING).clicked() {
//             versions.sort();
//         }
//     });
// }
