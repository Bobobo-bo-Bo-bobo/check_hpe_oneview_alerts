use crate::constants;

pub fn show_version() {
    println!(
        "{} version {}
Copyright (C) 2021-2022 by Andreas Maus <maus@ypbind.de>
This program comes with ABSOLUTELY NO WARRANTY.
    
{} is distributed under the Terms of the GNU General
Public License Version 3. (http://www.gnu.org/copyleft/gpl.html)
",
        constants::NAME,
        constants::VERSION,
        constants::NAME
    );
}

pub fn show_usage() {
    show_version();
    println!("Usage: {} [-C <ca>|--ca-file=<ca>] -H <host>|--host=<host> -P <file>|--password-file=<file> [-V|--version]
  [-h|--help] [-i|--insecure] -p <pass>|--password=<pass> -u <user>|--user=<user>

    -C <ca>                 CA certificate for validation of the remote SSL certificate
    --ca-file=<ca>

    -H <host>               Hostname (or IP) of the HPE OneView system
    --host=<host>           This option is mandatory

    -P <file>               Read password from file. Only the first line
    --password-file=<file>  of the file is read. -P/--password-file and
                            -p/--password are mutually excludsive.

    -V                      Show version information
    --version

    -h                      Show help text
    --help

    -i                      Don't validate remote certificate
    --insecure

    -p <pass>               Password for authentication
    --password=<pass>       -P/--password-file and -p/--password are
                            mutually exclusive.

    -u <user>               Username for authentication
    --user=<user>           This option is mandatory
", constants::NAME);
}
