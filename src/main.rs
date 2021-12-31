#[macro_use]
extern crate simple_error;

mod constants;
mod data;
mod hpe_oneview;
mod json;
mod nagios;
mod usage;

use getopts::Options;
use std::{env, process};

fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut options = Options::new();
    let mut insecure_ssl: bool = false;
    let mut ca_cert = Vec::<u8>::new();

    options.optopt("C", "ca-file", "CA certificate", "ca_file");
    options.optopt(
        "H",
        "host",
        "Hostname or IP of the HPE Oneview system",
        "hostname",
    );
    options.optopt(
        "P",
        "password-file",
        "Read password from <password_file>",
        "password_file",
    );
    options.optflag("V", "version", "Show version information");
    options.optflag("h", "help", "Show help text");
    options.optflag(
        "i",
        "insecure",
        "Disable certificate validation of the remote SSL certificate",
    );
    options.optopt("p", "password", "Authentication password", "password");
    options.optopt("u", "user", "Authentication user", "user");

    let opts = match options.parse(&argv[1..]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: Can't parse command line: {}", e);
            process::exit(nagios::UNKNOWN);
        }
    };

    if opts.opt_present("h") {
        usage::show_usage();
        process::exit(nagios::OK);
    }

    if opts.opt_present("V") {
        usage::show_version();
        process::exit(nagios::OK);
    }

    if opts.opt_present("i") {
        insecure_ssl = true;
    }

    let hostname = match opts.opt_str("H") {
        Some(v) => v,
        None => {
            eprintln!("Error: Hostname or IP address of the HPE OneView system is mandatory");
            println!();
            usage::show_usage();
            process::exit(nagios::UNKNOWN);
        }
    };

    let ca_file = match opts.opt_str("C") {
        Some(v) => v,
        None => String::new(),
    };

    let password_file = match opts.opt_str("P") {
        Some(v) => v,
        None => String::new(),
    };

    let mut password = match opts.opt_str("p") {
        Some(v) => v,
        None => String::new(),
    };

    let username = match opts.opt_str("u") {
        Some(v) => v,
        None => {
            eprintln!("Error: User name for authentication is mandatory");
            process::exit(nagios::UNKNOWN);
        }
    };

    if password.is_empty() && password_file.is_empty() {
        eprintln!("Error: No authentication password provided");
        println!();
        usage::show_usage();
        process::exit(nagios::UNKNOWN);
    };

    if !password.is_empty() && !password_file.is_empty() {
        eprintln!(
            "Error: Either provide authentication on the command line or from a file, not both"
        );
        println!();
        usage::show_usage();
        process::exit(nagios::UNKNOWN);
    };

    password = match data::read_password_from_file(&password_file) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "Error: Can't read password from file {}: {}",
                password_file, e
            );
            process::exit(nagios::UNKNOWN);
        }
    };

    if !ca_file.is_empty() {
        ca_cert = match data::read_file(&ca_file) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error: Can't read CA from {}: {}", ca_file, e);
                process::exit(nagios::UNKNOWN);
            }
        };
    }

    match hpe_oneview::check_alerts(&hostname, &username, &password, &ca_cert, insecure_ssl) {
        Ok(v) => {
            println!(
                "{} - {}",
                nagios::STATUS_PREFIX[v.status as usize],
                v.message
            );
            process::exit(v.status);
        }
        Err(e) => {
            println!("UNKNOWN - {}", e);
            process::exit(nagios::UNKNOWN);
        }
    };
}
