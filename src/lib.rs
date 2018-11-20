#![feature(pin, arbitrary_self_types, async_await, await_macro, futures_api)]
extern crate core;
#[macro_use]
extern crate clap;
extern crate futures;
// extern crate futures_cpupool;
extern crate rayon;
extern crate tokio_threadpool;

pub mod args;
// pub mod cpupool_demo;
// pub mod futures_demo;
// pub mod rayon_demo;
// pub mod tokio_demo;
pub mod futures_preview_demo;

pub use args::Args;
