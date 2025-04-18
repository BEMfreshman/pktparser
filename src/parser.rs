#[derive(Debug)]
pub struct KospiFormat {
    issue_code: String,

    best_bid_price_1st: f64,
    best_bid_quantity_1st: u64,
    best_bid_price_2nd: f64,
    best_bid_quantity_2nd: u64,
    best_bid_price_3rd: f64,
    best_bid_quantity_3rd: u64,
    best_bid_price_4th: f64,
    best_bid_quantity_4th: u64,
    best_bid_price_5th: f64,
    best_bid_quantity_5th: u64,

    best_ask_price_1st: f64,
    best_ask_quantity_1st: u64,
    best_ask_price_2nd: f64,
    best_ask_quantity_2nd: u64,
    best_ask_price_3rd: f64,
    best_ask_quantity_3rd: u64,
    best_ask_price_4th: f64,
    best_ask_quantity_4th: u64,
    best_ask_price_5th: f64,
    best_ask_quantity_5th: u64,

    pkt_time: PktTime,
}

impl KospiFormat {
    pub fn print(&self) {

        println!("{:?}", self.issue_code);
        println!("{:?}@{:?}", self.best_bid_quantity_1st, self.best_bid_price_1st);
        println!("{:?}@{:?}", self.best_bid_quantity_2nd, self.best_bid_price_2nd);
        println!("{:?}@{:?}", self.best_bid_quantity_3rd, self.best_bid_price_3rd);
        println!("{:?}@{:?}", self.best_bid_quantity_4th, self.best_bid_price_4th);
        println!("{:?}@{:?}", self.best_bid_quantity_5th, self.best_bid_price_5th);
    }
}

#[derive(Debug)]
pub struct PktTime {
    hour: u8,
    min: u8,
    second: u8,
    micro: u32,
}

use nom::bytes::complete::take;
use nom::number::complete::{u8, le_u32};
use nom::IResult;


pub fn parse(input: &[u8]) -> IResult<&[u8],KospiFormat> {

    let (input,_) = take(5usize)(input)?;
    let (input, issue_code) = read_string(input, 12)?;
    let (input,_) = take(12usize)(input)?;

    let(input, best_bid_price_1st) = read_price(input)?;
    let(input,best_bid_quantity_1st) = read_quantity(input)?;

    let(input, best_bid_price_2nd) = read_price( input)?;
    let(input,best_bid_quantity_2nd) = read_quantity(input)?;

    let(input, best_bid_price_3rd) = read_price(input)?;
    let(input,best_bid_quantity_3rd) = read_quantity(input)?;

    let(input, best_bid_price_4th) = read_price( input)?;
    let(input,best_bid_quantity_4th) = read_quantity(input)?;

    let(input, best_bid_price_5th) = read_price(input)?;
    let(input,best_bid_quantity_5th) = read_quantity(input)?;

    let(input,_) = take(7usize)(input)?;  // skip total ask quote volume

    let(input, best_ask_price_1st) = read_price(input)?;
    let(input,best_ask_quantity_1st) = read_quantity(input)?;

    let(input, best_ask_price_2nd) = read_price(    input)?;
    let(input,best_ask_quantity_2nd) = read_quantity(input)?;

    let(input, best_ask_price_3rd) = read_price(input)?;
    let(input, best_ask_quantity_3rd) = read_quantity(input)?;

    let(input, best_ask_price_4th) = read_price(input)?;
    let(input,best_ask_quantity_4th) = read_quantity(input)?;

    let(input, best_ask_price_5th) = read_price(input)?;
    let(input,best_ask_quantity_5th) = read_quantity(input)?;

    let(input,_) = take(30usize)(input)?; // skip 30 bytes

    let (input, pkt_time) = read_accept_time(input)?;

    Ok((input, KospiFormat{
        issue_code,
        best_bid_price_1st,
        best_bid_quantity_1st,
        best_bid_price_2nd,
        best_bid_quantity_2nd,
        best_bid_price_3rd,
        best_bid_quantity_3rd,
        best_bid_price_4th,
        best_bid_quantity_4th,
        best_bid_price_5th,
        best_bid_quantity_5th,

        best_ask_price_1st,
        best_ask_quantity_1st,
        best_ask_price_2nd,
        best_ask_quantity_2nd,
        best_ask_price_3rd,
        best_ask_quantity_3rd,
        best_ask_price_4th,
        best_ask_quantity_4th,
        best_ask_price_5th,
        best_ask_quantity_5th,
        
        pkt_time,
    }))
}

fn read_string(input:&[u8], n_bytes: u8) -> IResult<&[u8], String> {
    let(input, data_bytes) = take(n_bytes)(input)?;
    let string = String::from_utf8_lossy(data_bytes).to_string();
    Ok((input, string))
}

fn read_price(input: &[u8]) -> IResult<&[u8], f64> 
{
    let(input,data_bytes) = take(5usize)(input)?;
    let ascii_string = String::from_utf8_lossy(data_bytes).to_string();
    if let Ok(num) = ascii_string.parse::<f64>() {
        Ok((input, num))
    } else {
        panic!("Bad data for f64 converter")
    }
}

fn read_quantity(input: &[u8]) -> IResult<&[u8],u64> {

    let(input,data_bytes) = take(7usize)(input)?;
    let ascii_string = String::from_utf8_lossy(data_bytes).to_string();
    if let Ok(num) = ascii_string.parse::<u64>() {
        Ok((input, num))
    } else {
        panic!("Bad data for u64 converter")
    }
}

fn read_accept_time(input: &[u8]) -> IResult<&[u8], PktTime>{
    let (input, hour) = u8(input)?;
    let (input, min) = u8(input)?;
    let (input, second) = u8(input)?;
    let (input, micro) = le_u32(input)?;
    
    Ok((input, PktTime {hour, min, second,micro}))
}