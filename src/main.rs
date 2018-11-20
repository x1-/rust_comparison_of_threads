extern crate rust_comparison_of_threads;

use std::time::{Duration, Instant};
// use rust_comparison_of_threads::{cpupool_demo, futures_demo, rayon_demo, tokio_demo, Args};
use rust_comparison_of_threads::{futures_preview_demo, Args};

fn main() {

    let args = Args::new();

    let start = Instant::now();

    // if args.tpe() == "cpupool" {
    //     println!("cpupool");
    //     cpupool_demo::n_tasks(args.num_threads(), args.sleep_sec());
    // } else if args.tpe() == "futures" {
    //     println!("futures");
    //     futures_demo::n_tasks(args.num_threads(), args.sleep_sec());
    // } else if args.tpe() == "tokio" {
    //     println!("tokio");
    //     tokio_demo::n_tasks(args.num_threads(), args.sleep_sec());
    // } else if args.tpe() == "rayon" {
    //     println!("rayon");
    //     rayon_demo::n_tasks(args.num_threads(), args.sleep_sec());
    if args.tpe() == "futures_preview" {
        println!("futures_preview_demo");
        futures_preview_demo::n_tasks(args.num_threads(), args.sleep_sec());
    } else {
        println!("Not implemented.");
    }

    let end = start.elapsed();
    println!("{}.{:04} sec", end.as_secs(), end.subsec_nanos() / 1_000_000);
}
