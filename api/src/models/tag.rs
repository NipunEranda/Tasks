use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Visibility {
    PUBLIC,
    PRIVATE,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub _id: ObjectId,
    pub name: String,
    pub created_by: ObjectId,
    pub visibility: Visibility,
    pub workspace: ObjectId,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagRequest {
    pub name: String,
    pub visibility: Visibility,
    pub workspace: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagResponse {
    pub id: String,
    pub name: String,
    pub created_by: String,
    pub visibility: Visibility,
    pub deleted: bool,
}

impl TryFrom<TagRequest> for Tag {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: TagRequest) -> Result<Self, Self::Error> {
        if !ObjectId::parse_str(&item.workspace).is_ok() {
            return Err("Workspace cannot be empty".into());
        }

        let workspace_id = ObjectId::parse_str(item.workspace).ok().unwrap();

        Ok(Self {
            _id: ObjectId::new(),
            name: item.name,
            created_by: ObjectId::new(),
            visibility: Visibility::PUBLIC,
            workspace: workspace_id,
            deleted: false,
        })
    }
}

impl TagResponse {
    pub fn copy(obj: &Tag) -> Self {
        TagResponse {
            id: obj._id.to_hex(),
            name: obj.name.clone(),
            created_by: obj.created_by.to_hex(),
            visibility: obj.visibility,
            deleted: obj.deleted,
        }
    }
}

impl Tag {
    pub fn new() -> Self {
        Tag {
            _id: ObjectId::new(),
            name: String::from(""),
            created_by: ObjectId::new(),
            visibility:Visibility::PUBLIC,
            deleted: false,
            workspace: ObjectId::new(),
        }
    }
}