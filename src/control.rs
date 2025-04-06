use crate::types::Error;

pub trait Control {
    async fn set_power(&self) -> Result<(), Error>;
}
