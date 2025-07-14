use crate::{
    dto::metadata::{
        Language, ListLanguageReply, ListSdtmDomainsReply, ListSdtmDomainsRequest,
        ListSdtmVariableReply, ListSdtmVariableRequest, ListSdtmVersionReply,
        ListSdtmVersionRequest, SdtmDomain, SdtmVariable, SdtmVersion,
    },
    errors::Result,
};
use reqwest::Client;
use std::sync::Arc;
use url::Url;

#[derive(Clone)]
pub struct MetadataRepository {
    client: Arc<Client>,
    base_url: Url,
}

impl MetadataRepository {
    pub fn new<S: AsRef<str>>(client: Arc<Client>, base_url: S) -> Self {
        let base_url = Url::parse(base_url.as_ref()).expect("Invalid base URL");
        MetadataRepository { client, base_url }
    }

    pub async fn list_languages(&self) -> Result<Vec<Language>> {
        let url = self.base_url.join("language")?;
        let response = self.client.get(url).send().await?;
        let languages = response.json::<ListLanguageReply>().await?;
        Ok(languages.data)
    }

    pub async fn list_sdtm_versions(
        &self,
        request: &ListSdtmVersionRequest,
    ) -> Result<Vec<SdtmVersion>> {
        let url = self.base_url.join("version")?;
        let response = self.client.get(url).query(request).send().await?;
        let versions = response.json::<ListSdtmVersionReply>().await?;
        Ok(versions.data)
    }

    pub async fn list_sdtm_domains(
        &self,
        request: &ListSdtmDomainsRequest,
    ) -> Result<Vec<SdtmDomain>> {
        let url = self.base_url.join("domain")?;
        let response = self.client.get(url).query(request).send().await?;
        let domians = response.json::<ListSdtmDomainsReply>().await?;
        Ok(domians.data)
    }

    pub async fn list_sdtm_variable(
        &self,
        request: &ListSdtmVariableRequest,
    ) -> Result<Vec<SdtmVariable>> {
        let url = self.base_url.join("variable")?;
        let response = self.client.get(url).query(request).send().await?;
        let variables = response.json::<ListSdtmVariableReply>().await?;
        Ok(variables.data)
    }
}
