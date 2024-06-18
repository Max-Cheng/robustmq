use crate::{
    core::lock::Lock,
    storage::{
        mqtt::{session::MQTTSessionStorage, topic::MQTTTopicStorage, user::MQTTUserStorage},
        rocksdb::RocksDBEngine,
    },
};
use common_base::errors::RobustMQError;
use prost::Message as _;
use protocol::placement_center::generate::mqtt::{
    CreateSessionRequest, CreateTopicRequest, CreateUserRequest, DeleteSessionRequest,
    DeleteTopicRequest, DeleteUserRequest, SetTopicRetainMessageRequest,
};
use std::sync::Arc;
use tonic::Status;

pub struct DataRouteMQTT {
    pub rocksdb_engine_handler: Arc<RocksDBEngine>,
}
impl DataRouteMQTT {
    pub fn new(rocksdb_engine_handler: Arc<RocksDBEngine>) -> Self {
        return DataRouteMQTT {
            rocksdb_engine_handler,
        };
    }

    pub fn create_user(&self, value: Vec<u8>) -> Result<(), RobustMQError> {
        let req = CreateUserRequest::decode(value.as_ref())
            .map_err(|e| Status::invalid_argument(e.to_string()))
            .unwrap();
        let storage = MQTTUserStorage::new(self.rocksdb_engine_handler.clone());
        match storage.save(req.cluster_name, req.user_name, req.content) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub fn delete_user(&self, value: Vec<u8>) -> Result<(), RobustMQError> {
        let req = DeleteUserRequest::decode(value.as_ref())
            .map_err(|e| Status::invalid_argument(e.to_string()))
            .unwrap();
        let storage = MQTTUserStorage::new(self.rocksdb_engine_handler.clone());
        match storage.delete(req.cluster_name, req.user_name) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub fn create_topic(&self, value: Vec<u8>) -> Result<(), RobustMQError> {
        let req = CreateTopicRequest::decode(value.as_ref())
            .map_err(|e| Status::invalid_argument(e.to_string()))
            .unwrap();
        let storage = MQTTTopicStorage::new(self.rocksdb_engine_handler.clone());

        match storage.save(req.cluster_name, req.topic_name, req.content) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub fn delete_topic(&self, value: Vec<u8>) -> Result<(), RobustMQError> {
        let req = DeleteTopicRequest::decode(value.as_ref())
            .map_err(|e| Status::invalid_argument(e.to_string()))
            .unwrap();
        let storage = MQTTTopicStorage::new(self.rocksdb_engine_handler.clone());
        match storage.delete(req.cluster_name, req.topic_name) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub fn set_topic_retain_message(&self, value: Vec<u8>) -> Result<(), RobustMQError> {
        let req: SetTopicRetainMessageRequest =
            SetTopicRetainMessageRequest::decode(value.as_ref())
                .map_err(|e| Status::invalid_argument(e.to_string()))
                .unwrap();
        let storage = MQTTTopicStorage::new(self.rocksdb_engine_handler.clone());
        match storage.set_topic_retain_message(req.cluster_name, req.topic_name, req.retain_message)
        {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub fn create_session(&self, value: Vec<u8>) -> Result<(), RobustMQError> {
        let req = CreateSessionRequest::decode(value.as_ref())
            .map_err(|e| Status::invalid_argument(e.to_string()))
            .unwrap();
        let storage = MQTTSessionStorage::new(self.rocksdb_engine_handler.clone());

        match storage.save(req.cluster_name, req.client_id, req.session) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub fn delete_session(&self, value: Vec<u8>) -> Result<(), RobustMQError> {
        let req = DeleteSessionRequest::decode(value.as_ref())
            .map_err(|e| Status::invalid_argument(e.to_string()))
            .unwrap();
        let storage = MQTTSessionStorage::new(self.rocksdb_engine_handler.clone());
        match storage.delete(req.cluster_name, req.client_id) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
