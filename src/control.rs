use crate::types::{Error, PowerState};

pub trait Control {
    fn set_power(
        &self,
        state: PowerState,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}
