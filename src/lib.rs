#![deny(clippy::all)]

use pyo3::prelude::*;
use spider::lazy_static::lazy_static;

lazy_static! {
  pub static ref BUFFER: usize = (num_cpus::get() * 20).max(88);
}

pub mod npage;
pub mod nwebsite;
pub mod page;
pub mod shortcut;
pub mod utils;
pub mod website;

pub use npage::{new_page, page_title, NPage};
pub use nwebsite::NWebsite;
pub use page::Page;
pub use utils::pydict_to_json_value;
pub use website::Website;

#[pyfunction]
#[pyo3(signature = (url, raw_content=None))]
/// Crawl a website storing the links found.
fn crawl(py: Python, url: String, raw_content: Option<bool>) -> PyResult<Bound<PyAny>> {
  pyo3_async_runtimes::tokio::future_into_py(py, async move {
    let w = shortcut::crawl(url, raw_content).await;

    Ok(w)
  })
}

#[pymodule]
fn spider_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(crawl, m)?)?;
  m.add_class::<Website>()?;
  m.add_class::<Page>()?;

  Ok(())
}
