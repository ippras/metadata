pub mod readable;
pub mod writable;

use crate::{
    Metadata,
    egui::{readable::Readable, writable::Writable},
};
use egui::{Response, Ui};
use std::borrow::{Borrow, BorrowMut};

pub const EQUAL: &str = "=";
pub const SEMICOLON: &str = ";";

/// Metadata widget
pub struct MetadataWidget<T> {
    metadata: T,
}

impl<T> MetadataWidget<T> {
    pub fn new(metadata: T) -> Self {
        Self { metadata }
    }
}

impl MetadataWidget<&Metadata> {
    pub fn show(self, ui: &mut Ui) {
        self.readable(ui);
    }
}

impl MetadataWidget<&mut Metadata> {
    pub fn show(mut self, ui: &mut Ui) {
        self.writable(ui);
    }
}

impl<T: Borrow<Metadata>> MetadataWidget<T> {
    /// Readable
    pub fn readable(&self, ui: &mut Ui) -> Response {
        Readable::new(self.metadata.borrow()).show(ui)
    }
}

impl<T: BorrowMut<Metadata>> MetadataWidget<T> {
    /// Writable
    pub fn writable(&mut self, ui: &mut Ui) {
        Writable::new(self.metadata.borrow_mut()).show(ui);
    }
}
