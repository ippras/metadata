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
    writable: bool,
}

impl<T> MetadataWidget<T> {
    pub fn new(metadata: T) -> Self {
        Self {
            metadata,
            writable: false,
        }
    }
}

impl MetadataWidget<&mut Metadata> {
    pub fn with_writable(self, writable: bool) -> Self {
        Self { writable, ..self }
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
        Readable::new(self.metadata.borrow()).show(ui)
    }
}

impl<T: BorrowMut<Metadata>> MetadataWidget<T> {
    /// Writable
    fn writable(&mut self, ui: &mut Ui) {
        Writable::new(self.metadata.borrow_mut()).show(ui);
    }
}
