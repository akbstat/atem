use std::{path::Path, sync::Arc};

use super::ecollect::ECollectTransport;
use crate::{errors::Result, repository::RawdataRepository};
use async_trait::async_trait;

/// read metadata from edc configuration file to save into database
#[async_trait]
pub trait EdcTransport {
    /// read edc configuration file
    async fn read(&self) -> Result<()>;
    /// save edc metadata into database
    async fn save(&self) -> Result<()>;
}

pub enum EdcKind {
    ECollect,
    Unknown,
}

pub struct TransportParam<P: AsRef<Path>> {
    pub kind: EdcKind,
    pub config_filepath: P,
    pub project_version_id: i32,
}

pub async fn save_edc<P: AsRef<Path>>(
    param: &TransportParam<P>,
    repo: Arc<RawdataRepository>,
) -> Result<()> {
    let transport = match param.kind {
        EdcKind::ECollect => ECollectTransport::new(
            param.config_filepath.as_ref(),
            repo,
            param.project_version_id,
        )?,
        EdcKind::Unknown => return Ok(()),
    };
    transport.read().await?;
    transport.save().await?;
    Ok(())
}
