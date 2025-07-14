use std::collections::HashMap;

use crate::dto::annotation::{
    Annotation as AnnotationReply, AnnotationInForm, AnnotationKind, FormVariable,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnotationWithVariable {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    pub id: i32,
    pub annotation_version_id: i32,
    pub form_id: i32,
    pub variable: Option<FormVariable>,
    pub source_id: i32,
    pub kind: AnnotationKind,
    pub annotation_display: String,
    pub assign: bool,
}

impl Annotation {
    pub fn build(source: AnnotationReply, variable: Option<FormVariable>) -> Annotation {
        Annotation {
            id: source.id,
            annotation_version_id: source.annotation_version_id,
            form_id: source.form_id,
            variable,
            source_id: source.source_id,
            kind: source.kind,
            annotation_display: source.annotation_display,
            assign: source.assign,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnotationCollection {
    pub form: Vec<Annotation>,
    pub item: Vec<Annotation>,
    pub value: Vec<Annotation>,
    pub unit: Vec<Annotation>,
    pub option: Vec<Annotation>,
}

impl AnnotationCollection {
    pub fn build(
        annotations: AnnotationInForm,
        variables: Vec<FormVariable>,
    ) -> AnnotationCollection {
        let variable_map: HashMap<i32, FormVariable> =
            variables.into_iter().map(|v| (v.id, v)).collect();
        AnnotationCollection {
            form: build_annotations(&annotations.form, &variable_map),
            item: build_annotations(&annotations.item, &variable_map),
            value: build_annotations(&annotations.value, &variable_map),
            unit: build_annotations(&annotations.unit, &variable_map),
            option: build_annotations(&annotations.option, &variable_map),
        }
    }
}

fn build_annotations(
    source: &[AnnotationReply],
    variable_map: &HashMap<i32, FormVariable>,
) -> Vec<Annotation> {
    source
        .iter()
        .map(|source| {
            let variable = variable_map.get(&source.variable_id).map(|v| v.clone());
            Annotation::build(source.clone(), variable)
        })
        .collect()
}
