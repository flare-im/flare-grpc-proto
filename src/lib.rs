//! Flare IM **gRPC 服务** proto 生成层（tonic client/server + 各服务消息类型）。
//!
//! 依赖 [`flare_proto`](flare_proto) 中的 `flare.common.v1` 等基础类型；业务 crate 按需依赖本 crate。

pub mod flare {
    pub mod access_gateway {
        pub mod v1 {
            tonic::include_proto!("flare.access_gateway.v1");
        }
    }

    pub mod signaling {
        pub mod online {
            tonic::include_proto!("flare.signaling.online");
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

    pub mod hooks {
        pub mod v1 {
            tonic::include_proto!("flare.hooks.v1");
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
}

pub mod signaling {
    pub mod online {
        pub use crate::flare::signaling::online::*;
    }
    pub mod router {
        pub use crate::flare::signaling::router::v1::*;
    }
    pub use crate::flare::signaling::online::*;
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

pub mod hooks {
    pub use crate::flare::hooks::v1::*;
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

pub use flare_proto::common::{
    AuditContext, MediaAttachment, Pagination, PushTaskPayloadKind, ConflictResolution,
    ConversationSummary as ConversationSummaryProto, DeleteType, MarkType, ReactionAction,
    DeviceState as ConversationDeviceState, Message, MessageContent, MessageType, MessageStatus,
    MessageSource, MessageTimeline, MessageReadRecord, TextContent, ImageContent, VideoContent,
    AudioContent, FileContent, LocationContent, CardContent, NotificationContent, CustomContent,
    ForwardContent, Mention, ImageInfo, VideoInfo, AudioInfo, OfflinePushInfo, MqEnvelope,
    MqPayloadKind, ConnectionQuality, SyncKind, Sync, SyncRes, SingleConversationSync,
    MultiConversationSync, ConversationsIncrementalSync, ConversationsAllSync, ConversationDetailSync,
    QueryEventsSync, GetSyncCursorSync, UpdateSyncCursorSync, SingleConversationSyncRes,
    MultiConversationSyncRes, ConversationsIncrementalSyncRes, ConversationsAllSyncRes,
    ConversationDetailSyncRes, QueryEventsSyncRes, GetSyncCursorSyncRes, UpdateSyncCursorSyncRes,
    SyncSliceItem, ConversationSyncSlice, ConversationPatchType, ConversationPatch,
    ConversationSyncAllOptions, MultiDeviceCursor,
};

pub use storage::VisibilityStatus;

pub use signaling::online::{
    GetOnlineStatusRequest as SignalingGetOnlineStatusRequest,
    GetOnlineStatusResponse as SignalingGetOnlineStatusResponse, HeartbeatRequest, HeartbeatResponse,
    LoginRequest as SignalingLoginRequest, LoginResponse as SignalingLoginResponse, LogoutRequest,
    LogoutResponse, DeviceInfo, UserPresence,
};

pub use signaling::router::{
    PushStrategy, RouteTarget, RouterDownstreamPushAckRequest, RouterDownstreamPushCustomRequest,
    RouterDownstreamPushEventRequest, RouterDownstreamPushMessageRequest,
    RouterDownstreamPushNotificationRequest,
};

pub use push::{
    Notification as PushNotificationContent, PushCustomRequest as PushPushCustomRequest,
    PushCustomResponse as PushPushCustomResponse, PushFailure,
    PushMessageRequest as PushPushMessageRequest, PushMessageResponse as PushPushMessageResponse,
    PushNotificationRequest as PushPushNotificationRequest,
    PushNotificationResponse as PushPushNotificationResponse, PushOptions, QueryPushStatusRequest,
    QueryPushStatusResponse,
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

pub use hooks::{
    CustomHookRequest, CustomHookResponse, DeliveryHookRequest as ProtoDeliveryHookRequest,
    DeliveryHookResponse as ProtoDeliveryHookResponse, HookDeliveryEvent as ProtoHookDeliveryEvent,
    HookInvocationContext as ProtoHookInvocationContext, HookMessageDraft as ProtoHookMessageDraft,
    HookMessageRecord as ProtoHookMessageRecord, PostSendHookRequest as ProtoPostSendHookRequest,
    PostSendHookResponse as ProtoPostSendHookResponse, PreSendHookRequest as ProtoPreSendHookRequest,
    PreSendHookResponse as ProtoPreSendHookResponse, RecallHookRequest as ProtoRecallHookRequest,
    RecallHookResponse as ProtoRecallHookResponse, HookRecallEvent as ProtoHookRecallEvent,
    PresenceHookRequest, PresenceHookResponse, ConversationLifecycleHookRequest,
    ConversationLifecycleHookResponse,
};

#[cfg(not(target_arch = "wasm32"))]
pub use conversation::{
    DevicePresence as ConversationDevicePresence,
    ListConversationsRequest as ConversationListConversationsRequest,
    ListConversationsResponse as ConversationListConversationsResponse, ConversationBootstrapRequest,
    ConversationBootstrapResponse, SortOrder as ConversationSortOrder, ForceConversationSyncRequest,
    ConversationPolicy, SyncMessagesRequest as ConversationSyncMessagesRequest,
    SyncMessagesResponse as ConversationSyncMessagesResponse, UpdateCursorRequest,
    UpdatePresenceRequest,
};
