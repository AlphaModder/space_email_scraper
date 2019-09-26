use self::filter;


pub struct Arguments {
    pub filter: Filter,
    pub emails: usize,
    pub search_volume: usize,
    pub save_path: Option<PathBuf>,
    pub tries: u32,
    pub reconnect: Option<Duration>,
    pub cooldown: Duration,
}

impl Default for Arguments {
    fn default() -> Arguments {
        Arguments {
            filter: Filter::Empty,
            emails: std::usize::MAX,
            search_volume: std::usize::MAX,
            save_path: None,
            tries: 10,
            reconnect: Some(Duration::from_seconds(30)),
            cooldown: Duration::from_millis(300)
        }
    }
}