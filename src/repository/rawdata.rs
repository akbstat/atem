use super::dto::rawdata::{
    CreateFormReply, CreateFormRequest, CreateItemOptionReply, CreateItemOptionRequest,
    CreateItemReply, CreateItemRequest, CreateItemUnitReply, CreateItemUnitRequest,
    GetFormByIdReply, ListFormsReply, ListFormsRequest, ListItemTypesReply, ListItemsReply,
    ListItemsRequest, ListProjectVersionReply, ListProjectVersionRequest,
};
use crate::{
    dto::rawdata::{
        CreateProjectVersionReply, CreateProjectVersionRequest, FindProjectReply,
        FindProjectRequest, ModifyProjectVersionReply, ModifyProjectVersionRequest, Project,
        ProjectVersion,
    },
    errors::Result,
};
use reqwest::Client;
use std::sync::Arc;
use url::Url;

#[derive(Clone)]
pub struct RawdataRepository {
    client: Arc<Client>,
    base_url: Url,
}

impl RawdataRepository {
    pub fn new<S: AsRef<str>>(client: Arc<Client>, base_url: S) -> Self {
        let base_url = Url::parse(base_url.as_ref()).expect("Invalid base URL");
        RawdataRepository { client, base_url }
    }

    pub async fn create_form(&self, request: &CreateFormRequest) -> Result<i32> {
        let url = self.base_url.join("form")?;
        let response = self.client.post(url).json(request).send().await?;
        let form = response.json::<CreateFormReply>().await?;
        Ok(form.data.id)
    }

    pub async fn create_item(&self, request: &CreateItemRequest) -> Result<i32> {
        let url = self.base_url.join("item")?;
        let response = self.client.post(url).json(request).send().await?;
        let item = response.json::<CreateItemReply>().await?;
        Ok(item.data.id)
    }

    pub async fn create_item_option(&self, request: &CreateItemOptionRequest) -> Result<()> {
        let url = self.base_url.join("item/option")?;
        let response = self.client.post(url).json(request).send().await?;
        response.json::<CreateItemOptionReply>().await?;
        Ok(())
    }

    pub async fn create_item_unit(&self, request: &CreateItemUnitRequest) -> Result<()> {
        let url = self.base_url.join("item/unit")?;
        let response = self.client.post(url).json(request).send().await?;
        response.json::<CreateItemUnitReply>().await?;
        Ok(())
    }

    pub async fn find_project(&self, request: &FindProjectRequest) -> Result<Option<Project>> {
        let url = self.base_url.join("project")?;
        let response = self.client.get(url).query(request).send().await?;
        let data = response.json::<FindProjectReply>().await?;
        Ok(data.data)
    }

    pub async fn create_project_version(
        &self,
        request: &CreateProjectVersionRequest,
    ) -> Result<ProjectVersion> {
        let url = self.base_url.join("version")?;
        let response = self.client.post(url).json(request).send().await?;
        let version = response.json::<CreateProjectVersionReply>().await?;
        Ok(version.data)
    }

    pub async fn modify_project_version(
        &self,
        id: i32,
        request: &ModifyProjectVersionRequest,
    ) -> Result<ProjectVersion> {
        let url = self.base_url.join(&format!("version/{}", id))?;
        let response = self.client.put(url).json(request).send().await?;
        let version = response.json::<ModifyProjectVersionReply>().await?;
        Ok(version.data)
    }

    pub async fn list_project_versions(
        &self,
        request: &ListProjectVersionRequest,
    ) -> Result<ListProjectVersionReply> {
        let url = self.base_url.join("version")?;
        let response = self.client.get(url).query(request).send().await?;
        let project_versions = response.json::<ListProjectVersionReply>().await?;
        Ok(project_versions)
    }

    pub async fn list_item_types(&self) -> Result<ListItemTypesReply> {
        let url = self.base_url.join("item/type")?;
        let response = self.client.get(url).send().await?;
        let item_types = response.json::<ListItemTypesReply>().await?;
        Ok(item_types)
    }

    pub async fn list_forms(&self, request: &ListFormsRequest) -> Result<ListFormsReply> {
        let url = self.base_url.join("form")?;
        let response = self.client.get(url).query(request).send().await?;
        let reply = response.json::<ListFormsReply>().await?;
        Ok(reply)
    }

    pub async fn get_form_by_id(&self, id: i32) -> Result<GetFormByIdReply> {
        let url = self.base_url.join(&format!("form/{id}"))?;
        let response = self.client.get(url).send().await?;
        let reply = response.json::<GetFormByIdReply>().await?;
        Ok(reply)
    }

    pub async fn list_items(&self, request: &ListItemsRequest) -> Result<ListItemsReply> {
        let url = self.base_url.join("item")?;
        let response = self.client.get(url).query(request).send().await?;
        let reply = response.json::<ListItemsReply>().await?;
        Ok(reply)
    }
}
