[![crates.io](https://img.shields.io/crates/v/iter_fallback.svg)](https://crates.io/crates/iter_fallback)

# iter_fallback
An iterator that falls back to a second iterator when the first one is exusted.
```rs
 assert_eq!(
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3]
                .into_iter()
                .fallback(vec![0, 0, 0, 4, 5].into_iter())
                .collect::<Vec<_>>()
        )

```
