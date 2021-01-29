use rbatis::plugin::snowflake;

pub fn new_id() -> String {
    return snowflake::block_snowflake_id().to_string();
}