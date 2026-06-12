# Zenth DTO - Documentation Complète

Une bibliothèque Rust de Data Transfer Objects (DTO) pour le protocole de messagerie hybride post-quantique Zenth.

---

## Table des Matières

1. [Vue d'ensemble](#vue-densemble)
2. [Architecture Cryptographique](#architecture-cryptographique)
3. [Structure du Projet](#structure-du-projet)
4. [Fichiers Proto - Référence Complète](#fichiers-proto---référence-complète)
   - [auth.proto](#authproto---authentification-zkp)
   - [keys.proto](#keysproto---clés-cryptographiques)
   - [message.proto](#messageproto---messages-chiffrés)
   - [session.proto](#sessionproto---sessions-double-ratchet)
   - [group.proto](#groupproto---gestion-des-groupes)
   - [network.proto](#networkproto---transport-réseau-p2p)
   - [friend.proto](#friendproto---gestion-des-amis)
   - [storage.proto](#storageproto---stockage-local)
   - [report.proto](#reportproto---signalement)
   - [dht.proto](#dhtproto---table-de-hachage-distribuée)
5. [Relations entre les Fichiers Proto](#relations-entre-les-fichiers-proto)
6. [Utilisation en Rust](#utilisation-en-rust)
7. [Builders Disponibles](#builders-disponibles)
8. [Glossaire](#glossaire)

---

## Vue d'ensemble

Zenth DTO est une bibliothèque de structures de données pour un système de messagerie:
- **Décentralisé** (P2P, DHT)
- **Post-quantique** (Kyber, Dilithium, LMS)
- **Chiffré de bout en bout** (Triple couche: Serpent → AES → ChaCha20)
- **Respectueux de la vie privée** (ZKP, hachage des identifiants)

### Technologies utilisées

| Composant | Technologie |
|-----------|-------------|
| Langage | Rust (Edition 2024) |
| Sérialisation | Protocol Buffers (Prost) |
| RPC | gRPC (Tonic) |
| Crypto classique | AES-256-GCM, ChaCha20-Poly1305, Serpent-CBC |
| Crypto post-quantique | Kyber (KEM), Dilithium (signatures), LMS (groupes) |
| Preuves ZK | PLONK, STARK, GROTH16 |

---

## Architecture Cryptographique

### Triple Couche de Chiffrement

Les messages sont chiffrés en 3 couches successives:

```
┌─────────────────────────────────────────┐
│  Couche 1: Serpent-CBC + HMAC           │  ← Couche externe
│  ┌─────────────────────────────────────┐│
│  │  Couche 2: AES-256-GCM              ││
│  │  ┌─────────────────────────────────┐││
│  │  │  Couche 3: ChaCha20-Poly1305    │││  ← Couche interne
│  │  │  [Message en clair + hash]      │││
│  │  └─────────────────────────────────┘││
│  └─────────────────────────────────────┘│
└─────────────────────────────────────────┘
```

### Double Ratchet avec Extension Post-Quantique

Le protocole Signal (Double Ratchet) est étendu avec:
- **Kyber KEM** pour l'échange de clés post-quantique
- **Dilithium** pour les signatures utilisateur
- **LMS** pour les signatures de groupe

### Identification par Hash

Les utilisateurs sont identifiés par `user_hash_id` (hash de leur identité), jamais par leur nom d'utilisateur directement.

---

## Structure du Projet

```
zenth_dto/
├── proto/                      # Définitions Protocol Buffers
│   ├── auth.proto             # Authentification ZKP
│   ├── keys.proto             # Structures de clés
│   ├── message.proto          # Format des messages
│   ├── session.proto          # Sessions Double Ratchet
│   ├── group.proto            # Gestion des groupes
│   ├── network.proto          # Transport réseau P2P
│   ├── friend.proto           # Gestion des amis
│   ├── storage.proto          # Stockage local chiffré
│   ├── report.proto           # Système de signalement
│   └── dht.proto              # Table de hachage distribuée
├── src/
│   ├── lib.rs                 # Point d'entrée (exports)
│   └── builders.rs            # Builders fluent
├── build.rs                   # Script de compilation proto
└── Cargo.toml                 # Configuration Rust
```

---

## Fichiers Proto - Référence Complète

### auth.proto - Authentification ZKP

**But**: Authentification par preuves à divulgation nulle de connaissance (Zero-Knowledge Proofs).

#### Enums

| Enum | Valeurs | Description |
|------|---------|-------------|
| `ZKPType` | `ZKP_TYPE_UNKNOWN (0)`, `PLONK (1)`, `STARK (2)`, `GROTH16 (3)` | Types de preuves ZK supportés |

#### Messages

##### `AuthChallenge` - Challenge d'authentification
Le serveur envoie ce challenge que l'utilisateur doit résoudre.

| Champ | Type | Description |
|-------|------|-------------|
| `challenge_id` | `bytes` | Identifiant unique du challenge |
| `nonce` | `bytes` | Nonce aléatoire |
| `required_proof_type` | `ZKPType` | Type de preuve demandé |
| `public_parameters` | `bytes` | Paramètres publics du ZKP |
| `timestamp` | `int64` | Horodatage de création |
| `difficulty` | `uint32` | Niveau de difficulté |

##### `AuthProof` - Réponse au challenge
Preuve ZK générée par l'utilisateur.

| Champ | Type | Description |
|-------|------|-------------|
| `challenge_id` | `bytes` | ID du challenge répondu |
| `user_hash_id` | `bytes` | Hash de l'identifiant utilisateur |
| `proof_type` | `ZKPType` | Type de preuve utilisé |
| `proof` | `bytes` | La preuve cryptographique |
| `public_inputs` | `bytes` | Entrées publiques de la preuve |
| `timestamp` | `int64` | Horodatage |

##### `RegistrationRequest` - Inscription
Requête d'inscription d'un nouvel utilisateur.

| Champ | Type | Description |
|-------|------|-------------|
| `username_hash` | `bytes` | Hash du nom d'utilisateur |
| `pre_key_bundle` | `PreKeyBundle` | Bundle de pré-clés (voir keys.proto) |
| `proof_type` | `ZKPType` | Type de preuve choisi |
| `password_commitment` | `bytes` | Engagement sur le mot de passe |
| `initial_proof` | `bytes` | Preuve initiale |
| `identity_key_dilithium` | `bytes` | Clé publique Dilithium |
| `identity_signature` | `bytes` | Signature d'identité |
| `timestamp` | `int64` | Horodatage |

##### `RegistrationResponse` - Réponse d'inscription

| Champ | Type | Description |
|-------|------|-------------|
| `success` | `bool` | Succès ou échec |
| `user_hash_id` | `bytes` | Hash ID attribué |
| `challenge_parameters` | `bytes` | Paramètres pour futurs challenges |
| `error_message` | `string` | Message d'erreur si échec |

##### `LoginRequest` - Connexion

| Champ | Type | Description |
|-------|------|-------------|
| `user_hash_id` | `bytes` | Hash ID de l'utilisateur |
| `request_challenge` | `bool` | Demander un nouveau challenge |
| `proof` | `AuthProof` | Preuve (optionnelle) |
| `timestamp` | `int64` | Horodatage |

##### `LoginResponse` - Réponse de connexion

| Champ | Type | Description |
|-------|------|-------------|
| `success` | `bool` | Succès ou échec |
| `challenge` | `AuthChallenge` | Challenge si demandé |
| `session_token` | `bytes` | Token de session si succès |
| `session_expiry` | `int64` | Expiration de la session |
| `error_message` | `string` | Message d'erreur |

##### `FriendRequest` - Demande d'ami

| Champ | Type | Description |
|-------|------|-------------|
| `requester_hash_id` | `bytes` | Hash ID du demandeur |
| `target_hash_id` | `bytes` | Hash ID de la cible |
| `pre_key_bundle` | `PreKeyBundle` | Bundle de pré-clés |
| `dilithium_signature` | `bytes` | Signature Dilithium |
| `encrypted_message` | `bytes` | Message chiffré (optionnel) |
| `timestamp` | `int64` | Horodatage |

##### `FriendResponse` - Réponse à une demande d'ami

| Champ | Type | Description |
|-------|------|-------------|
| `responder_hash_id` | `bytes` | Hash ID du répondeur |
| `requester_hash_id` | `bytes` | Hash ID du demandeur |
| `accepted` | `bool` | Accepté ou refusé |
| `pre_key_bundle` | `PreKeyBundle` | Bundle si accepté |
| `dilithium_signature` | `bytes` | Signature |
| `timestamp` | `int64` | Horodatage |

---

### keys.proto - Clés Cryptographiques

**But**: Définitions des structures de clés pour la cryptographie hybride post-quantique.

#### Enums

| Enum | Valeurs | Description |
|------|---------|-------------|
| `KEMKeyType` | `KEM_KEY_TYPE_UNKNOWN (0)`, `KYBER_768 (0x07)`, `KYBER_1024 (0x08)`, `MLKEM_1024 (0x0A)` | Types de KEM post-quantique |
| `SignatureKeyType` | `SIGNATURE_KEY_TYPE_UNKNOWN (0)`, `DILITHIUM_2 (0x10)`, `DILITHIUM_3 (0x11)`, `DILITHIUM_5 (0x12)` | Types de signatures Dilithium |
| `GroupKeyType` | `GROUP_KEY_TYPE_UNKNOWN (0)`, `LMS_SHA256_M32_H10 (0x20)`, `LMS_SHA256_M32_H15 (0x21)`, `LMS_SHA256_M32_H20 (0x22)` | Types de clés LMS pour groupes |

#### Messages

##### `IdentityKey` - Clé d'identité long terme

| Champ | Type | Description |
|-------|------|-------------|
| `key_type` | `SignatureKeyType` | Type Dilithium |
| `public_key` | `bytes` | Clé publique |

##### `KEMPublicKey` - Clé publique KEM

| Champ | Type | Description |
|-------|------|-------------|
| `key_type` | `KEMKeyType` | Type Kyber/MLKEM |
| `public_key` | `bytes` | Clé publique |

##### `PreKeyBundle` - Bundle de pré-clés Signal+PQ

C'est la structure centrale pour l'échange de clés. Compatible Signal avec extensions post-quantiques.

| Champ | Type | Description |
|-------|------|-------------|
| `user_hash_id` | `bytes` | Hash ID de l'utilisateur |
| `registration_id` | `uint32` | ID d'enregistrement Signal |
| `identity_key` | `IdentityKey` | Clé d'identité Dilithium |
| `pre_key_id` | `uint32` | ID de la pré-clé |
| `pre_key_public` | `bytes` | Pré-clé publique (X25519) |
| `signed_pre_key_id` | `uint32` | ID de la pré-clé signée |
| `signed_pre_key_public` | `bytes` | Pré-clé signée publique |
| `signed_pre_key_signature` | `bytes` | Signature de la pré-clé |
| `pq_pre_key_id` | `uint32` | ID de la pré-clé PQ |
| `pq_pre_key_public` | `bytes` | Pré-clé Kyber publique |
| `pq_last_resort_key_id` | `uint32` | ID de la clé de dernier recours PQ |
| `pq_last_resort_key_public` | `bytes` | Clé de dernier recours Kyber |

##### `GroupKey` - Clé de groupe LMS

| Champ | Type | Description |
|-------|------|-------------|
| `key_type` | `GroupKeyType` | Type LMS |
| `public_key` | `bytes` | Clé publique |
| `tree_identifier` | `bytes` | Identifiant de l'arbre Merkle |

##### `GroupInvitation` - Invitation de groupe

| Champ | Type | Description |
|-------|------|-------------|
| `group_id` | `bytes` | ID du groupe |
| `inviter_hash_id` | `bytes` | Hash ID de l'inviteur |
| `invitee_hash_id` | `bytes` | Hash ID de l'invité |
| `group_key` | `GroupKey` | Clé du groupe |
| `lms_signature` | `bytes` | Signature LMS |
| `timestamp` | `int64` | Horodatage |

---

### message.proto - Messages Chiffrés

**But**: Format des messages avec chiffrement triple couche et ratchet post-quantique.

#### Enums

| Enum | Valeurs | Description |
|------|---------|-------------|
| `MessageType` | `MESSAGE_TYPE_UNKNOWN (0)`, `PREKEY_MESSAGE (1)`, `REGULAR_MESSAGE (2)`, `REKEY_MESSAGE (3)` | Type de message |
| `SymmetricLayer` | `SYMMETRIC_UNKNOWN (0)`, `SERPENT_CBC (1)`, `AES_256_GCM (2)`, `CHACHA20_POLY1305 (3)` | Algorithme de chaque couche |

#### Messages

##### `RatchetHeader` - En-tête Double Ratchet

| Champ | Type | Description |
|-------|------|-------------|
| `sender_ratchet_key` | `bytes` | Clé de ratchet de l'envoyeur |
| `previous_counter` | `uint32` | Compteur précédent |
| `counter` | `uint32` | Compteur actuel |
| `pq_ciphertext` | `bytes` | Chiffré Kyber (extension PQ) |

##### `Layer3Ciphertext` - Couche interne (ChaCha20-Poly1305)

| Champ | Type | Description |
|-------|------|-------------|
| `ciphertext` | `bytes` | Données chiffrées |
| `nonce` | `bytes` | Nonce (12 bytes) |
| `poly1305_tag` | `bytes` | Tag d'authentification |
| `plaintext_hash` | `bytes` | Hash du texte clair (intégrité) |

##### `Layer2Ciphertext` - Couche intermédiaire (AES-256-GCM)

| Champ | Type | Description |
|-------|------|-------------|
| `ciphertext` | `bytes` | Données chiffrées |
| `nonce` | `bytes` | Nonce (12 bytes) |
| `gcm_tag` | `bytes` | Tag GCM |
| `layer3` | `Layer3Ciphertext` | Couche 3 encapsulée |

##### `Layer1Ciphertext` - Couche externe (Serpent-CBC)

| Champ | Type | Description |
|-------|------|-------------|
| `ciphertext` | `bytes` | Données chiffrées |
| `iv` | `bytes` | Vecteur d'initialisation |
| `hmac` | `bytes` | HMAC pour authentification |
| `layer2` | `Layer2Ciphertext` | Couche 2 encapsulée |

##### `LayerKeyWrap` - Clé symétrique wrappée

| Champ | Type | Description |
|-------|------|-------------|
| `layer_type` | `SymmetricLayer` | Pour quelle couche |
| `wrapped_key` | `bytes` | Clé wrappée par le ratchet |
| `key_id` | `bytes` | Identifiant de la clé |

##### `EncryptedMessageBody` - Corps du message chiffré

| Champ | Type | Description |
|-------|------|-------------|
| `message_type` | `MessageType` | Type de message |
| `ratchet_header` | `RatchetHeader` | En-tête ratchet |
| `layer_keys` | `repeated LayerKeyWrap` | Clés des 3 couches |
| `encrypted_layers` | `Layer1Ciphertext` | Message triple-chiffré |
| `encrypted_metadata` | `bytes` | Métadonnées chiffrées |

##### `PreKeyMessage` - Message initial (premier contact)

| Champ | Type | Description |
|-------|------|-------------|
| `pre_key_id` | `uint32` | ID de la pré-clé utilisée |
| `signed_pre_key_id` | `uint32` | ID de la pré-clé signée |
| `base_key` | `bytes` | Clé de base éphémère |
| `identity_key` | `bytes` | Clé d'identité de l'envoyeur |
| `pq_pre_key_id` | `uint32` | ID de la pré-clé PQ |
| `pq_ciphertext` | `bytes` | Chiffré Kyber |
| `message` | `EncryptedMessageBody` | Message chiffré |

##### `ZenthSignalEnvelope` - Enveloppe complète (1-to-1)

| Champ | Type | Description |
|-------|------|-------------|
| `version` | `uint32` | Version du protocole |
| `sender_hash_id` | `bytes` | Hash ID envoyeur |
| `recipient_hash_id` | `bytes` | Hash ID destinataire |
| `content` | `oneof` | `prekey_message` ou `regular_message` |
| `dilithium_signature` | `bytes` | Signature Dilithium |
| `timestamp` | `int64` | Horodatage |
| `message_id` | `bytes` | ID unique du message |
| `sequence_number` | `uint64` | Numéro de séquence |

##### `GroupMessage` - Message de groupe

| Champ | Type | Description |
|-------|------|-------------|
| `group_id` | `bytes` | ID du groupe |
| `sender_hash_id` | `bytes` | Hash ID envoyeur |
| `encrypted_message_key` | `bytes` | Clé de message chiffrée |
| `recipient_hash_ids` | `repeated bytes` | Hash IDs des destinataires |
| `encrypted_message` | `bytes` | Message chiffré |
| `lms_signature` | `bytes` | Signature LMS |
| `timestamp` | `int64` | Horodatage |
| `message_id` | `bytes` | ID unique |

##### `MessageAck` - Accusé de réception

| Champ | Type | Description |
|-------|------|-------------|
| `message_id` | `bytes` | ID du message acquitté |
| `recipient_hash_id` | `bytes` | Hash ID du récepteur |
| `delivered` | `bool` | Livré |
| `read` | `bool` | Lu |
| `timestamp` | `int64` | Horodatage |
| `dilithium_signature` | `bytes` | Signature |

---

### session.proto - Sessions Double Ratchet

**But**: Gestion de l'état des sessions avec le protocole Double Ratchet étendu.

#### Messages

##### `Chain` - État d'une chaîne de ratchet

| Champ | Type | Description |
|-------|------|-------------|
| `sender_ratchet_key` | `bytes` | Clé de ratchet |
| `chain_key_index` | `uint32` | Index actuel |
| `chain_key` | `bytes` | Clé de chaîne |
| `message_keys` | `repeated MessageKey` | Clés de messages stockées |

##### `MessageKey` - Clé d'un message individuel

| Champ | Type | Description |
|-------|------|-------------|
| `index` | `uint32` | Index dans la chaîne |
| `cipher_key` | `bytes` | Clé de chiffrement |
| `mac_key` | `bytes` | Clé MAC |
| `iv` | `bytes` | Vecteur d'initialisation |

##### `PQRatchetState` - État du ratchet post-quantique

| Champ | Type | Description |
|-------|------|-------------|
| `kem_type` | `KEMKeyType` | Type Kyber |
| `current_pq_key` | `bytes` | Clé PQ actuelle |
| `pq_ratchet_index` | `uint32` | Index du ratchet PQ |
| `pending_pq_key` | `bytes` | Clé PQ en attente |

##### `SessionState` - État complet de la session

| Champ | Type | Description |
|-------|------|-------------|
| `session_version` | `uint32` | Version |
| `local_identity` | `bytes` | Identité locale |
| `remote_identity` | `bytes` | Identité distante |
| `root_key` | `bytes` | Clé racine |
| `sender_chain` | `Chain` | Chaîne d'envoi |
| `receiver_chains` | `repeated Chain` | Chaînes de réception |
| `pq_ratchet` | `PQRatchetState` | État PQ |
| `previous_counter` | `uint32` | Compteur précédent |
| `remote_registration_id` | `uint32` | ID d'enregistrement distant |
| `local_registration_id` | `uint32` | ID d'enregistrement local |
| `needs_refresh` | `bool` | Besoin de rafraîchissement |
| `last_refresh_timestamp` | `int64` | Dernier rafraîchissement |

##### `Session` - Session P2P

| Champ | Type | Description |
|-------|------|-------------|
| `local_user_hash_id` | `bytes` | Hash ID local |
| `remote_user_hash_id` | `bytes` | Hash ID distant |
| `session_id` | `bytes` | ID de session |
| `state` | `SessionState` | État |
| `created_timestamp` | `int64` | Création |
| `last_used_timestamp` | `int64` | Dernière utilisation |
| `is_active` | `bool` | Active |

##### `SessionStore` - Stockage des sessions

| Champ | Type | Description |
|-------|------|-------------|
| `sessions` | `repeated Session` | Liste des sessions |
| `owner_hash_id` | `bytes` | Hash ID du propriétaire |

##### `PreKeyRecord` - Pré-clé éphémère

| Champ | Type | Description |
|-------|------|-------------|
| `id` | `uint32` | ID |
| `public_key` | `bytes` | Clé publique |
| `private_key` | `bytes` | Clé privée |

##### `SignedPreKeyRecord` - Pré-clé signée

| Champ | Type | Description |
|-------|------|-------------|
| `id` | `uint32` | ID |
| `public_key` | `bytes` | Clé publique |
| `private_key` | `bytes` | Clé privée |
| `signature` | `bytes` | Signature Dilithium |
| `timestamp` | `int64` | Horodatage |

##### `PQPreKeyRecord` - Pré-clé post-quantique

| Champ | Type | Description |
|-------|------|-------------|
| `id` | `uint32` | ID |
| `key_type` | `KEMKeyType` | Type Kyber |
| `public_key` | `bytes` | Clé publique |
| `secret_key` | `bytes` | Clé secrète |
| `timestamp` | `int64` | Horodatage |

##### `PreKeyStore` - Stockage des pré-clés

| Champ | Type | Description |
|-------|------|-------------|
| `pre_keys` | `repeated PreKeyRecord` | Pré-clés |
| `signed_pre_keys` | `repeated SignedPreKeyRecord` | Pré-clés signées |
| `pq_pre_keys` | `repeated PQPreKeyRecord` | Pré-clés PQ |
| `next_pre_key_id` | `uint32` | Prochain ID |
| `next_signed_pre_key_id` | `uint32` | Prochain ID signé |
| `next_pq_pre_key_id` | `uint32` | Prochain ID PQ |

---

### group.proto - Gestion des Groupes

**But**: Création et gestion des groupes avec signatures LMS.

#### Enums

| Enum | Valeurs | Description |
|------|---------|-------------|
| `GroupType` | `GROUP_TYPE_UNKNOWN (0)`, `PRIVATE_GROUP (1)`, `PUBLIC_GROUP (2)`, `CHANNEL (3)` | Type de groupe |
| `GroupRole` | `ROLE_UNKNOWN (0)`, `MEMBER (1)`, `ADMIN (2)`, `OWNER (3)` | Rôle d'un membre |
| `GroupOperation` | `OPERATION_UNKNOWN (0)`, `CREATE_GROUP (1)`, `ADD_MEMBER (2)`, `REMOVE_MEMBER (3)`, `PROMOTE_MEMBER (4)`, `DEMOTE_MEMBER (5)`, `UPDATE_NAME (6)`, `UPDATE_KEY (7)`, `LEAVE_GROUP (8)` | Opérations possibles |

#### Messages principaux

##### `GroupMember` - Membre de groupe

| Champ | Type | Description |
|-------|------|-------------|
| `user_hash_id` | `bytes` | Hash ID |
| `role` | `GroupRole` | Rôle |
| `member_lms_key` | `GroupKey` | Clé LMS du membre |
| `pre_key_bundle` | `PreKeyBundle` | Bundle de pré-clés |
| `joined_timestamp` | `int64` | Date d'adhésion |
| `invited_by` | `bytes` | Hash ID de l'inviteur |

##### `GroupState` - État du groupe

| Champ | Type | Description |
|-------|------|-------------|
| `group_id` | `bytes` | ID du groupe |
| `group_name` | `bytes` | Nom (chiffré) |
| `group_type` | `GroupType` | Type |
| `master_lms_key` | `GroupKey` | Clé maître LMS |
| `members` | `repeated GroupMember` | Liste des membres |
| `member_count` | `uint32` | Nombre de membres |
| `sender_key_distribution_message` | `SenderKeyDistributionMessage` | Distribution des clés |
| `sender_key_chain_id` | `bytes` | ID de la chaîne |
| `created_timestamp` | `int64` | Création |
| `created_by` | `bytes` | Créateur |
| `last_updated_timestamp` | `int64` | Dernière mise à jour |
| `version` | `uint64` | Version (pour sync) |

##### `GroupChange` - Changement d'état

| Champ | Type | Description |
|-------|------|-------------|
| `group_id` | `bytes` | ID du groupe |
| `operation` | `GroupOperation` | Type d'opération |
| `initiator_hash_id` | `bytes` | Qui a initié |
| `change_details` | `oneof` | Détails (voir ci-dessous) |
| `lms_signature` | `bytes` | Signature LMS |
| `timestamp` | `int64` | Horodatage |
| `previous_version` | `uint64` | Version précédente |
| `new_version` | `uint64` | Nouvelle version |

##### Détails des changements (oneof de GroupChange)

- `CreateGroupDetails`: `group_name`, `group_type`, `master_lms_key`, `initial_members`
- `AddMemberDetails`: `new_member_hash_id`, `initial_role`, `member_lms_key`, `pre_key_bundle`
- `RemoveMemberDetails`: `removed_member_hash_id`, `reason`
- `UpdateRoleDetails`: `target_member_hash_id`, `old_role`, `new_role`
- `UpdateNameDetails`: `old_name`, `new_name`
- `UpdateKeyDetails`: `new_master_key`, `new_sender_key_distribution`, `new_chain_id`, `reason`

##### `SenderKeyDistributionMessage` - Distribution de clé sender

| Champ | Type | Description |
|-------|------|-------------|
| `group_id` | `bytes` | ID du groupe |
| `chain_id` | `bytes` | ID de chaîne |
| `iteration` | `uint32` | Itération |
| `chain_key` | `bytes` | Clé de chaîne |
| `signing_key` | `bytes` | Clé de signature |
| `lms_signature` | `bytes` | Signature LMS |

##### `GroupStore` - Stockage local des groupes

| Champ | Type | Description |
|-------|------|-------------|
| `groups` | `repeated GroupState` | Liste des groupes |
| `owner_hash_id` | `bytes` | Propriétaire |
| `sender_key_states` | `map<string, SenderKeyState>` | États sender key |

---

### network.proto - Transport Réseau P2P

**But**: Paquets réseau pour le transport décentralisé.

#### Enums

| Enum | Valeurs | Description |
|------|---------|-------------|
| `PacketType` | `PACKET_TYPE_UNKNOWN (0)`, `DIRECT_MESSAGE (1)`, `GROUP_MESSAGE (2)`, `KEY_EXCHANGE (3)`, `PRESENCE (4)`, `ACK (5)`, `SYNC (6)` | Type de paquet |

#### Messages

##### `NetworkPacket` - Paquet réseau de base

| Champ | Type | Description |
|-------|------|-------------|
| `version` | `uint32` | Version |
| `packet_type` | `PacketType` | Type |
| `packet_id` | `bytes` | ID unique |
| `sender_hash_id` | `bytes` | Envoyeur |
| `payload` | `oneof` | Contenu (voir types ci-dessous) |
| `timestamp` | `int64` | Horodatage |
| `ttl` | `uint32` | Time-to-live |
| `priority` | `uint32` | Priorité |
| `dilithium_signature` | `bytes` | Signature |

##### `PresencePacket` - Statut de présence

| Champ | Type | Description |
|-------|------|-------------|
| `user_hash_id` | `bytes` | Utilisateur |
| `status` | `enum` | `STATUS_UNKNOWN`, `ONLINE`, `AWAY`, `BUSY`, `OFFLINE` |
| `connected_peers` | `repeated bytes` | Pairs connectés |
| `capabilities` | `repeated string` | Capacités |
| `timestamp` | `int64` | Horodatage |
| `dilithium_signature` | `bytes` | Signature |

##### `SyncPacket` - Requête de synchronisation

| Champ | Type | Description |
|-------|------|-------------|
| `requester_hash_id` | `bytes` | Demandeur |
| `sync_type` | `enum` | `FULL_SYNC`, `PARTIAL_SYNC`, `MESSAGE_SYNC`, `GROUP_SYNC`, `CONTACT_SYNC` |
| `last_sync_timestamp` | `int64` | Dernier sync |
| `encrypted_sync_data` | `bytes` | Données chiffrées |
| `timestamp` | `int64` | Horodatage |

##### `FileTransferMessage` - Transfert de fichier

| Champ | Type | Description |
|-------|------|-------------|
| `transfer_id` | `bytes` | ID du transfert |
| `sender_hash_id` | `bytes` | Envoyeur |
| `recipient_hash_id` | `bytes` | Destinataire |
| `status` | `enum` | `OFFER`, `ACCEPT`, `REJECT`, `CHUNK`, `COMPLETE`, `ERROR` |
| `filename` | `string` | Nom du fichier |
| `file_size` | `uint64` | Taille |
| `mime_type` | `string` | Type MIME |
| `file_hash` | `bytes` | Hash du fichier |
| `chunk_index` | `uint32` | Index du chunk |
| `total_chunks` | `uint32` | Total de chunks |
| `chunk_data` | `bytes` | Données du chunk |
| `timestamp` | `int64` | Horodatage |
| `dilithium_signature` | `bytes` | Signature |

##### `RoutingRequest` - Routage onion-style

| Champ | Type | Description |
|-------|------|-------------|
| `packet_id` | `bytes` | ID |
| `destination_hash_id` | `bytes` | Destination |
| `payload` | `bytes` | Charge utile |
| `hop_count` | `uint32` | Sauts effectués |
| `max_hops` | `uint32` | Sauts max |
| `visited_peers` | `repeated bytes` | Pairs visités |
| `timestamp` | `int64` | Horodatage |

---

### friend.proto - Gestion des Amis

**But**: Recherche et gestion des contacts.

#### Messages

##### `LookupUserRequest` - Recherche d'utilisateur

| Champ | Type | Description |
|-------|------|-------------|
| `requester_hash` | `bytes` | Demandeur |
| `target_hash` | `bytes` | Cible |
| `timestamp` | `int64` | Horodatage |
| `dilithium_signature` | `bytes` | Signature |

##### `LookupUserResponse` - Réponse de recherche

| Champ | Type | Description |
|-------|------|-------------|
| `found` | `bool` | Trouvé |
| `user_hash` | `bytes` | Hash de l'utilisateur |
| `pre_key_bundle` | `PreKeyBundle` | Bundle si trouvé |
| `error_message` | `string` | Erreur |

##### `FetchFriendRequestsRequest` - Récupérer les demandes

| Champ | Type | Description |
|-------|------|-------------|
| `user_hash` | `bytes` | Utilisateur |
| `session_token` | `bytes` | Token de session |
| `since_timestamp` | `int64` | Depuis quand |
| `timestamp` | `int64` | Horodatage |
| `dilithium_signature` | `bytes` | Signature |

##### `FetchFriendRequestsResponse` - Liste des demandes

| Champ | Type | Description |
|-------|------|-------------|
| `requests` | `repeated FriendRequest` | Liste des demandes |
| `timestamp` | `int64` | Horodatage |

##### `StoredFriendInfo` - Info ami (stockage local)

| Champ | Type | Description |
|-------|------|-------------|
| `user_hash` | `bytes` | Hash |
| `pseudo` | `bytes` | Pseudo (chiffré) |
| `identity_key` | `bytes` | Clé d'identité |
| `kyber_key` | `bytes` | Clé Kyber |
| `x25519_key` | `bytes` | Clé X25519 |
| `verified` | `bool` | Vérifié |
| `blocked` | `bool` | Bloqué |
| `added_at` | `int64` | Date d'ajout |

---

### storage.proto - Stockage Local

**But**: Structures pour la base de données locale chiffrée.

#### Enums

| Enum | Valeurs | Description |
|------|---------|-------------|
| `TrustLevel` | `TRUST_UNKNOWN (0)`, `UNTRUSTED (1)`, `TRUSTED (2)`, `VERIFIED (3)` | Niveau de confiance |
| `ConversationType` | `TYPE_UNKNOWN (0)`, `DIRECT (1)`, `GROUP (2)` | Type de conversation |
| `MessageStatus` | `STATUS_UNKNOWN (0)`, `SENDING (1)`, `SENT (2)`, `DELIVERED (3)`, `READ (4)`, `FAILED (5)` | Statut du message |
| `KDFType` | `KDF_UNKNOWN (0)`, `ARGON2ID (1)`, `SCRYPT (2)`, `PBKDF2_SHA512 (3)` | Fonction de dérivation |

#### Messages

##### `LocalProfile` - Profil utilisateur local

| Champ | Type | Description |
|-------|------|-------------|
| `user_hash_id` | `bytes` | Hash ID |
| `username_hash` | `bytes` | Hash du nom |
| `identity_key_pair_public` | `bytes` | Clé publique |
| `identity_key_pair_private` | `bytes` | Clé privée |
| `registration_id` | `uint32` | ID Signal |
| `pre_key_store` | `PreKeyStore` | Pré-clés |
| `zkp_parameters` | `bytes` | Paramètres ZKP |
| `password_commitment` | `bytes` | Engagement mot de passe |
| `created_timestamp` | `int64` | Création |
| `last_updated_timestamp` | `int64` | Dernière MAJ |

##### `Contact` - Contact dans le carnet

| Champ | Type | Description |
|-------|------|-------------|
| `contact_hash_id` | `bytes` | Hash ID |
| `encrypted_display_name` | `bytes` | Nom affiché (chiffré) |
| `identity_key` | `IdentityKey` | Clé d'identité |
| `last_known_bundle` | `PreKeyBundle` | Dernier bundle connu |
| `added_timestamp` | `int64` | Date d'ajout |
| `last_message_timestamp` | `int64` | Dernier message |
| `trust_level` | `TrustLevel` | Confiance |
| `verification_signature` | `bytes` | Signature de vérification |
| `message_count` | `uint64` | Nombre de messages |
| `is_blocked` | `bool` | Bloqué |

##### `StoredMessage` - Message stocké

| Champ | Type | Description |
|-------|------|-------------|
| `message_id` | `bytes` | ID |
| `conversation_hash_id` | `bytes` | ID conversation |
| `conversation_type` | `ConversationType` | Type |
| `message` | `oneof` | `envelope` ou `group_message` |
| `status` | `MessageStatus` | Statut |
| `is_outgoing` | `bool` | Sortant |
| `created_timestamp` | `int64` | Création |
| `sent_timestamp` | `int64` | Envoyé |
| `delivered_timestamp` | `int64` | Livré |
| `read_timestamp` | `int64` | Lu |
| `is_edited` | `bool` | Édité |
| `is_deleted` | `bool` | Supprimé |
| `reply_to_message_id` | `bytes` | Réponse à |

##### `MasterKey` - Clé maître dérivée du mot de passe

| Champ | Type | Description |
|-------|------|-------------|
| `key_material` | `bytes` | Matériel de clé |
| `salt` | `bytes` | Sel |
| `iterations` | `uint32` | Itérations |
| `kdf_type` | `KDFType` | Type de KDF |
| `created_timestamp` | `int64` | Création |

##### `ZenthDatabase` - Base de données complète

| Champ | Type | Description |
|-------|------|-------------|
| `version` | `uint32` | Version |
| `profile` | `LocalProfile` | Profil |
| `encrypted_master_key` | `bytes` | Clé maître chiffrée |
| `session_db` | `SessionDatabase` | Sessions |
| `address_book` | `AddressBook` | Contacts |
| `message_db` | `MessageDatabase` | Messages |
| `group_store` | `GroupStore` | Groupes |
| `created_timestamp` | `int64` | Création |
| `last_backup_timestamp` | `int64` | Dernier backup |
| `database_hash` | `bytes` | Hash d'intégrité |

##### `LocalSettings` - Paramètres utilisateur

| Champ | Type | Description |
|-------|------|-------------|
| `user_hash_id` | `bytes` | Utilisateur |
| `require_zkp_on_each_login` | `bool` | ZKP à chaque connexion |
| `session_timeout_minutes` | `uint32` | Timeout session |
| `auto_lock_enabled` | `bool` | Verrouillage auto |
| `enable_peer_discovery` | `bool` | Découverte de pairs |
| `max_peer_connections` | `uint32` | Connexions max |
| `bootstrap_nodes` | `repeated string` | Noeuds bootstrap |
| `always_use_pq_keys` | `bool` | Toujours utiliser PQ |
| `auto_rotate_keys` | `bool` | Rotation auto |
| `key_rotation_days` | `uint32` | Jours entre rotations |
| `auto_download_media` | `bool` | DL auto médias |
| `show_read_receipts` | `bool` | Accusés de lecture |
| `show_typing_indicators` | `bool` | Indicateur de frappe |
| `last_updated_timestamp` | `int64` | Dernière MAJ |

---

### report.proto - Système de Signalement

**But**: Signalement d'abus préservant la vie privée.

#### Enums

| Enum | Valeurs | Description |
|------|---------|-------------|
| `ReportCategory` | `REPORT_CATEGORY_UNKNOWN (0)`, `SPAM (1)`, `HARASSMENT (2)`, `HATE_SPEECH (3)`, `ILLEGAL_CONTENT (4)`, `IMPERSONATION (5)`, `SCAM (6)`, `MALWARE (7)`, `UNWANTED_CONTACT (8)`, `OTHER (99)` | Catégories |
| `ReportSeverity` | `SEVERITY_UNKNOWN (0)`, `LOW (1)`, `MEDIUM (2)`, `HIGH (3)`, `CRITICAL (4)` | Gravité |
| `ReportStatus` | `STATUS_UNKNOWN (0)`, `PENDING (1)`, `ACKNOWLEDGED (2)`, `RESOLVED (3)`, `DISMISSED (4)` | Statut |
| `EvidenceType` | `EVIDENCE_TYPE_UNKNOWN (0)`, `MESSAGE_HASH (1)`, `MESSAGE_SIGNATURE (2)`, `SCREENSHOT_HASH (3)`, `CONVERSATION_PROOF (4)`, `TIMESTAMP_PROOF (5)` | Types de preuves |

#### Messages principaux

##### `ReportUserRequest` - Signaler un utilisateur

| Champ | Type | Description |
|-------|------|-------------|
| `reporter_hash` | `bytes` | Signaleur |
| `anonymous` | `bool` | Anonyme |
| `target_hash` | `bytes` | Cible |
| `category` | `ReportCategory` | Catégorie |
| `severity` | `ReportSeverity` | Gravité |
| `encrypted_description` | `bytes` | Description chiffrée |
| `description_nonce` | `bytes` | Nonce |
| `evidence` | `repeated ReportEvidence` | Preuves |
| `interaction_proof` | `bytes` | Preuve d'interaction |
| `timestamp` | `int64` | Horodatage |
| `request_id` | `bytes` | ID requête |
| `dilithium_signature` | `bytes` | Signature |

##### `BlockUserRequest` - Bloquer un utilisateur

| Champ | Type | Description |
|-------|------|-------------|
| `blocker_hash` | `bytes` | Bloqueur |
| `blocked_hash` | `bytes` | Bloqué |
| `timestamp` | `int64` | Horodatage |
| `dilithium_signature` | `bytes` | Signature |
| `also_report` | `bool` | Aussi signaler |
| `report_category` | `ReportCategory` | Catégorie si signalement |

---

### dht.proto - Table de Hachage Distribuée

**But**: Point d'entrée unifié pour toutes les requêtes DHT.

#### Enums

| Enum | Valeurs | Description |
|------|---------|-------------|
| `Method` | `METHOD_UNKNOWN (0)`, `REGISTER (1)`, `LOGIN (2)`, `DELETE (3)`, `BLOCK (4)`, `GROUP (5)`, `CONTACT (6)`, `REPORT (7)`, `LOOKUP_USER (8)`, `SEND_FRIEND_REQUEST (9)`, `FETCH_FRIEND_REQUESTS (10)`, `RESPOND_FRIEND_REQUEST (11)` | Méthodes disponibles |

#### Messages

##### `DhtRequest` - Requête DHT unifiée

| Champ | Type | Description |
|-------|------|-------------|
| `method` | `Method` | Méthode appelée |
| `payload` | `bytes` | Charge utile (message sérialisé) |
| `timestamp` | `int64` | Horodatage |
| `request_id` | `bytes` | ID de requête |

##### `DhtResponse` - Réponse DHT unifiée

| Champ | Type | Description |
|-------|------|-------------|
| `success` | `bool` | Succès |
| `method` | `Method` | Méthode répondue |
| `payload` | `bytes` | Charge utile de réponse |
| `error_message` | `string` | Message d'erreur |
| `timestamp` | `int64` | Horodatage |
| `request_id` | `bytes` | ID de requête |

---

## Relations entre les Fichiers Proto

```
┌─────────────────────────────────────────────────────────────┐
│                         dht.proto                           │
│                    (Point d'entrée unifié)                  │
│                 importe: auth, keys                         │
└─────────────────────────────────────────────────────────────┘
                              │
           ┌──────────────────┼──────────────────┐
           ▼                  ▼                  ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│   auth.proto    │  │   keys.proto    │  │  friend.proto   │
│    (ZKP Auth)   │  │ (Clés crypto)   │  │  (Contacts)     │
│  (pas d'import) │  │ (pas d'import)  │  │ importe: keys,  │
│                 │  │                 │  │          auth   │
└─────────────────┘  └─────────────────┘  └─────────────────┘
                              │
           ┌──────────────────┼──────────────────┐
           ▼                  ▼                  ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│ message.proto   │  │ session.proto   │  │  group.proto    │
│ (Messages E2E)  │  │ (Double Ratchet)│  │   (Groupes)     │
│ importe: keys   │  │ importe: keys   │  │ importe: keys   │
└─────────────────┘  └─────────────────┘  └─────────────────┘
           │                  │                  │
           └──────────────────┼──────────────────┘
                              ▼
           ┌─────────────────────────────────────┐
           │           network.proto             │
           │         (Transport P2P)             │
           │   importe: message, group, keys     │
           └─────────────────────────────────────┘
                              │
                              ▼
           ┌─────────────────────────────────────┐
           │           storage.proto             │
           │        (Base de données)            │
           │ importe: session, group, keys, msg  │
           └─────────────────────────────────────┘

           ┌─────────────────────────────────────┐
           │           report.proto              │
           │         (Signalements)              │
           │          importe: keys              │
           └─────────────────────────────────────┘
```

---

## Utilisation en Rust

### Import de base

```rust
use zenth_dto::*;
```

### Création de messages

```rust
// Créer une clé d'identité
let identity_key = IdentityKey {
    key_type: SignatureKeyType::Dilithium3 as i32,
    public_key: vec![/* bytes de la clé */],
};

// Créer un PreKeyBundle
let bundle = PreKeyBundle {
    user_hash_id: user_hash.to_vec(),
    registration_id: 12345,
    identity_key: Some(identity_key),
    pre_key_id: 1,
    pre_key_public: vec![/* ... */],
    signed_pre_key_id: 1,
    signed_pre_key_public: vec![/* ... */],
    signed_pre_key_signature: vec![/* ... */],
    pq_pre_key_id: 1,
    pq_pre_key_public: vec![/* ... */],
    pq_last_resort_key_id: 0,
    pq_last_resort_key_public: vec![],
};

// Créer une requête d'inscription
let reg_request = RegistrationRequest {
    username_hash: username_hash.to_vec(),
    pre_key_bundle: Some(bundle),
    proof_type: ZkpType::Plonk as i32,
    password_commitment: commitment.to_vec(),
    initial_proof: proof.to_vec(),
    identity_key_dilithium: dilithium_pub.to_vec(),
    identity_signature: signature.to_vec(),
    timestamp: chrono::Utc::now().timestamp(),
};
```

### Sérialisation/Désérialisation

```rust
use prost::Message;

// Sérialiser
let bytes = reg_request.encode_to_vec();

// Désérialiser
let decoded = RegistrationRequest::decode(&bytes[..])?;
```

### Utilisation avec DHT

```rust
// Créer une requête DHT
let dht_request = DhtRequest {
    method: Method::Register as i32,
    payload: reg_request.encode_to_vec(),
    timestamp: chrono::Utc::now().timestamp(),
    request_id: uuid::Uuid::new_v4().as_bytes().to_vec(),
};

// Côté serveur - décoder selon la méthode
match Method::try_from(dht_request.method) {
    Ok(Method::Register) => {
        let reg = RegistrationRequest::decode(&dht_request.payload[..])?;
        // Traiter l'inscription...
    }
    Ok(Method::Login) => {
        let login = LoginRequest::decode(&dht_request.payload[..])?;
        // Traiter la connexion...
    }
    // etc.
}
```

---

## Builders Disponibles

La bibliothèque fournit des builders fluent dans `src/builders.rs`:

```rust
use zenth_dto::builders::*;

// Builder pour IdentityKey
let key = IdentityKeyBuilder::dilithium3()
    .public_key(pub_key_bytes)
    .build();

// Builder pour PreKeyBundle (prend un username, calcule le hash en interne)
let bundle = PreKeyBundleBuilder::new("alice")
    .registration_id(12345)
    .identity_key(key)
    .pre_key(1, pre_key_pub)
    .signed_pre_key(1, signed_pub, signature)
    .pq_pre_key(1, kyber_pub)
    .build();

// Builder pour AuthChallenge (challenge_id et nonce générés automatiquement)
let challenge = AuthChallengeBuilder::new()
    .proof_type(ZkpType::Plonk)
    .difficulty(128)
    .build();
```

---

## Glossaire

| Terme | Définition |
|-------|------------|
| **DTO** | Data Transfer Object - structure pour transférer des données |
| **KEM** | Key Encapsulation Mechanism - mécanisme d'encapsulation de clé |
| **Kyber** | Algorithme KEM post-quantique (NIST standardisé) |
| **Dilithium** | Algorithme de signature post-quantique (NIST standardisé) |
| **LMS** | Leighton-Micali Signature - signature basée sur hash |
| **ZKP** | Zero-Knowledge Proof - preuve à divulgation nulle |
| **PLONK/STARK/GROTH16** | Systèmes de preuves ZK |
| **Double Ratchet** | Protocole Signal pour le chiffrement forward secrecy |
| **Pre-Key** | Clé pré-publiée pour établir une session sans être en ligne |
| **Ratchet** | Mécanisme de rotation de clés |
| **DHT** | Distributed Hash Table - table de hachage distribuée |
| **E2E** | End-to-End - chiffrement de bout en bout |
| **PQ** | Post-Quantum - résistant aux ordinateurs quantiques |
| **HMAC** | Hash-based Message Authentication Code |
| **GCM** | Galois/Counter Mode - mode de chiffrement authentifié |
| **Serpent** | Algorithme de chiffrement symétrique (finaliste AES) |
| **ChaCha20** | Algorithme de chiffrement stream |
| **Poly1305** | Algorithme d'authentification de message |

---

## Licence

MIT OR Apache-2.0
