# Benchmarks

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

The performance scales the larger the website and if throttling is needed. Linux benchmarks are about 10x faster than macOS for spider-rs.
