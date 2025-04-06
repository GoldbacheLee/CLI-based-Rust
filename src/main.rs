use std::fs;
use csv::Reader;
use clap::Parser;
use rcli::{Opts, Subcommand};
use serde::{Serialize,Deserialize};

// rcli  csv  -i   input.csv   -o  output.json   --header  -d "."

#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player{
    #[serde(rename = "Name")]
    name:String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: String
}


fn main() -> anyhow::Result<()>{
    let opts = Opts ::parse();
    println!("{:?}",    opts);
    match  opts.cmd{
        Subcommand::Csv(opts) => {
            let mut    reader = Reader::from_path(opts.input)?;
            // let records = reader
            //     .deserialize()
            //     .map(|record| record.unwrap())
            //     .collect::<Vec<Player>>();

            let mut ret = Vec::with_capacity(128);


            for result in reader.deserialize() {
                 let record : Player = result?;
                ret.push(record);
            }
            
            let json = serde_json::to_string_pretty(&ret)?;
            fs::write(opts.output, json)?;
        }
    }
    Ok(())
}

