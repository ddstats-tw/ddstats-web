use std::sync::{Arc, RwLock};

use tera::Tera;

use self::filters::*;

mod country_codes;
mod filters;

pub fn init_tera() -> Arc<RwLock<Tera>> {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("failed to initalize tera: {}", e);
            ::std::process::exit(1);
        }
    };
    let tera = Arc::new(RwLock::new(tera));

    let binding = tera.clone();
    let mut tera_writer = binding.write().unwrap();
    tera_writer.register_filter("map_thumbnail", map_thumbnail);
    tera_writer.register_filter("fancy_time", fancy_time);
    tera_writer.register_filter("code_to_country", code_to_country);
    tera_writer.register_filter("ordinal", ordinal);
    tera_writer.register_filter("time_format", time_format);
    tera_writer.register_filter("mapper_array", mapper_array);

    #[cfg(debug_assertions)]
    autoreload_templates(tera.clone(), "./templates");

    tera
}

#[cfg(debug_assertions)]
fn autoreload_templates(tera: Arc<RwLock<Tera>>, path: impl AsRef<std::path::Path>) {
    use notify::{PollWatcher, RecursiveMode};
    use notify_debouncer_mini::new_debouncer_opt;
    use std::sync::mpsc::channel;
    use std::thread;
    use std::time::Duration;

    let (tx, rx) = channel();
    let notify_config = notify::Config::default().with_poll_interval(Duration::from_secs(2));
    let debouncer_config = notify_debouncer_mini::Config::default()
        .with_timeout(Duration::from_secs(2))
        .with_notify_config(notify_config);
    let mut debouncer = new_debouncer_opt::<_, PollWatcher>(debouncer_config, tx).unwrap();
    debouncer
        .watcher()
        .watch(path.as_ref(), RecursiveMode::Recursive)
        .unwrap();

    thread::spawn(move || {
        while rx.recv().is_ok() {
            tracing::info!("Reloading Tera templates");
            let mut tera = tera.write().unwrap();
            if let Err(e) = tera.full_reload() {
                tracing::error!("Failed to reload Tera templates: {}", e);
            };
        }
        debouncer
    });
}
