use crate::{
    dto::annotation::{
        CreateAnnotationVersionReply, CreateAnnotationVersionRequest, FormVariable,
        ListVariableByFormReply, ListVariableByFormRequest, ModifyAnnotationVersionReply,
        ModifyAnnotationVersionRequest,
    },
    errors::Result,
    repository::dto::annotation::{
        Annotation, AnnotationInForm, AnnotationVersion, CreateAnnotationReply,
        CreateAnnotationRequest, CreateFormDomainReply, CreateFormDomainRequest, FormDomain,
        ListAnnotationByFormReply, ListAnnotationByFormRequest, ListAnnotationVersionReply,
        ListAnnotationVersionRequest, ListFormDomainReply, ListFormDomainRequest,
        UpdateAnnootationReply, UpdateAnnootationRequest,
    },
};
use reqwest::Client;
use std::sync::Arc;
use url::Url;

#[derive(Clone)]
pub struct AnnotationRepository {
    client: Arc<Client>,
    base_url: Url,
}

impl AnnotationRepository {
    pub fn new<S: AsRef<str>>(client: Arc<Client>, base_url: S) -> Self {
        let base_url = Url::parse(base_url.as_ref()).expect("Invalid base URL");
        AnnotationRepository { client, base_url }
    }

    pub async fn list_annotation_version(
        &self,
        request: ListAnnotationVersionRequest,
    ) -> Result<Vec<AnnotationVersion>> {
        let url = self.base_url.join("version")?;
        let result = self.client.get(url).query(&request).send().await?;
        let result = result.json::<ListAnnotationVersionReply>().await?;
        Ok(result.data)
    }

    // pub async fn create_annotation_version(&self)

    pub async fn list_form_domains(
        &self,
        request: ListFormDomainRequest,
    ) -> Result<Vec<FormDomain>> {
        let url = self.base_url.join("domain")?;
        let result = self.client.get(url).query(&request).send().await?;
        let result = result.json::<ListFormDomainReply>().await?;
        Ok(result.data)
    }

    pub async fn create_form_domain(&self, request: CreateFormDomainRequest) -> Result<FormDomain> {
        let url = self.base_url.join("domain")?;
        let result = self.client.post(url).json(&request).send().await?;
        let result = result.json::<CreateFormDomainReply>().await?;
        Ok(result.data)
    }

    pub async fn remove_form_domain(&self, id: i32) -> Result<()> {
        let url = self.base_url.join(&format!("domain/{}", id))?;
        self.client.delete(url).send().await?;
        Ok(())
    }

    pub async fn list_variable_by_form(
        &self,
        request: ListVariableByFormRequest,
    ) -> Result<Vec<FormVariable>> {
        let url = self.base_url.join("variable")?;
        let result = self.client.get(url).query(&request).send().await?;
        let result = result.json::<ListVariableByFormReply>().await?;
        Ok(result.data)
    }

    pub async fn create_annotation_version(
        &self,
        request: &CreateAnnotationVersionRequest,
    ) -> Result<AnnotationVersion> {
        let url = self.base_url.join("version")?;
        let result = self.client.post(url).json(request).send().await?;
        let result = result.json::<CreateAnnotationVersionReply>().await?;
        Ok(result.data)
    }

    pub async fn modify_annotation_version(
        &self,
        id: i32,
        request: &ModifyAnnotationVersionRequest,
    ) -> Result<AnnotationVersion> {
        let url = self.base_url.join(&format!("version/{}", id))?;
        let result = self.client.put(url).json(request).send().await?;
        let result = result.json::<ModifyAnnotationVersionReply>().await?;
        Ok(result.data)
    }

    pub async fn remove_annotation_version(&self, id: i32) -> Result<()> {
        let url = self.base_url.join(&format!("version/{}", id))?;
        self.client.delete(url).send().await?;
        Ok(())
    }

    pub async fn list_annotation_by_form(
        &self,
        request: ListAnnotationByFormRequest,
    ) -> Result<AnnotationInForm> {
        let url = self.base_url.join("annotation")?;
        let result = self.client.get(url).query(&request).send().await?;
        let result = result.json::<ListAnnotationByFormReply>().await?;
        Ok(result.data)
    }

    pub async fn create_annotation(&self, request: CreateAnnotationRequest) -> Result<Annotation> {
        let url = self.base_url.join("annotation")?;
        let result = self.client.post(url).json(&request).send().await?;
        let result = result.json::<CreateAnnotationReply>().await?;
        Ok(result.data)
    }

    pub async fn update_annotation(
        &self,
        annotation_id: i32,
        request: UpdateAnnootationRequest,
    ) -> Result<Annotation> {
        let url = self
            .base_url
            .join(&format!("annotation/{}", annotation_id))?;
        let result = self.client.put(url).json(&request).send().await?;
        let result = result.json::<UpdateAnnootationReply>().await?;
        Ok(result.data)
    }

    pub async fn remove_annotation(&self, annotation_id: i32) -> Result<()> {
        let url = self
            .base_url
            .join(&format!("annotation/{}", annotation_id))?;
        self.client.delete(url).send().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::Result;
    use crate::repository::annotation::AnnotationRepository;
    use crate::repository::dto::annotation::{
        AnnotationKind, AnnotationLocation, CreateAnnotationRequest, CreateFormDomainRequest,
        ListAnnotationByFormRequest, ListAnnotationVersionRequest, ListFormDomainRequest,
        UpdateAnnootationRequest, VariableBinding,
    };
    use reqwest::header::{HeaderMap, HeaderValue};
    use reqwest::Client;
    use std::sync::Arc;

    #[tokio::test]
    async fn annotation_repo_test() -> Result<()> {
        let mut headers = HeaderMap::new();
        headers.insert("Compass-User", HeaderValue::from_static("yuqi01.chen"));
        let client = Client::builder().default_headers(headers).build().unwrap();
        let client = Arc::new(client);
        let base_url = "http://localhost:2217/api/sdtm/annotation/";
        let repo = AnnotationRepository::new(client, base_url);
        let project_version_id = 1;
        let versions = repo
            .list_annotation_version(ListAnnotationVersionRequest { project_version_id })
            .await?;
        assert_eq!(1, versions.len());
        let annotation_version_id = versions.first().unwrap().id;
        let form_id = 408;

        // create new domain
        let domain = repo
            .create_form_domain(CreateFormDomainRequest {
                annotation_version_id,
                form_id,
                name: "DM".into(),
                description: "Demographic".into(),
            })
            .await?;

        assert_eq!(
            1,
            repo.list_form_domains(ListFormDomainRequest {
                form_id,
                annotation_version_id,
            })
            .await?
            .len()
        );

        // create annotation
        let annotation = repo
            .create_annotation(CreateAnnotationRequest {
                annotation_version_id,
                form_id,
                variable: None,
                location: AnnotationLocation {
                    source_id: 2494,
                    kind: AnnotationKind::Item,
                },
                annotation_display: "[NOT SUBMITTED]".into(),
                assign: true,
            })
            .await?;
        let annotations = repo
            .list_annotation_by_form(ListAnnotationByFormRequest {
                form_id,
                annotation_version_id,
            })
            .await?
            .item;

        assert_eq!(1, annotations.len());
        assert_eq!(annotations.first().unwrap().variable_id, -1);

        // update annotaion
        repo.update_annotation(
            annotation.id,
            UpdateAnnootationRequest {
                variable: Some(VariableBinding {
                    domain_id: domain.id,
                    variable_name: "SUBJID".into(),
                    supp: false,
                }),
                annotation_display: "SUBJID".into(),
                assign: false,
                not_submit: false,
            },
        )
        .await?;

        let annotations = repo
            .list_annotation_by_form(ListAnnotationByFormRequest {
                form_id,
                annotation_version_id,
            })
            .await?
            .item;
        assert_eq!(1, annotations.len());
        assert_ne!(annotations.first().unwrap().variable_id, -1);

        // remove domain
        repo.remove_form_domain(domain.id).await?;

        assert_eq!(
            0,
            repo.list_annotation_by_form(ListAnnotationByFormRequest {
                form_id,
                annotation_version_id,
            })
            .await?
            .item
            .len()
        );
        Ok(())
    }
}
