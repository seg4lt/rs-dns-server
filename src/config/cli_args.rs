use std::{collections::HashMap, sync::OnceLock};

use tracing::debug;

static CLI_ARGS: OnceLock<HashMap<String, String>> = OnceLock::new();

pub struct CliArgs {}

impl CliArgs {
    pub fn resolver() -> Option<String> {
        let args = CLI_ARGS.get().expect("ARGS is not initialized");
        args.get("--resolver").map(|s| s.clone())
    }
    pub fn init() {
        let arg_vec = std::env::args().collect::<Vec<String>>();
        let params = arg_vec[1..]
            .chunks(2)
            .map(|chunk| (chunk[0].clone(), chunk[1].clone()))
            .collect::<HashMap<_, _>>();
        debug!("CLI args: {:?}", params);
        CLI_ARGS.set(params).expect("unable to set ARGS once lock");
    }
}
