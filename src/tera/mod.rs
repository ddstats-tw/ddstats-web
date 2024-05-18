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

    autoreload_templates(tera.clone(), "./templates");
    tera
}

fn autoreload_templates(tera: Arc<RwLock<Tera>>, path: impl AsRef<std::path::Path>) {
    use notify::{watcher, RecursiveMode, Watcher};
    use std::sync::mpsc::channel;
    use std::thread;
    use std::time::Duration;

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(30)).unwrap();
    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    thread::spawn(move || {
        while rx.recv().is_ok() {
            tracing::info!("Reloading Tera templates");
            let mut tera = tera.write().unwrap();
            match tera.full_reload() {
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("Failed to reload Tera templates: {}", e);
                }
            };
        }
        watcher
    });
}
