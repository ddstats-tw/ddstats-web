use tera::Tera;

use self::filters::*;

mod country_codes;
mod filters;

pub fn init_tera() -> Tera {
    let mut tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("failed to initalize tera: {}", e);
            ::std::process::exit(1);
        }
    };
    tera.register_filter("map_thumbnail", map_thumbnail);
    tera.register_filter("fancy_time", fancy_time);
    tera.register_filter("code_to_country", code_to_country);
    tera.register_filter("ordinal", ordinal);
    tera.register_filter("time_format", time_format);

    tera
}
