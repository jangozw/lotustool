extern crate clap;


use clap::{Arg, Command};

const CMD_MAINNET_HEIGHT_TIME: &str = "main-height-date";
const CMD_CALIBRATION_HEIGHT_TIME: &str = "calib-height-date";


fn main() {
    let app = Command::new("lotustool")
        .version("1.0.0")
        .author("Django")
        .about("lotus tool")
        .subcommands(vec![
            Command::new(CMD_MAINNET_HEIGHT_TIME).args(vec![
                Arg::new("height")
                    .index(1)
                    .help("mainnet height")
                    .takes_value(true)
                    .required(true)
            ]).about("convert mainnet height to datetime"),
            Command::new(CMD_CALIBRATION_HEIGHT_TIME).args(vec![
                Arg::new("height")
                    .index(1)
                    .help("calibration height")
                    .takes_value(true)
                    .required(true)
            ]).about("convert calibration net height to datetime"),
        ]).get_matches();

    match app.subcommand() {
        Some((CMD_MAINNET_HEIGHT_TIME, arg)) => {
            let height = arg.value_of("height").unwrap();
            let height = height.replace(",", "");
            let height = height.replace(" ", "");
            let height = height.parse::<u64>().unwrap();
            let d = util::lotus::mainnet_height_to_datetime(height);
            println!("mainnet height: {} ", height);
            println!("mainnet date: {}", d);
        }
        Some((CMD_CALIBRATION_HEIGHT_TIME, arg)) => {
            let height = arg.value_of("height").unwrap();
            let height = height.replace(",", "");
            let height = height.replace(" ", "");
            let height = height.parse::<u64>().unwrap();
            let d = util::lotus::calibration_height_to_datetime(height);
            println!("calibration height: {} ", height);
            println!("calibration date: {} ",  d);

        }
        _ => { println!("invalid subcommand") }
    }
}
