use super::Display;
use crate::repository::Repository;
use anyhow::Result;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;

pub struct TableDisplay {
    table: Table,
}

impl TableDisplay {
    pub fn new() -> Self {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec!["Repository", "Status"]);
        Self { table }
    }
}

impl Display for TableDisplay {
    fn init(&self, _total: u64) -> Result<()> {
        todo!();
    }

    fn create_step(&self, _repo: &Repository) -> Result<()> {
        todo!();
    }

    fn finish_step(&self, _repo: &Repository) -> Result<()> {
        todo!();
    }
    fn set_message(&self, _message: String) -> Result<()> {
        Ok(())
    }

    fn finish(&self) -> Result<()> {
        println!("{}", self.table);
        Ok(())
    }
}
