use pgrx::prelude::*;

pgrx::pg_module_magic!();

mod errors;
mod network;
use errors::balance::BalanceError;
mod rpc;

#[pg_extern]
fn pg_set_network(network: &str) -> bool {
    let set_sql = format!("SELECT set_config('pg_vars.network','{}',false);", network);
    Spi::run(&set_sql).ok();
    true
}

#[pg_extern]
fn pg_get_network() -> Option<String> {
    let set_sql = "SELECT current_setting('pg_vars.network');";
    Spi::get_one(&set_sql).ok()?
}
