#![allow(non_snake_case)]

use clap::Arg;
use clap::ArgAction;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn readFile(path: &Path) -> io::Result<Vec<u8>> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    // Read file into vector.
    reader.read_to_end(&mut buffer)?;
    return Ok(buffer);
}

#[derive(Debug)]
struct DisplayInfo {
    pub shortName: String,
    pub serialNumber: u32,
}

#[derive(Debug)]
struct MapRule {
    pub variableName: String,
    pub serialNumber: u32,
}

// use std::path::Path;

// fn fileExists(path: & str)->bool{
//     return Path::new(path).exists();
// }

fn getDisplayInfo() -> Result<Vec<DisplayInfo>> {
    let devices = fs::read_dir("/sys/class/drm")?;
    let mut ret: Vec<DisplayInfo> = Vec::new();
    let prefixRegex = Regex::new(r"^card\d-")?;
    for devPath in devices {
        let namedDir = devPath?.path();
        let path = namedDir.as_path().join("edid");
        if path.exists() {
            let bytes = readFile(&path)?;
            if bytes.is_empty() {
                continue;
            }
            let edid = edid_rs::parse(&mut Cursor::new(bytes));

            let deviceName = prefixRegex
                .replace(
                    namedDir.as_path().file_name().unwrap().to_str().unwrap(),
                    "",
                )
                .to_string();

            ret.push(DisplayInfo {
                shortName: deviceName,
                serialNumber: edid?.product.serial_number,
            });
        }
    }
    return Ok(ret);
}

fn printExistingDisplays() {
    let displays = getDisplayInfo().unwrap();
    for displayInfo in displays {
        println!(
            "Short Name: {}   Serial: {}",
            displayInfo.shortName, displayInfo.serialNumber
        );
    }
}

fn printMappedConfig(rules: Vec<MapRule>) {
    let displays = getDisplayInfo()
        .unwrap()
        .iter()
        .map(|x| (x.serialNumber, x.shortName.clone()))
        .collect::<HashMap<_, _>>();

    for rule in rules {
        let shortName: String;

        if displays.contains_key(&rule.serialNumber) {
            shortName = displays.get(&rule.serialNumber).unwrap().clone();
        } else {
            shortName = "Unknown".to_owned();
            println!("# No display for serial number {}", rule.serialNumber);
        }
        println!("set ${} {}", rule.variableName, shortName);
    }
    // println!("displays {:?}", displays);
    // println!("rules {:?}", rules);
}

fn main() {
    let matches = clap::command!()
        .arg(
            Arg::new("list")
                .long("list")
                .help("Display table of Short Names and Serial Numbers")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("map")
                .long("map")
                .value_name("VAR:S/N")
                .num_args(1)
                .help("Mappings rule from Sway Variable Name to Serial Number (multiple allowed)")
                .action(ArgAction::Append),
        )
        .get_matches();

    // println!("LIST:: {:?}", matches.get_one::<bool>("list"));

    if *matches.get_one::<bool>("list").unwrap_or(&false) {
        printExistingDisplays();
        return;
    }

    let mapRules = matches.get_occurrences::<String>("map");
    if mapRules.is_some() {
        let vals: Vec<MapRule> = mapRules
            .unwrap()
            .map(Iterator::collect)
            .map(|x: Vec<&String>| {
                let chunks: Vec<&str> = x.first().unwrap().split(":").collect();
                return MapRule {
                    variableName: chunks[0].to_string(),
                    serialNumber: chunks[1].parse::<u32>().unwrap(),
                };
            })
            .collect();
        printMappedConfig(vals);
        return;
    }
}
