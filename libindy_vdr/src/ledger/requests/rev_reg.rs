use ursa::cl::{
    RevocationRegistry as CryptoRevocationRegistry,
    RevocationRegistryDelta as CryptoRevocationRegistryDelta,
};

use super::constants::{GET_REVOC_REG, GET_REVOC_REG_DELTA, REVOC_REG_ENTRY};
use super::identifiers::rev_reg::RevocationRegistryId;
use super::rev_reg_def::RegistryType;
use super::{get_sp_key_marker, ProtocolVersion, RequestType};
use crate::common::error::prelude::*;
use crate::utils::validation::Validatable;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "ver")]
pub enum RevocationRegistry {
    #[serde(rename = "1.0")]
    RevocationRegistryV1(RevocationRegistryV1),
}

impl From<RevocationRegistry> for RevocationRegistryV1 {
    fn from(rev_reg: RevocationRegistry) -> Self {
        match rev_reg {
            RevocationRegistry::RevocationRegistryV1(rev_reg) => rev_reg,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "ver")]
pub enum RevocationRegistryDelta {
    #[serde(rename = "1.0")]
    RevocationRegistryDeltaV1(RevocationRegistryDeltaV1),
}

impl From<RevocationRegistryDelta> for RevocationRegistryDeltaV1 {
    fn from(rev_reg_delta: RevocationRegistryDelta) -> Self {
        match rev_reg_delta {
            RevocationRegistryDelta::RevocationRegistryDeltaV1(rev_reg_delta) => rev_reg_delta,
        }
    }
}

impl Validatable for RevocationRegistryDelta {}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevocationRegistryV1 {
    pub value: CryptoRevocationRegistry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevocationRegistryDeltaV1 {
    pub value: CryptoRevocationRegistryDelta,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RevRegEntryOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub revoc_reg_def_id: RevocationRegistryId,
    pub revoc_def_type: String,
    pub value: CryptoRevocationRegistryDelta,
}

impl RevRegEntryOperation {
    pub fn new(
        rev_def_type: &RegistryType,
        revoc_reg_def_id: &RevocationRegistryId,
        value: RevocationRegistryDeltaV1,
    ) -> RevRegEntryOperation {
        RevRegEntryOperation {
            _type: Self::get_txn_type().to_string(),
            revoc_def_type: rev_def_type.to_str().to_string(),
            revoc_reg_def_id: revoc_reg_def_id.clone(),
            value: value.value,
        }
    }
}

impl RequestType for RevRegEntryOperation {
    fn get_txn_type<'a>() -> &'a str {
        REVOC_REG_ENTRY
    }
}

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRevRegOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub revoc_reg_def_id: RevocationRegistryId,
    pub timestamp: i64,
}

impl GetRevRegOperation {
    pub fn new(revoc_reg_def_id: &RevocationRegistryId, timestamp: i64) -> GetRevRegOperation {
        GetRevRegOperation {
            _type: Self::get_txn_type().to_string(),
            revoc_reg_def_id: revoc_reg_def_id.clone(),
            timestamp,
        }
    }
}

impl RequestType for GetRevRegOperation {
    fn get_txn_type<'a>() -> &'a str {
        GET_REVOC_REG
    }

    fn get_sp_key(&self, protocol_version: ProtocolVersion) -> VdrResult<Option<Vec<u8>>> {
        let marker = get_sp_key_marker(6, protocol_version);
        Ok(Some(
            format!("{}:{}", marker, self.revoc_reg_def_id.to_string())
                .as_bytes()
                .to_vec(),
        ))
    }

    fn get_sp_timestamps(&self) -> VdrResult<(Option<u64>, Option<u64>)> {
        Ok((None, Some(std::cmp::max(0, self.timestamp) as u64)))
    }
}

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRevRegDeltaOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub revoc_reg_def_id: RevocationRegistryId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    pub to: i64,
}

impl GetRevRegDeltaOperation {
    pub fn new(
        revoc_reg_def_id: &RevocationRegistryId,
        from: Option<i64>,
        to: i64,
    ) -> GetRevRegDeltaOperation {
        GetRevRegDeltaOperation {
            _type: Self::get_txn_type().to_string(),
            revoc_reg_def_id: revoc_reg_def_id.clone(),
            from,
            to,
        }
    }
}

impl RequestType for GetRevRegDeltaOperation {
    fn get_txn_type<'a>() -> &'a str {
        GET_REVOC_REG_DELTA
    }

    fn get_sp_key(&self, protocol_version: ProtocolVersion) -> VdrResult<Option<Vec<u8>>> {
        let marker = get_sp_key_marker(if self.from.is_some() { 6 } else { 5 }, protocol_version);
        Ok(Some(
            format!("{}:{}", marker, self.revoc_reg_def_id.to_string())
                .as_bytes()
                .to_vec(),
        ))
    }

    fn get_sp_timestamps(&self) -> VdrResult<(Option<u64>, Option<u64>)> {
        Ok((
            self.from.map(|ts| std::cmp::max(0, ts) as u64),
            Some(std::cmp::max(0, self.to) as u64),
        ))
    }
}
