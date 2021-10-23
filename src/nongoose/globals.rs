use std::sync::Mutex;

use once_cell::sync::OnceCell;

use crate::schema::SchemaData;

static SCHEMAS: OnceCell<Mutex<Vec<SchemaData>>> = OnceCell::new();

pub(crate) fn add_schema(schema: &SchemaData) {
  if let Some(schemas) = SCHEMAS.get() {
    let mut schemas = schemas.lock().unwrap();
    schemas.push(schema.clone());
  } else {
    SCHEMAS.set(Mutex::new(vec![schema.clone()])).unwrap();
  }
}

pub(crate) fn get_schema(name: &String) -> Option<SchemaData> {
  if let Some(schemas) = SCHEMAS.get() {
    let schemas = schemas.lock().unwrap();

    for schema in schemas.iter() {
      if &schema.get_name() == name {
        return Some(schema.clone());
      }
    }
  }

  None
}
