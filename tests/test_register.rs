use prost::Message;
use zenth_dto::*;

fn round_trip<T: Message + Default>(msg: &T) -> T {
    T::decode(msg.encode_to_vec().as_slice()).unwrap()
}

#[test]
fn test_auth_challenge_round_trip() {
    let challenge = AuthChallenge {
        challenge_id: vec![0u8; 16],
        nonce: vec![0u8; 32],
        required_proof_type: ZkpType::Stark as i32,
        public_parameters: vec![1, 2, 3],
        timestamp: 1696880000000,
        difficulty: 128,
    };

    let decoded = round_trip(&challenge);
    assert_eq!(decoded.challenge_id, challenge.challenge_id);
    assert_eq!(decoded.nonce.len(), 32);
    assert_eq!(decoded.difficulty, 128);
    assert_eq!(decoded.timestamp, 1696880000000);
    assert_eq!(decoded.required_proof_type, ZkpType::Stark as i32);
}

#[test]
fn test_registration_request_round_trip() {
    let req = RegistrationRequest {
        username_hash: vec![0u8; 32],
        pre_key_bundle: vec![0u8; 64],
        proof_type: ZkpType::Plonk as i32,
        password_commitment: vec![0u8; 32],
        initial_proof: vec![0u8; 64],
        identity_key_dilithium: vec![0u8; 64],
        identity_signature: vec![0u8; 64],
        timestamp: 1696880000000,
    };

    let decoded = round_trip(&req);
    assert_eq!(decoded.username_hash.len(), 32);
    assert_eq!(decoded.timestamp, 1696880000000);
    assert_eq!(decoded.proof_type, ZkpType::Plonk as i32);
}

#[test]
fn test_identity_key_round_trip() {
    let key = IdentityKey {
        key_type: SignatureKeyType::Dilithium3 as i32,
        public_key: vec![0u8; 64],
    };

    let decoded = round_trip(&key);
    assert_eq!(decoded.key_type, SignatureKeyType::Dilithium3 as i32);
    assert_eq!(decoded.public_key.len(), 64);
}

#[test]
fn test_pre_key_bundle_round_trip() {
    let bundle = PreKeyBundle {
        user_hash_id: vec![0u8; 32],
        registration_id: 12345,
        identity_key: Some(IdentityKey {
            key_type: SignatureKeyType::Dilithium3 as i32,
            public_key: vec![0u8; 64],
        }),
        pre_key_id: 1,
        pre_key_public: vec![0u8; 32],
        signed_pre_key_id: 1,
        signed_pre_key_public: vec![0u8; 32],
        signed_pre_key_signature: vec![0u8; 64],
        pq_pre_key_id: 1,
        pq_pre_key_public: Some(KemPublicKey {
            key_type: 0,
            public_key: vec![0u8; 32],
        }),
        pq_last_resort_key_id: 0,
        pq_last_resort_key_public: None,
    };

    let decoded = round_trip(&bundle);
    assert_eq!(decoded.registration_id, 12345);
    assert_eq!(decoded.user_hash_id.len(), 32);
    assert!(decoded.identity_key.is_some());
    assert!(decoded.pq_pre_key_public.is_some());
}

#[test]
fn test_dht_request_round_trip() {
    let inner = RegistrationRequest {
        username_hash: vec![1u8; 32],
        pre_key_bundle: vec![2u8; 64],
        proof_type: ZkpType::Groth16 as i32,
        password_commitment: vec![3u8; 32],
        initial_proof: vec![4u8; 64],
        identity_key_dilithium: vec![5u8; 64],
        identity_signature: vec![6u8; 64],
        timestamp: 1696880000000,
    };

    let req = DhtRequest {
        method: Method::Register as i32,
        payload: inner.encode_to_vec(),
        timestamp: 1696880000000,
        request_id: vec![0u8; 16],
    };

    let decoded = round_trip(&req);
    assert_eq!(decoded.method, Method::Register as i32);
    assert_eq!(decoded.timestamp, 1696880000000);

    let inner_decoded = RegistrationRequest::decode(decoded.payload.as_slice()).unwrap();
    assert_eq!(inner_decoded.username_hash, vec![1u8; 32]);
    assert_eq!(inner_decoded.timestamp, 1696880000000);
}

#[test]
fn test_login_request_round_trip() {
    let req = LoginRequest {
        user_hash_id: vec![0u8; 32],
        request_challenge: true,
        proof: None,
        timestamp: 1696880000000,
        app_version: "1.0.0".to_string(),
    };

    let decoded = round_trip(&req);
    assert_eq!(decoded.user_hash_id.len(), 32);
    assert!(decoded.request_challenge);
    assert_eq!(decoded.app_version, "1.0.0");
}
