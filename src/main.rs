//! Mark binary entry — imperative shell only.

use mark::bootstrap::{self, Config};

#[tokio::main]
async fn main() {
    if bootstrap::maybe_print_cli_and_exit() {
        return;
    }
    bootstrap::init_tracing();
    bootstrap::serve(Config::from_env()).await;
}
