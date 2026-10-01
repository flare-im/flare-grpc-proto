# 变更记录

本文件只记录**已发布到 crates.io 的版本**。版本号遵循
[语义化版本](https://semver.org/lang/zh-CN/)。

## 2.3.0 — 未发布

### 新增

- `control_plane.proto`(`flare.control.v1`):控制面 → 数据面的租户/功能开关投影协议
  `TenantProjection { UpsertTenant, UpsertFeatureFlags, GetVersions }`。真源在控制面
  (flare-im-console),Social 与核各实现一份接收端,按 `version` 幂等。整包派生 serde。
- `OnlineService.KickTenant`:按租户踢出全部连接(租户停用时由控制面调用)。
- `ConversationManageService.PurgeConversationHistory`:物理删除会话里服务端时间早于
  `before_ms` 的消息、事件、置顶与标记,会话、成员与边界之后的消息保留,重复调用幂等。
  用于「重新加好友时只丢弃旧关系的历史」——整会话硬删会连同接受之后已经发出的消息一起删掉。
  新增 RPC 与消息,不改动已有字段;老服务端对它返回 UNIMPLEMENTED,调用方据此重试而不是
  退回整会话硬删。
  `PurgeConversationHistoryRequest` 另有 `epoch_attribute`(3)与 `before_epoch`(4):
  按消息 attributes 里的纪元清除,带纪元的消息只看纪元、不看时间,没有纪元的消息回落到
  `before_ms`;纪元围栏下 `before_ms` 可以为 0。老服务端忽略这两个字段,按时间边界执行。
- 三个管理员只读服务,供 admin-gateway 以原样转发的 admin 令牌调用,租户只取自令牌:
  - `ConversationAdminService { AdminListConversations, AdminGetConversation, AdminListParticipants }`:
    按租户分页列会话(游标编码 `updated_at` + `conversation_id`,不接收 `user_id`)、
    读会话详情、分页读成员;
  - `OnlineAdminService { AdminListUserDevices, AdminGetUserPresence }`;
  - `CapabilityAdminService { ListTenantCapabilitySwitches, GetTenantRuntime }`:
    `capability_service.proto` 为此引入 `control_plane.proto`,复用 `TenantStatus` 与 `TenantQuota`。

  都是新服务,没有往已有服务里加方法:已有的 `OnlineService`、`CapabilityService`、
  会话读写服务实现不需要改动就能继续编译。

## 2.1.0 — 未发布

### 新增

- `UpdateConversationUserSettingsRequest` 增加 `is_mention_only`（字段号 7）。
  语义是「只接收提到我的消息」，与 `is_muted` 正交——两者可以同时开启。

  这是**向后兼容的新增**：字段是 `optional`，字段号此前未被占用（旧版止于 6），
  老客户端不发送它、老服务端忽略它，两侧都不受影响。因此升 minor 而非 major。

## 2.0.1 — 2026-08-03

与实现层 1.1.0 对齐的契约层发布。
