use uuid::Uuid;

use crate::resource::ResourceKey;
pub type ResourceAmount = usize;

#[derive(Debug, Clone)]
pub struct ResourceRequest(Vec<(ResourceKey, ResourceAmount)>);

impl ResourceRequest {
    pub fn new(req: Vec<(ResourceKey, ResourceAmount)>) -> Self { Self(req) }

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
    pub fn new(payload: P, name: impl Into<String>, resources: ResourceRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            resources,
            payload,
        }
    }
}
