#![deny(clippy::all)]

use spider::lazy_static::lazy_static;

lazy_static! {
  pub static ref BUFFER: usize = (num_cpus::get() * 20).max(88);
}

pub mod npage;
pub mod nwebsite;
pub mod page;
pub mod shortcut;
pub mod website;

pub use npage::{page_title, NPage};
pub use nwebsite::NWebsite;
pub use page::Page;
pub use shortcut::crawl;
pub use website::Website;
