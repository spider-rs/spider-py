# Benches

You can run the benches with python in terminal.

```sh
python scrappy.py && python spider.py
```

## Cases

```
mac Apple M1 Max
10-core CPU
64 GB of RAM memory
```

URL used `https://rsseau.fr`

[Scrapy](scrapy.py)

```
Scrapy
pages found 188
elasped duration 9.301506042480469
```

[Spider-RS](spider.py)

```
Spider
pages found 200
elasped duration 5.860108852386475
```

Test url: `https://a11ywatch.com` (medium)
648 pages

| `libraries`                       | `speed` |
| :-------------------------------- | :------ |
| **`spider-rs: crawl 10 samples`** | `2s`    |
| **`scrapy: crawl 10 samples`**    | `7.7s`  |

Test url: `https://espn.com` (large)
150,387 pages

| `libraries`                               | `pages`   | `speed` |
| :---------------------------------------- | :-------- | :------ |
| **`spider-rs(python): crawl 10 samples`** | `150,387` | `186s`  |
| **`scrapy(python): crawl 10 samples`**    | `49,598`  | `1h`    |

Scrapy used too much memory, crawl cancelled after an hour.

Note: The performance scales the larger the website and if throttling is needed. Linux benchmarks are about 10x faster than macOS for spider-rs.
