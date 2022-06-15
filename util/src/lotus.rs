extern crate chrono;

use std::time::{Duration, UNIX_EPOCH};

use chrono::Local;
use chrono::prelude::DateTime;

const MAIN_NET_HEIGHT: u64 = 148888;
// base Beijing time
const MAIN_NET_TIMESTAMP: u64 = 1602773040;

const CALIBRATION_HEIGHT: u64 = 1039920;
// Beijing time
const CALIBRATION_TIMESTAMP: u64 = 1655258400;

// 1655196193
pub fn timestamp_to_mainnet_height(timestamp: u64) -> u64 {
    (timestamp - MAIN_NET_TIMESTAMP) / 30 + MAIN_NET_HEIGHT
}

pub fn mainnet_height_to_timestamp(height: u64) -> u64 {
    (height - MAIN_NET_HEIGHT) * 30 + MAIN_NET_TIMESTAMP
}

pub fn mainnet_height_to_datetime(height: u64) -> String {
    let stamp = (height - MAIN_NET_HEIGHT) * 30 + MAIN_NET_TIMESTAMP;
    // Creates a new SystemTime from the specified number of whole seconds
    let d = UNIX_EPOCH + Duration::from_secs(stamp);
    // Create DateTime from SystemTime
    let datetime = DateTime::<Local>::from(d);
    // Formats the combined date and time with the specified format string.
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn timestamp_to_calibration_height(timestamp: u64) -> u64 {
    (timestamp - CALIBRATION_TIMESTAMP) / 30 + CALIBRATION_HEIGHT
}

pub fn calibration_height_to_timestamp(height: u64) -> u64 {
    (height - CALIBRATION_HEIGHT) * 30 + CALIBRATION_TIMESTAMP
}

pub fn calibration_height_to_datetime(height: u64) -> String {
    let stamp = (height - CALIBRATION_HEIGHT) * 30 + CALIBRATION_TIMESTAMP;
    // Creates a new SystemTime from the specified number of whole seconds
    let d = UNIX_EPOCH + Duration::from_secs(stamp);
    // Create DateTime from SystemTime
    let datetime = DateTime::<Local>::from(d);
    // Formats the combined date and time with the specified format string.
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}




