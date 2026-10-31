// TODO: TextEdit::singleline lost_focus: https://github.com/emilk/egui/issues/2142

use std::sync::LazyLock;

use crate::{
    Metadata, Parameter,
    r#const::{
        AUTHOR, DATE, DESCRIPTION, IDENTIFIER, LIPIDS, NAME, STEREOSPECIFIC_NUMBERS, VERSION,
    },
    egui::EQUAL,
    rule::{ALL_ALL_ASCENDING, Name, Order, Rule, Value},
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

pub static SORT_RULES: LazyLock<[Rule; 9]> = LazyLock::new(|| {
    [
        Rule::builder()
            .name(Name::exact(AUTHOR))
            .sort(Order::Ascending)
            .build(),
        Rule::builder()
            .name(Name::exact(DATE))
            .sort(Order::Ascending)
            .build(),
        Rule::builder()
            .name(Name::exact(DESCRIPTION))
            .sort(Order::Ascending)
            .build(),
        Rule::builder()
            .name(Name::exact(IDENTIFIER))
            .sort(Order::Ascending)
            .build(),
        Rule::builder()
            .name(Name::exact(LIPIDS))
            .sort(Order::Ascending)
            .build(),
        Rule::builder()
            .name(Name::exact(NAME))
            .sort(Order::Ascending)
            .build(),
        Rule::builder()
            .name(Name::exact(STEREOSPECIFIC_NUMBERS))
            .sort(Order::Ascending)
            .build(),
        Rule::builder()
            .name(Name::exact(VERSION))
            .sort(Order::Ascending)
            .build(),
        Rule::builder()
            .name(Name::All)
            .sort(Order::Ascending)
            .build(),
    ]
});

/// Writable parameters widget
pub struct Writable<'a> {
    parameters: &'a mut Metadata,
    // sort: &'a [&'a Rule],
}

impl<'a> Writable<'a> {
    pub fn new(parameters: &'a mut Metadata) -> Self {
        Self {
            parameters,
            // sort: &[&ALL_ALL_ASCENDING],
        }
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

                    // Столбец 4: Поле значения со специальными эдиторами
                    row.col(|ui| {
                        match &*parameter.name {
                            DATE => {
                                let mut date = parameter.value.parse().unwrap_or_default();
                                if DatePickerButton::new(&mut date)
                                    .id_salt(&ui.next_auto_id().value().to_string())
                                    .ui(ui)
                                    .changed()
                                {
                                    parameter.value = date.to_string();
                                }
                            }
                            IDENTIFIER => {
                                let mut number = parameter.value.parse::<u64>().unwrap_or(0);
                                if DragValue::new(&mut number).speed(0.1).ui(ui).changed() {
                                    parameter.value = number.to_string();
                                }
                            }
                            DESCRIPTION => {
                                // Многострочный редактор для описания
                                let response = TextEdit::multiline(&mut parameter.value)
                                    // .desired_width(f32::INFINITY)
                                    .ui(ui);
                                if response.lost_focus() || response.clicked_elsewhere() {
                                    parameter.value = parameter.value.trim().to_owned();
                                }
                            }
                            _ => {
                                // Стандартный однострочный редактор для всех остальных
                                let response = TextEdit::singleline(&mut parameter.value)
                                    // .desired_width(f32::INFINITY)
                                    .ui(ui);
                                if response.lost_focus() || response.clicked_elsewhere() {
                                    parameter.value = parameter.value.trim().to_owned();
                                }
                            }
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
            self.parameters
                .retain_mut(|parameter| !parameter.name.is_empty() || !parameter.value.is_empty());
            self.parameters.filter_and_sort(&*SORT_RULES);
        }

        // Кнопка добавления
        if ui.button(PLUS).clicked() {
            self.parameters.push(Parameter::default());
        }
    }
}
