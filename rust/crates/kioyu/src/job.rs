use std::fmt::Display;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::resource::ResourceKey;

pub type ResourceAmount = usize;

#[derive(Debug, Clone)]
pub struct ResourceRequest(Vec<(ResourceKey, ResourceAmount)>);

impl ResourceRequest {
    pub fn new(req: Vec<(ResourceKey, ResourceAmount)>) -> Self {
        clerk::debug!("creating resource request with {} resource(s)", req.len());
        Self(req)
    }
    pub fn none() -> Self {
        clerk::debug!("creating empty resource request (no resources)");
        Self(Vec::new())
    }
    pub fn iter(&self) -> impl Iterator<Item = &(ResourceKey, ResourceAmount)> { self.0.iter() }

    pub fn as_slice(&self) -> &[(ResourceKey, ResourceAmount)] { &self.0 }
}

#[derive(Debug, Clone)]
pub struct Job<P> {
    pub id: Uuid,
    pub name: String,
    pub resources: ResourceRequest,
    pub payload: P,
}

impl<P> Job<P> {
    pub fn new(name: impl Into<String>, payload: P, resources: ResourceRequest) -> Self {
        let name = name.into();
        let id = Uuid::new_v4();
        clerk::debug!(
            "created job '{}' (id={}) with {} resource(s)",
            name,
            id,
            resources.as_slice().len()
        );
        Self {
            id,
            name,
            resources,
            payload,
        }
    }
}

#[async_trait]
pub trait IPayload: Send + Sync {
    type Error: Display;

    async fn execute(&self, cancel: CancellationToken) -> Result<(), Self::Error>;
    async fn post_process(&self) {}
}
