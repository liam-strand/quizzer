use murder_mystery::Config;

fn main() {
    let c = Config::from_str(include_str!("../murder.toml")).expect("something went wrong reading the configuration");
    // let c = Config::new("murder.toml").expect("something went wrong reading the configuration");
    play(&c);
}

fn play(config: &Config) -> () {
    for (num, level) in config.levels.iter().enumerate() {
        level.play(config, num);
    }
}
