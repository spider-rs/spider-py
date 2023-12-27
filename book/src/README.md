# Introduction

`Spider-Py` is the fastest web crawler and indexer written in Rust ported to Python.

- Concurrent
- Streaming
- Decentralization
- Headless Chrome [Rendering](https://github.com/mattsse/chromiumoxide)
- HTTP Proxies
- Cron Jobs
- Subscriptions
- Blacklisting and Budgeting Depth
- Written in [Rust](https://www.rust-lang.org/) for speed, safety, and simplicity

Spider powers some big tools and helps bring the crawling aspect to almost no downtime with the correct setup, view the [spider](https://github.com/spider-rs/spider) project to learn more.

Test url: `https://espn.com`

| `libraries`                    | `pages`   | `speed` |
| :----------------------------- | :-------- | :------ |
| **`spider-rs(python): crawl`** | `150,387` | `186s`  |
| **`scrapy(python): crawl`**    | `49,598`  | `1h`    |

The benches above were ran on a mac m1, spider on linux arm machines performs about 2-10x faster.
