use crate::error::ApplicationError;

pub trait Service {
    fn bootstrap(&self) -> Result<(), ApplicationError>;
}
