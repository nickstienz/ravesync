#[derive(Debug)]
pub enum Command {
    Exit,
    SetCellData(usize, usize, u32),
}
