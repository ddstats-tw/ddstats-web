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
    use notify::{RecommendedWatcher, RecursiveMode};
    use notify_debouncer_mini::new_debouncer_opt;
    use std::sync::mpsc::channel;
    use std::thread;

    let (tx, rx) = channel();
    let debouncer_config = notify_debouncer_mini::Config::default();
    let mut debouncer = new_debouncer_opt::<_, RecommendedWatcher>(debouncer_config, tx).unwrap();
    debouncer
        .watcher()
        .watch(path.as_ref(), RecursiveMode::Recursive)
        .unwrap();

    thread::spawn(move || {
        while rx.recv().is_ok() {
            let _ = rx.recv().unwrap(); // wtf?

            tracing::info!("Reloading Tera templates");
            let mut tera = tera.write().unwrap();
            if let Err(e) = tera.full_reload() {
                tracing::error!("Failed to reload Tera templates: {}", e);
            };
        }
        debouncer
    });
}
