pub mod parser;

use clap::Parser;
use parser::parse;
use std::{fs::File, io::Read};
use pcap_file::pcap::PcapReader;

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

use nom::bytes::complete::take;
fn main() {
    let cli = Cli::parse();
    let file_name = cli.file_name;

    parse_pcap_file(&file_name);
}

fn parse_pcap_file(file_name: &String) -> () {

    let file_in = File::open(file_name).expect("Error opening");
    let mut pcap_reader = PcapReader::new(file_in).unwrap();

    while let Some(pkt) = pcap_reader.next_packet() {
        let pkt = pkt.unwrap();
        
        match parse(&pkt.data) {
            Ok((_, kospi_info)) => {
                kospi_info.print();
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
}