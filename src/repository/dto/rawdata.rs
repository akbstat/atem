use serde::{Deserialize, Serialize};

pub struct CreateProjectRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindProjectRequest {
    pub product: String,
    pub trial: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindProjectReply {
    pub data: Option<Project>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectVersionRequest {
    pub project_id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectVersionReply {
    pub data: ProjectVersion,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyProjectVersionRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyProjectVersionReply {
    pub data: ProjectVersion,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEDCVersionRequest {
    pub product: String,
    pub trial: String,
    pub version_name: String,
    pub edc_filepath: String,
    pub edc_kind: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectVersion {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListProjectVersionRequest {
    pub product: String,
    pub trial: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListProjectVersionReply {
    pub project_id: i32,
    pub data: Vec<ProjectVersion>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub form_order: i32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFormRequest {
    pub version_id: i32,
    pub name: String,
    pub description: String,
    pub form_order: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateFormReply {
    pub data: Form,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListFormsRequest {
    pub version_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListFormsReply {
    pub data: Vec<Form>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFormByIdReply {
    pub data: Option<Form>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemType {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ListItemTypesReply {
    pub data: Vec<ItemType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub label: String,
    pub item_type_id: i32,
    pub item_order: i32,
    pub item_defualt_value: String,
    pub item_repeat_index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemDetail {
    pub id: i32,
    pub name: String,
    pub label: String,
    pub item_type: Option<ItemType>,
    pub item_option: Option<Vec<ItemOption>>,
    pub item_unit: Option<Vec<ItemUnit>>,
    pub item_order: i32,
    pub item_defualt_value: String,
    pub item_repeat_index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListItemsRequest {
    pub form_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListItemsReply {
    pub data: Vec<ItemDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAnnotationByFormRequest {
    pub form_id: i32,
    pub annotation_version_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListAnnotationByFormReply {
    pub data: FormAnnotation,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateItemRequest {
    pub form_id: i32,
    pub name: String,
    pub label: String,
    pub item_type_id: i32,
    pub item_order: i32,
    pub item_defualt_value: String,
    pub item_repeat_index: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateItemReply {
    pub data: Item,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemOption {
    pub id: i32,
    pub item_id: i32,
    pub option_value: String,
    pub option_display: String,
    pub option_order: i32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateItemOptionRequest {
    pub item_id: i32,
    pub option_value: String,
    pub option_display: String,
    pub option_order: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateItemOptionReply {}

#[derive(Debug, Serialize)]
pub struct CreateItemUnitRequest {
    pub item_id: i32,
    pub name: String,
    pub unit_order: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemUnit {
    pub id: i32,
    pub item_id: i32,
    pub name: String,
    pub unit_order: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateItemUnitReply {
    pub data: ItemUnit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnotationDomain {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnotationVariable {
    pub id: i32,
    pub domain_id: i32,
    pub name: String,
    pub supp: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    pub content: AnnotationContent,
    pub location: AnnotationLocation,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AnnotationKind {
    Form,
    Item,
    ItemValue,
    Unit,
    Option,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnotationLocation {
    pub source_id: i32,
    pub kind: AnnotationKind,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnotationContent {
    pub id: i32,
    pub variable: Option<AnnotationVariable>,
    pub assign: bool,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormAnnotation {
    pub domain: Vec<AnnotationDomain>,
    pub form_annotation: Vec<Annotation>,
    pub item_annotation: Vec<Annotation>,
    pub option_annotation: Vec<Annotation>,
    pub unit_annotation: Vec<Annotation>,
    pub value_annotation: Vec<Annotation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetItemByIdReply {
    pub data: Option<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOptionByIdReply {
    pub data: Option<ItemOption>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUnitByIdReply {
    pub data: Option<ItemUnit>,
}
