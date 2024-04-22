use std::{fmt::Display, fs::File};

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

pub fn make_tsv() -> Result<String, MicroarrayError> {
    let file = match File::open("../../data/microarray/hgu133plus2.rma.collapsed.tsv") {
        Ok(val) => val,
        _ => return Err(MicroarrayError::FileError("file issue".to_string())),
    };

    let cols = vec![0, 1, 2, 8, 10];

    let mut data: Vec<Vec<String>> = vec![vec!["".to_string(); cols.len()]; 60000];

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .from_reader(file);

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

    let headers = match rdr.headers() {
        Ok(val) => val,
        _ => return Err(MicroarrayError::FileError("header issue".to_string())),
    };

    //println!("{:?}", &cols.into_iter().map(|c|&headers[c]).collect::<Vec<&str>>());

    let header = cols.iter().map(|c| &headers[*c]).collect::<Vec<&str>>();

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

    let vec = match wtr.into_inner() {
        Ok(val) => val,
        _ => return Err(MicroarrayError::FileError("header issue".to_string())),
    };

    let data = match String::from_utf8(vec) {
        Ok(val) => val,
        _ => return Err(MicroarrayError::FileError("header issue".to_string())),
    };

    Ok(data)
}
