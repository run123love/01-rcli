use csv::Reader;
use serde::{Deserialize, Serialize};

use crate::opts::CsvOpts;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    nationality: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(opts: CsvOpts) -> anyhow::Result<()> {
    let mut rdr = Reader::from_path(opts.input)?;
    let records = rdr.deserialize().collect::<Result<Vec<Player>, _>>()?;
    let json = serde_json::to_string_pretty(&records)?;
    let output_file = opts.output;
    std::fs::write(output_file, json)?;
    Ok(())
}
