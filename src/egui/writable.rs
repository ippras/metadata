// TODO: TextEdit::singleline lost_focus: https://github.com/emilk/egui/issues/2142

use crate::{
    Metadata, Parameter,
    r#const::{DATE, DESCRIPTION, IDENTIFIER},
    rule::ALL_ALL_ASCENDING,
};
use egui::{
    Align, Button, DragValue, InnerResponse, Layout, Response, TextEdit, TextStyle, Ui, Widget,
};
use egui_extras::{Column, DatePickerButton, TableBuilder};
use egui_phosphor::regular::{MINUS, PLUS, SORT_ASCENDING};

/// Какой редактор нужен значению параметра; определяется по имени.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ValueKind {
    Date,
    Identifier,
    Description,
    Text,
}

impl ValueKind {
    fn of(name: &str) -> Self {
        match name {
            DATE => Self::Date,
            IDENTIFIER => Self::Identifier,
            DESCRIPTION => Self::Description,
            _ => Self::Text,
        }
    }
}

/// Итог отрисовки таблицы: что изменилось и какие действия запросил пользователь.
#[derive(Default)]
struct Outcome {
    changed: bool,
    remove: Option<usize>,
    sort: bool,
}

/// Виджет редактирования параметров метаданных.
pub struct Writable<'a> {
    metadata: &'a mut Metadata,
}

impl<'a> Writable<'a> {
    pub fn new(metadata: &'a mut Metadata) -> Self {
        Self { metadata }
    }

    fn show(&mut self, ui: &mut Ui) -> Response {
        let InnerResponse {
            inner: changed,
            mut response,
        } = ui.scope(|ui| {
            let outcome = self.table(ui);
            let mut changed = outcome.changed;

            if let Some(index) = outcome.remove {
                self.metadata.remove(index);
                changed = true;
            }
            if outcome.sort {
                clean_and_sort(self.metadata);
                changed = true;
            }
            if ui.button(PLUS).clicked() {
                self.metadata.push(Parameter::default());
                changed = true;
            }
            changed
        });
        if changed {
            response.mark_changed();
        }
        response
    }

    /// Рисует таблицу. Структуру `Metadata` не меняет: удаление и сортировка
    /// выполняются после отрисовки, чтобы не менять список во время обхода.
    fn table(&mut self, ui: &mut Ui) -> Outcome {
        let mut outcome = Outcome::default();
        let header_height = ui.text_style_height(&TextStyle::Heading);
        let row_height = ui.text_style_height(&TextStyle::Body);
        let name_width = ui.style().spacing.text_edit_width;
        let is_empty = self.metadata.is_empty();
        let rows = self.metadata.len();

        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(Layout::left_to_right(Align::Center))
            .column(Column::auto().resizable(false)) // Удаление
            .column(Column::auto_with_initial_suggestion(name_width)) // Имя
            .column(Column::remainder()) // Значение
            .header(header_height, |mut header| {
                header.col(|ui| {
                    outcome.sort = ui
                        .add_enabled(!is_empty, Button::new(SORT_ASCENDING))
                        .clicked();
                });
                header.col(|ui| {
                    ui.heading("Name");
                });
                header.col(|ui| {
                    ui.heading("Value");
                });
            })
            .body(|body| {
                body.rows(row_height, rows, |mut row| {
                    let index = row.index();
                    let parameter = &mut self.metadata[index];
                    row.col(|ui| {
                        if ui.button(MINUS).clicked() {
                            outcome.remove = Some(index);
                        }
                    });
                    row.col(|ui| {
                        outcome.changed |= name_editor(ui, &mut parameter.name);
                    });
                    row.col(|ui| {
                        outcome.changed |= value_editor(ui, index, parameter);
                    });
                });
            });
        outcome
    }
}

impl Widget for Writable<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        self.show(ui)
    }
}

/// Убирает полностью пустые параметры и сортирует остальные.
fn clean_and_sort(metadata: &mut Metadata) {
    metadata.retain(|parameter| !parameter.name.is_empty() || !parameter.value.is_empty());
    metadata.filter_and_sort([&*ALL_ALL_ASCENDING]);
}

/// Редактор имени. Возвращает `true`, если имя изменилось.
fn name_editor(ui: &mut Ui, name: &mut String) -> bool {
    let response = TextEdit::singleline(&mut *name)
        .desired_width(f32::INFINITY)
        .ui(ui);
    finish_edit(&response, name)
}

/// Выбирает редактор по имени параметра. Возвращает `true`, если значение изменилось.
fn value_editor(ui: &mut Ui, index: usize, parameter: &mut Parameter) -> bool {
    match ValueKind::of(&parameter.name) {
        ValueKind::Date => date_editor(ui, index, &mut parameter.value),
        ValueKind::Identifier => identifier_editor(ui, &mut parameter.value),
        ValueKind::Description => {
            let response = TextEdit::multiline(&mut parameter.value).ui(ui);
            finish_edit(&response, &mut parameter.value)
        }
        ValueKind::Text => {
            let response = TextEdit::singleline(&mut parameter.value).ui(ui);
            finish_edit(&response, &mut parameter.value)
        }
    }
}

fn date_editor(ui: &mut Ui, index: usize, value: &mut String) -> bool {
    // Некорректная строка показывается как 1970-01-01 и заменяется только при правке.
    let mut date = value.parse().unwrap_or_default();
    let changed = DatePickerButton::new(&mut date)
        // .id_salt(ui.id().with(index))
        .id_salt(&index.to_string())
        .ui(ui)
        .changed();
    if changed {
        *value = date.to_string();
    }
    changed
}

fn identifier_editor(ui: &mut Ui, value: &mut String) -> bool {
    let mut number = value.parse::<u64>().unwrap_or_default();
    let changed = DragValue::new(&mut number).speed(0.1).ui(ui).changed();
    if changed {
        *value = number.to_string();
    }
    changed
}

/// Обрезает пробелы по окончании редактирования. Возвращает `true`, если текст изменился.
fn finish_edit(response: &Response, text: &mut String) -> bool {
    let trimmed = (response.lost_focus() || response.clicked_elsewhere()) && trim_in_place(text);
    response.changed() || trimmed
}

/// Обрезает пробелы с обоих концов без новой аллокации. `true`, если строка изменилась.
fn trim_in_place(text: &mut String) -> bool {
    let before = text.len();
    text.truncate(text.trim_end().len());
    let leading = text.len() - text.trim_start().len();
    text.replace_range(..leading, "");
    text.len() != before
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_both_ends() {
        let mut text = String::from("  a b \t");
        assert!(trim_in_place(&mut text));
        assert_eq!(text, "a b");
        assert!(!trim_in_place(&mut text));
    }

    #[test]
    fn kind_by_name() {
        assert_eq!(ValueKind::of(DATE), ValueKind::Date);
        assert_eq!(ValueKind::of(IDENTIFIER), ValueKind::Identifier);
        assert_eq!(ValueKind::of(DESCRIPTION), ValueKind::Description);
        assert_eq!(ValueKind::of("Anything"), ValueKind::Text);
    }
}
