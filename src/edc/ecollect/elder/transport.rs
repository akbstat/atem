use super::{
    column,
    schema::{CodeList, Form, FormId, Item, ItemSheetRow},
};
use crate::{
    edc::{item_type::ItemType, transport::EdcTransport},
    errors::Result,
    repository::{
        dto::rawdata::{
            CreateFormRequest, CreateItemOptionRequest, CreateItemRequest, CreateItemUnitRequest,
        },
        RawdataRepository,
    },
};
use async_trait::async_trait;
use calamine::{open_workbook, DataType, Reader, Xlsx};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Seek},
    path::Path,
    sync::Arc,
};
use tokio::sync::Mutex;

const ITEM_GROUP_SEGEMENT_SIZE: i32 = 1000;

pub struct ECollectElderTransport {
    data: Arc<Mutex<Xlsx<BufReader<File>>>>,
    unit_mapper: Arc<Mutex<HashMap<String, Vec<String>>>>,
    form_order: HashMap<String, i32>,
    form_mapper: Arc<Mutex<HashMap<String, Form>>>,
    analyest_mapper: Arc<Mutex<HashMap<String, String>>>,
    codelist_mapper: Arc<Mutex<HashMap<String, Vec<CodeList>>>>,
    item_mapper: Arc<Mutex<HashMap<String, Vec<Item>>>>,
    form_id_list: Arc<Mutex<Vec<FormId>>>,
    repository: Arc<RawdataRepository>,
}

impl ECollectElderTransport {
    pub fn new<P: AsRef<Path>>(
        filepath: P,
        repository: Arc<RawdataRepository>,
    ) -> Result<ECollectElderTransport> {
        let mut data: Xlsx<BufReader<File>> = open_workbook(filepath)?;
        let subject_form_id = find_subejct_form_oid(&mut data)?;
        let mut form_order = build_form_order_map(&mut data)?;
        // if subject form exists, add to form order map
        if let Some(subject_form_id) = &subject_form_id {
            form_order.insert(subject_form_id.to_owned(), 0);
        }
        let data = Arc::new(Mutex::new(data));
        let unit_mapper = Arc::new(Mutex::new(HashMap::<String, Vec<String>>::new()));
        let form_mapper = Arc::new(Mutex::new(HashMap::<String, Form>::new()));
        let analyest_mapper = Arc::new(Mutex::new(HashMap::<String, String>::new()));
        let codelist_mapper = Arc::new(Mutex::new(HashMap::<String, Vec<CodeList>>::new()));
        let item_mapper = Arc::new(Mutex::new(HashMap::<String, Vec<Item>>::new()));
        let form_id_list = Arc::new(Mutex::new(vec![]));
        Ok(ECollectElderTransport {
            data,
            unit_mapper,
            form_mapper,
            analyest_mapper,
            codelist_mapper,
            item_mapper,
            form_id_list,
            repository,
            form_order,
        })
    }

    async fn read_unit(&self) -> Result<()> {
        let mut unit_mapper = HashMap::<String, Vec<String>>::new();
        let mut data_ref = self.data.lock().await;
        let sheet = data_ref.worksheet_range("Units")?;
        for (index, row) in sheet.rows().enumerate() {
            if index.eq(&0) {
                // Skip the header row
                continue;
            }
            let group_cell = row.get(column::UNIT_SHEET_GROUP);
            let unit_cell = row.get(column::UNIT_SHEET_UNIT);
            if contains_none(&vec![group_cell, unit_cell]) {
                continue;
            }
            let group_cell = group_cell.unwrap().to_string();
            let unit_cell = unit_cell.unwrap().to_string();

            if contains_empty(&vec![&group_cell, &unit_cell]) {
                continue;
            }

            if let Some(units) = unit_mapper.get_mut(&group_cell) {
                units.push(unit_cell);
            } else {
                unit_mapper.insert(group_cell, vec![unit_cell]);
            }
        }
        let mut unit_mapper_ref = self.unit_mapper.lock().await;
        *unit_mapper_ref = unit_mapper;
        Ok(())
    }

    async fn read_codelist(&self) -> Result<()> {
        let mut code_list_mapper = HashMap::<String, Vec<CodeList>>::new();
        let mut data_ref = self.data.lock().await;
        let sheet = data_ref.worksheet_range("CodeListItems")?;
        for (index, row) in sheet.rows().enumerate() {
            if index.eq(&0) {
                // Skip the header row
                continue;
            }
            let id = row.get(column::CODELIST_SHEET_NAME);
            let order = row.get(column::CODELIST_SHEET_ORDER);
            let display = row.get(column::CODELIST_SHEET_DISPLAY);
            let value = row.get(column::CODELIST_SHEET_VALUE);

            if contains_none(&vec![id, order, display, value]) {
                continue;
            }

            let order = order.unwrap().as_i64();
            if order.is_none() {
                continue;
            }
            let order = order.unwrap() as i32;
            let id = id.unwrap().to_string();
            let display = display.unwrap().to_string();
            let value = value.unwrap().to_string();

            if contains_empty(&vec![&id, &display, &value]) {
                continue;
            }

            let code_list = CodeList {
                display,
                value,
                order,
            };
            if let Some(code_lists) = code_list_mapper.get_mut(&id) {
                code_lists.push(code_list);
            } else {
                code_list_mapper.insert(id, vec![code_list]);
            }
        }

        let mut codelist_ref = self.codelist_mapper.lock().await;
        *codelist_ref = code_list_mapper;
        Ok(())
    }

    async fn read_form(&self) -> Result<()> {
        let mut form_mapper = HashMap::<String, Form>::new();
        let mut data_ref = self.data.lock().await;
        let sheet = data_ref.worksheet_range("Forms")?;
        for (index, row) in sheet.rows().enumerate() {
            if index.eq(&0) {
                // Skip the header row
                continue;
            }
            let name = row.get(column::FORM_SHEET_NAME);
            let label = row.get(column::FORM_SHEET_LABEL);
            if contains_none(&vec![name, label]) {
                continue;
            }
            let name = name.unwrap().to_string();
            let label = label.unwrap().to_string();
            if contains_empty(&vec![&name, &label]) {
                continue;
            }

            let order = self.form_order.get(&name);
            if order.is_none() {
                continue;
            }

            form_mapper.insert(
                name.clone(),
                Form {
                    name,
                    label,
                    order: *order.unwrap(),
                },
            );
        }
        let mut form_mapper_ref = self.form_mapper.lock().await;
        *form_mapper_ref = form_mapper;
        Ok(())
    }

    async fn read_analyest(&self) -> Result<()> {
        let mut analyest_mapper = HashMap::<String, String>::new();
        let mut data_ref = self.data.lock().await;
        let sheet = data_ref.worksheet_range("AnalytesInTheStudy")?;
        for (index, row) in sheet.rows().enumerate() {
            if index.eq(&0) {
                // Skip the header row
                continue;
            }
            let id = row.get(column::ANALYTES_SHEET_CODE);
            let name = row.get(column::ANALYTES_SHEET_NAME);
            if contains_none(&vec![id, name]) {
                continue;
            }
            let id = id.unwrap().to_string();
            let name = name.unwrap().to_string();
            if contains_empty(&vec![&id, &name]) {
                continue;
            }
            analyest_mapper.insert(id, name);
        }
        let mut analyest_mapper_ref = self.analyest_mapper.lock().await;
        *analyest_mapper_ref = analyest_mapper;
        Ok(())
    }

    async fn read_item(&self) -> Result<()> {
        let mut data_ref = self.data.lock().await;
        let sheet = data_ref.worksheet_range("GroupItems")?;

        let mut last_group_oid: Option<String> = None;
        let mut next_order_segment_start = 0;
        let mut group_item_segement = vec![];
        for (index, row) in sheet.rows().enumerate() {
            if index.eq(&0) {
                // Skip the header row
                continue;
            }
            let row: ItemSheetRow = row.into();
            if !row.valid {
                continue;
            }

            match last_group_oid {
                Some(ref last_group) => {
                    if last_group.eq(&row.group_oid) {
                        group_item_segement.push(row);
                    } else {
                        last_group_oid = Some(row.group_oid.clone());
                        self.read_group_items(next_order_segment_start, &group_item_segement)
                            .await;
                        next_order_segment_start += ITEM_GROUP_SEGEMENT_SIZE;
                        group_item_segement.clear();
                        group_item_segement.push(row);
                    }
                }
                None => {
                    last_group_oid = Some(row.group_oid.clone());
                    group_item_segement.push(row);
                }
            }
        }
        Ok(())
    }

    async fn read_group_items(&self, base_order: i32, rows: &[ItemSheetRow]) {
        if rows.is_empty() {
            return;
        }
        // detact if in logline mode
        let default_value = split_default_values(&rows[0].default_value);
        if default_value.is_empty() {
            self.read_group_items_general(base_order, rows).await;
        } else {
            self.read_group_items_logline(base_order, rows, &default_value)
                .await;
        }
    }

    async fn read_group_items_logline<S: AsRef<str>>(
        &self,
        base_order: i32,
        rows: &[ItemSheetRow],
        default_values: &[S],
    ) {
        let default_value_display = self
            .build_default_value_display_list(
                &rows[0].display_mode,
                &rows[0].codelist_oid,
                default_values,
            )
            .await;
        let mut item_mapper = self.item_mapper.lock().await;
        let mut sub_order = 0;
        for (index, value) in default_value_display.into_iter().enumerate() {
            for (row_index, row) in rows.iter().enumerate() {
                let form_name = &row.form_oid;
                let item_type = convert_item_type(&row.display_mode);
                if let ItemType::Unknown = item_type {
                    continue;
                }
                let item = Item {
                    name: row.item_oid.clone(),
                    label: row.item_name.clone(),
                    item_type,
                    order: base_order + sub_order,
                    codelist_id: if row.codelist_oid.is_empty() {
                        None
                    } else {
                        Some(row.codelist_oid.clone())
                    },
                    unit_id: if row.unit_oid.is_empty() {
                        None
                    } else {
                        Some(row.unit_oid.clone())
                    },
                    default_value: if row_index.eq(&0) {
                        Some(value.clone())
                    } else {
                        None
                    },
                    repeat_id: index as i32,
                };
                if let Some(items) = item_mapper.get_mut(form_name) {
                    items.push(item);
                } else {
                    item_mapper.insert(form_name.to_owned(), vec![item]);
                }
                sub_order += 1;
            }
        }
    }

    async fn read_group_items_general(&self, base_order: i32, rows: &[ItemSheetRow]) {
        let mut item_mapper = self.item_mapper.lock().await;
        for (index, row) in rows.iter().enumerate() {
            let form_name = &row.form_oid;
            let item_type = convert_item_type(&row.display_mode);
            if let ItemType::Unknown = item_type {
                continue;
            }
            let item = Item {
                name: row.item_oid.clone(),
                label: row.item_name.clone(),
                item_type,
                order: index as i32 + base_order,
                codelist_id: if row.codelist_oid.is_empty() {
                    None
                } else {
                    Some(row.codelist_oid.clone())
                },
                unit_id: if row.unit_oid.is_empty() {
                    None
                } else {
                    Some(row.unit_oid.clone())
                },
                default_value: None,
                repeat_id: 0,
            };
            if let Some(items) = item_mapper.get_mut(form_name) {
                items.push(item);
            } else {
                item_mapper.insert(form_name.to_owned(), vec![item]);
            }
        }
    }

    async fn build_default_value_display_list<S: AsRef<str>>(
        &self,
        item_type: &str,
        codelist_id: &str,
        values: &[S],
    ) -> Vec<String> {
        match item_type {
            "AnalytesOption" => self.build_default_value_display_list_analytes(values).await,
            "RadioButton" | "RadioButton(Vertical)" | "DropDownList" => {
                self.build_default_value_display_list_codelist(codelist_id, values)
                    .await
            }
            _ => Vec::new(),
        }
    }

    async fn build_default_value_display_list_codelist<S: AsRef<str>>(
        &self,
        codelist_id: &str,
        values: &[S],
    ) -> Vec<String> {
        let codelist_mapper = self.codelist_mapper.lock().await;
        match codelist_mapper.get(codelist_id) {
            Some(codelist) => {
                let mapper = codelist
                    .iter()
                    .map(|c| (c.value.as_str(), c.display.as_str()))
                    .collect::<HashMap<_, _>>();
                values
                    .into_iter()
                    .map(|value| {
                        mapper
                            .get(value.as_ref())
                            .map(|display| display.to_string())
                            .unwrap_or(String::new())
                    })
                    .collect()
            }
            None => Vec::new(),
        }
    }

    async fn build_default_value_display_list_analytes<S: AsRef<str>>(
        &self,
        values: &[S],
    ) -> Vec<String> {
        let analytes_mapper = self.analyest_mapper.lock().await;
        values
            .into_iter()
            .map(|s| {
                analytes_mapper
                    .get(s.as_ref())
                    .unwrap_or(&String::new())
                    .to_owned()
            })
            .collect()
    }

    async fn save_forms(&self, project_version_id: i32) -> Result<()> {
        let form_mapper = &self.form_mapper.lock().await;
        let mut forms = form_mapper.values().cloned().collect::<Vec<_>>();
        forms.sort_by(|a, b| a.order.cmp(&b.order));
        let mut form_id_list = Vec::with_capacity(form_mapper.len());
        for form in forms {
            let request = CreateFormRequest {
                name: form.name.clone(),
                description: form.label.clone(),
                version_id: project_version_id,
                form_order: form.order,
            };
            let form_id = self.repository.create_form(&request).await?;
            form_id_list.push(FormId {
                name: form.name,
                id: form_id,
            });
        }
        let mut form_id_list_ref = self.form_id_list.lock().await;
        *form_id_list_ref = form_id_list;
        Ok(())
    }

    async fn save_items(&self) -> Result<()> {
        let form_id_list = self.form_id_list.lock().await;
        let item_mapper = self.item_mapper.lock().await;
        let item_types = self.repository.list_item_types().await?;
        let type_mapper = item_types
            .data
            .iter()
            .map(|item_type| (item_type.name.clone(), item_type.id))
            .collect::<HashMap<_, _>>();
        for form in form_id_list.iter() {
            let items = item_mapper.get(&form.name);
            if items.is_none() {
                continue;
            }
            let items = items.unwrap();
            for item in items.iter() {
                let item_type_str = convert_item_type_to_string(&item.item_type);
                if item_type_str.is_none() {
                    continue;
                }
                let item_type_id = type_mapper.get(&item_type_str.unwrap());
                if item_type_id.is_none() {
                    continue;
                }
                let request = CreateItemRequest {
                    name: item.name.clone(),
                    label: item.label.clone(),
                    item_type_id: *item_type_id.unwrap(),
                    item_order: item.order,
                    form_id: form.id,
                    item_defualt_value: item.default_value.clone().unwrap_or_default(),
                    item_repeat_index: item.repeat_id,
                };
                let item_id = self.repository.create_item(&request).await?;
                if let Some(codelist_id) = &item.codelist_id {
                    if let None = item.default_value {
                        self.save_item_codelist_options(&item_id, codelist_id)
                            .await?;
                    }
                }
                if let Some(unit_id) = &item.unit_id {
                    self.save_item_unit(&item_id, unit_id).await?;
                }
            }
        }
        Ok(())
    }

    async fn save_item_codelist_options(&self, item_id: &i32, codelist_id: &str) -> Result<()> {
        let codelist_mapper = self.codelist_mapper.lock().await;
        let code_list = codelist_mapper.get(codelist_id);
        if code_list.is_none() {
            return Ok(());
        }
        let code_list = code_list.unwrap();
        for code in code_list.iter() {
            let request = CreateItemOptionRequest {
                item_id: *item_id,
                option_display: code.display.clone(),
                option_value: code.value.clone(),
                option_order: code.order,
            };
            self.repository.create_item_option(&request).await?;
        }
        Ok(())
    }

    async fn save_item_unit(&self, item_id: &i32, unit: &str) -> Result<()> {
        let unit_mapper = self.unit_mapper.lock().await;
        let units = unit_mapper.get(unit);
        if units.is_none() {
            return Ok(());
        }
        let units = units.unwrap();
        for (order, unit) in units.iter().enumerate() {
            let request = CreateItemUnitRequest {
                item_id: *item_id,
                name: unit.clone(),
                unit_order: order as i32,
            };
            self.repository.create_item_unit(&request).await?;
        }
        Ok(())
    }
}

#[async_trait]
impl EdcTransport for ECollectElderTransport {
    async fn read(&self) -> crate::errors::Result<()> {
        self.read_unit().await?;
        self.read_codelist().await?;
        self.read_analyest().await?;
        self.read_form().await?;
        self.read_item().await?;
        // Implement the logic to read from the ECollect file
        // For now, just return Ok
        Ok(())
    }

    async fn save(&self, project_version_id: i32) -> Result<()> {
        self.save_forms(project_version_id).await?;
        self.save_items().await?;
        Ok(())
    }
}

pub fn convert_item_type(source: &str) -> ItemType {
    let source = source.trim();
    match source {
        "TextField" | "DateTime" | "DynamicOptions" | "LongText" | "Number" | "AnalytesResult"
        | "AnalytesOption" => ItemType::Text,
        "RadioButton" | "RadioButton(Vertical)" | "DropDownList" => ItemType::Option,
        "CheckBox" => ItemType::Checkbox,
        "Label" => ItemType::Label,
        _ => ItemType::Unknown,
    }
}

pub fn convert_item_type_to_string(item_type: &ItemType) -> Option<String> {
    match item_type {
        ItemType::Text => Some("Text".to_string()),
        ItemType::Option => Some("Option".to_string()),
        ItemType::Checkbox => Some("Checkbox".to_string()),
        ItemType::Label => Some("Label".to_string()),
        ItemType::Unknown => None,
    }
}

fn contains_none<T>(list: &[Option<T>]) -> bool {
    for item in list {
        if item.is_none() {
            return true;
        }
    }
    false
}

fn contains_empty<T: AsRef<str>>(list: &[T]) -> bool {
    for item in list {
        if item.as_ref().is_empty() {
            return true;
        }
    }
    false
}

fn split_default_values<S: AsRef<str>>(value: S) -> Vec<String> {
    let value = value.as_ref();
    if value.is_empty() {
        vec![]
    } else {
        value.split("|").map(|s| s.to_owned()).collect::<Vec<_>>()
    }
}

fn find_subejct_form_oid<T: Read + Seek>(workbook: &mut Xlsx<T>) -> Result<Option<String>> {
    // find subject form oid
    let sheet = workbook.worksheet_range("ECRFDraft")?;
    let subject_form_id = sheet.get_value((1, 2)).map(|cell| cell.to_string());
    Ok(subject_form_id)
}

fn build_form_order_map<T: Read + Seek>(workbook: &mut Xlsx<T>) -> Result<HashMap<String, i32>> {
    let mut form_order_map = HashMap::new();
    let sheet = workbook.worksheet_range("EventForm")?;
    let mut order = 1; // preserve order number '0' for subejct form
    for (index, row) in sheet.rows().enumerate() {
        if index.eq(&0) {
            continue;
        }
        let form_oid_cell = row
            .get(column::EVENTFORM_SHEET_FORMOID)
            .map(|cell| cell.to_string());
        if let Some(form_oid) = form_oid_cell {
            if !form_oid.is_empty() {
                if let None = form_order_map.get(&form_oid) {
                    form_order_map.insert(form_oid, order);
                    order += 1;
                }
            }
        }
    }
    Ok(form_order_map)
}

#[cfg(test)]
mod tests {
    use reqwest::Client;

    use super::*;
    #[tokio::test]
    async fn test_transport() {
        let repo = Arc::new(RawdataRepository::new(
            Arc::new(Client::new()),
            "http://localhost:2217/api/sdtm/rawdata/",
        ));
        let transport = ECollectElderTransport::new(
            r"D:\projects\rusty\mobius_kit\.mocks\edc\ecollect-test.xlsx",
            repo,
        )
        .unwrap();
        transport.read().await.unwrap();
        {
            let codelist = transport.codelist_mapper.lock().await;
            assert!(codelist.len().gt(&0));
            let form_mapper = transport.form_mapper.lock().await;
            assert!(form_mapper.len().gt(&0));
            let unit_mapper = transport.unit_mapper.lock().await;
            assert!(unit_mapper.len().gt(&0));
            let analyest_mapper = transport.analyest_mapper.lock().await;
            assert!(analyest_mapper.len().gt(&0));
            let item_mapper = transport.item_mapper.lock().await;
            assert!(item_mapper.len().gt(&0));
        }
        transport.save(1).await.unwrap();
    }
}
