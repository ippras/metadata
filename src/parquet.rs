use crate::{MetaDataFrame, Metadata, Result};
use polars::{io::mmap::MmapBytesReader, prelude::*};
use polars_parquet::write::KeyValue;
use std::{borrow::BorrowMut, io::Write};

// pub fn read_parquet_file<R: MmapBytesReader, D: BorrowMut<DataFrame>>(
//     path: impl AsRef<Path>,
// ) -> Result<MetaDataFrame> {
//     let file = File::open(path)?;
//     MetaDataFrame::read_parquet(file)
// }

// pub fn write_parquet<D: BorrowMut<DataFrame>>(
//     frame: MetaDataFrame<Metadata, D>,
//     path: impl AsRef<Path>,
// ) -> Result<()> {
//     let file = File::create(path)?;
//     frame.write_parquet(file)?;
//     Ok(())
// }

impl MetaDataFrame {
    pub fn read_parquet(reader: impl MmapBytesReader) -> Result<Self> {
        let mut reader = ParquetReader::new(reader).set_rechunk(true);
        let meta = reader
            .get_metadata()?
            .key_value_metadata()
            .as_ref()
            .map(|key_values| {
                Metadata(
                    key_values
                        .into_iter()
                        .filter_map(|KeyValue { key, value }| Some((key.clone(), value.clone()?)))
                        .collect(),
                )
            })
            .unwrap_or_default();
        let data = reader.finish()?;
        Ok(Self { meta, data })
    }
}

impl<D: BorrowMut<DataFrame>> MetaDataFrame<Metadata, D> {
    pub fn write_parquet(mut self, writer: impl Write) -> Result<()> {
        let writer = ParquetWriter::new(writer);
        writer
            .with_key_value_metadata(Some(KeyValueMetadata::Static(
                self.meta
                    .0
                    .into_iter()
                    .map(|(key, value)| KeyValue {
                        key,
                        value: Some(value),
                    })
                    .collect(),
            )))
            .finish(self.data.borrow_mut())?;
        Ok(())
    }
}
