use phf::phf_map;

pub static SERVER_COUNTRIES: phf::Map<&'static str, &'static str> = phf_map! {
    "" => "default",
    "UNK" => "default",
    "ARG" => "AR",
    "BHR" => "BH",
    "BRA" => "BR",
    "CHL" => "CL",
    "CHN" => "CN",
    "COL" => "CO",
    "CRI" => "CR",
    "FIN" => "FI",
    "FRA" => "FR",
    "GER" => "DE",
    "IND" => "IN",
    "IRN" => "IR",
    "KOR" => "KR",
    "KSA" => "SA",
    "NLD" => "NL",
    "PER" => "PE",
    "POL" => "PL",
    "RUS" => "RU",
    "RUS3" => "RU", // deen, please fix
    "SGP" => "SG",
    "TUR" => "TR",
    "TWN" => "TW",
    "UAE" => "UA",
    "USA" => "US",
    "ZAF" => "ZA"
};
