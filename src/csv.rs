use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::opts::CsvOpts;
use crate::opts::OutputFormat;

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
    let mut records: Vec<Value> = Vec::with_capacity(128);
    let headers = rdr.headers()?.clone();
    for result in rdr.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        records.push(json_value);
    }
    let content = match opts.format {
        OutputFormat::Json => serde_json::to_string_pretty(&records)?,
        OutputFormat::Yaml => serde_yaml::to_string(&records)?,
    };
    let output = if let Some(output_file) = opts.output {
        output_file
    } else {
        format!("output.{}", opts.format)
    };
    std::fs::write(output, content)?;

    Ok(())
}
