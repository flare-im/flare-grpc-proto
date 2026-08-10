//! Flare IM **gRPC 服务** proto 生成层（tonic client/server + 各服务消息类型）。
//!
//! 依赖 [`flare_proto`](flare_proto) 中的 `flare.common.v1` 等基础类型；业务 crate 按需依赖本 crate。

// 本 crate 内容全部由 prost/tonic 从 .proto 生成，代码风格不由我们控制。
// large_enum_variant 针对 oneof 生成枚举的变体大小差异 —— 这是 protobuf 固有形态，
// 真要收敛得改 .proto 的消息设计，而不是在生成代码里加 Box。
#![allow(clippy::large_enum_variant)]

pub mod flare {
    pub mod access_gateway {
        pub mod v1 {
            tonic::include_proto!("flare.access_gateway.v1");
        }
    }

    pub mod signaling {
        pub mod online {
            pub mod v1 {
                tonic::include_proto!("flare.signaling.online.v1");
            }
        }
        pub mod router {
            pub mod v1 {
                tonic::include_proto!("flare.signaling.router.v1");
            }
        }
    }

    pub mod push {
        pub mod v1 {
            tonic::include_proto!("flare.push.v1");
        }
    }

    pub mod storage {
        pub mod v1 {
            tonic::include_proto!("flare.storage.v1");
        }
    }

    pub mod media {
        pub mod v1 {
            tonic::include_proto!("flare.media.v1");
        }
    }

    pub mod capability {
        pub mod v1 {
            tonic::include_proto!("flare.capability.v1");
        }
    }

    pub mod conversation {
        pub mod v1 {
            tonic::include_proto!("flare.conversation.v1");
        }
    }

    pub mod message {
        pub mod v1 {
            tonic::include_proto!("flare.message.v1");
        }
    }

    pub mod sync {
        pub mod v1 {
            tonic::include_proto!("flare.sync.v1");
        }
    }

    #[cfg(feature = "sfu_control")]
    pub mod sfu {
        pub mod control {
            pub mod v1 {
                tonic::include_proto!("flare.sfu.control.v1");
            }
        }
    }
}

pub mod signaling {
    pub mod online {
        pub use crate::flare::signaling::online::v1::*;
    }
    pub mod router {
        pub use crate::flare::signaling::router::v1::*;
    }
    pub use crate::flare::signaling::online::v1::*;
}

pub mod push {
    pub use crate::flare::push::v1::*;
}

pub mod storage {
    pub use crate::flare::storage::v1::*;
}

pub mod media {
    pub use crate::flare::media::v1::*;
}

pub mod capability {
    pub use crate::flare::capability::v1::*;
}

pub mod conversation {
    pub use crate::flare::conversation::v1::*;
}

pub mod message {
    pub use crate::flare::message::v1::*;
}

pub mod access_gateway {
    pub use crate::flare::access_gateway::v1::*;
}

pub mod sync {
    pub use crate::flare::sync::v1::*;
}

/// SFU 插件控制面（`sfu_control.proto` → `flare.sfu.control.v1`）。需启用 crate feature `sfu_control`。
#[cfg(feature = "sfu_control")]
pub mod sfu_control {
    pub use crate::flare::sfu::control::v1::*;
}

pub use flare_proto::PushTaskPayloadKind;
pub use flare_proto::common::{
    AppCardAction, AppCardContent, AudioContent, AudioInfo, AuditContext, CapabilityPacket,
    CardContent, ConflictResolution, ContentVisibility, ConversationDetailSync,
    ConversationDetailSyncRes, ConversationSummary as ConversationSummaryProto,
    ConversationSyncSlice, ConversationUserSettingsSync, ConversationUserSettingsSyncRes,
    ConversationsSync, ConversationsSyncRes, CustomContent, DataPacket, DeleteType,
    DeviceState as ConversationDeviceState, FileContent, ForwardContent, GetSyncCursorSync,
    GetSyncCursorSyncRes, ImageContent, ImageInfo, LocationContent, MarkType, MediaAttachment,
    Mention, Message, MessageContent, MessageReadRecord, MessageRetentionExpiredEvent,
    MessageRetentionLifecycle, MessageRetentionPolicy, MessageRetentionPurgedEvent,
    MessageRetentionScheduledEvent, MessageRetentionState, MessageSource, MessageStatus,
    MessageTimeline, MessageType, MqEnvelope, MqPayloadKind, MultiConversationSync,
    MultiConversationSyncRes, MultiDeviceCursor, NotificationContent, OfflinePushInfo, Pagination,
    PresenceHintPacket, QueryEventsSync, QueryEventsSyncRes, ReactionAction, RealtimeControlPacket,
    RetentionMode, RetentionTrigger, SingleConversationSync, SingleConversationSyncRes, Sync,
    SyncRes, SyncSliceItem, TextContent, TypingStatePacket, UpdateSyncCursorSync,
    UpdateSyncCursorSyncRes, VideoContent, VideoInfo,
};

pub use storage::VisibilityStatus;

pub use signaling::online::{
    DeviceInfo, GetOnlineStatusRequest as SignalingGetOnlineStatusRequest,
    GetOnlineStatusResponse as SignalingGetOnlineStatusResponse, HeartbeatRequest,
    HeartbeatResponse, LoginRequest as SignalingLoginRequest,
    LoginResponse as SignalingLoginResponse, LogoutRequest, LogoutResponse, UserPresence,
};

pub use signaling::router::{
    PushStrategy, RouteTarget, RouterDownstreamPushAckRequest, RouterDownstreamPushCustomRequest,
    RouterDownstreamPushEventRequest, RouterDownstreamPushMessageRequest,
    RouterDownstreamPushNotificationRequest,
};

pub use push::{
    DevicePushProvider, Notification as PushNotificationContent,
    PushCustomRequest as PushPushCustomRequest, PushCustomResponse as PushPushCustomResponse,
    PushFailure, PushMessageRequest as PushPushMessageRequest,
    PushMessageResponse as PushPushMessageResponse,
    PushNotificationRequest as PushPushNotificationRequest,
    PushNotificationResponse as PushPushNotificationResponse, PushOptions, QueryPushStatusRequest,
    QueryPushStatusResponse, RegisterDevicePushTokenRequest, RegisterDevicePushTokenResponse,
    UnregisterDevicePushTokenRequest, UnregisterDevicePushTokenResponse,
};

pub use storage::{
    ExportMessagesRequest, ExportMessagesResponse, FailedMessage,
    GetMessageRequest as StorageGetMessageRequest, GetMessageResponse as StorageGetMessageResponse,
    QueryMessagesRequest as StorageQueryMessagesRequest,
    QueryMessagesResponse as StorageQueryMessagesResponse, SearchMessagesRequest,
    SearchMessagesResponse,
};

pub use media::{
    AbortMultipartUploadRequest, AbortMultipartUploadResponse, AccessControlEntry,
    CleanupOrphanedAssetsRequest, CleanupOrphanedAssetsResponse, CompleteMultipartUploadRequest,
    CompressOperation, CompressVideoOperation, CreateReferenceRequest, CreateReferenceResponse,
    DeleteFileRequest as MediaDeleteFileRequest, DeleteFileResponse as MediaDeleteFileResponse,
    DeleteReferenceRequest, DeleteReferenceResponse, DescribeBucketRequest, DescribeBucketResponse,
    FileInfo, GenerateUploadUrlRequest, GenerateUploadUrlResponse, GetFileInfoRequest,
    GetFileInfoResponse, GetFileUrlRequest, GetFileUrlResponse, ImageOperation,
    InitiateMultipartUploadRequest, InitiateMultipartUploadResponse, ListObjectsRequest,
    ListObjectsResponse, ListReferencesRequest, ListReferencesResponse, MediaReferenceInfo,
    ProcessImageRequest, ProcessImageResponse, ProcessVideoRequest, ProcessVideoResponse,
    ResizeOperation, SetObjectAclRequest, SubtitleBurnOperation, ThumbnailOperation,
    UploadFileMetadata, UploadFileRequest, UploadFileResponse, UploadMultipartChunkRequest,
    UploadMultipartChunkResponse, VideoOperation, WatermarkOperation,
};

pub use capability::{
    ConversationLifecycleHookRequest, ConversationLifecycleHookResponse, CustomHookRequest,
    CustomHookResponse, DeliveryHookRequest as ProtoDeliveryHookRequest,
    DeliveryHookResponse as ProtoDeliveryHookResponse, GenericRequest, GenericResponse,
    HookDeliveryEvent as ProtoHookDeliveryEvent,
    HookInvocationContext as ProtoHookInvocationContext, HookMessageDraft as ProtoHookMessageDraft,
    HookMessageRecord as ProtoHookMessageRecord, HookRecallEvent as ProtoHookRecallEvent,
    PostSendHookRequest as ProtoPostSendHookRequest,
    PostSendHookResponse as ProtoPostSendHookResponse,
    PreSendHookRequest as ProtoPreSendHookRequest, PreSendHookResponse as ProtoPreSendHookResponse,
    PresenceHookRequest, PresenceHookResponse, RecallHookRequest as ProtoRecallHookRequest,
    RecallHookResponse as ProtoRecallHookResponse,
};

#[cfg(not(target_arch = "wasm32"))]
pub use conversation::{
    ConversationBootstrapRequest, ConversationBootstrapResponse, ConversationPolicy,
    DevicePresence as ConversationDevicePresence, ForceConversationSyncRequest,
    ListConversationsRequest as ConversationListConversationsRequest,
    ListConversationsResponse as ConversationListConversationsResponse,
    SortOrder as ConversationSortOrder, SyncMessagesRequest as ConversationSyncMessagesRequest,
    SyncMessagesResponse as ConversationSyncMessagesResponse, UpdateCursorRequest,
    UpdatePresenceRequest,
};

/// 「确保会话存在」的建群请求约定（唯一实现）：
/// - `conversation_id` 经 `attributes["conversation_id"]` 携带——服务端按此 id 建群（幂等），
///   与客户端 `hash(group_key)` 推导的会话 id 对齐；改动此约定必须三方（ingest 管线 /
///   sync 编排 / conversation 消费端）同步，故收敛为单一构造函数。
/// - 成员默认无角色/未静音/未置顶；可见性恒 Private。
#[cfg(not(target_arch = "wasm32"))]
pub fn ensure_conversation_request(
    conversation_id: &str,
    conversation_type: i32,
    business_type: String,
    member_ids: impl IntoIterator<Item = String>,
    channel_id: String,
) -> conversation::CreateConversationRequest {
    let participants = member_ids
        .into_iter()
        .map(|user_id| flare_proto::common::ConversationParticipant {
            user_id,
            roles: vec![],
            muted: false,
            pinned: false,
            attributes: std::collections::HashMap::new(),
            joined_at: 0,
            // 建群时的初始成员无历史下限：他们本来就在场，没有「入群前」可言。
            visible_from_seq: 0,
        })
        .collect();
    let mut attributes = std::collections::HashMap::new();
    attributes.insert("conversation_id".to_string(), conversation_id.to_string());
    conversation::CreateConversationRequest {
        conversation_type,
        business_type,
        participants,
        attributes,
        visibility: 0, // SessionVisibility::Private
        channel_id,
    }
}
