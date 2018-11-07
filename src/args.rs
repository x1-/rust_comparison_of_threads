use clap::App;

#[derive(PartialEq, Clone, Debug, Default)]
pub struct Args {
    tpe        : String,
    num_threads: usize,
    sleep_sec  : u64,
}

#[macro_use]
impl Args {
    ///
    /// Args を初期化して返却します.
    /// `args.yml` を読み込みコマンドライン・オプションとして使用します.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let args = Args::new();
    /// println!(args.select());
    /// ```
    /// 
    pub fn new() -> Self {
        let yaml    = load_yaml!("args.yml");
        let matches = App::from_yaml(yaml).get_matches();
        Self {
            tpe        : matches.value_of("type").unwrap_or("cpupool").to_string(),
            num_threads: matches.value_of("num_threads").unwrap_or("1").parse::<usize>().unwrap_or(1),
            sleep_sec  : matches.value_of("sleep_sec").unwrap_or("0").parse::<u64>().unwrap_or(0),
        }
    }
    pub fn tpe(&self) -> &str {
        &self.tpe
    }
    pub fn num_threads(&self) -> usize {
        self.num_threads
    }
    pub fn sleep_sec(&self) -> u64 {
        self.sleep_sec
    }
}
