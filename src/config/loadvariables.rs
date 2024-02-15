use std::error::Error;
use std::collections::HashMap;

use csv;
use serde::Deserialize;

use crate::builtin::base::*;

#[derive(Debug, Deserialize)]
struct Record {
    reference: String,
    description: String,
    t: String,
    format: String,
    min: Option<f64>,
    max: Option<f64>,
    range: Option<i64>,
    group: Option<String>,
    operation: Option<String>,
}

fn analyze_numeric(strformat: String) -> NumericFormat {
    let mut format = NumericFormat {integer: true, signed: false, size: 8};
    match strformat.as_str() {
        "FLOAT32" => {
            format.integer = false;
            format.signed = true;
            format.size = 32;
        },
        "INT64" => {
            format.integer = true;
            format.signed = true;
            format.size = 64;
        },
        &_ => todo!(),
    }
    return format;
}

pub fn load_csv(filename: String, variable_list: &mut HashMap<String, Variable>) -> Result<(), Box<dyn Error>> {
    println!("Loading {}", filename);

    let mut rdr = csv::Reader::from_path(filename)?;

    let headers = rdr.headers()?;
    println!("{:?}", headers);

    for result in rdr.deserialize() {
        let record: Record =  result?;
        println!("{:?}", record);

        let reference = record.reference.clone();
        let strgroup: &str = record.group.as_deref().unwrap_or("");
        let group = strgroup.to_string();
        let stroperation: &str = record.operation.as_deref().unwrap_or("");
        let operation = stroperation.to_string();
        match record.t.as_str() {
            "TOOGLE" => {
                let specific: VarSpecific = VarToogle::new();
                let variable: Variable = Variable::new(record.reference, record.description, group, operation, specific);
                variable_list.insert(reference, variable);
        },
            "NUMERIC" => {
                let localformat = analyze_numeric(record.format);
                let specific: VarSpecific = VarNumeric::new(localformat); 
                let variable: Variable = Variable::new(record.reference, record.description, group, operation, specific);
                variable_list.insert(reference, variable);
        }
            "SENSOR4_20" => {
                let specific: VarSpecific = VarSensor4_20::new(10.0, 100.0, 256);
                let variable: Variable = Variable::new(record.reference, record.description, group, operation, specific);
                variable_list.insert(reference, variable);
            },
            &_ => todo!(),
        }
    }



    Ok(())
}
