struct Mt940{

}

impl Mt940 {
    pub fn from_read<R: std::io::Read>(r: &mut R) -> Result<Self> {
        todo!()
    }

    pub fn write_to<W: std::io::Write>(&mut self, writer: &mut W) -> Result<()> {
        todo!()
    }
}