pub fn get_opt_str(opt: Option<&String>, default: &str) -> String {
    return opt.unwrap_or(&default.to_string()).to_string();
}

pub fn get_like_str(opt: Option<&String>) -> String {
    let default=String::from("");
    let opt_str = opt.unwrap_or(&default);
    return if opt_str.is_empty() {
        opt_str.to_string()
    } else {
        format!("%{}%", opt_str)
    }
}

pub fn parse_uint(opt: Option<&String>, default: &str) -> u64 {
    return opt.unwrap_or(&default.to_string()).parse::<u64>().unwrap();
}

pub fn parse_int(opt: Option<&String>, default: &str) -> i64 {
    return opt.unwrap_or(&default.to_string()).parse::<i64>().unwrap();
}