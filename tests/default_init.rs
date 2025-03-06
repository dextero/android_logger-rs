extern crate android_logger;
extern crate log;

#[test]
fn default_init() {
    android_logger::init_once(Default::default());

    // android_logger sets the default log level on the log crate to "trace" to handle the log
    // filtering itself.
    assert_eq!(log::max_level(), log::LevelFilter::Trace);
}
