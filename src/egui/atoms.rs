//! Метаданные в виде [`egui::Atoms`].

use crate::{Metadata, rule::Rule};
use egui::{Atoms, IntoAtoms};
use std::borrow::Borrow;

/// Параметры, отобранные и упорядоченные правилами, как атомы вида `{имя=значение}`.
pub struct MetadataAtoms<'a, T> {
    metadata: &'a Metadata,
    rules: &'a [T],
}

impl<'a, T> MetadataAtoms<'a, T> {
    pub fn new(metadata: &'a Metadata, rules: &'a [T]) -> Self {
        Self { metadata, rules }
    }
}

impl<'a, T: Borrow<Rule>> IntoAtoms<'a> for MetadataAtoms<'_, T> {
    fn collect(self, atoms: &mut Atoms<'a>) {
        // `filter_and_sort` мутирует набор, поэтому работаем с копией.
        let mut selected = self.metadata.clone();
        selected.filter_and_sort(self.rules);
        for parameter in &selected {
            atoms.push_right(format!("{{{parameter}}}"));
        }
    }
}
