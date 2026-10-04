//! Wire contract only; no claim of receiver/Platform API acceptance.
use flare_grpc_proto::control::{ApiKeyMode, CredentialKey, CredentialsProjectionUpsert};
use prost::Message;

#[test]
fn snapshot_roundtrip_preserves_binding_bytes_policy_and_version() {
    let snapshot = CredentialsProjectionUpsert {
        app_id: "app-a".into(),
        version: 42,
        keys: vec![CredentialKey {
            key_id: "ABCDEFGH2345".into(),
            service_account_id: "sa_fixture".into(),
            mode: ApiKeyMode::Test as i32,
            hash: vec![42; 32],
            pepper_fingerprint: vec![7; 32],
            scopes: vec!["platform.users.read".into()],
            expires_at_ms: 1_900_000_000_000,
            ip_allow: vec!["192.0.2.0/24".into()],
            revoked: false,
            service_account_enabled: true,
        }],
    };
    assert_eq!(
        CredentialsProjectionUpsert::decode(snapshot.encode_to_vec().as_slice()).unwrap(),
        snapshot
    );
}

#[test]
fn absent_fields_do_not_default_to_an_enabled_live_grant() {
    let key = CredentialKey::decode(&[][..]).unwrap();
    assert_eq!(key.mode(), ApiKeyMode::Unspecified);
    assert!(!key.service_account_enabled);
    assert!(key.hash.is_empty());
    assert!(key.pepper_fingerprint.is_empty());
    let clear = CredentialsProjectionUpsert {
        app_id: "app-a".into(),
        version: 43,
        keys: vec![],
    };
    assert_eq!(
        CredentialsProjectionUpsert::decode(clear.encode_to_vec().as_slice()).unwrap(),
        clear
    );
}
