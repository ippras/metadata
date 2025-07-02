use crate::{MetaDataFrame, Metadata, Result};
use polars::{io::mmap::MmapBytesReader, prelude::*};
use std::{borrow::BorrowMut, collections::BTreeMap, io::Write};

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

impl From<Arc<BTreeMap<PlSmallStr, PlSmallStr>>> for Metadata {
    fn from(value: Arc<BTreeMap<PlSmallStr, PlSmallStr>>) -> Self {
        Self(
            value
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect(),
        )
    }
}

impl From<Metadata> for Arc<BTreeMap<PlSmallStr, PlSmallStr>> {
    fn from(value: Metadata) -> Self {
        Arc::new(
            value
                .iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}

impl MetaDataFrame {
    pub fn read_ipc(reader: impl MmapBytesReader) -> Result<Self> {
        let mut reader = IpcReader::new(reader);
        let meta = reader.custom_metadata()?.unwrap_or_default().into();
        let data = reader.finish()?;
        Ok(Self { meta, data })
    }
}

impl<D: BorrowMut<DataFrame>> MetaDataFrame<Metadata, D> {
    pub fn write_ipc(mut self, writer: impl Write) -> Result<()> {
        let mut writer = IpcWriter::new(writer);
        writer.set_custom_schema_metadata(self.meta.into());
        writer.finish(self.data.borrow_mut())?;
        Ok(())
    }
}
