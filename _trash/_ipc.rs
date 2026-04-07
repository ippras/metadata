use crate::{MetaDataFrame, Metadata, Result};
use polars::{io::mmap::MmapBytesReader, prelude::*};
use std::{borrow::BorrowMut, io::Write};

// /// Extension methods for [`IpcReader`]
// pub trait IpcReaderExt {
//     fn metadata(&mut self) -> Result<Option<Metadata>>;
// }

// impl<R: MmapBytesReader> IpcReaderExt for IpcReader<R> {
//     fn metadata(&mut self) -> Result<Option<Metadata>> {
//         let Some(metadata) = self.custom_metadata()? else {
//             return Ok(None);
//         };
//         Ok(Some(Metadata::from(metadata)))
//     }
// }

// /// Extension methods for [`IpcWriter`]
// pub trait IpcWriterExt {
//     fn metadata(&mut self, metadata: Metadata);
// }

// impl<W: Write> IpcWriterExt for IpcWriter<W> {
//     fn metadata(&mut self, metadata: Metadata) {
//         self.set_custom_schema_metadata(Arc::new(metadata.into()));
//     }
// }

// impl MetaDataFrame {
//     pub fn read_ipc(reader: impl MmapBytesReader) -> Result<Self> {
//         let mut reader = IpcReader::new(reader);
//         let meta = reader.metadata()?.unwrap_or_default();
//         let data = reader.finish()?;
//         Ok(Self { meta, data })
//     }
// }

// impl<D: BorrowMut<DataFrame>> MetaDataFrame<Metadata, D> {
//     pub fn write_ipc(mut self, writer: impl Write) -> Result<()> {
//         let mut writer = IpcWriter::new(writer);
//         writer.metadata(self.meta);
//         writer.finish(self.data.borrow_mut())?;
//         Ok(())
//     }
// }
