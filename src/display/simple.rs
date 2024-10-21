use super::Display;
use crate::repository::Repository;
use anyhow::Result;
pub struct SimpleDisplay {}
impl SimpleDisplay {
    pub fn new() -> Self {
        Self {}
    }
}
impl Display for SimpleDisplay {
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
        Ok(())
    }
}