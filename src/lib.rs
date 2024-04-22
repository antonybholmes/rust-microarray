use std::{collections::HashMap, fmt::Display, fs::File};

use csv::{Reader, StringRecord};

mod tests;

#[derive(Debug, Clone)]
pub enum MicroarrayError {
    FileError(String),
}

// impl From< std::error::Error> for MicroarrayError {
//     fn from(error:  Error) -> Self {
//         MicroarrayError::FileError(error.to_string())
//     }
// }

//impl std::error::Error for AuthError {}

impl Display for MicroarrayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MicroarrayError::FileError(user) => {
                write!(f, "account for {} does not exist", user)
            }
        }
    }
}

pub fn make_tsv(samples: &Vec<String>) -> Result<String, MicroarrayError> {
    let file = File::open("./data/microarray/hgu133plus2.rma.collapsed.tsv")
        .map_err(|_| MicroarrayError::FileError("file issue".to_string()))?;

    //let cols = vec![0, 1, 2, 8, 10];

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .from_reader(file);

    let headers = rdr
        .headers()
        .map_err(|_| MicroarrayError::FileError("file issue".to_string()))?;

    let mut header_map: HashMap<String, usize> = HashMap::<String, usize>::new();

    for i in 0..headers.len() {
        header_map.insert(headers[i].to_string(), i as usize);
    }

    let cols = samples
        .iter()
        .map(|sample| header_map.get(sample).unwrap_or_else(|| &std::usize::MAX))
        .filter(|c| **c < std::usize::MAX)
        .collect::<Vec<&usize>>();

    //let mut data: Vec<Vec<String>> = vec![vec!["".to_string(); cols.len()]; 60000];

    //let cols = (0..samp.len()).into_iter().filter(|c|headers[c]).collect::<Vec<u32>>();

    // for result in rdr.records() {
    //     let record = match result {
    //         Ok(val) => val,
    //         _ => return Err(MicroarrayError::FileError("header issue".to_string())),
    //     };

    //     let row = cols.iter().map(|c| &record[**c]).collect::<Vec<&str>>();

    //     data.push(row.into_iter().map(|x| x.to_string()).collect());
    // }

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        //.quote_style(csv::QuoteStyle::NonNumeric)
        .has_headers(true)
        .quote_style(csv::QuoteStyle::Never)
        .from_writer(vec![]);

    let headers = match rdr.headers() {
        Ok(val) => val,
        _ => return Err(MicroarrayError::FileError("header issue".to_string())),
    };

    //println!("{:?}", &cols.into_iter().map(|c|&headers[c]).collect::<Vec<&str>>());

    let header = cols.iter().map(|c| &headers[**c]).collect::<Vec<&str>>();

    wtr.write_record(&header)
        .map_err(|_| MicroarrayError::FileError("header issue".to_string()))?;
    //println!("{:?}", headers);

    for result in rdr.records() {
        //for row in data {
        let record = result.map_err(|_| MicroarrayError::FileError("header issue".to_string()))?;

        let row = cols.iter().map(|c| &record[**c]).collect::<Vec<&str>>();
        //println!("{}", &record[0]);
        wtr.write_record(&row)
            .map_err(|_| MicroarrayError::FileError("header issue".to_string()))?;
    }

    let vec: Vec<u8> = wtr
        .into_inner()
        .map_err(|_| MicroarrayError::FileError("header issue".to_string()))?;

    let data = String::from_utf8(vec)
        .map_err(|_| MicroarrayError::FileError("header issue".to_string()))?;

    Ok(data)
}

pub fn make_tsv_from_indices(cols: &Vec<usize>) -> Result<String, MicroarrayError> {
    let file: File = File::open("../../data/microarray/hgu133plus2.rma.collapsed.tsv")
        .map_err(|_| MicroarrayError::FileError("file issue".to_string()))?;

    //let cols = vec![0, 1, 2, 8, 10];

    let mut data: Vec<Vec<String>> = vec![vec!["".to_string(); cols.len()]; 60000];

    let mut rdr: Reader<File> = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .from_reader(file);

    let headers: &StringRecord = rdr
        .headers()
        .map_err(|_| MicroarrayError::FileError("file issue".to_string()))?;

    let mut header_map: HashMap<String, usize> = HashMap::<String, usize>::new();

    for i in 0..headers.len() {
        header_map.insert(headers[i].to_string(), i as usize);
    }

    // let cols = samples
    //     .iter()
    //     .map(|sample| header_map.get(sample).unwrap_or_else(|| &std::usize::MAX))
    //     .filter(|c| **c < std::usize::MAX)
    //     .collect::<Vec<&usize>>();

    //let cols = (0..samp.len()).into_iter().filter(|c|headers[c]).collect::<Vec<u32>>();

    for result in rdr.records() {
        let record = match result {
            Ok(val) => val,
            _ => return Err(MicroarrayError::FileError("header issue".to_string())),
        };

        let row = cols.iter().map(|c| &record[*c]).collect::<Vec<&str>>();

        data.push(row.into_iter().map(|x| x.to_string()).collect());
    }

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        //.quote_style(csv::QuoteStyle::NonNumeric)
        .has_headers(true)
        .quote_style(csv::QuoteStyle::Never)
        .from_writer(vec![]);

    let headers: &StringRecord = match rdr.headers() {
        Ok(val) => val,
        _ => return Err(MicroarrayError::FileError("header issue".to_string())),
    };

    //println!("{:?}", &cols.into_iter().map(|c|&headers[c]).collect::<Vec<&str>>());

    let header: Vec<&str> = cols.iter().map(|c| &headers[*c]).collect::<Vec<&str>>();

    match wtr.write_record(&header) {
        Ok(val) => val,
        _ => return Err(MicroarrayError::FileError("header issue".to_string())),
    };

    //println!("{:?}", headers);

    //for result in rdr.records() {
    for row in data {
        //let record = match result {
        //    Ok(val) => val,
        //    _ => return Err(MicroarrayError::FileError("header issue".to_string())),
        //};

        //let row = cols.iter().map(|c| &record[*c]).collect::<Vec<&str>>();
        //println!("{}", &record[0]);
        match wtr.write_record(&row) {
            Ok(val) => val,
            _ => return Err(MicroarrayError::FileError("header issue".to_string())),
        };
    }

    let vec: Vec<u8> = match wtr.into_inner() {
        Ok(val) => val,
        _ => return Err(MicroarrayError::FileError("header issue".to_string())),
    };

    let data: String = match String::from_utf8(vec) {
        Ok(val) => val,
        _ => return Err(MicroarrayError::FileError("header issue".to_string())),
    };

    Ok(data)
}
