use pyo3::prelude::*;
use crate::NPage;
use compact_str::CompactString;
use indexmap::IndexMap;
use spider::utils::shutdown;
use spider::tokio::task::JoinHandle;
use std::time::Duration;

#[pyfunction]
fn crawl(py: Python, url: String, raw_content: Option<bool>) -> PyResult<&PyAny> {    
    pyo3_asyncio::tokio::future_into_py(py, async move {
      let w = crate::crawl(url, raw_content).await;

      Ok(w)
  })
}

#[pymodule]
fn spider_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(crawl, m)?)?;
    Ok(())
}

/// a website holding the inner spider::website::Website from Rust fit for python.
#[pyclass]
pub struct Website {
  /// the website from spider.
  inner: spider::website::Website,
  /// spawned subscription handles.
  subscription_handles: IndexMap<u32, JoinHandle<()>>,
  /// spawned crawl handles.
  crawl_handles: IndexMap<u32, JoinHandle<()>>,
  /// do not convert content to UT8.
  raw_content: bool,
  /// the data collected.
  collected_data: Box<Vec<u8>>,
  /// is the crawl running in the background.
  running_in_background: bool
   // /// the file handle for storing data
                                // file_handle: Option<spider::tokio::fs::File>,
}

struct PageEvent {
  pub page: NPage,
}

#[pymethods]
impl Website {
  /// a new website.
  pub fn new(&self, url: String, raw_content: Option<bool>) -> Self {
    Website {
      inner: spider::website::Website::new(&url),
      subscription_handles: IndexMap::new(),
      crawl_handles: IndexMap::new(),
      raw_content: raw_content.unwrap_or_default(),
      collected_data: Box::new(Vec::new()),
      running_in_background: false
      // file_handle: None,
    }
  }

  /// Get the crawl status.
  pub fn status(&self) -> String {
    self.inner.get_status().to_string()
  }
  
  // /// store data to memory for disk storing. This will create the path if not exist and defaults to ./storage.
  // pub async fn export_jsonl_data(&self, export_path: Option<String>) -> std::io::Result<()> {
  //   use spider::tokio::io::AsyncWriteExt;
  //   let file = match export_path {
  //     Some(p) => {
  //       let base_dir = p
  //         .split("/")
  //         .into_iter()
  //         .map(|f| {
  //           if f.contains(".") {
  //             "".to_string()
  //           } else {
  //             f.to_string()
  //           }
  //         })
  //         .collect::<String>();

  //       spider::tokio::fs::create_dir_all(&base_dir).await?;

  //       if !p.contains(".") {
  //         p + ".jsonl"
  //       } else {
  //         p
  //       }
  //     }
  //     _ => {
  //       spider::tokio::fs::create_dir_all("./storage").await?;
  //       "./storage/".to_owned()
  //         + &self
  //           .inner
  //           .get_domain()
  //           .inner()
  //           .replace("http://", "")
  //           .replace("https://", "")
  //         + "jsonl"
  //     }
  //   };
  //   let mut file = spider::tokio::fs::File::create(file).await?;
  //   // transform data step needed to auto convert type ..
  //   file.write_all(&self.collected_data).await?;
  //   Ok(())
  // }

  
  // /// subscribe and add an event listener.
  // pub fn subscribe(
  //   &mut self,
  //   on_page_event: napi::threadsafe_function::ThreadsafeFunction<NPage>,
  // ) -> u32 {
  //   let mut rx2 = self
  //     .inner
  //     .subscribe(*BUFFER / 2)
  //     .expect("sync feature should be enabled");
  //   let raw_content = self.raw_content;

  //   let handle = spider::tokio::spawn(async move {
  //     while let Ok(res) = rx2.recv().await {
  //       on_page_event.call(
  //         Ok(NPage::new(&res, raw_content)),
  //         napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
  //       );
  //     }
  //   });

  //   // always return the highest value as the next id.
  //   let id = match self.subscription_handles.last() {
  //     Some(handle) => handle.0 + 1,
  //     _ => 0,
  //   };

  //   self.subscription_handles.insert(id, handle);

  //   id
  // }

  
  // /// remove a subscription listener.
  // pub fn unsubscribe(&mut self, id: Option<u32>) -> bool {
  //   match id {
  //     Some(id) => {
  //       let handle = self.subscription_handles.get(&id);

  //       match handle {
  //         Some(h) => {
  //           h.abort();
  //           self.subscription_handles.remove_entry(&id);
  //           true
  //         }
  //         _ => false,
  //       }
  //     }
  //     // we may want to get all subs and remove them
  //     _ => {
  //       let keys = self.subscription_handles.len();
  //       for k in self.subscription_handles.drain(..) {
  //         k.1.abort();
  //       }
  //       keys > 0
  //     }
  //   }
  // }

  
  // /// stop a crawl
  // pub async unsafe fn stop(&mut self, id: Option<u32>) -> bool {
  //   self.inner.stop();

  //   // prevent the last background run
  //   if self.running_in_background {
  //     // we may want ID's to be used as an option along with urls for complete shutdowns.
  //     shutdown(self.inner.get_domain().inner()).await;
  //     self.running_in_background = false;
  //   }

  //   match id {
  //     Some(id) => {
  //       let handle = self.crawl_handles.get(&id);

  //       match handle {
  //         Some(h) => {
  //           h.abort();
  //           self.crawl_handles.remove_entry(&id);
  //           true
  //         }
  //         _ => false,
  //       }
  //     }
  //     _ => {
  //       let keys = self.crawl_handles.len();
  //       for k in self.crawl_handles.drain(..) {
  //         k.1.abort();
  //       }
  //       keys > 0
  //     }
  //   }
  // }

  
  // /// crawl a website
  // pub async unsafe fn crawl(
  //   &mut self,
  //   // on_page_event: Option<napi::threadsafe_function::ThreadsafeFunction<NPage>>,
  //   background: Option<bool>,
  //   headless: Option<bool>,
  // ) {
  //   // only run in background if on_page_event is handled for streaming.
  //   let background = background.is_some() && background.unwrap_or_default();
  //   let headless = headless.is_some() && headless.unwrap_or_default();
  //   // let raw_content = self.raw_content;
    
  //   if background {
  //     self.running_in_background = background;
  //   }

  //   if background {
  //     let mut website = self.inner.clone();

  //     let crawl_id = match self.crawl_handles.last() {
  //       Some(handle) => handle.0 + 1,
  //       _ => 0,
  //     };

  //     let crawl_handle = spider::tokio::spawn(async move {
  //       if headless {
  //         website.crawl().await;
  //       } else {
  //         website.crawl_raw().await;
  //       }
  //     });

  //     self.crawl_handles.insert(crawl_id, crawl_handle);
  //   } else {
  //     if headless {
  //       self.inner.crawl().await;
  //     } else {
  //       self.inner.crawl_raw().await;
  //     }
  //   }

  //   // match on_page_event {
  //   //   Some(callback) => {
  //   //     if background {
  //   //       let mut website = self.inner.clone();
  //   //       let mut rx2 = website
  //   //         .subscribe(*BUFFER / 2)
  //   //         .expect("sync feature should be enabled");

  //   //       let handle = spider::tokio::spawn(async move {
  //   //         while let Ok(res) = rx2.recv().await {
  //   //           callback.call(
  //   //             Ok(NPage::new(&res, raw_content)),
  //   //             napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
  //   //           );
  //   //         }
  //   //       });

  //   //       let crawl_id = match self.crawl_handles.last() {
  //   //         Some(handle) => handle.0 + 1,
  //   //         _ => 0,
  //   //       };

  //   //       let crawl_handle = spider::tokio::spawn(async move {
  //   //         if headless {
  //   //           website.crawl().await;
  //   //         } else {
  //   //           website.crawl_raw().await;
  //   //         }
  //   //       });

  //   //       let id = match self.subscription_handles.last() {
  //   //         Some(handle) => handle.0 + 1,
  //   //         _ => 0,
  //   //       };

  //   //       self.crawl_handles.insert(crawl_id, crawl_handle);
  //   //       self.subscription_handles.insert(id, handle);
  //   //     } else {
  //   //       let mut rx2 = self
  //   //         .inner
  //   //         .subscribe(*BUFFER / 2)
  //   //         .expect("sync feature should be enabled");

  //   //       let handle = spider::tokio::spawn(async move {
  //   //         while let Ok(res) = rx2.recv().await {
  //   //           callback.call(
  //   //             Ok(NPage::new(&res, raw_content)),
  //   //             napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
  //   //           );
  //   //         }
  //   //       });

  //   //       if headless {
  //   //         self.inner.crawl().await;
  //   //       } else {
  //   //         self.inner.crawl_raw().await;
  //   //       }

  //   //       let id = match self.subscription_handles.last() {
  //   //         Some(handle) => handle.0 + 1,
  //   //         _ => 0,
  //   //       };

  //   //       self.subscription_handles.insert(id, handle);
  //   //     }
  //   //   }
  //   //   _ => {
  //   //     if background {
  //   //       let mut website = self.inner.clone();

  //   //       let crawl_id = match self.crawl_handles.last() {
  //   //         Some(handle) => handle.0 + 1,
  //   //         _ => 0,
  //   //       };

  //   //       let crawl_handle = spider::tokio::spawn(async move {
  //   //         if headless {
  //   //           website.crawl().await;
  //   //         } else {
  //   //           website.crawl_raw().await;
  //   //         }
  //   //       });

  //   //       self.crawl_handles.insert(crawl_id, crawl_handle);
  //   //     } else {
  //   //       if headless {
  //   //         self.inner.crawl().await;
  //   //       } else {
  //   //         self.inner.crawl_raw().await;
  //   //       }
  //   //     }
  //   //   }
  //   // }
  // }

  
  // /// scrape a website
  // pub async unsafe fn scrape(
  //   &mut self,
  //   // on_page_event: Option<napi::threadsafe_function::ThreadsafeFunction<NPage>>,
  //   background: Option<bool>,
  //   headless: Option<bool>,
  // ) {
  //   let headless = headless.is_some() && headless.unwrap_or_default();
  //   let raw_content = self.raw_content;
  //   let background = background.is_some() && background.unwrap_or_default();

  //   if background {
  //     self.running_in_background = background;
  //   }

  //   if background {
  //     let mut website = self.inner.clone();

  //     let crawl_id = match self.crawl_handles.last() {
  //       Some(handle) => handle.0 + 1,
  //       _ => 0,
  //     };

  //     let crawl_handle = spider::tokio::spawn(async move {
  //       if headless {
  //         website.scrape().await;
  //       } else {
  //         website.scrape_raw().await;
  //       }
  //     });

  //     self.crawl_handles.insert(crawl_id, crawl_handle);
  //   } else {
  //     if headless {
  //       self.inner.scrape().await;
  //     } else {
  //       self.inner.scrape_raw().await;
  //     }
  //   }

  //   // match on_page_event {
  //   //   Some(callback) => {
  //   //     if background {
  //   //       let mut website = self.inner.clone();
  //   //       let mut rx2 = website
  //   //         .subscribe(*BUFFER / 2)
  //   //         .expect("sync feature should be enabled");

  //   //       let handle = spider::tokio::spawn(async move {
  //   //         while let Ok(res) = rx2.recv().await {
  //   //           callback.call(
  //   //             Ok(NPage::new(&res, raw_content)),
  //   //             napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
  //   //           );
  //   //         }
  //   //       });

  //   //       let crawl_id = match self.crawl_handles.last() {
  //   //         Some(handle) => handle.0 + 1,
  //   //         _ => 0,
  //   //       };

  //   //       let crawl_handle = spider::tokio::spawn(async move {
  //   //         if headless {
  //   //           website.scrape().await;
  //   //         } else {
  //   //           website.scrape_raw().await;
  //   //         }
  //   //       });

  //   //       let id = match self.subscription_handles.last() {
  //   //         Some(handle) => handle.0 + 1,
  //   //         _ => 0,
  //   //       };

  //   //       self.crawl_handles.insert(crawl_id, crawl_handle);
  //   //       self.subscription_handles.insert(id, handle);
  //   //     } else {
  //   //       let mut rx2 = self
  //   //         .inner
  //   //         .subscribe(*BUFFER / 2)
  //   //         .expect("sync feature should be enabled");

  //   //       let handle = spider::tokio::spawn(async move {
  //   //         while let Ok(res) = rx2.recv().await {
  //   //           callback.call(
  //   //             Ok(NPage::new(&res, raw_content)),
  //   //             napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
  //   //           );
  //   //         }
  //   //       });

  //   //       if headless {
  //   //         self.inner.scrape().await;
  //   //       } else {
  //   //         self.inner.scrape_raw().await;
  //   //       }

  //   //       let id = match self.subscription_handles.last() {
  //   //         Some(handle) => handle.0 + 1,
  //   //         _ => 0,
  //   //       };

  //   //       self.subscription_handles.insert(id, handle);
  //   //     }
  //   //   }
  //   //   _ => {
  //   //     if background {
  //   //       let mut website = self.inner.clone();

  //   //       let crawl_id = match self.crawl_handles.last() {
  //   //         Some(handle) => handle.0 + 1,
  //   //         _ => 0,
  //   //       };

  //   //       let crawl_handle = spider::tokio::spawn(async move {
  //   //         if headless {
  //   //           website.scrape().await;
  //   //         } else {
  //   //           website.scrape_raw().await;
  //   //         }
  //   //       });

  //   //       self.crawl_handles.insert(crawl_id, crawl_handle);
  //   //     } else {
  //   //       if headless {
  //   //         self.inner.scrape().await;
  //   //       } else {
  //   //         self.inner.scrape_raw().await;
  //   //       }
  //   //     }
  //   //   }
  //   // }
  // }

  //  /// run a cron job
  // pub async unsafe fn run_cron(
  //   &mut self,
  //   on_page_event: Option<napi::threadsafe_function::ThreadsafeFunction<NPage>>,
  // ) -> Cron {
  //   let cron_handle = match on_page_event {
  //     Some(callback) => {
  //       let mut rx2 = self
  //         .inner
  //         .subscribe(*BUFFER / 2)
  //         .expect("sync feature should be enabled");
  //       let raw_content = self.raw_content;

  //       let handler = spider::tokio::spawn(async move {
  //         while let Ok(res) = rx2.recv().await {
  //           callback.call(
  //             Ok(NPage::new(&res, raw_content)),
  //             napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
  //           );
  //         }
  //       });

  //       Some(handler)
  //     }
  //     _ => None,
  //   };

  //   let inner = self.inner.run_cron().await;

  //   Cron { inner, cron_handle }
  // }

  
  // /// get all the links of a website
  // pub fn get_links(&self) -> Vec<String> {
  //   let links = self
  //     .inner
  //     .get_links()
  //     .iter()
  //     .map(|x| x.as_ref().to_string())
  //     .collect::<Vec<String>>();
  //   links
  // }

  // /// get the size of the website in amount of pages crawled. If you ran the page in the background, this value will not update.
  // pub fn size(&mut self) -> u32 {
  //   self.inner.size() as u32
  // }

  // /// get all the pages of a website - requires calling website.scrape
  
  // pub fn get_pages(&self) -> Vec<NPage> {
  //   let mut pages: Vec<NPage> = Vec::new();
  //   let raw_content = self.raw_content;

  //   match self.inner.get_pages() {
  //     Some(p) => {
  //       for page in p.iter() {
  //         pages.push(NPage::new(page, raw_content));
  //       }
  //     }
  //     _ => (),
  //   }

  //   pages
  // }

  
  // /// drain all links from storing
  // pub fn drain_links(&mut self) -> Vec<String> {
  //   let links = self
  //     .inner
  //     .drain_links()
  //     .map(|x| x.as_ref().to_string())
  //     .collect::<Vec<String>>();

  //   links
  // }

  
  // /// clear all links and page data
  // pub fn clear(&mut self) {
  //   self.inner.clear();
  // }

  
  // /// Set HTTP headers for request using [reqwest::header::HeaderMap](https://docs.rs/reqwest/latest/reqwest/header/struct.HeaderMap.html).
  // pub fn with_headers(&mut self, headers: Option<Object>) -> &Self {
  //   use std::str::FromStr;

  //   match headers {
  //     Some(obj) => {
  //       let mut h = spider::reqwest::header::HeaderMap::new();
  //       let keys = Object::keys(&obj).unwrap_or_default();

  //       for key in keys.into_iter() {
  //         let header_key = spider::reqwest::header::HeaderName::from_str(&key);

  //         match header_key {
  //           Ok(hn) => {
  //             let header_value = obj
  //               .get::<String, String>(key)
  //               .unwrap_or_default()
  //               .unwrap_or_default();

  //             match spider::reqwest::header::HeaderValue::from_str(&header_value) {
  //               Ok(hk) => {
  //                 h.append(hn, hk);
  //               }
  //               _ => (),
  //             }
  //           }
  //           _ => (),
  //         }
  //       }
  //       self.inner.with_headers(Some(h));
  //     }
  //     _ => {
  //       self.inner.with_headers(None);
  //     }
  //   };

  //   self
  // }

  // /// Add user agent to request.
  
  // pub fn with_user_agent(&mut self, user_agent: Option<&str>) -> &Self {
  //   self.inner.configuration.with_user_agent(user_agent);
  //   self
  // }

  // /// Respect robots.txt file.
  
  // pub fn with_respect_robots_txt(&mut self, respect_robots_txt: bool) -> &Self {
  //   self
  //     .inner
  //     .configuration
  //     .with_respect_robots_txt(respect_robots_txt);
  //   self
  // }

  // /// Include subdomains detection.
  
  // pub fn with_subdomains(&mut self, subdomains: bool) -> &Self {
  //   self.inner.configuration.with_subdomains(subdomains);
  //   self
  // }

  // /// Include tld detection.
  
  // pub fn with_tld(&mut self, tld: bool) -> &Self {
  //   self.inner.configuration.with_tld(tld);
  //   self
  // }

  // /// Only use HTTP/2.
  
  // pub fn with_http2_prior_knowledge(&mut self, http2_prior_knowledge: bool) -> &Self {
  //   self
  //     .inner
  //     .configuration
  //     .with_http2_prior_knowledge(http2_prior_knowledge);
  //   self
  // }

  // /// Max time to wait for request duration to milliseconds.
  
  // pub fn with_request_timeout(&mut self, request_timeout: Option<u32>) -> &Self {
  //   self
  //     .inner
  //     .configuration
  //     .with_request_timeout(match request_timeout {
  //       Some(d) => Some(Duration::from_millis(d.into())),
  //       _ => None,
  //     });
  //   self
  // }

  /// add external domains
  pub fn with_external_domains(mut slf: PyRefMut<'_, Self>, external_domains: Option<Vec<String>>) -> PyRefMut<'_, Self> {
    slf.inner.with_external_domains(match external_domains {
      Some(ext) => Some(ext.into_iter()),
      _ => None,
    });
    slf
  }

  
  // /// Set the crawling budget
  // pub fn with_budget(&mut self, budget: Option<std::collections::HashMap<String, u32>>) -> &Self {
  //   use spider::hashbrown::hash_map::HashMap;

  //   match budget {
  //     Some(d) => {
  //       let v = d
  //         .iter()
  //         .map(|e| e.0.to_owned() + "," + &e.1.to_string())
  //         .collect::<String>();
  //       let v = v
  //         .split(",")
  //         .collect::<Vec<_>>()
  //         .chunks(2)
  //         .map(|x| (x[0], x[1].parse::<u32>().unwrap_or_default()))
  //         .collect::<HashMap<&str, u32>>();

  //       self.inner.with_budget(Some(v));
  //     }
  //     _ => (),
  //   }

  //   self
  // }

  
  // /// Regex black list urls from the crawl
  // pub fn with_blacklist_url(&mut self, blacklist_url: Option<Vec<String>>) -> &Self {
  //   self
  //     .inner
  //     .configuration
  //     .with_blacklist_url(match blacklist_url {
  //       Some(v) => {
  //         let mut blacklist: Vec<CompactString> = Vec::new();
  //         for item in v {
  //           blacklist.push(CompactString::new(item));
  //         }
  //         Some(blacklist)
  //       }
  //       _ => None,
  //     });

  //   self
  // }

  // /// Setup cron jobs to run
  
  // pub fn with_cron(&mut self, cron_str: String, cron_type: Option<String>) -> &Self {
  //   self.inner.with_cron(
  //     cron_str.as_str(),
  //     if cron_type.unwrap_or_default() == "scrape" {
  //       spider::website::CronType::Scrape
  //     } else {
  //       spider::website::CronType::Crawl
  //     },
  //   );
  //   self
  // }

  // /// Delay between request as ms.
  
  // pub fn with_delay(&mut self, delay: u32) -> &Self {
  //   self.inner.configuration.with_delay(delay.into());
  //   self
  // }

  // /// Use proxies for request.
  
  // pub fn with_proxies(&mut self, proxies: Option<Vec<String>>) -> &Self {
  //   self.inner.configuration.with_proxies(proxies);
  //   self
  // }

  
  // /// build the inner website - not required for all builder_steps
  // pub fn build(&mut self) -> &Self {
  //   match self.inner.build() {
  //     Ok(w) => self.inner = w,
  //     _ => (),
  //   }
  //   self
  // }
}

/// a runner for handling crons

pub struct Cron {
  /// the runner task
  inner: spider::async_job::Runner,
  /// inner cron handle
  cron_handle: Option<JoinHandle<()>>,
}


impl Cron {
  /// stop the cron instance
  
  pub async unsafe fn stop(&mut self) {
    self.inner.stop().await;
    match &self.cron_handle {
      Some(h) => h.abort(),
      _ => (),
    }
  }
}
