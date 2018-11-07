# rust_comparison_of_threads
Comparison of the libraries treating threads by Rust.

## Objects

- [rayon](https://github.com/rayon-rs/rayon)
- [tokio-threadpool](https://github.com/tokio-rs/tokio/tree/master/tokio-threadpool)
- [futures-cpupool](https://crates.io/crates/futures-cpupool)
- [futures-0.3](https://rust-lang-nursery.github.io/futures-rs/)

### tokio-threadpool say...

> **Why not Rayon?**  
> Rayon does not provide any guarantees of fairness with regards to how each task gets scheduled.  
> On the other hand, tokio-threadpool is a general purpose scheduler and attempts to schedule each task fairly.  


> **Why not futures-cpupool?**  
> It's 10x slower.  

