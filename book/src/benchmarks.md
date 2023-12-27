# Benchmarks

Test url: `https://espn.com`
Mac M1 64gb 10-core CPU

| `libraries`                  | `pages`   | `speed` |
| :--------------------------- | :-------- | :------ |
| **`spider(rust): crawl`**    | `150,387` | `1m`    |
| **`spider(nodejs): crawl`**  | `150,387` | `153s`  |
| **`spider(python): crawl`**  | `150,387` | `186s`  |
| **`scrapy(python): crawl`**  | `49,598`  | `1h`    |
| **`crawlee(nodejs): crawl`** | `18,779`  | `30m`   |

View the latest runs on [github](https://github.com/spider-rs/spider-py/actions/workflows/bench.yml).

```sh
-----------------------
Linux
2-core CPU
7 GB of RAM memory
-----------------------
```

Test url: `https://choosealicense.com` (small)
32 pages

| `libraries`                       | `speed` |
| :-------------------------------- | :------ |
| **`spider-rs: crawl 10 samples`** | `76ms`  |
| **`scrapy: crawl 10 samples`**    | `2s`    |

Test url: `https://rsseau.fr` (medium)
211 pages

| `libraries`                       | `speed` |
| :-------------------------------- | :------ |
| **`spider-rs: crawl 10 samples`** | `3s`    |
| **`scrapy: crawl 10 samples`**    | `8s`    |

```sh
----------------------
mac Apple M1 Max
10-core CPU
64 GB of RAM memory
-----------------------
```

Test url: `https://choosealicense.com` (small)
32 pages

| `libraries`                       | `speed` |
| :-------------------------------- | :------ |
| **`spider-rs: crawl 10 samples`** | `286ms` |
| **`scrapy: crawl 10 samples`**    | `2.5s`  |

Test url: `https://rsseau.fr` (medium)
211 pages

| `libraries`                       | `speed` |
| :-------------------------------- | :------ |
| **`spider-rs: crawl 10 samples`** | `2.5s`  |
| **`scrapy: crawl 10 samples`**    | `10s`   |

Test url: `https://a11ywatch.com` (medium)
648 pages

| `libraries`                       | `speed` |
| :-------------------------------- | :------ |
| **`spider-rs: crawl 10 samples`** | `2s`    |
| **`scrapy: crawl 10 samples`**    | `7.7s`  |

Test url: `https://espn.com` (large)
150,387 pages

| `libraries`                       | `pages`   | `speed` |
| :-------------------------------- | :-------- | :------ |
| **`spider-rs: crawl 10 samples`** | `150,387` | `186s`  |
| **`scrapy: crawl 10 samples`**    | `49,598`  | `1h`    |

Scrapy used too much memory, crawl cancelled after an hour.

Note: The performance scales the larger the website and if throttling is needed. Linux benchmarks are about 10x faster than macOS for spider-rs.
