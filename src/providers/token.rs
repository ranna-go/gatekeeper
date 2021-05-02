use std::{env, fs, io};

pub struct BasicTokenPool {
    pool: Vec<String>,
}

impl BasicTokenPool {
    fn from_env(key: &str) -> Vec<String> {
        std::env::var(key)
            .unwrap_or_default()
            .split(",")
            .map(|v| v.trim().to_string())
            .collect()
    }

    fn from_file(file_name: String) -> io::Result<Vec<String>> {
        let res = fs::read_to_string(file_name)?;
        Ok(res.split("\n").map(|v| v.trim().to_string()).collect())
    }

    pub fn new() -> io::Result<BasicTokenPool> {
        let mut pool = BasicTokenPool::from_env("BASICTOKENPOOL");

        if let Ok(file_name) = env::var("BASICTOKENFILE") {
            pool.append(&mut BasicTokenPool::from_file(file_name)?);
        }

        Ok(BasicTokenPool { pool })
    }

    pub fn check(&self, token: &String) -> bool {
        self.pool.contains(token)
    }
}
