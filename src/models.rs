use tokio::sync::broadcast;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct UserProfile {
    pub username: String,
    pub peer_addr: String,
}
pub struct ChatServerState {
    pub tx: broadcast::Sender<String>,
    pub online_users: Arc<RwLock<Vec<UserProfile>>>,
}