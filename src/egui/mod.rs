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
    options: MetadataOptions,
    writable: bool,
}

impl<T> MetadataWidget<T> {
    pub fn new(metadata: T) -> Self {
        Self {
            metadata,
            options: MetadataOptions::new(),
            writable: false,
        }
    }
}

impl MetadataWidget<&mut Metadata> {
    pub fn with_authors(self, authors: bool) -> Self {
        Self {
            options: self.options.with_authors(authors),
            ..self
        }
    }

    pub fn with_date(self, date: bool) -> Self {
        Self {
            options: self.options.with_date(date),
            ..self
        }
    }

    pub fn with_description(self, description: bool) -> Self {
        Self {
            options: self.options.with_description(description),
            ..self
        }
    }

    pub fn with_name(self, name: bool) -> Self {
        Self {
            options: self.options.with_name(name),
            ..self
        }
    }

    pub fn with_parameters(self, parameters: bool) -> Self {
        Self {
            options: self.options.with_parameters(parameters),
            ..self
        }
    }

    pub fn with_version(self, version: bool) -> Self {
        Self {
            options: self.options.with_version(version),
            ..self
        }
    }

    pub fn with_writable(self, writable: bool) -> Self {
        Self { writable, ..self }
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
        Readable::new(self.metadata.borrow(), self.options).show(ui)
    }
}

impl<T: BorrowMut<Metadata>> MetadataWidget<T> {
    /// Writable
    fn writable(&mut self, ui: &mut Ui) {
        Writable::new(self.metadata.borrow_mut(), self.options).show(ui);
    }
}

/// Metadata options
#[derive(Clone, Copy, Debug, Default)]
struct MetadataOptions {
    authors: bool,
    dates: bool,
    description: bool,
    name: bool,
    parameters: bool,
    versions: bool,
}

impl MetadataOptions {
    fn new() -> Self {
        Self {
            authors: true,
            dates: true,
            description: true,
            name: true,
            parameters: true,
            versions: true,
        }
    }
}

impl MetadataOptions {
    fn with_authors(self, authors: bool) -> Self {
        Self { authors, ..self }
    }

    fn with_date(self, date: bool) -> Self {
        Self { dates: date, ..self }
    }

    fn with_description(self, description: bool) -> Self {
        Self {
            description,
            ..self
        }
    }

    fn with_name(self, name: bool) -> Self {
        Self { name, ..self }
    }

    fn with_parameters(self, parameters: bool) -> Self {
        Self { parameters, ..self }
    }

    fn with_version(self, version: bool) -> Self {
        Self { versions: version, ..self }
    }
}

mod readable;
mod writable;
