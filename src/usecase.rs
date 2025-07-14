use crate::{
    dto::{
        annotation::{
            Annotation, AnnotationVersion, CreateAnnotationRequest, CreateAnnotationVersionRequest,
            CreateFormDomainRequest, FormDomain, FormVariable, ListAnnotationVersionRequest,
            ListFormDomainRequest, ListVariableByFormRequest, ModifyAnnotationVersionRequest,
            UpdateAnnootationRequest,
        },
        metadata::{
            Language, ListSdtmDomainsRequest, ListSdtmVariableRequest, ListSdtmVersionRequest,
            SdtmDomain, SdtmVariable, SdtmVersion,
        },
        rawdata::{
            CreateEDCVersionRequest, CreateProjectVersionRequest, FindProjectRequest,
            ModifyProjectVersionRequest,
        },
    },
    edc::{save_edc, TransportParam},
    entity::annotation::AnnotationCollection,
    errors::Result,
    repository::{
        dto::{
            annotation::ListAnnotationByFormRequest,
            rawdata::{
                GetFormByIdReply, ListFormsReply, ListFormsRequest, ListItemsReply,
                ListItemsRequest, ListProjectVersionReply, ListProjectVersionRequest,
            },
        },
        AnnotationRepository, MetadataRepository, RawdataRepository,
    },
    EdcKind,
};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::{path::Path, sync::Arc};

pub struct AtemUsecase {
    rawdata: RawdataRepository,
    annotation: AnnotationRepository,
    metadata: MetadataRepository,
}

pub struct AtemUsecaseParam {
    pub base_url: String,
    pub user: String,
}

impl AtemUsecase {
    pub fn new(param: &AtemUsecaseParam) -> Self {
        let AtemUsecaseParam { user, base_url } = param;
        let mut headers = HeaderMap::new();
        headers.insert("Compass-User", HeaderValue::from_str(user).unwrap());
        let client = Arc::new(
            Client::builder()
                .default_headers(headers)
                .build()
                .expect("Failed to build AtemUsecase Rest Client"),
        );
        let rawdata =
            RawdataRepository::new(Arc::clone(&client), format!("{}/sdtm/rawdata/", base_url));
        let annotation = AnnotationRepository::new(
            Arc::clone(&client),
            format!("{}/sdtm/annotation/", base_url),
        );
        let metadata =
            MetadataRepository::new(Arc::clone(&client), format!("{}/sdtm/metadata/", base_url));
        AtemUsecase {
            rawdata,
            annotation,
            metadata,
        }
    }

    pub async fn save_edc<P: AsRef<Path>>(&self, param: TransportParam<P>) -> Result<()> {
        let repo = Arc::new(self.rawdata.clone());
        save_edc(&param, repo).await?;
        Ok(())
    }

    pub async fn list_project_version(
        &self,
        request: &ListProjectVersionRequest,
    ) -> Result<ListProjectVersionReply> {
        let project_versions = self.rawdata.list_project_versions(request).await?;
        Ok(project_versions)
    }

    pub async fn create_project_version(&self, request: &CreateEDCVersionRequest) -> Result<()> {
        if let Some(data) = self
            .rawdata
            .find_project(&FindProjectRequest {
                product: request.product.to_owned(),
                trial: request.trial.to_owned(),
            })
            .await?
        {
            let version = self
                .rawdata
                .create_project_version(&CreateProjectVersionRequest {
                    project_id: data.id,
                    name: request.version_name.to_owned(),
                })
                .await?;
            let kind = match request.edc_kind {
                1 => EdcKind::ECollect,
                _ => return Ok(()),
            };
            save_edc(
                &TransportParam {
                    kind,
                    config_filepath: Path::new(&request.edc_filepath),
                    project_version_id: version.id,
                },
                Arc::new(self.rawdata.clone()),
            )
            .await?;
        }
        Ok(())
    }

    pub async fn modify_project_version(
        &self,
        id: i32,
        request: &ModifyProjectVersionRequest,
    ) -> Result<()> {
        self.rawdata.modify_project_version(id, request).await?;
        Ok(())
    }

    pub async fn list_forms(&self, request: &ListFormsRequest) -> Result<ListFormsReply> {
        let forms = self.rawdata.list_forms(request).await?;
        Ok(forms)
    }

    pub async fn list_items(&self, request: &ListItemsRequest) -> Result<ListItemsReply> {
        let items = self.rawdata.list_items(request).await?;
        Ok(items)
    }

    pub async fn get_form_by_id(&self, id: i32) -> Result<GetFormByIdReply> {
        let form = self.rawdata.get_form_by_id(id).await?;
        Ok(form)
    }

    pub async fn list_annotation_version(
        &self,
        request: ListAnnotationVersionRequest,
    ) -> Result<Vec<AnnotationVersion>> {
        self.annotation.list_annotation_version(request).await
    }

    pub async fn create_annotation_version(
        &self,
        request: &CreateAnnotationVersionRequest,
    ) -> Result<()> {
        self.annotation.create_annotation_version(request).await?;
        Ok(())
    }

    pub async fn modify_annotation_version(
        &self,
        id: i32,
        request: &ModifyAnnotationVersionRequest,
    ) -> Result<()> {
        self.annotation
            .modify_annotation_version(id, request)
            .await?;
        Ok(())
    }

    pub async fn list_annotation_by_form(
        &self,
        request: ListAnnotationByFormRequest,
    ) -> Result<AnnotationCollection> {
        let variables = self
            .list_variable_list(ListVariableByFormRequest {
                annotation_version_id: request.annotation_version_id,
                form_id: request.form_id,
            })
            .await?;
        let annotations = self.annotation.list_annotation_by_form(request).await?;
        Ok(AnnotationCollection::build(annotations, variables))
    }

    pub async fn create_annotation(&self, request: CreateAnnotationRequest) -> Result<Annotation> {
        self.annotation.create_annotation(request).await
    }

    pub async fn update_annotation(
        &self,
        annotation_id: i32,
        request: UpdateAnnootationRequest,
    ) -> Result<Annotation> {
        self.annotation
            .update_annotation(annotation_id, request)
            .await
    }

    pub async fn remove_annotation(&self, annotation_id: i32) -> Result<()> {
        self.annotation.remove_annotation(annotation_id).await
    }

    pub async fn list_form_domains(
        &self,
        request: ListFormDomainRequest,
    ) -> Result<Vec<FormDomain>> {
        self.annotation.list_form_domains(request).await
    }

    pub async fn create_form_domain(&self, request: CreateFormDomainRequest) -> Result<FormDomain> {
        self.annotation.create_form_domain(request).await
    }

    pub async fn remove_form_domain(&self, id: i32) -> Result<()> {
        self.annotation.remove_form_domain(id).await
    }

    pub async fn list_languages(&self) -> Result<Vec<Language>> {
        self.metadata.list_languages().await
    }

    pub async fn list_sdtm_version(
        &self,
        request: ListSdtmVersionRequest,
    ) -> Result<Vec<SdtmVersion>> {
        self.metadata.list_sdtm_versions(&request).await
    }

    pub async fn list_sdtm_domains(
        &self,
        request: ListSdtmDomainsRequest,
    ) -> Result<Vec<SdtmDomain>> {
        self.metadata.list_sdtm_domains(&request).await
    }

    pub async fn list_sdtm_variables(
        &self,
        request: ListSdtmVariableRequest,
    ) -> Result<Vec<SdtmVariable>> {
        self.metadata.list_sdtm_variable(&request).await
    }

    async fn list_variable_list(
        &self,
        request: ListVariableByFormRequest,
    ) -> Result<Vec<FormVariable>> {
        self.annotation.list_variable_by_form(request).await
    }
}
