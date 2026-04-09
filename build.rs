use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // 与 `flare-proto` 同级：`../flare-proto/proto`（基础类型，include 放前面以优先解析同名 import）
    let base_proto = manifest.join("..").join("flare-proto").join("proto");
    let grpc_proto = manifest.join("proto");

    let protos = [
        grpc_proto.join("access_gateway.proto"),
        grpc_proto.join("conversation_service.proto"),
        grpc_proto.join("hooks.proto"),
        grpc_proto.join("media_service.proto"),
        grpc_proto.join("message_service.proto"),
        grpc_proto.join("online.proto"),
        grpc_proto.join("push_service.proto"),
        grpc_proto.join("router.proto"),
        grpc_proto.join("storage_service.proto"),
        grpc_proto.join("sync_service.proto"),
    ];

    for p in &protos {
        println!("cargo:rerun-if-changed={}", p.display());
    }
    println!("cargo:rerun-if-changed={}", base_proto.display());

    // 基础 proto 目录优先，避免与 `*_service.proto` 中的 `import "message.proto"` 等同名引用歧义
    let includes = [
        base_proto.to_string_lossy().to_string(),
        grpc_proto.to_string_lossy().to_string(),
    ];

    let mut config = tonic_prost_build::configure();
    config = config
        .build_client(true)
        .build_server(true)
        .compile_well_known_types(false)
        .extern_path(".flare.common.v1", "::flare_proto::flare::common::v1")
        // 为 HTTP 网关直连 proto message 提供 serde 序列化能力
        .type_attribute(
            ".flare.media.v1.GenerateUploadUrlRequest",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            ".flare.media.v1.GenerateUploadUrlResponse",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            ".flare.media.v1.GetFileUrlRequest",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            ".flare.media.v1.GetFileInfoRequest",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            ".flare.media.v1.DeleteFileRequest",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            ".flare.media.v1.DeleteFileResponse",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            ".flare.media.v1.CreateReferenceRequest",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            ".flare.media.v1.DeleteReferenceRequest",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            ".flare.media.v1.CleanupOrphanedAssetsRequest",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            ".flare.media.v1.CleanupOrphanedAssetsResponse",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            ".flare.media.v1.SetObjectAclRequest",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            ".flare.media.v1.AccessControlEntry",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            ".flare.media.v1.DescribeBucketRequest",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            ".flare.media.v1.DescribeBucketResponse",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        );

    let proto_paths: Vec<_> = protos.iter().map(|p| p.to_string_lossy().to_string()).collect();

    config.compile_protos(&proto_paths, &includes)?;
    Ok(())
}
