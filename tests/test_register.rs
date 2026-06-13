/*

pub mod generated {
    tonic::include_proto!("register");
}

#[cfg(test)]
mod tests {
    use super::generated::*;
    use prost::Message;

    fn round_trip<T: Message + Default>(
        msg: &T
    ) -> T {
        let mut buf = Vec::new();
        msg.encode(
            &mut buf
        ).unwrap();
        T::decode(
            &*buf
        ).unwrap()
    }

    #[test]
    fn test_inner_register_payload() {
        let inner = InnerRegisterPayload {
            username: "alice".to_string(),
            password_proof: vec![
                1, 
                2, 
                3, 
                4
            ],
            timestamp: 1696880000000,
        };

        let decoded = round_trip(
            &inner
        );
        assert_eq!(
            decoded.username, 
            "alice"
        );
        assert_eq!(
            decoded.password_proof.len(), 
            4
        );
        assert_eq!(
            decoded.timestamp, 
            1696880000000
        );
    }

    #[test]
    fn test_layer1_ciphertext() {
        let layer1 = Layer1Ciphertext {
            ciphertext: vec![
                0u8; 
                16
            ],
            nonce: vec![
                0u8; 
                12
            ],
            inner: Some(
                InnerRegisterPayload {
                    username: "bob"
                    .to_string(),
                    password_proof: vec![
                        0u8; 
                        32
                        ],
                    timestamp: 1,
                }
            ),
            aead_tag: vec![
                0u8; 
                16
                ],
        };

        let decoded = round_trip(
            &layer1
        );
        assert!(
            decoded.inner.is_some()
        );
        assert_eq!(
            decoded.inner.unwrap().username, 
            "bob"
        );
    }

    #[test]
    fn test_layer2_ciphertext() {
        let layer2 = Layer2Ciphertext {
            ciphertext: vec![
                0u8; 
                32
            ],
            nonce: vec![
                0u8; 
                12
            ],
            layer1: Some(
                Layer1Ciphertext {
                    ciphertext: vec![
                        1, 
                        2, 
                        3
                    ],
                    nonce: vec![
                        0u8; 
                        12
                    ],
                    inner: None,
                    aead_tag: vec![0u8; 16],
                }
            ),
            aead_tag: vec![0u8; 16],
        };

        let decoded = round_trip(&layer2);
        assert!(decoded.layer1.is_some());
        assert_eq!(decoded.layer1.unwrap().ciphertext, vec![1, 2, 3]);
    }

    #[test]
    fn test_key_wrap() {
        let key_wrap = KeyWrap {
            alg: "X25519-HKDF-SHA3-512".to_string(),
            wrapped_key: vec![0u8; 32],
            key_id: vec![1, 2, 3],
        };

        let decoded = round_trip(&key_wrap);
        assert_eq!(decoded.alg, "X25519-HKDF-SHA3-512");
        assert_eq!(decoded.key_id, vec![1, 2, 3]);
    }

    #[test]
    fn test_encrypted_envelope() {
        let envelope = EncryptedEnvelope {
            client_ephemeral_pub: vec![0u8; 32],
            key_wraps: vec![KeyWrap {
                alg: "X25519-HKDF-SHA3-512".to_string(),
                wrapped_key: vec![0u8; 32],
                key_id: vec![1, 2, 3],
            }],
            layer2: Some(Layer2Ciphertext {
                ciphertext: vec![0u8; 32],
                nonce: vec![0u8; 12],
                layer1: None,
                aead_tag: vec![0u8; 16],
            }),
            hmac: vec![0u8; 64],
            client_signature: vec![],
        };

        let decoded = round_trip(&envelope);
        assert_eq!(decoded.key_wraps.len(), 1);
        assert!(decoded.layer2.is_some());
        assert_eq!(decoded.hmac.len(), 64);
    }

    #[test]
    fn test_register_request() {
        let req = RegisterRequest {
            client_nonce: vec![0u8; 16],
            timestamp: 1696880000000,
            envelope: Some(EncryptedEnvelope {
                client_ephemeral_pub: vec![0u8; 32],
                key_wraps: vec![],
                layer2: None,
                hmac: vec![0u8; 64],
                client_signature: vec![],
            }),
        };

        let decoded = round_trip(&req);
        assert_eq!(decoded.client_nonce.len(), 16);
        assert!(decoded.envelope.is_some());
    }

    #[test]
    fn test_full_nested_register_request() {
        let inner = InnerRegisterPayload {
            username: "testuser".to_string(),
            password_proof: vec![0u8; 32],
            timestamp: 1696880000000,
        };
        let layer1 = Layer1Ciphertext {
            ciphertext: vec![0u8; 64],
            nonce: vec![0u8; 12],
            inner: Some(inner),
            aead_tag: vec![0u8; 16],
        };
        let layer2 = Layer2Ciphertext {
            ciphertext: vec![0u8; 128],
            nonce: vec![0u8; 12],
            layer1: Some(layer1),
            aead_tag: vec![0u8; 16],
        };
        let key_wrap = KeyWrap {
            alg: "X25519-HKDF-SHA3-512".to_string(),
            wrapped_key: vec![0u8; 32],
            key_id: vec![1, 2, 3, 4],
        };
        let envelope = EncryptedEnvelope {
            client_ephemeral_pub: vec![
                0u8; 
                32
            ],
            key_wraps: vec![
                key_wrap
            ],
            layer2: Some(
                layer2
            ),
            hmac: vec![
                0u8; 
                64
            ],
            client_signature: vec![],
        };
        let req = RegisterRequest {
            client_nonce: vec![
                0u8; 
                16
            ],
            timestamp: 1696880000000,
            envelope: Some(
                envelope
            ),
        };

        let decoded = round_trip(
            &req
        );
        let username = decoded
            .envelope
            .unwrap()
            .layer2
            .unwrap()
            .layer1
            .unwrap()
            .inner
            .unwrap()
            .username;
        assert_eq!(username, "testuser");
    }
}



*/