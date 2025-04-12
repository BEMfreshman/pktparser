pub mod parser;

use clap::Parser;
use nom::bytes::complete::tag;
use nom::error::Error;
use nom::character::complete::char;
use parser::parse;
use pcap::Capture;

#[derive(Parser)]
#[command(version = "0.1")]
#[command(name = "pcap parser")]
#[command(about = "Codes to grab data from pcap", long_about = None)]
struct Cli {
    file_name : String,

    /// Reorder the record
    #[arg(short, long)]
    reorder: bool,
}

fn main() {
    let cli = Cli::parse();
    let file_name = cli.file_name;

    parse_pcap_file(&file_name);
}

fn parse_pcap_file(file_name: &String) {

    let mut cap = Capture::from_file(file_name).expect("Failed to open pcap file");
    while let Ok(packet) = cap.next_packet() {

        // For test
        // let data = [
        //     b'b', b'6', b'0', b'3', b'4', b'a', 10, 20, 0, 0, 0, 30, 0, 0, 0, 40,
        // ];
        //let n = match_tag_label(&data);

        if match_tag_label(&packet.data) {
            match parse(&packet.data) {
                Ok((_,kospi_info)) => {
                    kospi_info.print();
                }
                Err(error) => {
                    println!("{:?}", error);
                }
            }
        } else {
            continue;
        }
    }
}

fn match_tag_label(input: &[u8]) -> bool {
    match tag::<_, _, Error<_>>(b"b6034")(input) {
        Ok((_, _)) => {
            true
        }
        Err(_) => {
            false
        }
    }
}