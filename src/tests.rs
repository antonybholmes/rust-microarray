
 

#[cfg(test)]
use std::{error::Error, fs::File};


 

#[test]
fn load_tsv() -> Result<(), Box<dyn Error>> {
    let file = File::open("../../data/microarray/hgu133plus2.rma.collapsed.tsv")?;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .from_reader(file);

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        //.quote_style(csv::QuoteStyle::NonNumeric)
        .has_headers(true)
        .quote_style(csv::QuoteStyle::Never)
        .from_writer(vec![]);

    let cols = vec![0, 1, 2, 8, 10];

    let headers = rdr.headers()?;

    //println!("{:?}", &cols.into_iter().map(|c|&headers[c]).collect::<Vec<&str>>());

    let header = cols.iter().map(|c| &headers[*c]).collect::<Vec<&str>>();

    wtr.write_record(&header)?;

    //println!("{:?}", headers);

    for result in rdr.records() {
        let record = result?;
        let row = cols.iter().map(|c| &record[*c]).collect::<Vec<&str>>();
        //println!("{}", &record[0]);
        wtr.write_record(&row)?;
    }

    let data = String::from_utf8(wtr.into_inner()?)?;

    println!("{:?}", data);

    Ok(())
}
