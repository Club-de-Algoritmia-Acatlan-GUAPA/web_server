pub mod authentication;
pub mod broker;
pub mod configuration;
pub mod database;
pub mod domain;
pub mod email_client;
pub mod filters;
pub mod ftp;
pub mod pubsub;
pub mod relations;
pub mod rendering;
pub mod routes;
pub mod routing;
pub mod session;
pub mod startup;
pub mod status;
pub mod telemetry;
pub mod utils;
pub mod with_axum;

const ALPHABET: [&str; 52] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z", "AA", "AB", "AC", "AD", "AE", "AF", "AG", "AH", "AI", "AJ",
    "AK", "AL", "AM", "AN", "AO", "AP", "AQ", "AR", "AS", "AT", "AU", "AV", "AW", "AX", "AY", "AZ",
];
