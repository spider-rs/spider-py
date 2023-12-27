use pyo3::{pyclass, pymethods, PyRef, PyRefMut};
use spider::compact_str::CompactString;

/// a simple page object
#[derive(Default)]
#[pyclass]
pub struct Page {
  /// the page object from spider
  inner: Option<spider::page::Page>,
  /// selectors
  selectors: Option<(
    CompactString,
    spider::smallvec::SmallVec<[CompactString; 2]>,
  )>,
  /// the url for the page
  pub url: String,
  pub subdomains: Option<bool>,
  pub tld: Option<bool>,
  pub status_code: u16,
}

#[pymethods]
impl Page {
  /// a new page
  #[new]
  pub fn new(url: String, subdomains: Option<bool>, tld: Option<bool>) -> Self {
    Page {
      url,
      subdomains,
      tld,
      ..Default::default()
    }
  }

  /// get the page content
  pub fn fetch(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
    use spider::{
      lazy_static::lazy_static, reqwest::Client, reqwest_middleware::ClientWithMiddleware,
      ClientBuilder,
    };
    lazy_static! {
      /// top level single page client to re-use.
      pub static ref PAGE_CLIENT: ClientWithMiddleware = {
        let reqwest_client = Client::builder().build().unwrap_or_default();
        let client = ClientBuilder::new(reqwest_client).build();

        client
      };
    }
    let s = pyo3_asyncio::tokio::get_runtime()
      .block_on(async move {
        let page = spider::page::Page::new_page(&slf.url, &PAGE_CLIENT).await;
        slf.status_code = page.status_code.into();
        slf.inner = Some(page);
        slf.selectors = spider::page::get_page_selectors(
          &slf.url,
          slf.subdomains.unwrap_or_default(),
          slf.tld.unwrap_or_default(),
        );
        Ok::<PyRefMut<'_, Page>, ()>(slf)
      })
      .unwrap();

    s
  }

  /// all links on the page
  pub fn get_links(slf: PyRef<'_, Self>) -> Vec<String> {
    match &slf.selectors {
      Some(selectors) => match &slf.inner {
        Some(inner) => {
          let links = pyo3_asyncio::tokio::get_runtime()
            .block_on(async move {
              let links = inner.links(&selectors).await;
              Ok::<spider::hashbrown::HashSet<spider::CaseInsensitiveString>, ()>(links)
            })
            .unwrap_or_default();

          links
            .into_iter()
            .map(|i| i.as_ref().to_string())
            .collect::<Vec<String>>()
        }
        _ => Default::default(),
      },
      _ => Default::default(),
    }
  }

  /// get the html for the page
  pub fn get_html(&self) -> String {
    match &self.inner {
      Some(inner) => inner.get_html(),
      _ => Default::default(),
    }
  }

  /// get the bytes for the page
  pub fn get_bytes(&self) -> &[u8] {
    match &self.inner {
      Some(inner) => inner.get_html_bytes_u8(),
      _ => Default::default(),
    }
  }
}
