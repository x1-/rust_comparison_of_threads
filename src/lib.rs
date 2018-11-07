#[macro_use]
extern crate clap;
extern crate futures;
extern crate futures_cpupool;
extern crate rayon;
extern crate tokio_threadpool;

pub mod args;
pub mod cpupool_demo;
pub mod futures_demo;
pub mod rayon_demo;
pub mod tokio_demo;

pub use args::Args;
