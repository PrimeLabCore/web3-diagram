use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "standard")]
#[serde(rename_all = "snake_case")]
pub enum NearEvent {
    Nep171(Nep171Event),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Nep171Event {
    pub version: String,
    #[serde(flatten)]
    pub event_kind: Nep171EventKind,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum Nep171EventKind {
    NftMint(Vec<NftMintData>),
    NftTransfer(Vec<NftTransferData>),
    NftBurn(Vec<NftBurnData>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NftMintData {
    pub owner_id: String,
    pub token_ids: Vec<String>,
    pub memo: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NftTransferData {
    pub authorized_id: Option<String>,
    pub old_owner_id: String,
    pub new_owner_id: String,
    pub token_ids: Vec<String>,
    pub memo: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NftBurnData {
    pub authorized_id: Option<String>,
    pub owner_id: String,
    pub token_ids: Vec<String>,
    pub memo: Option<String>,
}

impl Display for NearEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("EVENT_JSON:{}", self.to_json_string()))
    }
}

impl NearEvent {
    pub fn new_171(version: String, event_kind: Nep171EventKind) -> Self {
        NearEvent::Nep171(Nep171Event {
            version,
            event_kind,
        })
    }

    pub fn new_171_v1(event_kind: Nep171EventKind) -> Self {
        NearEvent::new_171("1.0.0".to_string(), event_kind)
    }

    pub fn nft_burn(data: Vec<NftBurnData>) -> Self {
        NearEvent::new_171_v1(Nep171EventKind::NftBurn(data))
    }
    pub fn nft_transfer(data: Vec<NftTransferData>) -> Self {
        NearEvent::new_171_v1(Nep171EventKind::NftTransfer(data))
    }

    pub fn nft_mint(data: Vec<NftMintData>) -> Self {
        NearEvent::new_171_v1(Nep171EventKind::NftMint(data))
    }

    pub(crate) fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn log(&self) {
        near_sdk::env::log(&self.to_string().as_bytes());
    }

    pub fn log_nft_mint(owner_id: String, token_ids: Vec<String>, memo: Option<String>) {
        NearEvent::log_nft_mints(vec![NftMintData {
            owner_id,
            token_ids,
            memo,
        }]);
    }

    pub fn log_nft_mints(data: Vec<NftMintData>) {
        NearEvent::nft_mint(data).log();
    }

    pub fn log_nft_transfer(
        old_owner_id: String,
        new_owner_id: String,
        token_ids: Vec<String>,
        memo: Option<String>,
        authorized_id: Option<String>,
    ) {
        NearEvent::log_nft_transfers(vec![NftTransferData {
            authorized_id,
            old_owner_id,
            new_owner_id,
            token_ids,
            memo,
        }]);
    }

    pub fn log_nft_transfers(data: Vec<NftTransferData>) {
        NearEvent::nft_transfer(data).log();
    }

    pub fn log_nft_burn(
        owner_id: String,
        token_ids: Vec<String>,
        memo: Option<String>,
        authorized_id: Option<String>,
    ) {
        NearEvent::log_nft_burns(vec![NftBurnData {
            owner_id,
            authorized_id,
            token_ids,
            memo,
        }]);
    }

    pub fn log_nft_burns(data: Vec<NftBurnData>) {
        NearEvent::nft_burn(data).log();
    }
}
