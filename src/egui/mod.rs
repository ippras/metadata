//! egui-виджеты для [`Metadata`](crate::Metadata).
//!
//! ```ignore
//! ui.add(Readable::new(&metadata));
//! ui.add(Writable::new(&mut metadata));
//! ```

mod atoms;
mod readable;
mod writable;

pub use self::{
    atoms::MetadataAtoms, readable::Readable as ReadableMetadata,
    writable::Writable as WritableMetadata,
};
