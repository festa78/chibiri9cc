#[derive(Debug)]
pub struct StatementWithLocation {
    pub statement: String,
    pub index: usize,
}

impl StatementWithLocation {
    pub fn str(&self) -> String {
        let mut pointer = (0..(self.index as i32)).map(|_| ' ').collect::<String>();
        pointer.push('^');
        format!("{}\n{}\n", self.statement, pointer)
    }
}
