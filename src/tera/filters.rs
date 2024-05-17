use regex::Regex;
use std::collections::HashMap;
use tera::{to_value, try_get_value, Error, Value};

use super::country_codes::COUNTRY_CODES;

pub fn map_thumbnail(value: &Value, _: &HashMap<String, Value>) -> Result<Value, Error> {
    let mut s = try_get_value!("map_thumbnail", "value", String, value);
    let re1 = Regex::new(r"[À-ž]").unwrap();
    let re2 = Regex::new(r"[^a-zA-Z0-9]").unwrap();
    s = re1.replace_all(&s, "__").to_string();
    s = re2.replace_all(&s, "_").to_string();
    Ok(to_value(s)?)
}

pub fn fancy_time(value: &Value, _: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_secs = value.as_f64().unwrap_or(0.0);

    let seconds = input_secs.floor();
    let hour_secs = (seconds % 3600.0).floor();
    let hours = (seconds - hour_secs) / 3600.0;
    let min_secs = (seconds - hours * 3600.0).floor() % 60.0;
    let mins = (seconds - hours * 3600.0 - min_secs) / 60.0;
    let secs_millis = input_secs - hours * 3600.0 - mins * 60.0;
    let secs = secs_millis.floor();
    let millis = ((secs_millis - secs) * 100.0).round();

    let mut time_str = String::new();
    if hours > 0.0 {
        time_str.push_str(&format!("{}:", hours));
    }

    time_str.push_str(&format!("{:02}:", mins));
    time_str.push_str(&format!("{:02}", secs));
    // if !remove_decimals {
    time_str.push_str(&format!(".{:02}", millis));
    // }

    Ok(to_value(time_str)?)
}

pub fn code_to_country(value: &Value, _: &HashMap<String, Value>) -> Result<Value, Error> {
    let country_code = value.as_i64().unwrap_or(-1);

    let country_string = COUNTRY_CODES.get(&country_code).unwrap_or(&"default");

    Ok(to_value(country_string)?)
}

pub fn ordinal(value: &Value, _: &HashMap<String, Value>) -> Result<Value, Error> {
    let s = value.as_i64().unwrap_or(0).to_string();

    if s.ends_with('1') && !s.ends_with("11") {
        Ok(to_value("st")?)
    } else if s.ends_with('2') && !s.ends_with("12") {
        Ok(to_value("nd")?)
    } else if s.ends_with('3') && !s.ends_with("13") {
        Ok(to_value("rd")?)
    } else {
        Ok(to_value("th")?)
    }
}

pub fn time_format(value: &Value, _: &HashMap<String, Value>) -> Result<Value, Error> {
    let seconds = value.as_u64().unwrap_or(0);

    let mut remaining = seconds;
    let years = remaining / 31536000;
    remaining %= 31536000;
    let hours = remaining / 3600;
    remaining %= 3600;
    let minutes = remaining / 60;
    remaining %= 60;

    if years > 0 {
        Ok(to_value(plural(years, "year"))?)
    } else if hours > 0 {
        Ok(to_value(plural(hours, "hour"))?)
    } else if minutes > 0 {
        Ok(to_value(plural(minutes, "minute"))?)
    } else {
        Ok(to_value(plural(remaining, "second"))?)
    }
}

fn plural(n: u64, unit: &str) -> String {
    if n == 1 {
        format!("{} {}", n, unit)
    } else {
        format!("{} {}s", n, unit)
    }
}

pub fn mapper_array(value: &Value, _: &HashMap<String, Value>) -> Result<Value, Error> {
    let v = value.as_str().unwrap();
    let r: Vec<_> = v.split([',', '&']).map(|s| s.trim()).collect();
    Ok(to_value(r)?)
}
