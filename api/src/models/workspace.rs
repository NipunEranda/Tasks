use chrono::Local;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Visibility {
    PUBLIC,
    PRIVATE,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub _id: ObjectId,
    pub owner: ObjectId,
    pub name: String,
    pub visibility: Visibility,
    pub deleted: bool,
    pub is_active: bool,
    pub created: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceRequest {
    pub name: String,
    pub visibility: Visibility
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceResponse {
    pub id: String,
    pub owner: String,
    pub name: String,
    pub visibility: Visibility,
    pub deleted: bool,
    pub is_active: bool,
    pub created: String,
}

impl TryFrom<WorkspaceRequest> for Workspace {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: WorkspaceRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            owner: ObjectId::new(),
            name: item.name,
            visibility: item.visibility,
            deleted: false,
            is_active: true,
            created: Local::now().to_string(),
        })
    }
}

impl WorkspaceResponse {
    pub fn new(
        id: String,
        owner: String,
        name: String,
        visibility: Visibility,
        deleted: bool,
        is_active: bool,
        created: String,
    ) -> Self {
        WorkspaceResponse {
            id,
            owner,
            name,
            visibility,
            deleted,
            is_active,
            created,
        }
    }
}
