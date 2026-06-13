//! Builders pour construire facilement les structures protobuf

use crate::*;
use crate::helpers::*;

/// Builder pour IdentityKey
pub struct IdentityKeyBuilder {
    key_type: SignatureKeyType,
    public_key: Vec<u8>,
}

impl IdentityKeyBuilder {
    pub fn new(key_type: SignatureKeyType) -> Self {
        Self {
            key_type,
            public_key: Vec::new(),
        }
    }

    pub fn dilithium3() -> Self {
        Self::new(SignatureKeyType::Dilithium3)
    }

    pub fn public_key(mut self, key: Vec<u8>) -> Self {
        self.public_key = key;
        self
    }

    pub fn build(self) -> IdentityKey {
        IdentityKey {
            key_type: self.key_type as i32,
            public_key: self.public_key,
        }
    }
}

/// Builder pour PreKeyBundle
pub struct PreKeyBundleBuilder {
    user_hash_id: Vec<u8>,
    registration_id: u32,
    identity_key: Option<IdentityKey>,
    pre_key_id: u32,
    pre_key_public: Vec<u8>,
    signed_pre_key_id: u32,
    signed_pre_key_public: Vec<u8>,
    signed_pre_key_signature: Vec<u8>,
    pq_pre_key_id: u32,
    pq_pre_key_public: Option<KemPublicKey>,
}

impl PreKeyBundleBuilder {
    pub fn new(username: &str) -> Self {
        Self {
            user_hash_id: username_to_hash_id(username),
            registration_id: rand::random(),
            identity_key: None,
            pre_key_id: 1,
            pre_key_public: Vec::new(),
            signed_pre_key_id: 1,
            signed_pre_key_public: Vec::new(),
            signed_pre_key_signature: Vec::new(),
            pq_pre_key_id: 1,
            pq_pre_key_public: None,
        }
    }

    pub fn registration_id(mut self, id: u32) -> Self {
        self.registration_id = id;
        self
    }

    pub fn identity_key(mut self, key: IdentityKey) -> Self {
        self.identity_key = Some(key);
        self
    }

    pub fn pre_key(mut self, id: u32, public: Vec<u8>) -> Self {
        self.pre_key_id = id;
        self.pre_key_public = public;
        self
    }

    pub fn signed_pre_key(mut self, id: u32, public: Vec<u8>, signature: Vec<u8>) -> Self {
        self.signed_pre_key_id = id;
        self.signed_pre_key_public = public;
        self.signed_pre_key_signature = signature;
        self
    }

    pub fn pq_pre_key(mut self, id: u32, key: KemPublicKey) -> Self {
        self.pq_pre_key_id = id;
        self.pq_pre_key_public = Some(key);
        self
    }

    pub fn build(self) -> PreKeyBundle {
        PreKeyBundle {
            user_hash_id: self.user_hash_id,
            registration_id: self.registration_id,
            identity_key: self.identity_key,
            pre_key_id: self.pre_key_id,
            pre_key_public: self.pre_key_public,
            signed_pre_key_id: self.signed_pre_key_id,
            signed_pre_key_public: self.signed_pre_key_public,
            signed_pre_key_signature: self.signed_pre_key_signature,
            pq_pre_key_id: self.pq_pre_key_id,
            pq_pre_key_public: self.pq_pre_key_public,
            pq_last_resort_key_id: 0,
            pq_last_resort_key_public: None,
        }
    }
}

/// Builder pour AuthChallenge
pub struct AuthChallengeBuilder {
    challenge_id: Vec<u8>,
    nonce: Vec<u8>,
    required_proof_type: ZkpType,
    public_parameters: Vec<u8>,
    difficulty: u32,
}

impl AuthChallengeBuilder {
    pub fn new() -> Self {
        Self {
            challenge_id: generate_challenge_id(),
            nonce: generate_nonce(32),
            required_proof_type: ZkpType::Stark,
            public_parameters: Vec::new(),
            difficulty: 128,
        }
    }

    pub fn proof_type(mut self, ptype: ZkpType) -> Self {
        self.required_proof_type = ptype;
        self
    }

    pub fn public_parameters(mut self, params: Vec<u8>) -> Self {
        self.public_parameters = params;
        self
    }

    pub fn difficulty(mut self, d: u32) -> Self {
        self.difficulty = d;
        self
    }

    pub fn build(self) -> AuthChallenge {
        AuthChallenge {
            challenge_id: self.challenge_id,
            nonce: self.nonce,
            required_proof_type: self.required_proof_type as i32,
            public_parameters: self.public_parameters,
            timestamp: current_timestamp(),
            difficulty: self.difficulty,
        }
    }
}

impl Default for AuthChallengeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder pour ZenthSignalEnvelope
pub struct MessageBuilder {
    sender: String,
    recipient: String,
    message_type: MessageType,
    content: Vec<u8>,
}

impl MessageBuilder {
    pub fn new(sender: &str, recipient: &str) -> Self {
        Self {
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            message_type: MessageType::RegularMessage,
            content: Vec::new(),
        }
    }

    pub fn message_type(mut self, mtype: MessageType) -> Self {
        self.message_type = mtype;
        self
    }

    pub fn content(mut self, content: Vec<u8>) -> Self {
        self.content = content;
        self
    }

    pub fn build_envelope(self, encrypted_body: EncryptedMessageBody, signature: Vec<u8>) -> ZenthSignalEnvelope {
        ZenthSignalEnvelope {
            version: 1,
            sender_hash_id: username_to_hash_id(&self.sender),
            recipient_hash_id: username_to_hash_id(&self.recipient),
            content: Some(zenth_signal_envelope::Content::RegularMessage(encrypted_body)),
            dilithium_signature: signature,
            timestamp: current_timestamp(),
            message_id: generate_message_id(),
            sequence_number: 1,
        }
    }
}

/// Builder pour GroupState
pub struct GroupBuilder {
    group_name: String,
    group_type: GroupType,
    creator: String,
    members: Vec<GroupMember>,
}

impl GroupBuilder {
    pub fn new(name: &str, creator: &str) -> Self {
        Self {
            group_name: name.to_string(),
            group_type: GroupType::PrivateGroup,
            creator: creator.to_string(),
            members: Vec::new(),
        }
    }

    pub fn group_type(mut self, gtype: GroupType) -> Self {
        self.group_type = gtype;
        self
    }

    pub fn add_member(mut self, username: &str, role: GroupRole) -> Self {
        let member = GroupMember {
            user_hash_id: username_to_hash_id(username),
            role: role as i32,
            member_lms_key: None,
            pre_key_bundle: Vec::new(),
            joined_timestamp: current_timestamp(),
            invited_by: username_to_hash_id(&self.creator),
        };
        self.members.push(member);
        self
    }

    pub fn build(self) -> GroupState {
        GroupState {
            group_id: generate_message_id(),
            group_name: self.group_name,
            group_type: self.group_type as i32,
            master_lms_key: None,
            members: self.members,
            member_count: self.members.len() as u32,
            sender_key_distribution_message: Vec::new(),
            sender_key_chain_id: 1,
            created_timestamp: current_timestamp(),
            created_by: username_to_hash_id(&self.creator),
            last_updated_timestamp: current_timestamp(),
            version: 1,
        }
    }
}

/// Builder pour NetworkPacket
pub struct NetworkPacketBuilder {
    sender: String,
    packet_type: PacketType,
}

impl NetworkPacketBuilder {
    pub fn new(sender: &str, ptype: PacketType) -> Self {
        Self {
            sender: sender.to_string(),
            packet_type: ptype,
        }
    }

    pub fn build_direct_message(self, envelope: ZenthSignalEnvelope, signature: Vec<u8>) -> NetworkPacket {
        NetworkPacket {
            version: 1,
            packet_type: self.packet_type as i32,
            packet_id: generate_message_id(),
            sender_hash_id: username_to_hash_id(&self.sender),
            payload: Some(network_packet::Payload::DirectMessage(envelope)),
            timestamp: current_timestamp(),
            ttl: 10,
            priority: 1,
            dilithium_signature: signature,
        }
    }

    pub fn build_presence(self, status: presence_packet::PresenceStatus) -> NetworkPacket {
        let presence = PresencePacket {
            user_hash_id: username_to_hash_id(&self.sender),
            status: status as i32,
            connected_peers: Vec::new(),
            capabilities: vec!["messaging".to_string(), "file_transfer".to_string()],
            timestamp: current_timestamp(),
            dilithium_signature: Vec::new(),
        };

        NetworkPacket {
            version: 1,
            packet_type: self.packet_type as i32,
            packet_id: generate_message_id(),
            sender_hash_id: username_to_hash_id(&self.sender),
            payload: Some(network_packet::Payload::Presence(presence)),
            timestamp: current_timestamp(),
            ttl: 10,
            priority: 0,
            dilithium_signature: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_key_builder() {
        let key = IdentityKeyBuilder::dilithium3()
            .public_key(vec![1, 2, 3, 4])
            .build();

        assert_eq!(key.key_type, SignatureKeyType::Dilithium3 as i32);
        assert_eq!(key.public_key, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_pre_key_bundle_builder() {
        let bundle = PreKeyBundleBuilder::new("alice")
            .registration_id(12345)
            .pre_key(1, vec![1, 2, 3])
            .build();

        assert_eq!(bundle.registration_id, 12345);
        assert!(is_valid_hash_id(&bundle.user_hash_id));
    }

    #[test]
    fn test_group_builder() {
        let group = GroupBuilder::new("My Group", "alice")
            .add_member("bob", GroupRole::Member)
            .add_member("charlie", GroupRole::Admin)
            .build();

        assert_eq!(group.group_name, "My Group");
        assert_eq!(group.members.len(), 2);
        assert_eq!(group.member_count, 2);
    }
}
