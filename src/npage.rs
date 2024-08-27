use crate::page::header_map_to_hash_map;
use pyo3::prelude::*;
use spider::{
  lazy_static::lazy_static,
  packages::scraper::{Html, Selector},
};
use std::collections::{HashMap, HashSet};

/// a simple page object
#[derive(Default, Clone)]
#[pyclass]
pub struct NPage {
  #[pyo3(get)]
  /// The url of the resource.
  pub url: String,
  #[pyo3(get)]
  /// The content of the page found as UTF-8.
  pub content: String,
  #[pyo3(get)]
  /// The HTTP status code.
  pub status_code: u16,
  #[pyo3(get)]
  /// The raw content in bytes.
  pub raw_content: Option<Vec<u8>>,
  #[pyo3(get)]
  /// The HTTP headers.
  pub headers: Option<HashMap<String, String>>,
  #[pyo3(get)]
  /// The links found on the page. Requires the website.builder method website.with_subscription_return_page_links to be set to true.
  pub links: Option<HashSet<String>>,
}

/// get the page title.
pub fn page_title(page: NPage) -> String {
  page.title()
}

/// get a new Page
pub fn new_page(res: &spider::page::Page, raw: bool) -> NPage {
  NPage {
    url: res.get_url().into(),
    status_code: res.status_code.as_u16(),
    content: if raw {
      Default::default()
    } else {
      res.get_html()
    },
    raw_content: if raw {
      Some(res.get_html_bytes_u8().into())
    } else {
      None
    },
    headers: match res.headers {
      Some(ref headers) => Some(header_map_to_hash_map(headers)),
      _ => None,
    },
    links: match res.page_links {
      Some(ref links) => Some(
        links
          .iter()
          .map(|link| link.as_ref().to_string())
          .collect::<HashSet<String>>(),
      ),
      _ => None,
    },
  }
}

#[pymethods]
impl NPage {
  fn __call__(&self) {}

  /// the html page title.
  pub fn title(&self) -> String {
    lazy_static! {
      static ref TITLE_SELECTOR: Selector = Selector::parse("title").unwrap();
    }
    let fragment: Html = Html::parse_document(&self.content);
    match fragment.select(&TITLE_SELECTOR).next() {
      Some(title) => title.inner_html(),
      _ => Default::default(),
    }
  }
}
