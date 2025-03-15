use std::time::SystemTime;

use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    _id: ObjectId,
    name: String,
    deleted: bool,
    is_active: bool,
    created: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceResponse {
    pub id: String,
    pub name: String,
    pub deleted: bool,
    pub is_active: bool,
    pub created: DateTime,
}

impl TryFrom<WorkspaceRequest> for Workspace {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: WorkspaceRequest) -> Result<Self, Self::Error> {
        let chrono_datetime: SystemTime = chrono::Utc::now().into();

        Ok(Self {
            _id: ObjectId::new(),
            name: item.name,
            deleted: false,
            is_active: true,
            created: DateTime::from(chrono_datetime),
        })
    }
}

impl WorkspaceResponse {
    pub fn new(
        id: String,
        name: String,
        deleted: bool,
        is_active: bool,
        created: DateTime,
    ) -> Self {
        WorkspaceResponse {
            id,
            name,
            deleted,
            is_active,
            created,
        }
    }
}
