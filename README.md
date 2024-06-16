### Currently 2 Executable project


```bash
# Dependency index lookup layer 
cargo run --bin index_scanner

# repo package crawler
cargo run --bin deps_fetcher

```

> Docs <br>
https://doc.rust-lang.org/cargo/reference/registry-index.html#index-files

### Example response 
```
Dependency {
    name: "base64",
    vers: "0.22.1",
    deps: [
        Dep {
            name: "clap",
            req: "^3.2.25",
            features: [
                "derive",
            ],
            optional: false,
            default_features: true,
            target: None,
            kind: Some(
                "dev",
            ),
            crates_url: Some(
                "https:/crates.io/crates/clap",
            ),
        },
        Dep {
            name: "criterion",
            req: "^0.4.0",
            features: [],
            optional: false,
            default_features: true,
            target: None,
            kind: Some(
                "dev",
            ),
            crates_url: Some(
                "https:/crates.io/crates/criterion",
            ),
        },
        Dep {
            name: "once_cell",
            req: "^1",
            features: [],
            optional: false,
            default_features: true,
            target: None,
            kind: Some(
                "dev",
            ),
            crates_url: Some(
                "https:/crates.io/crates/once_cell",
            ),
        },
        Dep {
            name: "rand",
            req: "^0.8.5",
            features: [
                "small_rng",
            ],
            optional: false,
            default_features: true,
            target: None,
            kind: Some(
                "dev",
            ),
            crates_url: Some(
                "https:/crates.io/crates/rand",
            ),
        },
        Dep {
            name: "rstest",
            req: "^0.13.0",
            features: [],
            optional: false,
            default_features: true,
            target: None,
            kind: Some(
                "dev",
            ),
            crates_url: Some(
                "https:/crates.io/crates/rstest",
            ),
        },
        Dep {
            name: "rstest_reuse",
            req: "^0.6.0",
            features: [],
            optional: false,
            default_features: true,
            target: None,
            kind: Some(
                "dev",
            ),
            crates_url: Some(
                "https:/crates.io/crates/rstest_reuse",
            ),
        },
        Dep {
            name: "strum",
            req: "^0.25",
            features: [
                "derive",
            ],
            optional: false,
            default_features: true,
            target: None,
            kind: Some(
                "dev",
            ),
            crates_url: Some(
                "https:/crates.io/crates/strum",
            ),
        },
    ],
    cksum: "72b3254f16251a8381aa12e40e3c4d2f0199f8c6508fbecb9d91f575e0fbb8c6",
    features: {
        "alloc": [],
        "default": [
            "std",
        ],
        "std": [
            "alloc",
        ],
    },
    yanked: false,
    rust_version: Some(
        "1.48.0",
    ),
}
```