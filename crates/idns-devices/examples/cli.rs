use hbb_common::log;
use idns_eth_devices::*;

fn main() {
    use clap::App;
    let args = format!(
        "-p, --port-forward=[PORT-FORWARD-OPTIONS] 'Format: remote-id:local-port:remote-port[:remote-host]'
        -k, --key=[KEY] ''
       -s, --server... 'Start server'",
    );
    let matches = App::new("rustdesk")
        .version(crate::VERSION)
        .author("CarrieZ Studio<info@rustdesk.com>")
        .about("IdnsEth command line tool")
        .args_from_usage(&args)
        .get_matches();
    use hbb_common::env_logger::*;
    init_from_env(Env::default().filter_or(DEFAULT_FILTER_ENV, "info"));
    if let Some(p) = matches.value_of("port-forward") {
        let options: Vec<String> = p.split(":").map(|x| x.to_owned()).collect();
        if options.len() < 3 {
            log::error!("Wrong port-forward options");
            return;
        }
        let mut port = 0;
        if let Ok(v) = options[1].parse::<i32>() {
            port = v;
        } else {
            log::error!("Wrong local-port");
            return;
        }
        let mut remote_port = 0;
        if let Ok(v) = options[2].parse::<i32>() {
            remote_port = v;
        } else {
            log::error!("Wrong remote-port");
            return;
        }
        let mut remote_host = "localhost".to_owned();
        if options.len() > 3 {
            remote_host = options[3].clone();
        }
        let key = matches.value_of("key").unwrap_or("").to_owned();
        cli::start_one_port_forward(options[0].clone(), port, remote_host, remote_port, key);
    }
}
