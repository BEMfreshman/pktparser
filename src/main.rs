pub mod parser;

use clap::Parser;
use nom::bytes::complete::take_until;
use nom::error::Error;
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

        if let Ok((_,input)) = take_until::<_, _, Error<_>>(b"B6034".as_slice())(packet.data){
            match parse(input) {
                Ok((_,kospi_info)) => {
                    kospi_info.print();
                }
                Err(error) => {
                    println!("{:?}", error);
                }
            }
        }
    }
}