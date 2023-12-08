use pyo3::prelude::*;

use crate::NPage;

/// website main data from rust to node.
#[pyclass]
pub struct NWebsite {
  /// all of the website links.
  #[pyo3(get)]
  pub links: Vec<String>,
  /// the pages found.
  #[pyo3(get)]
  pub pages: Vec<NPage>,
}
