use crate::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Store {
    type Configuration: Send + Sync + 'static;
    type Pool: Send + Sync + 'static;

    async fn connect(config: &Self::Configuration) -> Result<Self::Pool>;
    async fn run_migrations(&self) -> Result<()>;
    async fn pool(&self) -> Result<Self::Pool>;
}
