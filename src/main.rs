mod for_main;
mod fragmenting;
mod all_ip;
mod capute_isn;
mod help_function;
mod net_filter_queue;

use colored::Colorize;
use for_main::like_main;
#[tokio::main]
async fn main(){
    tokio::select! {
        _ = like_main() => {
            
        }
        _ = tokio::signal::ctrl_c() => {
            println!("{}", "Stop the programm's work".black().on_yellow());
            std::process::exit(0);
        }
    }
}