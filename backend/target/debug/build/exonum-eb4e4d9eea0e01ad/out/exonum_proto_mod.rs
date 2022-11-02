pub mod proofs ; pub mod blockchain ; pub mod lifecycle ; pub mod base ; pub mod errors ; pub mod auth ; pub mod key_value_sequence ; pub mod messages ; # [doc = r" Original proto files which were be used to generate this module."] # [doc = r" First element in tuple is file name, second is proto file content."] # [allow (dead_code)] # [allow (clippy :: unseparated_literal_suffix)] pub const PROTO_SOURCES : [(& str , & str) ; 8usize] = [("exonum/key_value_sequence.proto" , "// Copyright 2020 The Exonum Team\n//\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n//   http://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n\n// Storage with keys and values represented as bytes arrays.\n\nsyntax = \"proto3\";\n\npackage exonum;\n\noption java_package = \"com.exonum.messages.core\";\n\n// Some non-scalar key-value pair.\nmessage KeyValue {\n  string key = 1;\n  bytes value = 2;\n}\n\n// A sequence of key-value pairs.\nmessage KeyValueSequence {\n  repeated KeyValue entries = 1;\n}\n") , ("exonum/runtime/base.proto" , "// Copyright 2020 The Exonum Team\n//\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n//   http://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n\nsyntax = \"proto3\";\n\npackage exonum.runtime;\n\noption java_package = \"com.exonum.messages.core.runtime\";\n\n// Unique service transaction identifier.\nmessage CallInfo {\n  // Unique service instance identifier. The dispatcher uses this identifier to\n  // find the corresponding runtime to execute a transaction.\n  uint32 instance_id = 1;\n  // Identifier of the method in the service interface required for the call.\n  uint32 method_id = 2;\n}\n\n// Transaction with the information required to dispatch it to a service.\nmessage AnyTx {\n  // Information required for the call of the corresponding executor.\n  CallInfo call_info = 1;\n  // Serialized transaction arguments.\n  bytes arguments = 2;\n}\n\n// The artifact identifier is required to construct service instances.\n// In other words, an artifact identifier is similar to a class name,\n// and a specific service instance is similar to a class instance.\nmessage ArtifactId {\n  // Runtime identifier.\n  uint32 runtime_id = 1;\n  // Artifact name.\n  string name = 2;\n  // Semantic version of the artifact.\n  string version = 3;\n}\n\n// Exhaustive artifact specification. This information is enough\n// to deploy an artifact.\nmessage ArtifactSpec {\n  // Information uniquely identifying the artifact.\n  ArtifactId artifact = 1;\n  // Runtime-specific artifact payload.\n  bytes payload = 2;\n}\n\n// Exhaustive service instance specification.\nmessage InstanceSpec {\n  // Unique numeric ID of the service instance.\n  //\n  // Exonum assigns an ID to the service on instantiation. It is mainly used\n  // to route transaction messages belonging to this instance.\n  uint32 id = 1;\n  // Unique name of the service instance.\n  //\n  // The name serves as a primary identifier of this service in most operations.\n  // It is assigned by the network administrators.\n  //\n  // The name must correspond to the following regular expression: `[a-zA-Z0-9/\\:-_]+`.\n  string name = 2;\n  // Identifier of the corresponding artifact.\n  ArtifactId artifact = 3;\n}\n") , ("exonum/proofs.proto" , "// Copyright 2020 The Exonum Team\n//\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n//   http://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n\nsyntax = \"proto3\";\n\npackage exonum;\n\noption java_package = \"com.exonum.messages.core\";\n\nimport \"exonum/blockchain.proto\";\nimport \"exonum/messages.proto\";\nimport \"exonum/proof/map_proof.proto\";\n\n// Block with its `Precommit` messages.\n//\n// This structure contains enough information to prove the correctness of\n// a block. It consists of the block itself and the `Precommit`\n// messages related to this block.\nmessage BlockProof {\n  // Block header containing such information as the ID of the node which\n  // proposed the block, the height of the block, the number of transactions\n  // in the block, etc.\n  Block block = 1;\n  // List of `Precommit` messages for the block.\n  repeated SignedMessage precommits = 2;\n}\n\n// Proof of authenticity for a single index within the database.\nmessage IndexProof {\n  // Proof of authenticity for the block header.\n  BlockProof block_proof = 1;\n  // Proof of authenticity for the index. Must contain a single key - a full index name\n  // in the form `$service_name.$name_within_service`, e.g., `cryptocurrency.wallets`.\n  // The root hash of the proof must be equal to the `state_hash` mentioned in `block_proof`.\n  proof.MapProof index_proof = 2;\n}\n\n// Proof of authenticity for a single top-level call in a block, such as a transaction.\nmessage CallProof {\n  // Proof of authenticity for the block header.\n  BlockProof block_proof = 1;\n  // Proof from the error aggregator (i.e., a `ProofMapIndex` the Merkle root\n  // of which is recorded in the block header as `error_hash`).\n  proof.MapProof call_proof = 2;\n  // Human-readable description of an error if the call status is erroneous.\n  string error_description = 3;\n}\n") , ("exonum/messages.proto" , "// Copyright 2020 The Exonum Team\n//\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n//   http://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n\n// The messages collection for the default Exonum consensus implementation.\n\nsyntax = \"proto3\";\n\npackage exonum;\n\noption java_package = \"com.exonum.messages.core\";\n\nimport \"exonum/crypto/types.proto\";\nimport \"exonum/runtime/base.proto\";\nimport \"google/protobuf/timestamp.proto\";\n\n// Container for the signed messages.\nmessage SignedMessage {\n  // Payload of the message as a serialized `ExonumMessage`.\n  bytes payload = 1;\n  // Public key of the author of the message.\n  exonum.crypto.PublicKey author = 2;\n  // Digital signature over the payload created with a secret key of the author of the message.\n  exonum.crypto.Signature signature = 3;\n}\n\n// Subset of Exonum messages defined in the Exonum core.\nmessage CoreMessage {\n  oneof kind {\n    // Transaction message.\n    exonum.runtime.AnyTx any_tx = 1;\n    // Precommit (block endorsement) message.\n    Precommit precommit = 2;\n  }\n}\n\n// Pre-commit for a block, essentially meaning that a validator node endorses the block.\nmessage Precommit {\n  // ID of the validator endorsing the block.\n  uint32 validator = 1;\n  // The consensus algorithm epoch to which the message is related.\n  uint64 epoch = 2;\n  // The round to which the message is related.\n  uint32 round = 3;\n  // Hash of the block proposal. Note that the proposal format is not defined by the core.\n  exonum.crypto.Hash propose_hash = 4;\n  // Hash of the new block.\n  exonum.crypto.Hash block_hash = 5;\n  // Local time of the validator node when the `Precommit` was created.\n  google.protobuf.Timestamp time = 6;\n}\n") , ("exonum/blockchain.proto" , "// Copyright 2020 The Exonum Team\n//\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n//   http://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n\nsyntax = \"proto3\";\n\npackage exonum;\n\noption java_package = \"com.exonum.messages.core\";\n\nimport \"exonum/crypto/types.proto\";\nimport \"exonum/key_value_sequence.proto\";\n\n// Extensible set of additional headers, represented\n// as a sequence of key-value pairs.\nmessage AdditionalHeaders {\n  KeyValueSequence headers = 1;\n}\n\nmessage Block {\n  uint32 proposer_id = 1;\n  uint64 height = 2;\n  uint32 tx_count = 3;\n  exonum.crypto.Hash prev_hash = 4;\n  exonum.crypto.Hash tx_hash = 5;\n  exonum.crypto.Hash state_hash = 6;\n  exonum.crypto.Hash error_hash = 7;\n  AdditionalHeaders additional_headers = 8;\n}\n\nmessage TxLocation {\n  uint64 block_height = 1;\n  uint32 position_in_block = 2;\n}\n\n// Location of an isolated call within a block.\nmessage CallInBlock {\n  oneof call {\n    // Call of a transaction within the block. The value is the zero-based\n    // transaction index.\n    uint32 transaction = 1;\n    // Call of `before_transactions` hook in a service. The value is\n    // the service identifier.\n    uint32 before_transactions = 2;\n    // Call of `after_transactions` hook in a service. The value is\n    // the service identifier.\n    uint32 after_transactions = 3;\n  }\n}\n\n// Consensus configuration parameters\n\n// Public keys of a validator.\nmessage ValidatorKeys {\n  // Consensus key is used for messages related to the consensus algorithm.\n  exonum.crypto.PublicKey consensus_key = 1;\n  // Service key is used for services, for example, the configuration\n  // updater service, the anchoring service, etc.\n  exonum.crypto.PublicKey service_key = 2;\n}\n\n// Consensus algorithm parameters.\nmessage Config {\n  // List of validators public keys.\n  repeated ValidatorKeys validator_keys = 1;\n  // Interval between first two rounds.\n  uint64 first_round_timeout = 2;\n  // Period of sending a Status message.\n  uint64 status_timeout = 3;\n  // Peer exchange timeout.\n  uint64 peers_timeout = 4;\n  // Maximum number of transactions per block.\n  uint32 txs_block_limit = 5;\n  // Maximum message length (in bytes).\n  uint32 max_message_len = 6;\n  // Minimal propose timeout.\n  uint64 min_propose_timeout = 7;\n  // Maximal propose timeout.\n  uint64 max_propose_timeout = 8;\n  // Amount of transactions in pool to start use `min_propose_timeout`.\n  uint32 propose_timeout_threshold = 9;\n}\n") , ("exonum/runtime/lifecycle.proto" , "// Copyright 2020 The Exonum Team\n//\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n//   http://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n\nsyntax = \"proto3\";\n\npackage exonum.runtime;\n\noption java_package = \"com.exonum.messages.core.runtime\";\n\nimport \"exonum/blockchain.proto\";\nimport \"exonum/crypto/types.proto\";\nimport \"exonum/runtime/base.proto\";\n\n// Data that is required for initialization of a service instance.\nmessage InstanceInitParams {\n  // Instance specification.\n  InstanceSpec instance_spec = 1;\n  // Constructor argument for the instance.\n  bytes constructor = 2;\n}\n\n// Genesis config parameters.\n//\n// Information from this entity get saved to the genesis block.\nmessage GenesisConfig {\n  // Blockchain configuration used to create the genesis block.\n  exonum.Config consensus_config = 1;\n  // Artifact specification of the built-in services.\n  repeated ArtifactSpec artifacts = 2;\n  // List of services with their configuration parameters that are created directly\n  // in the genesis block.\n  repeated InstanceInitParams builtin_instances = 3;\n}\n\n// Current state of the artifact in dispatcher.\nmessage ArtifactState {\n  // Status of an artifact deployment.\n  enum Status {\n    // The artifact is pending unload.\n    UNLOADING = 0;\n    // The artifact is pending deployment.\n    DEPLOYING = 1;\n    // The artifact has been successfully deployed.\n    ACTIVE = 2;\n  }\n\n  // Runtime-specific artifact specification.\n  bytes deploy_spec = 1;\n  // Artifact deployment status.\n  Status status = 2;\n}\n\nmessage InstanceStatus {\n  enum Simple {\n    // The service instance has no status, i.e. this value corresponds to\n    // `Option::None` on the Rust code level and other corresponds to\n    // `Some(...)`.\n    NONE = 0;\n    // The service instance is active.\n    ACTIVE = 1;\n    // The service instance is stopped.\n    STOPPED = 2;\n    // The service instance is frozen; it can process read-only requests,\n    // but not transactions or `before_transactions` / `after_transactions` hooks.\n    FROZEN = 3;\n  }\n\n  oneof status {\n    // Service has a status from the `Simple` enum.\n    Simple simple = 1;\n    // Service is in process of migration.\n    InstanceMigration migration = 2;\n  }\n}\n\nmessage InstanceMigration {\n  // Migration target to obtain migration scripts from. This artifact\n  // must be deployed on the blockchain.\n  ArtifactId target = 1;\n  // Version of the instance data after the migration is completed.\n  // Note that it does not necessarily match the version of `target`,\n  // but should be not greater.\n  string end_version = 2;\n  // Consensus-wide outcome of the migration, in the form of\n  // the aggregation hash of the migrated data.\n  // The lack of value signifies that the migration is not finished yet.\n  exonum.crypto.Hash completed_hash = 3;\n}\n\n// Current state of service instance in dispatcher.\nmessage InstanceState {\n  // Service instance specification.\n  InstanceSpec spec = 1;\n  // Service instance activity status.\n  //\n  // Status can be `NONE` only during the block execution if instance was created,\n  // but activation routine for it is not yet completed, and this value can occur no more\n  // than once in a service lifetime.\n  //\n  // If this field is set to `NONE`, the pending_status must have value `ACTIVE`.\n  InstanceStatus status = 2;\n  // Pending status of the instance.\n  //\n  // Pending state can be not `NONE` if core is in process of changing service status,\n  // e.g. service initialization, resuming or migration. If this field was set to value\n  // other than `NONE`, it always will be reset to `NONE` in the next block.\n  //\n  // The purpose of this field is to keep information about further service status during the\n  // block execution because the service status can be changed only after that block is\n  // committed. This approach is needed because there is no guarantee that the executed\n  // block will be committed.\n  InstanceStatus pending_status = 3;\n  // Version of the service data. The empty value means that the data version\n  // is the same as the `spec.artifact`. Non-empty value means that one or more\n  // data migrations have been performed on the service, so that the service data\n  // is compatible with a newer artifact.\n  string data_version = 4;\n}\n\n// Local result of a migration.\nmessage MigrationStatus {\n  oneof result {\n    // Hash of the successfully migrated data.\n    exonum.crypto.Hash hash = 1;\n    // Human-readable description of an error that has occurred\n    // during migration.\n    string error = 2;\n  }\n}\n") , ("exonum/runtime/auth.proto" , "// Copyright 2020 The Exonum Team\n//\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n//   http://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n\nsyntax = \"proto3\";\n\npackage exonum.runtime;\n\noption java_package = \"com.exonum.messages.core.runtime\";\n\nimport \"exonum/crypto/types.proto\";\nimport \"google/protobuf/empty.proto\";\n\n// The authorization information for a call to the service.\nmessage Caller {\n  oneof caller {\n    // The caller is identified by the specified Ed25519 public key.\n    exonum.crypto.PublicKey transaction_author = 1;\n    // The call is invoked with the authority of a blockchain service\n    // with the specified identifier.\n    uint32 instance_id = 2;\n    // The call is invoked by one of the blockchain lifecycle events.\n    google.protobuf.Empty blockchain = 3;\n  }\n}\n") , ("exonum/runtime/errors.proto" , "// Copyright 2020 The Exonum Team\n//\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n//   http://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n\nsyntax = \"proto3\";\n\npackage exonum.runtime;\n\noption java_package = \"com.exonum.messages.core.runtime\";\n\nimport \"google/protobuf/empty.proto\";\n\n// The type of ExecutionError.\n// Note that ErrorKind isn't a primary source for determining the origin of the error.\n// First, see the 'runtime_id' and 'call_site' fields of ExecutionError, and only then\n// you can rely on `ErrorKind` to resolve ambiguity.\nenum ErrorKind {\n  // An unexpected error which does not have a specific cause.\n  // The description of the error is the only source of information about this kind of errors.\n  UNEXPECTED = 0;\n  // An error specific to the core. See core error codes for details.\n  CORE = 1;\n  // An error specific to the certain runtime. See error codes of corresponding runtime for details.\n  RUNTIME = 2;\n  // An error specific to the certain service. See error codes of corresponding service for details.\n  SERVICE = 3;\n  // A common error which can occur in different contexts. See common error codes for details.\n  COMMON = 4;\n}\n\n// Result of unsuccessful runtime execution.\n//\n// `ExecutionError` message provides the information about the source of the error.\n// The source of error is determined as following:\n//\n// - If both runtime ID and call site are set, then error is related to the service code.\n// - If runtime ID is set, and call site is not set, then error is related to the runtime code.\n// - If none of runtime ID and call site is set, then error originates in the core code.\n//\n// Option with set call site and unset runtime ID is not valid, receiving a message\n// with such a combination means receiving a malformed message.\n//\n// Though in most cases `runtime_id` and `call_site` are enough to deduce the source of error,\n// 'ErrorKind' type can be used to resolve ambiguity.\nmessage ExecutionError {\n  // The kind of error that indicates its type.\n  ErrorKind kind = 1;\n  // User defined error code that can have different meanings for the different\n  // error kinds.\n  uint32 code = 2;\n  // Optional error description. When one verifies proof of error authenticity,\n  // the description should not be included into error serialization.\n  string description = 3;\n\n  // Runtime ID will be set if the error is related to the certain runtime.\n  oneof runtime {\n    // Identifier of runtime associated with the error.\n    uint32 runtime_id = 4;\n    // There was no runtime to process an erroneous call.\n    google.protobuf.Empty no_runtime_id = 5;\n  }\n\n  // Call site will be set if the error is related to the certain service.\n  oneof call_info {\n    // Information about service call associated with the error.\n    CallSite call_site = 6;\n    // There was no service to process an erroneous call.\n    google.protobuf.Empty no_call_site = 7;\n  }\n}\n\n// Additional details about an `ExecutionError` that do not influence\n// blockchain state hash.\nmessage ExecutionErrorAux {\n  // Human-readable error description.\n  string description = 1;\n}\n\n// Call site associated with an error.\nmessage CallSite {\n  enum Type {\n    // Service method.\n    METHOD = 0;\n    // Service constructor.\n    CONSTRUCTOR = 1;\n    // Hook executing before processing transactions in a block.\n    BEFORE_TRANSACTIONS = 2;\n    // Hook executing after processing transactions in a block.\n    AFTER_TRANSACTIONS = 3;\n    // Service resuming routine.\n    RESUME = 4;\n  }\n\n  // Type of the call.\n  Type call_type = 1;\n  // Identifier of the service being called.\n  uint32 instance_id = 2;\n  // Numeric ID of the method. Set only for `call_type == METHOD`.\n  uint32 method_id = 3;\n  // Name of the interface defining the method. This field is empty for the\n  // default service interface. Set only for `call_type == METHOD`.\n  string interface = 4;\n}\n\n// Result of runtime execution.\nmessage ExecutionStatus {\n  oneof result {\n    // Successful execution.\n    google.protobuf.Empty ok = 1;\n    // Execution ended with an error.\n    ExecutionError error = 2;\n  }\n}\n") ,] ; # [doc = r" Original proto files which were be used to generate this module."] # [doc = r" First element in tuple is file name, second is proto file content."] # [allow (dead_code)] # [allow (clippy :: unseparated_literal_suffix)] pub const INCLUDES : [(& str , & str) ; 4usize] = [("exonum/common/bit_vec.proto" , "// Copyright 2020 The Exonum Team\n//\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n//   http://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n\nsyntax = \"proto3\";\n\npackage exonum.common;\n\noption java_package = \"com.exonum.messages.common\";\n\n// Vector of bits.\nmessage BitVec {\n  // Buffer containing the bits. Most significant bits of each byte\n  // have lower indexes in the vector.\n  bytes data = 1;\n  // Number of bits.\n  uint64 len = 2;\n}\n") , ("exonum/proof/map_proof.proto" , "// Copyright 2020 The Exonum Team\n//\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n//   http://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n\nsyntax = \"proto3\";\nimport \"exonum/crypto/types.proto\";\nimport \"google/protobuf/empty.proto\";\n\npackage exonum.proof;\n\noption java_package = \"com.exonum.messages.proof\";\n\n// Subset of map elements coupled with a proof. MapProof can assert existence/absence of certain keys\n// from the underlying map.\nmessage MapProof {\n  // Array with 2 kinds of objects: `{ key, no_value }` for keys missing from\n  // the underlying index, and `{ key, value }` for key-value pairs, existence of\n  // which is asserted by the proof.\n  repeated OptionalEntry entries = 1;\n  // Array of { path: ProofPath, hash: Hash } objects.\n  repeated MapProofEntry proof = 2;\n}\n\n// Key with corresponding value or an empty value if the key is missing\n// from the underlying map.\nmessage OptionalEntry {\n  // Key serialized as per `BinaryValue` implementation (usually as\n  // a Protobuf message, except for primitive types).\n  bytes key = 1;\n  oneof maybe_value {\n    // Value serialized per `BinaryValue` implementation (usually as\n    // a Protobuf message, except for primitive types).\n    bytes value = 2;\n    // Indicator that `key` is missing from the underlying map.\n    google.protobuf.Empty no_value = 3;\n  }\n}\n\n// Path to an intermediate Merkle Patricia tree node and a corresponding\n// hash value.\nmessage MapProofEntry {\n  // Path to the node, expressed with the minimum necessary number of bytes.\n  // Bits within each byte are indexed from the least significant to\n  // the most significant.\n  // The last byte may be padded with zeros if necessary.\n  bytes path = 1;\n  // Hash associated with the node.\n  exonum.crypto.Hash hash = 2;\n  // Number of zero bit padding at the end of the path. Must be in the `0..8`\n  // interval.\n  uint32 path_padding = 3;\n}\n") , ("exonum/crypto/types.proto" , "// Copyright 2020 The Exonum Team\n//\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n//   http://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n\nsyntax = \"proto3\";\n\npackage exonum.crypto;\n\noption java_package = \"com.exonum.messages.crypto\";\n\nmessage Hash { bytes data = 1; }\n\nmessage PublicKey { bytes data = 1; }\n\nmessage Signature { bytes data = 1; }\n") , ("exonum/proof/list_proof.proto" , "// Copyright 2020 The Exonum Team\n//\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n//   http://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n\nsyntax = \"proto3\";\n\nimport \"exonum/crypto/types.proto\";\n\npackage exonum.proof;\n\noption java_package = \"com.exonum.messages.proof\";\n\n// Subset of ProofList elements coupled with a proof. ListProof` can assert existence of\n// certain elements and that the list is shorter than the requested range of indexes.\nmessage ListProof {\n  // Array of { ProofListKey, Hash } objects.\n  repeated HashedEntry proof = 1;\n  // Array with list elements and their indexes.\n  repeated ListProofEntry entries = 2;\n  // Length of the underlying `ProofListIndex`.\n  uint64 length = 3;\n}\n\n// Represents list key and corresponding hash value.\nmessage HashedEntry {\n  // Location of the node within the Merkle tree.\n  ProofListKey key = 1;\n  // Hash associated with the node.\n  exonum.crypto.Hash hash = 2;\n}\n\n// Index of the list element and its value.\nmessage ListProofEntry {\n  // Zero-based index of the element.\n  uint64 index = 1;\n  // Value serialized per `BinaryValue` implementation (usually as\n  // a Protobuf message, except for primitive types).\n  bytes value = 2;\n}\n\n// Node position in the Merkle tree.\nmessage ProofListKey {\n  // Zero-based index of the node on the level.\n  uint64 index = 1;\n  // Height of the element. Should always be greater than 0.\n  // 1 corresponds to the hashes of single elements, 2 to hashes\n  // obtained by hashing together pairs of hashes at height 1, etc.\n  uint32 height = 2;\n}\n") ,] ;