use crate::new_page;
use crate::NWebsite;
use crate::BUFFER;

// base website crawl
pub async fn crawl(url: String, raw_content: Option<bool>) -> NWebsite {
  let mut website = spider::website::Website::new(&url);
  let mut rx2 = website
    .subscribe(*BUFFER / 2)
    .expect("sync feature should be enabled");
  let (tx, mut rx) = spider::tokio::sync::mpsc::channel(*BUFFER);
  let raw_content = raw_content.unwrap_or_default();

  spider::tokio::spawn(async move {
    while let Ok(res) = rx2.recv().await {
      if let Err(_) = tx.send(new_page(&res, raw_content)).await {
        println!("receiver dropped");
        return;
      }
    }
  });

  spider::tokio::spawn(async move {
    website.crawl_raw().await;
  });

  let mut pages = Vec::new();

  while let Some(i) = rx.recv().await {
    pages.push(i)
  }

  let links = pages.iter().map(|x| x.url.clone()).collect::<Vec<String>>();

  NWebsite { links, pages }
}
