use super::ecollect::ECollectElderTransport;
use crate::{edc::ecollect::ECollectV6Transport, errors::Result, repository::RawdataRepository};
use async_trait::async_trait;
use std::{path::Path, sync::Arc};

/// read metadata from edc configuration file to save into database
#[async_trait]
pub trait EdcTransport {
    /// read edc configuration file
    async fn read(&self) -> Result<()>;
    /// save edc metadata into database
    async fn save(&self, project_version_id: i32) -> Result<()>;
}

pub enum EdcKind {
    ECollectElder,
    ECollectV6,
    Unknown,
}

pub struct TransportParam<P: AsRef<Path>> {
    pub kind: EdcKind,
    pub config_filepath: P,
}

pub async fn new_edc_transport<P: AsRef<Path>>(
    param: &TransportParam<P>,
    repo: Arc<RawdataRepository>,
) -> Result<Option<Box<dyn EdcTransport + Send>>> {
    match param.kind {
        EdcKind::ECollectElder => Ok(Some(Box::new(ECollectElderTransport::new(
            param.config_filepath.as_ref(),
            repo,
        )?))),
        EdcKind::ECollectV6 => Ok(Some(Box::new(ECollectV6Transport::new(
            param.config_filepath.as_ref(),
            repo,
        )?))),
        EdcKind::Unknown => Ok(None),
    }
}
