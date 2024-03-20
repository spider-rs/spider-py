use pyo3::types::{PyAny, PyDict, PyList};
use pyo3::PyResult;
use serde_json::Value as JsonValue;

/// convert pyobject to json value
pub fn pyobj_to_json_value(obj: &PyAny) -> PyResult<JsonValue> {
  // Handle None
  if obj.is_none() {
    Ok(JsonValue::Null)
  } else if let Ok(val) = obj.extract::<bool>() {
    Ok(JsonValue::Bool(val))
  } else if let Ok(val) = obj.extract::<i64>() {
    Ok(JsonValue::Number(val.into()))
  } else if let Ok(val) = obj.extract::<f64>() {
    if let Some(num) = serde_json::Number::from_f64(val) {
      Ok(JsonValue::Number(num))
    } else {
      Err(pyo3::exceptions::PyValueError::new_err(
        "Float value out of range",
      ))
    }
  } else if let Ok(val) = obj.extract::<&str>() {
    Ok(JsonValue::String(val.to_string()))
  } else if let Ok(list) = obj.downcast::<PyList>() {
    let mut vec = Vec::new();
    for item in list.iter() {
      vec.push(pyobj_to_json_value(item)?);
    }
    Ok(JsonValue::Array(vec))
  } else if let Ok(dict) = obj.downcast::<PyDict>() {
    let mut map = serde_json::Map::new();
    for (k, v) in dict.iter() {
      let key: &str = k.extract()?;
      let value = pyobj_to_json_value(v)?;
      map.insert(key.to_string(), value);
    }
    Ok(JsonValue::Object(map))
  } else {
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
