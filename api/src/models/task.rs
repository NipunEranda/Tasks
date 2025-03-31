use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub workspace: ObjectId,
    pub sub_tasks: Vec<SubTask>,
    pub tags: Vec<ObjectId>,
    pub completed: bool,
    pub created_by: ObjectId,
    pub last_update: DateTime,
    pub updated_by: ObjectId,
    pub is_private: bool,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubTask {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub assignees: Vec<ObjectId>,
    pub completed: bool,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskRequest {
    pub name: String,
    pub description: String,
    pub workspace: String,
    pub sub_tasks: Vec<SubTaskRequest>,
    pub tags: Vec<String>,
    pub is_private: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubTaskRequest {
    pub name: String,
    pub description: String,
    pub assignees: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub workspace: String,
    pub sub_tasks: Vec<SubTask>,
    pub tags: Vec<String>,
    pub completed: bool,
    pub created_by: String,
    pub last_update: DateTime,
    pub updated_by: String,
    pub is_private: bool,
    pub deleted: bool,
}

impl TryFrom<TaskRequest> for Task {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: TaskRequest) -> Result<Self, Self::Error> {
        if !ObjectId::parse_str(&item.workspace).is_ok() {
            return Err("Workspace cannot be empty".into());
        }

        let workspace_id = ObjectId::parse_str(item.workspace).ok().unwrap();
        let user_id = ObjectId::new(); // Replace with appropriate logic to set user_id

        let conv_sub_tasks = item
            .sub_tasks
            .into_iter()
            .map(SubTask::try_from)
            .collect::<Result<Vec<SubTask>, _>>()?;

        let conv_tags: Vec<ObjectId> = item
        .tags
        .into_iter()
        .filter_map(|tag| ObjectId::parse_str(&tag).ok())
        .collect();

        Ok(Self {
            _id: ObjectId::new(),
            name: item.name,
            created_by: user_id,
            is_private: item.is_private,
            workspace: workspace_id,
            description: item.description,
            sub_tasks: conv_sub_tasks,
            tags: conv_tags,
            completed: false,
            last_update: DateTime::now(),
            updated_by: user_id,
            deleted: false,
        })
    }
}

impl TryFrom<SubTaskRequest> for SubTask {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: SubTaskRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            name: item.name,
            description: item.description,
            assignees: item
                .assignees
                .into_iter()
                .filter_map(|assignee| ObjectId::parse_str(&assignee).ok())
                .collect(),
            completed: false,
            deleted: false,
        })
    }
}

impl TaskResponse {
    pub fn copy(obj: &Task) -> Self {
        TaskResponse {
            id: obj._id.to_hex(),
            name: obj.name.clone(),
            description: obj.description.clone(),
            workspace: obj.workspace.to_hex(),
            sub_tasks: obj.sub_tasks.clone(),
            tags: obj.tags.iter().map(|tag| tag.to_hex()).collect(),
            completed: obj.completed,
            created_by: obj.created_by.to_hex(),
            last_update: obj.last_update,
            updated_by: obj.updated_by.to_hex(),
            is_private: obj.is_private,
            deleted: obj.deleted
        }
    }
}
