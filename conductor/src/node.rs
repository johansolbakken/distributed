#[derive(Debug, Clone)]
pub struct Node {
    id: String,
    role: NodeRole,
    tags: Vec<String>,
}

impl Node {
    pub fn new(
        id: String,
        role: NodeRole,
        tags: Vec<String>,
    ) -> Self {
        Node {
            id,
            role,
            tags,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn role(&self) -> &NodeRole {
        &self.role
    }

    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }
}

#[derive(Debug, Clone)]
pub enum NodeRole {
    Leader,
    Follower,
    Worker,
    Coordinator,
}
