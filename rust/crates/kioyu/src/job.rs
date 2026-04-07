use async_trait::async_trait;
use uuid::Uuid;

use crate::resource::ResourceKey;

pub type ResourceRequest = Vec<(ResourceKey, usize)>;

#[derive(Debug)]
pub struct Job<P> {
    pub id: Uuid,
    pub name: String,
    pub resources: ResourceRequest,
    pub payload: P,
}

impl<P> Job<P> {
    pub fn new(name: String, payload: P, resources: ResourceRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            resources,
            payload,
        }
    }
}

#[async_trait]
pub trait IPayload<P>: Send + Sync + 'static {
    async fn execute(&self, job: Job<P>);
}
