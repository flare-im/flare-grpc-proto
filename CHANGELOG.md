# 变更记录

本文件只记录**已发布到 crates.io 的版本**。版本号遵循
[语义化版本](https://semver.org/lang/zh-CN/)。

## 2.3.0 — 未发布

### 新增

- `control_plane.proto`(`flare.control.v1`):控制面 → 数据面的租户/功能开关投影协议
  `TenantProjection { UpsertTenant, UpsertFeatureFlags, GetVersions }`。真源在控制面
  (flare-im-console),Social 与核各实现一份接收端,按 `version` 幂等。整包派生 serde。
- `OnlineService.KickTenant`:按租户踢出全部连接(租户停用时由控制面调用)。

## 2.1.0 — 未发布

### 新增

- `UpdateConversationUserSettingsRequest` 增加 `is_mention_only`（字段号 7）。
  语义是「只接收提到我的消息」，与 `is_muted` 正交——两者可以同时开启。

  这是**向后兼容的新增**：字段是 `optional`，字段号此前未被占用（旧版止于 6），
  老客户端不发送它、老服务端忽略它，两侧都不受影响。因此升 minor 而非 major。

## 2.0.1 — 2026-08-03

与实现层 1.1.0 对齐的契约层发布。
