#[cfg(test)]
mod tests {
    use std::assert_matches;

    #[test]
    fn db_url_env_var_is_non_empty() {
        assert_matches!(std::env::var("DATABASE_URL"), Ok(val) if !val.is_empty());
    }
}
