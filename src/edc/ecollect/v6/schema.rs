use super::column;
use crate::edc::item_type::ItemType;
use calamine::Data;

#[derive(Debug)]
pub struct CodeList {
    pub display: String,
    pub value: String,
    pub order: i32,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub label: String,
    pub item_type: ItemType,
    pub order: i32,
    pub codelist_id: Option<String>,
    pub unit_id: Option<String>,
    pub default_value: Option<String>,
    pub repeat_id: i32,
    // pub default_value: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Form {
    pub name: String,
    pub label: String,
    pub order: i32,
}

#[derive(Debug)]
pub struct FormId {
    pub name: String,
    pub id: i32,
}

#[derive(Debug, Default, Clone)]
pub struct ItemSheetRow {
    pub form_oid: String,
    pub item_oid: String,
    pub item_name: String,
    pub display_mode: String,
    pub codelist_oid: String,
    pub unit_oid: String,
    pub default_value: String,
    pub multiple: bool,
    pub visible: bool,
    pub valid: bool,
}

impl Into<ItemSheetRow> for &[Data] {
    fn into(self) -> ItemSheetRow {
        let form_oid = self
            .get(column::ITEM_SHEET_FORM_NAME)
            .map(|cell| cell.to_string());
        let item_oid = self
            .get(column::ITEM_SHEET_NAME)
            .map(|cell| cell.to_string());
        let item_name = self
            .get(column::ITEM_SHEET_LABEL)
            .map(|cell| cell.to_string());
        let display_mode = self
            .get(column::ITEM_SHEET_ITEM_TYPE)
            .map(|cell| cell.to_string());
        let codelist_oid = self
            .get(column::ITEM_SHEET_CODELIST_NAME)
            .map(|cell| cell.to_string())
            .map(|cell| {
                for s in cell.split("=") {
                    return s.trim().to_string();
                }
                "".to_string()
            });
        let unit_oid = self
            .get(column::ITEM_SHEET_UNIT_GROUP)
            .map(|cell| cell.to_string())
            .map(|cell| {
                for s in cell.split("=") {
                    return s.trim().to_string();
                }
                "".to_string()
            });
        let default_value = self
            .get(column::ITEM_SHEET_DEFAULT)
            .map(|cell| cell.to_string());

        let multiple = match self.get(column::ITEM_SHEET_MULTIPLE) {
            Some(cell) => match cell.to_string().as_str() {
                "Multiple" => true,
                _ => false,
            },
            None => false,
        };
        let visible = match self.get(column::ITEM_SHEET_VISIBLE) {
            Some(cell) => match cell.to_string().as_str() {
                "N" => false,
                _ => true,
            },
            None => true,
        };

        for item in [
            &form_oid,
            &item_oid,
            &item_name,
            &display_mode,
            &codelist_oid,
            &unit_oid,
            &default_value,
        ] {
            if item.is_none() {
                return ItemSheetRow::default();
            }
        }
        ItemSheetRow {
            form_oid: form_oid.unwrap(),
            item_oid: item_oid.unwrap(),
            // group_sort: group_sort.unwrap().parse::<i32>().unwrap_or_default(),
            item_name: item_name.unwrap(),
            display_mode: display_mode.unwrap(),
            codelist_oid: codelist_oid.unwrap(),
            unit_oid: unit_oid.unwrap(),
            default_value: default_value.unwrap(),
            multiple,
            visible,
            valid: true,
        }
    }
}
