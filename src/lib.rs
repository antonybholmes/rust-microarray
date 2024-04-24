use std::{fmt::Display, fs::{remove_dir, File}, io, path::Path, string::FromUtf8Error};

use csv::{IntoInnerError, StringRecord};

mod tests;

#[derive(Debug, Clone)]
pub enum MicroarrayError {
    FileError(String),
}

impl From<csv::Error> for MicroarrayError {
    fn from(error: csv::Error) -> Self {
        MicroarrayError::FileError(error.to_string())
    }
}

impl From<FromUtf8Error> for MicroarrayError {
    fn from(error: FromUtf8Error) -> Self {
        MicroarrayError::FileError(error.to_string())
    }
}

impl<W> From<IntoInnerError<W>> for MicroarrayError {
    fn from(error: IntoInnerError<W>) -> Self {
        MicroarrayError::FileError(error.to_string())
    }
}

impl From<io::Error> for MicroarrayError {
    fn from(error: io::Error) -> Self {
        MicroarrayError::FileError(error.to_string())
    }
}

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

pub struct ExpressionData {
    data:Vec<Vec<f64>>,
    headers: Vec<String>,
    probe_ids: Vec<String>,
    entrez_ids: Vec<String>
    gene_symbols: Vec<String>
}

pub struct Microarray {
    path: String,
}

impl Microarray {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn make_tsv(&self, sample_ids: &Vec<&str>) -> Result<String, MicroarrayError> {
        // let sample_ids = vec![
        //     "0c3b8a19-1975-4c6e-aece-44a59c71719d",
        //     "0c4f0c89-af16-484a-a408-8dfde25d8f10",
        // ];

        //eprintln!("{:?}", Path::new(&self.path).join("meta.tsv").to_str());
        // open meta data
        let file = File::open(Path::new(&self.path).join("meta.tsv"))?;

        //eprintln!("{:?}", file.metadata());

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_reader(file);

        let mut probe_ids: StringRecord = StringRecord::new();
        rdr.read_record(&mut probe_ids)?;

        // probes/rows is -1 because first col is a header so ignore that
        let n_probes: usize = probe_ids.len() - 1;

        let mut entrez_ids: StringRecord = StringRecord::new();
        rdr.read_record(&mut entrez_ids)?;

        let mut gene_symbols: StringRecord = StringRecord::new();
        rdr.read_record(&mut gene_symbols)?;

        let mut row_records: Vec<Vec<f64>> = Vec::with_capacity(sample_ids.len());
        let mut samples_names: Vec<String> = Vec::with_capacity(sample_ids.len());

        // let mut rdr = csv::ReaderBuilder::new()
        //     .has_headers(true)
        //     .delimiter(b'\t')
        //     .from_reader(file);

        for sample_id in sample_ids {
            let file = File::open(Path::new(&self.path).join(format!("{}.tsv", sample_id)))?;

            rdr = csv::ReaderBuilder::new()
                .has_headers(true)
                .delimiter(b'\t')
                .from_reader(file);

            let mut row: StringRecord = StringRecord::new();
            rdr.read_record(&mut row)?;
            samples_names.push(row[0].to_string());

            // read data into array
            let values = (1..row.len())
                .into_iter()
                .map(|i| row[i].parse::<f64>().unwrap())
                .collect::<Vec<f64>>();

            row_records.push(values);
        }

        let n_samples = samples_names.len();

        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'\t')
            //.quote_style(csv::QuoteStyle::NonNumeric)
            .has_headers(true)
            .quote_style(csv::QuoteStyle::Never)
            .from_writer(vec![]);

        let mut header: Vec<String> = vec![
            "Probe Id".to_string(),
            "Entrez".to_string(),
            "Gene Symbol".to_string(),
        ];
        header.append(&mut samples_names);

        //eprintln!("{:?}", header);

        wtr.write_record(&header)?;
        //println!("{:?}", headers);

        let mut out_row: Vec<String> = Vec::with_capacity(header.len());

        // We are going to transpose the data we read into this output
        // array
        let mut data:Vec<Vec<f64>> = vec![vec![0.0; n_samples]; n_probes] ;

        for row in 0..n_probes {
            out_row[0] = probe_ids[row + 1].to_string();
            out_row[1] = entrez_ids[row + 1].to_string();
            out_row[2] = gene_symbols[row + 1].to_string();

            for col in 0..n_samples {
                out_row[3 + col] = format!("{}", row_records[col][row]);

                 data[row][col] = row_records[col][row];
            }

            //eprintln!("{:?} {} out_row", out_row, n_samples);

            wtr.write_record(&out_row)?;
        }

        // for result in rdr.records() {
        //     //for row in data {
        //     let record =
        //         result.map_err(|_| MicroarrayError::FileError("header issue".to_string()))?;

        //     let row = cols.iter().map(|c| &record[**c]).collect::<Vec<&str>>();
        //     //println!("{}", &record[0]);
        //     wtr.write_record(&row)
        //         .map_err(|_| MicroarrayError::FileError("header issue".to_string()))?;
        // }

        let vec: Vec<u8> = wtr.into_inner()?;

        let data = String::from_utf8(vec)?;

        Ok(data)
    }

    pub fn expression(&self, sample_ids: &Vec<&str>) -> Result<ExpressionData, MicroarrayError> {
        // let sample_ids = vec![
        //     "0c3b8a19-1975-4c6e-aece-44a59c71719d",
        //     "0c4f0c89-af16-484a-a408-8dfde25d8f10",
        // ];

        let n_samples = sample_ids.len();

        //eprintln!("{:?}", Path::new(&self.path).join("meta.tsv").to_str());
        // open meta data
        let file = File::open(Path::new(&self.path).join("meta.tsv"))?;

        //eprintln!("{:?}", file.metadata());

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_reader(file);

        let mut record: StringRecord = StringRecord::new();
        //let mut probe_ids: StringRecord = StringRecord::new();
        rdr.read_record(&mut record)?;

        let probe_ids:Vec<String> = (1..record.len()).map(|i|record[i].to_string()).collect();

        // probes/rows is -1 because first col is a header so ignore that
        let n_probes: usize = probe_ids.len() ;

        rdr.read_record(&mut record)?;
        let entrez_ids:Vec<String> = (1..record.len()).map(|i|record[i].to_string()).collect();

        rdr.read_record(&mut record)?;
        let gene_symbols:Vec<String> = (1..record.len()).map(|i|record[i].to_string()).collect();

        let mut row_records: Vec<Vec<f64>> = Vec::with_capacity(n_samples);
        let mut samples_names: Vec<String> = Vec::with_capacity(n_samples);

        // let mut rdr = csv::ReaderBuilder::new()
        //     .has_headers(true)
        //     .delimiter(b'\t')
        //     .from_reader(file);

        for sample_id in sample_ids {
            let file = File::open(Path::new(&self.path).join(format!("{}.tsv", sample_id)))?;

            rdr = csv::ReaderBuilder::new()
                .has_headers(true)
                .delimiter(b'\t')
                .from_reader(file);

            let mut row: StringRecord = StringRecord::new();
            rdr.read_record(&mut row)?;
            samples_names.push(row[0].to_string());

            // read data into array
            let values = (1..row.len())
                .into_iter()
                .map(|i| row[i].parse::<f64>().unwrap())
                .collect::<Vec<f64>>();

            row_records.push(values);
        }

        

       

        let mut exp = ExpressionData {
            data: vec![vec![0.0; n_samples]; n_probes],
            headers:samples_names,
            probe_ids ,
            entrez_ids ,
            gene_symbols ,
        };

        // We are going to transpose the data we read into this output
        // array
        //let mut data:Vec<Vec<f64>> = vec![vec![0.0; n_samples]; n_probes] ;

        for row in 0..n_probes {
     
            for col in 0..n_samples {
            
                 exp.data[row][col] = row_records[col][row];
            }

            //eprintln!("{:?} {} out_row", out_row, n_samples);

           // wtr.write_record(&out_row)?;
        }

        // for result in rdr.records() {
        //     //for row in data {
        //     let record =
        //         result.map_err(|_| MicroarrayError::FileError("header issue".to_string()))?;

        //     let row = cols.iter().map(|c| &record[**c]).collect::<Vec<&str>>();
        //     //println!("{}", &record[0]);
        //     wtr.write_record(&row)
        //         .map_err(|_| MicroarrayError::FileError("header issue".to_string()))?;
        // }

        //let vec: Vec<u8> = wtr.into_inner()?;

        //let data = String::from_utf8(vec)?;

        Ok(exp)
    }
}
