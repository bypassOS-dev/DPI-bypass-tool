mod for_main;
mod all_ip;
mod help_function;
mod net_filter_queue;
mod fake_ttl;     
mod sni_parser;    
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