use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) type Storage = Arc<RwLock<HashMap<u128, Vec<String>>>>;

pub(crate) type WarpResult<T> = Result<T, warp::Error>;

pub(crate) type WsMessage = warp::ws::Message;
