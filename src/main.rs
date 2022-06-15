extern crate clap;


use clap::{Arg, Command};

const CMD_MAINNET_HEIGHT_TIME: &str = "main-height-time";
const CMD_MAINNET_TIME_HEIGHT: &str = "main-time-height";
const CMD_CALIBRATION_HEIGHT_TIME: &str = "calib-height-time";
const CMD_CALIBRATION_TIME_HEIGHT: &str = "calib-time-height";


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
            Command::new(CMD_MAINNET_TIME_HEIGHT).args(vec![
                Arg::new("time")
                    .index(1)
                    .help("mainnet timestamp")
                    .takes_value(true)
                    .required(true)
            ]).about("convert mainnet timestamp to height"),

            Command::new(CMD_CALIBRATION_HEIGHT_TIME).args(vec![
                Arg::new("height")
                    .index(1)
                    .help("calibration height")
                    .takes_value(true)
                    .required(true)
            ]).about("convert calibration net height to datetime"),

            Command::new(CMD_CALIBRATION_TIME_HEIGHT).args(vec![
                Arg::new("time")
                    .index(1)
                    .help("calibration time")
                    .takes_value(true)
                    .required(true)
            ]).about("convert calibration timestamp to height"),
        ]).get_matches();

    match app.subcommand() {
        Some((CMD_MAINNET_HEIGHT_TIME, arg)) => {
            let height = arg.value_of("height").unwrap();
            let height = height.replace(",", "");
            let height = height.replace(" ", "");
            let height = height.parse::<i64>().unwrap();
            let stamp = util::lotus::mainnet_height_to_timestamp(height);
            println!("mainnet timestamp: {}", stamp);
            println!("mainnet datetime: {}", util::time::to_datetime(stamp));
        }
        Some((CMD_MAINNET_TIME_HEIGHT, arg)) => {
            let stamp = arg.value_of("time").unwrap();
            let stamp = stamp.replace(",", "");
            let stamp = stamp.replace(" ", "");
            let stamp = stamp.parse::<i64>().unwrap();
            let height = util::lotus::mainnet_timestamp_to_height(stamp);
            println!("mainnet height: {} ", height);
            println!("mainnet datetime: {}", util::time::to_datetime(stamp));
        }
        Some((CMD_CALIBRATION_HEIGHT_TIME, arg)) => {
            let height = arg.value_of("height").unwrap();
            let height = height.replace(",", "");
            let height = height.replace(" ", "");
            let height = height.parse::<i64>().unwrap();
            let stamp = util::lotus::calibration_height_to_timestamp(height);
            println!("calibration timestamp: {} ",  stamp);
            println!("calibration datetime: {}", util::time::to_datetime(stamp));
        }
        Some((CMD_CALIBRATION_TIME_HEIGHT, arg)) => {
            let stamp = arg.value_of("time").unwrap();
            let stamp = stamp.replace(",", "");
            let stamp = stamp.replace(" ", "");
            let stamp = stamp.parse::<i64>().unwrap();
            let height = util::lotus::calibration_timestamp_to_height(stamp);
            println!("mainnet height: {} ", height);
            println!("mainnet datetime: {}", util::time::to_datetime(stamp));
        }
        _ => { println!("invalid subcommand") }
    }
}
