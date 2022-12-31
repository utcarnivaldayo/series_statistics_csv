use series_statistics::series_statistics;
use std::collections::HashMap;

pub trait SeriesStatisticsCsv {
    fn write(
        &mut self,
        file_path: String,
        x: &Vec<f64>,
        x_name: String,
        mean_name: String,
        standard_deviation_name: String,
        un_upper_name: String,
        un_lower_name: String,
    );
    fn read(file_path: String) -> HashMap<String, Vec<f64>>;
}

impl SeriesStatisticsCsv for series_statistics::SeriesStatistics {
    fn write(
        &mut self,
        file_path: String,
        x: &Vec<f64>,
        x_name: String,
        mean_name: String,
        standard_deviation_name: String,
        un_upper_name: String,
        un_lower_name: String,
    ) {
        if self.len() != x.len() {
            return;
        }
        let mut writer = csv::Writer::from_path(file_path).expect("Invaild file path.");
        writer
            .write_record(&[
                "".to_string(),
                x_name.to_string(),
                mean_name.to_string(),
                standard_deviation_name.to_string(),
                un_upper_name.to_string(),
                un_lower_name.to_string(),
            ])
            .expect("Write Error");
        for i in 0..self.count() {
            writer
                .write_record(&[
                    i.to_string(),
                    x[i].to_string(),
                    self.mean(i).to_string(),
                    self.un_standard_deviation(i).to_string(),
                    self.un_upper(i).to_string(),
                    self.un_lower(i).to_string(),
                ])
                .expect("Write Error");
        }
        writer.flush().expect("Failed flush");
    }

    fn read(file_path: String) -> HashMap<String, Vec<f64>> {
        let mut table: HashMap<String, Vec<f64>> = HashMap::new();
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(file_path);

        let mut x: Vec<f64> = Vec::new();
        let mut means: Vec<f64> = Vec::new();
        let mut standard_deviations: Vec<f64> = Vec::new();
        let mut uppers: Vec<f64> = Vec::new();
        let mut lowers: Vec<f64> = Vec::new();

        // header
        let header_result = reader.as_mut().unwrap().headers();
        let header = header_result.unwrap();

        let x_name: String = header[1].to_string();
        let mean_name: String = header[2].to_string();
        let standard_deviation_name: String = header[3].to_string();
        let upper_name: String = header[4].to_string();
        let lower_name: String = header[5].to_string();

        // record
        for record_result in reader.unwrap().records() {
            let record = record_result.unwrap();
            x.push(record[1].parse::<f64>().unwrap());
            means.push(record[2].parse::<f64>().unwrap());
            standard_deviations.push(record[3].parse::<f64>().unwrap());
            uppers.push(record[4].parse::<f64>().unwrap());
            lowers.push(record[5].parse::<f64>().unwrap());
        }

        table.insert(x_name, x);
        table.insert(mean_name, means);
        table.insert(standard_deviation_name, standard_deviations);
        table.insert(upper_name, uppers);
        table.insert(lower_name, lowers);
        table
    }
}
