//! 管理员只读服务的契约：服务全名与字段号一旦发布就不能改。
//!
//! 三个服务都是新增的独立服务；已有服务的方法集合不变，由各实现方继续编译来保证。

use flare_grpc_proto::capability::{
    GetTenantRuntimeResponse, ListTenantCapabilitySwitchesResponse, TenantCapabilitySwitch,
    capability_admin_service_server,
};
use flare_grpc_proto::control::TenantStatus;
use flare_grpc_proto::conversation::{
    AdminGetConversationRequest, AdminListConversationsRequest, AdminListParticipantsRequest,
    AdminListParticipantsResponse, conversation_admin_service_server,
};
use flare_grpc_proto::signaling::online::{
    AdminGetUserPresenceRequest, AdminListUserDevicesRequest, online_admin_service_server,
};
use prost::Message as _;

#[test]
fn admin_service_names_are_stable() {
    assert_eq!(
        conversation_admin_service_server::SERVICE_NAME,
        "flare.conversation.v1.ConversationAdminService"
    );
    assert_eq!(
        online_admin_service_server::SERVICE_NAME,
        "flare.signaling.online.v1.OnlineAdminService"
    );
    assert_eq!(
        capability_admin_service_server::SERVICE_NAME,
        "flare.capability.v1.CapabilityAdminService"
    );
}

#[test]
fn admin_requests_carry_no_tenant_or_caller_identity() {
    // 字段 1 cursor="c"、2 limit=5、3 include_deleted=true；没有 tenant_id / user_id 字段。
    let list = AdminListConversationsRequest {
        cursor: "c".into(),
        limit: 5,
        include_deleted: true,
    };
    assert_eq!(
        list.encode_to_vec(),
        vec![0x0a, 0x01, b'c', 0x10, 0x05, 0x18, 0x01]
    );
    assert_eq!(
        AdminGetConversationRequest {
            conversation_id: "g".into()
        }
        .encode_to_vec(),
        vec![0x0a, 0x01, b'g']
    );
    let participants = AdminListParticipantsRequest {
        conversation_id: "g".into(),
        cursor: "x".into(),
        limit: 2,
        include_removed: true,
    };
    assert_eq!(
        participants.encode_to_vec(),
        vec![0x0a, 0x01, b'g', 0x12, 0x01, b'x', 0x18, 0x02, 0x20, 0x01]
    );
    for encoded in [
        AdminListUserDevicesRequest {
            user_id: "u".into(),
        }
        .encode_to_vec(),
        AdminGetUserPresenceRequest {
            user_id: "u".into(),
        }
        .encode_to_vec(),
    ] {
        assert_eq!(encoded, vec![0x0a, 0x01, b'u']);
    }
}

#[test]
fn admin_responses_keep_field_numbers() {
    let page = AdminListParticipantsResponse {
        conversation_id: "g".into(),
        participants: Vec::new(),
        next_cursor: "n".into(),
        has_more: true,
        member_count: 3,
    };
    assert_eq!(
        page.encode_to_vec(),
        vec![0x0a, 0x01, b'g', 0x1a, 0x01, b'n', 0x20, 0x01, 0x28, 0x03]
    );

    let switches = ListTenantCapabilitySwitchesResponse {
        tenant_id: "t".into(),
        switches: vec![TenantCapabilitySwitch {
            capability_id: "c".into(),
            enabled: true,
            updated_at: None,
        }],
    };
    assert_eq!(
        switches.encode_to_vec(),
        vec![0x0a, 0x01, b't', 0x12, 0x05, 0x0a, 0x01, b'c', 0x10, 0x01]
    );

    let runtime = GetTenantRuntimeResponse {
        tenant_id: "t".into(),
        projected: true,
        status: TenantStatus::Suspended as i32,
        version: 9,
        ..Default::default()
    };
    assert_eq!(
        runtime.encode_to_vec(),
        vec![0x0a, 0x01, b't', 0x10, 0x01, 0x20, 0x02, 0x38, 0x09]
    );
    // 未投影：空响应解码为零值，projected=false。
    assert!(!GetTenantRuntimeResponse::decode(&[][..]).unwrap().projected);
}
