// Copyright 2023 RobustMQ Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_base::error::common::CommonError;
use mobc::Connection;
use prost::Message;
use protocol::placement_center::generate::{
    common::CommonReply,
    mqtt::{
        mqtt_service_client::MqttServiceClient, CreateAclRequest, CreateBlacklistRequest, CreateSessionRequest, CreateTopicRequest, CreateUserRequest, DeleteAclRequest, DeleteBlacklistRequest, DeleteSessionRequest, DeleteTopicRequest, DeleteUserRequest, Empty, GetShareSubLeaderReply, GetShareSubLeaderRequest, ListAclReply, ListAclRequest, ListBlacklistReply, ListBlacklistRequest, ListSessionReply, ListSessionRequest, ListTopicReply, ListTopicRequest, ListUserReply, ListUserRequest, SaveLastWillMessageRequest, SetTopicRetainMessageRequest, UpdateSessionRequest
    },
};
use tonic::transport::Channel;

use super::MQTTServiceManager;

pub(crate) async fn inner_get_share_sub_leader(
    mut client: Connection<MQTTServiceManager>,
    request: Vec<u8>,
) -> Result<Vec<u8>, CommonError> {
    match GetShareSubLeaderRequest::decode(request.as_ref()) {
        Ok(request) => match client.get_share_sub_leader(request).await {
            Ok(result) => {
                return Ok(GetShareSubLeaderReply::encode_to_vec(&result.into_inner()));
            }
            Err(e) => return Err(CommonError::GrpcServerStatus(e)),
        },
        Err(e) => {
            return Err(CommonError::CommmonError(e.to_string()));
        }
    }
}

pub(crate) async fn inner_create_user(
    mut client: Connection<MQTTServiceManager>,
    request: Vec<u8>,
) -> Result<Vec<u8>, CommonError> {
    match CreateUserRequest::decode(request.as_ref()) {
        Ok(request) => match client.create_user(request).await {
            Ok(result) => {
                return Ok(CommonReply::encode_to_vec(&result.into_inner()));
            }
            Err(e) => return Err(CommonError::GrpcServerStatus(e)),
        },
        Err(e) => {
            return Err(CommonError::CommmonError(e.to_string()));
        }
    }
}

pub(crate) async fn inner_list_user(
    mut client: Connection<MQTTServiceManager>,
    request: Vec<u8>,
) -> Result<Vec<u8>, CommonError> {
    match ListUserRequest::decode(request.as_ref()) {
        Ok(request) => match client.list_user(request).await {
            Ok(result) => {
                return Ok(ListUserReply::encode_to_vec(&result.into_inner()));
            }
            Err(e) => return Err(CommonError::GrpcServerStatus(e)),
        },
        Err(e) => {
            return Err(CommonError::CommmonError(e.to_string()));
        }
    }
}

pub(crate) async fn inner_delete_user(
    mut client: Connection<MQTTServiceManager>,
    request: Vec<u8>,
) -> Result<Vec<u8>, CommonError> {
    match DeleteUserRequest::decode(request.as_ref()) {
        Ok(request) => match client.delete_user(request).await {
            Ok(result) => {
                return Ok(CommonReply::encode_to_vec(&result.into_inner()));
            }
            Err(e) => return Err(CommonError::GrpcServerStatus(e)),
        },
        Err(e) => {
            return Err(CommonError::CommmonError(e.to_string()));
        }
    }
}

pub(crate) async fn inner_create_topic(
    mut client: Connection<MQTTServiceManager>,
    request: Vec<u8>,
) -> Result<Vec<u8>, CommonError> {
    match CreateTopicRequest::decode(request.as_ref()) {
        Ok(request) => match client.create_topic(request).await {
            Ok(result) => {
                return Ok(CommonReply::encode_to_vec(&result.into_inner()));
            }
            Err(e) => return Err(CommonError::GrpcServerStatus(e)),
        },
        Err(e) => {
            return Err(CommonError::CommmonError(e.to_string()));
        }
    }
}

pub(crate) async fn inner_list_topic(
    mut client: Connection<MQTTServiceManager>,
    request: Vec<u8>,
) -> Result<Vec<u8>, CommonError> {
    match ListTopicRequest::decode(request.as_ref()) {
        Ok(request) => match client.list_topic(request).await {
            Ok(result) => {
                return Ok(ListTopicReply::encode_to_vec(&result.into_inner()));
            }
            Err(e) => return Err(CommonError::GrpcServerStatus(e)),
        },
        Err(e) => {
            return Err(CommonError::CommmonError(e.to_string()));
        }
    }
}

pub(crate) async fn inner_delete_topic(
    mut client: Connection<MQTTServiceManager>,
    request: Vec<u8>,
) -> Result<Vec<u8>, CommonError> {
    match DeleteTopicRequest::decode(request.as_ref()) {
        Ok(request) => match client.delete_topic(request).await {
            Ok(result) => {
                return Ok(CommonReply::encode_to_vec(&result.into_inner()));
            }
            Err(e) => return Err(CommonError::GrpcServerStatus(e)),
        },
        Err(e) => {
            return Err(CommonError::CommmonError(e.to_string()));
        }
    }
}

pub(crate) async fn inner_set_topic_retain_message(
    mut client: Connection<MQTTServiceManager>,
    request: Vec<u8>,
) -> Result<Vec<u8>, CommonError> {
    match SetTopicRetainMessageRequest::decode(request.as_ref()) {
        Ok(request) => match client.set_topic_retain_message(request).await {
            Ok(result) => {
                return Ok(CommonReply::encode_to_vec(&result.into_inner()));
            }
            Err(e) => return Err(CommonError::GrpcServerStatus(e)),
        },
        Err(e) => {
            return Err(CommonError::CommmonError(e.to_string()));
        }
    }
}

pub(crate) async fn inner_create_session(
    mut client: Connection<MQTTServiceManager>,
    request: Vec<u8>,
) -> Result<Vec<u8>, CommonError> {
    match CreateSessionRequest::decode(request.as_ref()) {
        Ok(request) => match client.create_session(request).await {
            Ok(result) => {
                return Ok(CommonReply::encode_to_vec(&result.into_inner()));
            }
            Err(e) => return Err(CommonError::GrpcServerStatus(e)),
        },
        Err(e) => {
            return Err(CommonError::CommmonError(e.to_string()));
        }
    }
}

pub(crate) async fn inner_list_session(
    mut client: Connection<MQTTServiceManager>,
    request: Vec<u8>,
) -> Result<Vec<u8>, CommonError> {
    match ListSessionRequest::decode(request.as_ref()) {
        Ok(request) => match client.list_session(request).await {
            Ok(result) => {
                return Ok(ListSessionReply::encode_to_vec(&result.into_inner()));
            }
            Err(e) => return Err(CommonError::GrpcServerStatus(e)),
        },
        Err(e) => {
            return Err(CommonError::CommmonError(e.to_string()));
        }
    }
}

pub(crate) async fn inner_delete_session(
    mut client: Connection<MQTTServiceManager>,
    request: Vec<u8>,
) -> Result<Vec<u8>, CommonError> {
    match DeleteSessionRequest::decode(request.as_ref()) {
        Ok(request) => match client.delete_session(request).await {
            Ok(result) => {
                return Ok(CommonReply::encode_to_vec(&result.into_inner()));
            }
            Err(e) => return Err(CommonError::GrpcServerStatus(e)),
        },
        Err(e) => {
            return Err(CommonError::CommmonError(e.to_string()));
        }
    }
}

pub(crate) async fn inner_update_session(
    mut client: Connection<MQTTServiceManager>,
    request: Vec<u8>,
) -> Result<Vec<u8>, CommonError> {
    match UpdateSessionRequest::decode(request.as_ref()) {
        Ok(request) => match client.update_session(request).await {
            Ok(result) => {
                return Ok(CommonReply::encode_to_vec(&result.into_inner()));
            }
            Err(e) => return Err(CommonError::GrpcServerStatus(e)),
        },
        Err(e) => {
            return Err(CommonError::CommmonError(e.to_string()));
        }
    }
}

pub(crate) async fn inner_save_last_will_message(
    mut client: Connection<MQTTServiceManager>,
    request: Vec<u8>,
) -> Result<Vec<u8>, CommonError> {
    match SaveLastWillMessageRequest::decode(request.as_ref()) {
        Ok(request) => match client.save_last_will_message(request).await {
            Ok(result) => {
                return Ok(CommonReply::encode_to_vec(&result.into_inner()));
            }
            Err(e) => return Err(CommonError::GrpcServerStatus(e)),
        },
        Err(e) => {
            return Err(CommonError::CommmonError(e.to_string()));
        }
    }
}

pub(crate) async fn inner_list_acl(
    mut client: Connection<MQTTServiceManager>,
    request: Vec<u8>,
) -> Result<Vec<u8>, CommonError> {
    match ListAclRequest::decode(request.as_ref()) {
        Ok(request) => match client.list_acl(request).await {
            Ok(result) => {
                return Ok(ListAclReply::encode_to_vec(&result.into_inner()));
            }
            Err(e) => return Err(CommonError::GrpcServerStatus(e)),
        },
        Err(e) => {
            return Err(CommonError::CommmonError(e.to_string()));
        }
    }
}

pub(crate) async fn inner_create_acl(
    mut client: Connection<MQTTServiceManager>,
    request: Vec<u8>,
) -> Result<Vec<u8>, CommonError> {
    match CreateAclRequest::decode(request.as_ref()) {
        Ok(request) => match client.create_acl(request).await {
            Ok(result) => {
                return Ok(CommonReply::encode_to_vec(&result.into_inner()));
            }
            Err(e) => return Err(CommonError::GrpcServerStatus(e)),
        },
        Err(e) => {
            return Err(CommonError::CommmonError(e.to_string()));
        }
    }
}

pub(crate) async fn inner_delete_acl(
    mut client: Connection<MQTTServiceManager>,
    request: Vec<u8>,
) -> Result<Vec<u8>, CommonError> {
    match DeleteAclRequest::decode(request.as_ref()) {
        Ok(request) => match client.delete_acl(request).await {
            Ok(result) => {
                return Ok(CommonReply::encode_to_vec(&result.into_inner()));
            }
            Err(e) => return Err(CommonError::GrpcServerStatus(e)),
        },
        Err(e) => {
            return Err(CommonError::CommmonError(e.to_string()));
        }
    }
}

pub(crate) async fn inner_list_blacklist(
    mut client: Connection<MQTTServiceManager>,
    request: Vec<u8>,
) -> Result<Vec<u8>, CommonError> {
    match ListBlacklistRequest::decode(request.as_ref()) {
        Ok(request) => match client.list_blacklist(request).await {
            Ok(result) => {
                return Ok(ListBlacklistReply::encode_to_vec(&result.into_inner()));
            }
            Err(e) => return Err(CommonError::GrpcServerStatus(e)),
        },
        Err(e) => {
            return Err(CommonError::CommmonError(e.to_string()));
        }
    }
}

pub(crate) async fn inner_create_blacklist(
    mut client: Connection<MQTTServiceManager>,
    request: Vec<u8>,
) -> Result<Vec<u8>, CommonError> {
    match CreateBlacklistRequest::decode(request.as_ref()) {
        Ok(request) => match client.create_blacklist(request).await {
            Ok(result) => {
                return Ok(CommonReply::encode_to_vec(&result.into_inner()));
            }
            Err(e) => return Err(CommonError::GrpcServerStatus(e)),
        },
        Err(e) => {
            return Err(CommonError::CommmonError(e.to_string()));
        }
    }
}

pub(crate) async fn inner_delete_blacklist(
    mut client: Connection<MQTTServiceManager>,
    request: Vec<u8>,
) -> Result<Vec<u8>, CommonError> {
    match DeleteBlacklistRequest::decode(request.as_ref()) {
        Ok(request) => match client.delete_blacklist(request).await {
            Ok(result) => {
                return Ok(CommonReply::encode_to_vec(&result.into_inner()));
            }
            Err(e) => return Err(CommonError::GrpcServerStatus(e)),
        },
        Err(e) => {
            return Err(CommonError::CommmonError(e.to_string()));
        }
    }
}

pub(crate) async fn inner_is_placement_leader(
    mut client: MqttServiceClient<Channel>,
) -> Result<bool, CommonError> {
    match client.is_placement_center_leader(Empty {}).await {
        Ok(result) => {
            Ok(result.into_inner().is_leader)
        },
        Err(e) => {
            Err(CommonError::GrpcServerStatus(e))
        },
    }
}

pub(crate) async fn inner_get_placement_center_leader_address(
    mut client: MqttServiceClient<Channel>,
) -> Result<Option<String>, CommonError> {
    match client.get_placement_center_leader_address(Empty {}).await {
        Ok(result) => {
            let address = result.into_inner().address;
            if address.is_empty() {
                Ok(None)
            } else {
                Ok(Some(address))
            }
        },
        Err(e) => {
            Err(CommonError::GrpcServerStatus(e))
        },
    }
}
