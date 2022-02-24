extern crate csv;

use clap::Parser;
use regex::Regex;
use std::io::{self, BufRead};
use substring::Substring;

/// A simple command line tool to convert SQL database output to CSV.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Whether or not the header row should be included, defaults to true
    #[clap(short, long)]
    exclude_header: bool,
}

fn main() {
    // get MySQL table input data
    // determine size of each column
    // loop over input data and parse data based on size of columns

    let args = Args::parse();
    let stdin = io::stdin();

    let mut rows: Vec<String> = Vec::new();
    let mut formatting_row_num = 0;

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let re = Regex::new(r"^(?:\+-+)+\+\s*$").unwrap();

        let is_formatting_row = re.is_match(&line);

        if is_formatting_row {
            formatting_row_num += 1
        }

        if formatting_row_num == 0 {
            continue;
        } else if formatting_row_num == 3 {
            break;
        }

        rows.push(line)
    }

    if rows.len() == 0 {
        return;
    }

    let mut index = 0;
    let mut column_indexes: Vec<u32> = Vec::new();

    for character in rows[0].chars() {
        if character == '+' {
            column_indexes.push(index);
        }

        index += 1;
    }

    let mut wtr = csv::Writer::from_writer(io::stdout());
    let mut excluded_rows: Vec<usize> = Vec::from([0, 2, rows.len() - 1]);

    if args.exclude_header {
        excluded_rows.push(1)
    }

    for (row_index, row) in rows.iter().enumerate() {
        // ignore any formatting rows
        if excluded_rows.contains(&row_index) {
            continue;
        }

        let mut csv_row: Vec<String> = Vec::new();
        for i in 1..column_indexes.len() {
            let prev: usize = column_indexes[i - 1].try_into().unwrap();
            let next: usize = column_indexes[i].try_into().unwrap();

            let value = row.substring(prev + 1, next - 1).trim().to_string();

            csv_row.push(value)
        }

        wtr.write_record(&csv_row);
    }
}
