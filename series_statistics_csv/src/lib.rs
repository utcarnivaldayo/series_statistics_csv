pub mod series_statistics_csv;

#[cfg(test)]
mod tests {
    use super::series_statistics_csv::SeriesStatisticsCsv;
    use series_statistics::series_statistics;

    #[test]
    fn test_write() {
        let length: usize = 5;
        let mut series = series_statistics::SeriesStatistics::new(length);

        for i in 1..=length {
            let v: Vec<f64> = vec![i as f64; length];
            series.add(&v);
        }
        let times: Vec<f64> = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        series.write(
            "./sample.csv".to_string(),
            &times,
            "time".to_string(),
            "mean".to_string(),
            "std.dev.".to_string(),
            "upper".to_string(),
            "lower".to_string(),
        );
    }

    #[test]
    fn test_read() {
        let hash_map = series_statistics::SeriesStatistics::read("./sample/sample.csv".to_string());
        println!("time: {:?}", hash_map["time"]);
        println!("mean: {:?}", hash_map["mean"]);
        println!("std.dev.: {:?}", hash_map["std.dev."]);
        println!("upper: {:?}", hash_map["upper"]);
        println!("lower: {:?}", hash_map["lower"]);
    }
}
