use crate::types::Error;

pub trait Monitoring {
    fn get_power(&self) -> impl std::future::Future<Output = Result<isize, Error>> + Send;
}
