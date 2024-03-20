use pyo3::types::{PyAny, PyDict, PyList};
use pyo3::PyResult;
use serde_json::Value as JsonValue;

/// convert pyobject to json value
pub fn pyobj_to_json_value(obj: &PyAny) -> PyResult<JsonValue> {
  // Handle None
  if obj.is_none() {
    return Ok(JsonValue::Null);
  }
  // Handle boolean
  else if let Ok(val) = obj.extract::<bool>() {
    return Ok(JsonValue::Bool(val));
  }
  // Handle integers
  else if let Ok(val) = obj.extract::<i64>() {
    return Ok(JsonValue::Number(val.into()));
  }
  // Handle floats
  else if let Ok(val) = obj.extract::<f64>() {
    if let Some(num) = serde_json::Number::from_f64(val) {
      return Ok(JsonValue::Number(num));
    } else {
      return Err(pyo3::exceptions::PyValueError::new_err(
        "Float value out of range",
      ));
    }
  }
  // Handle strings
  else if let Ok(val) = obj.extract::<&str>() {
    return Ok(JsonValue::String(val.to_string()));
  }
  // Handle lists
  else if let Ok(list) = obj.downcast::<PyList>() {
    let mut vec = Vec::new();
    for item in list.iter() {
      vec.push(pyobj_to_json_value(item)?);
    }
    return Ok(JsonValue::Array(vec));
  }
  // Handle dictionaries
  else if let Ok(dict) = obj.downcast::<PyDict>() {
    let mut map = serde_json::Map::new();
    for (k, v) in dict.iter() {
      let key: &str = k.extract()?;
      let value = pyobj_to_json_value(v)?;
      map.insert(key.to_string(), value);
    }
    return Ok(JsonValue::Object(map));
  }
  // Catch-all for unsupported types
  else {
    Err(pyo3::exceptions::PyTypeError::new_err(
      "Unsupported Python type",
    ))
  }
}

/// convert pydict to json value
pub fn pydict_to_json_value(py_dict: &pyo3::types::PyDict) -> PyResult<JsonValue> {
  let mut map = serde_json::Map::new();

  for (k, v) in py_dict.iter() {
    let key: &str = k.extract()?;
    let value: JsonValue = pyobj_to_json_value(v)?;
    map.insert(key.to_string(), value);
  }

  Ok(serde_json::Value::Object(map))
}
