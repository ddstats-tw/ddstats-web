use std::collections::HashMap;

use phf::phf_map;
use regex::Regex;
use tera::{to_value, try_get_value, Error, Value};

// TODO: move this
static COUNTRY_CODES: phf::Map<i64, &'static str> = phf_map! {
    -1i64 => "default",
    901i64 => "XEN",
    902i64 => "XNI",
    903i64 => "XSC",
    904i64 => "XWA",
    905i64 => "XEU",
    906i64 => "XCA",
    737i64 => "SS",
    4i64 => "AF",
    248i64 => "AX",
    8i64 => "AL",
    12i64 => "DZ",
    16i64 => "AS",
    20i64 => "AD",
    24i64 => "AO",
    660i64 => "AI",
    28i64 => "AG",
    32i64 => "AR",
    51i64 => "AM",
    533i64 => "AW",
    36i64 => "AU",
    40i64 => "AT",
    31i64 => "AZ",
    44i64 => "BS",
    48i64 => "BH",
    50i64 => "BD",
    52i64 => "BB",
    112i64 => "BY",
    56i64 => "BE",
    84i64 => "BZ",
    204i64 => "BJ",
    60i64 => "BM",
    64i64 => "BT",
    68i64 => "BO",
    535i64 => "BQ",
    70i64 => "BA",
    72i64 => "BW",
    74i64 => "BV",
    76i64 => "BR",
    86i64 => "IO",
    96i64 => "BN",
    100i64 => "BG",
    854i64 => "BF",
    108i64 => "BI",
    116i64 => "KH",
    120i64 => "CM",
    124i64 => "CA",
    132i64 => "CV",
    136i64 => "KY",
    140i64 => "CF",
    148i64 => "TD",
    152i64 => "CL",
    156i64 => "CN",
    162i64 => "CX",
    166i64 => "CC",
    170i64 => "CO",
    174i64 => "KM",
    178i64 => "CG",
    180i64 => "CD",
    184i64 => "CK",
    188i64 => "CR",
    384i64 => "CI",
    191i64 => "HR",
    192i64 => "CU",
    531i64 => "CW",
    196i64 => "CY",
    203i64 => "CZ",
    208i64 => "DK",
    262i64 => "DJ",
    212i64 => "DM",
    214i64 => "DO",
    218i64 => "EC",
    818i64 => "EG",
    222i64 => "SV",
    226i64 => "GQ",
    232i64 => "ER",
    233i64 => "EE",
    231i64 => "ET",
    238i64 => "FK",
    234i64 => "FO",
    242i64 => "FJ",
    246i64 => "FI",
    250i64 => "FR",
    254i64 => "GF",
    258i64 => "PF",
    260i64 => "TF",
    266i64 => "GA",
    270i64 => "GM",
    268i64 => "GE",
    276i64 => "DE",
    288i64 => "GH",
    292i64 => "GI",
    300i64 => "GR",
    304i64 => "GL",
    308i64 => "GD",
    312i64 => "GP",
    316i64 => "GU",
    320i64 => "GT",
    831i64 => "GG",
    324i64 => "GN",
    624i64 => "GW",
    328i64 => "GY",
    332i64 => "HT",
    334i64 => "HM",
    336i64 => "VA",
    340i64 => "HN",
    344i64 => "HK",
    348i64 => "HU",
    352i64 => "IS",
    356i64 => "IN",
    360i64 => "ID",
    364i64 => "IR",
    368i64 => "IQ",
    372i64 => "IE",
    833i64 => "IM",
    376i64 => "IL",
    380i64 => "IT",
    388i64 => "JM",
    392i64 => "JP",
    832i64 => "JE",
    400i64 => "JO",
    398i64 => "KZ",
    404i64 => "KE",
    296i64 => "KI",
    408i64 => "KP",
    410i64 => "KR",
    414i64 => "KW",
    417i64 => "KG",
    418i64 => "LA",
    428i64 => "LV",
    422i64 => "LB",
    426i64 => "LS",
    430i64 => "LR",
    434i64 => "LY",
    438i64 => "LI",
    440i64 => "LT",
    442i64 => "LU",
    446i64 => "MO",
    807i64 => "MK",
    450i64 => "MG",
    454i64 => "MW",
    458i64 => "MY",
    462i64 => "MV",
    466i64 => "ML",
    470i64 => "MT",
    584i64 => "MH",
    474i64 => "MQ",
    478i64 => "MR",
    480i64 => "MU",
    175i64 => "YT",
    484i64 => "MX",
    583i64 => "FM",
    498i64 => "MD",
    492i64 => "MC",
    496i64 => "MN",
    499i64 => "ME",
    500i64 => "MS",
    504i64 => "MA",
    508i64 => "MZ",
    104i64 => "MM",
    516i64 => "NA",
    520i64 => "NR",
    524i64 => "NP",
    528i64 => "NL",
    540i64 => "NC",
    554i64 => "NZ",
    558i64 => "NI",
    562i64 => "NE",
    566i64 => "NG",
    570i64 => "NU",
    574i64 => "NF",
    580i64 => "MP",
    578i64 => "NO",
    512i64 => "OM",
    586i64 => "PK",
    585i64 => "PW",
    275i64 => "PS",
    591i64 => "PA",
    598i64 => "PG",
    600i64 => "PY",
    604i64 => "PE",
    608i64 => "PH",
    612i64 => "PN",
    616i64 => "PL",
    620i64 => "PT",
    630i64 => "PR",
    634i64 => "QA",
    638i64 => "RE",
    642i64 => "RO",
    643i64 => "RU",
    646i64 => "RW",
    652i64 => "BL",
    654i64 => "SH",
    659i64 => "KN",
    662i64 => "LC",
    663i64 => "MF",
    666i64 => "PM",
    670i64 => "VC",
    882i64 => "WS",
    674i64 => "SM",
    678i64 => "ST",
    682i64 => "SA",
    686i64 => "SN",
    688i64 => "RS",
    690i64 => "SC",
    694i64 => "SL",
    702i64 => "SG",
    534i64 => "SX",
    703i64 => "SK",
    705i64 => "SI",
    90i64 => "SB",
    706i64 => "SO",
    710i64 => "ZA",
    239i64 => "GS",
    724i64 => "ES",
    144i64 => "LK",
    736i64 => "SD",
    740i64 => "SR",
    744i64 => "SJ",
    748i64 => "SZ",
    752i64 => "SE",
    756i64 => "CH",
    760i64 => "SY",
    158i64 => "TW",
    762i64 => "TJ",
    834i64 => "TZ",
    764i64 => "TH",
    626i64 => "TL",
    768i64 => "TG",
    772i64 => "TK",
    776i64 => "TO",
    780i64 => "TT",
    788i64 => "TN",
    792i64 => "TR",
    795i64 => "TM",
    796i64 => "TC",
    798i64 => "TV",
    800i64 => "UG",
    804i64 => "UA",
    784i64 => "AE",
    826i64 => "GB",
    840i64 => "US",
    581i64 => "UM",
    858i64 => "UY",
    860i64 => "UZ",
    548i64 => "VU",
    862i64 => "VE",
    704i64 => "VN",
    92i64 => "VG",
    850i64 => "VI",
    876i64 => "WF",
    732i64 => "EH",
    887i64 => "YE",
    894i64 => "ZM",
    716i64 => "ZW"
};

pub fn map_thumbnail(value: &Value, _: &HashMap<String, Value>) -> Result<Value, Error> {
    let mut s = try_get_value!("map_thumbnail", "value", String, value);
    let re1 = Regex::new(r"[À-ž]").unwrap();
    let re2 = Regex::new(r"[^a-zA-Z0-9]").unwrap();
    s = re1.replace_all(&s, "__").to_string();
    s = re2.replace_all(&s, "_").to_string();
    Ok(to_value(s).unwrap())
}

pub fn fancy_time(value: &Value, _: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_secs = value.as_f64().unwrap();

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

    Ok(to_value(time_str).unwrap())
}

pub fn code_to_country(value: &Value, _: &HashMap<String, Value>) -> Result<Value, Error> {
    let country_code = value.as_i64().unwrap();

    let country_string = COUNTRY_CODES.get(&country_code).unwrap_or(&"default");

    Ok(to_value(country_string).unwrap())
}

pub fn ordinal(value: &Value, _: &HashMap<String, Value>) -> Result<Value, Error> {
    let n = value.as_i64().unwrap();

    let s = ["th", "st", "nd", "rd"];
    let v = n % 100;
    Ok(to_value(match (v - 20) % 10 {
        1 => s[0],
        2 => s[1],
        3 => s[2],
        _ => match v % 10 {
            1 => s[0],
            2 => s[1],
            3 => s[2],
            _ => s[3],
        },
    })
    .unwrap())
}

pub fn time_format(value: &Value, _: &HashMap<String, Value>) -> Result<Value, Error> {
    let seconds = value.as_u64().unwrap();

    let mut remaining = seconds;
    let years = remaining / 31536000;
    remaining %= 31536000;
    let hours = remaining / 3600;
    remaining %= 3600;
    let minutes = remaining / 60;
    remaining %= 60;

    if years > 0 {
        Ok(to_value(plural(years, "year")).unwrap())
    } else if hours > 0 {
        Ok(to_value(plural(hours, "hour")).unwrap())
    } else if minutes > 0 {
        Ok(to_value(plural(minutes, "minute")).unwrap())
    } else {
        Ok(to_value(plural(remaining, "second")).unwrap())
    }
}

fn plural(n: u64, unit: &str) -> String {
    if n == 1 {
        format!("{} {}", n, unit)
    } else {
        format!("{} {}s", n, unit)
    }
}
