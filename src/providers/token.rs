use std::{collections::HashMap, env, fs, io};

use super::provider::{Ident, Provider};

pub struct BasicTokenPool {
    pool: HashMap<String, Ident>,
}

impl BasicTokenPool {
    fn parse_line(l: &String) -> Option<(String, Ident)> {
        let split: Vec<&str> = l.split(&['=', ':', ' '][..]).collect();
        if split.len() < 2 {
            return None;
        }
        let ident = Ident {
            ident: split[0].trim().to_string(),
        };
        let token = split[1].trim().to_string();
        Some((token, ident))
    }

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

        let pool = pool
            .iter()
            .map(BasicTokenPool::parse_line)
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();

        Ok(BasicTokenPool { pool })
    }
}

impl Provider for BasicTokenPool {
    fn check(&self, token: &String) -> Option<&Ident> {
        self.pool.get(token)
    }
}
