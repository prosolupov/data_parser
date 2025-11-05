use crate::format::Format;
use crate::models::Record;
use std::error::Error;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct BinForma {
    pub bin_rows: Vec<Record>,
}

impl Format for BinForma {
    fn from_read<R: Read>(r: &mut R) -> Result<Self, Box<dyn Error>> {
        todo!()
    }

    fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
