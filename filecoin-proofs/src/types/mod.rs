use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use storage_proofs::hasher::Hasher;
use storage_proofs::porep::stacked;
use storage_proofs::post::fallback::*;
use storage_proofs::sector::*;

use crate::constants::*;
use crate::parameters::*;

mod bytes_amount;
mod piece_info;
mod porep_config;
mod porep_proof_partitions;
mod post_config;
mod post_proof_partitions;
mod sector_class;
mod sector_size;

pub use self::bytes_amount::*;
pub use self::piece_info::*;
pub use self::porep_config::*;
pub use self::porep_proof_partitions::*;
pub use self::post_config::*;
pub use self::post_proof_partitions::*;
pub use self::sector_class::*;
pub use self::sector_size::*;

pub type Commitment = [u8; 32];
pub type ChallengeSeed = [u8; 32];
pub use stacked::PersistentAux;
pub use stacked::TemporaryAux;
pub type ProverId = [u8; 32];
pub type Ticket = [u8; 32];

pub type Tree = storage_proofs::merkle::OctMerkleTree<DefaultTreeHasher>;
pub type LCTree = storage_proofs::merkle::OctLCMerkleTree<DefaultTreeHasher>;

pub use storage_proofs::porep::stacked::Labels;
pub type DataTree = storage_proofs::merkle::BinaryMerkleTree<DefaultPieceHasher>;

pub use storage_proofs::merkle::MerkleProof;
pub use storage_proofs::merkle::MerkleTreeTrait;

/// Arity for oct trees, used for comm_r_last.
pub const OCT_ARITY: usize = 8;

/// Arity for binary trees, used for comm_d.
pub const BINARY_ARITY: usize = 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealPreCommitOutput {
    pub comm_r: Commitment,
    pub comm_d: Commitment,
}

pub type VanillaSealProof<Tree> = storage_proofs::porep::stacked::Proof<Tree, DefaultPieceHasher>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SealCommitPhase1Output<Tree: MerkleTreeTrait> {
    #[serde(bound(
        serialize = "VanillaSealProof<Tree>: Serialize",
        deserialize = "VanillaSealProof<Tree>: Deserialize<'de>"
    ))]
    pub vanilla_proofs: Vec<Vec<VanillaSealProof<Tree>>>,
    pub comm_r: Commitment,
    pub comm_d: Commitment,
    pub replica_id: <Tree::Hasher as Hasher>::Domain,
    pub seed: Ticket,
    pub ticket: Ticket,
}

#[derive(Clone, Debug)]
pub struct SealCommitOutput {
    pub proof: Vec<u8>,
}

pub use merkletree::store::StoreConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct SealPreCommitPhase1Output<Tree: MerkleTreeTrait> {
    #[serde(bound(
        serialize = "Labels<Tree>: Serialize",
        deserialize = "Labels<Tree>: Deserialize<'de>"
    ))]
    pub labels: Labels<Tree>,
    pub config: StoreConfig,
    pub comm_d: Commitment,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowPoStChallenges {
    pub post_config: PoStConfig,
    pub randomness: ChallengeSeed,
    pub prover_id: ProverId,
    pub sector_challenges: BTreeMap<SectorId, Vec<u64>>,
}

pub type SnarkProof = Vec<u8>;
pub type VanillaProof<Tree> = Proof<<Tree as MerkleTreeTrait>::Proof>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowPoStVanillaProofs<Tree: MerkleTreeTrait> {
    pub vanilla_params: WindowPostSetupParams,
    pub partitions: Option<usize>,
    #[serde(bound(
        serialize = "Vec<PublicSector<<Tree::Hasher as Hasher>::Domain>>: Serialize",
        deserialize = "Vec<PublicSector<<Tree::Hasher as Hasher>::Domain>>: Deserialize<'de>"
    ))]
    pub pub_sectors: Vec<PublicSector<<Tree::Hasher as Hasher>::Domain>>,
    #[serde(bound(
        serialize = "Vec<VanillaProof<Tree>>: Serialize",
        deserialize = "Vec<VanillaProof<Tree>>: Deserialize<'de>"
    ))]
    pub vanilla_proofs: Vec<VanillaProof<Tree>>,
}
