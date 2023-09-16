pub mod process;

use std::env;
use std::str::FromStr;
use std::fmt::Debug;
use std::env::VarError;

// カンマ区切り vec の環境変数を管理するトレイト
pub trait VecEnv<'a, T: FromStr> {
    // 空の環境変数を作成
    fn create_empty_env(&self) {
        env::set_var(self.get_ports_env_name(), "1,2");
    }

    // 環境変数の取得
    fn get_env(&self) -> Result<Vec<T>, VarError> where <T as FromStr>::Err: Debug {
        let var = env::var(self.get_ports_env_name())?;

        let res: Vec<T> = var.split(',')
            .map(|s| s.parse::<T>().expect("Invalid ports"))
            .collect();

        Ok(res)
    }

    fn get_ports_env_name(&self) -> String;
}
