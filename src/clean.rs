use csv::ReaderBuilder;
use std::error::Error; 
use std::fs::File;
use std::result::Result;

pub fn zscore(data: &Vec<f64>) -> Vec<f64> {
    if data.is_empty() {
        return vec![]; 
    }
    let mean: f64 = data.iter().sum::<f64>() / data.len() as f64;
    let std_dev: f64 = (data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64).sqrt();
    data.iter().map(|&x| (x - mean) / std_dev).collect()
}

pub fn filter_outliers(data: &Vec<f64>, threshold: f64) -> Vec<f64> {
    if data.is_empty() {
        return vec![];
    }
    let z_scores = zscore(data);
    data.iter().zip(z_scores.iter())
        .filter_map(|(&value, &z)| if z.abs() <= threshold { Some(value) } else { None })
        .collect()
}

pub fn norm_column(path: &str, indices: &[usize], threshold: f64) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    let mut columns: Vec<Vec<f64>> = vec![Vec::new(); indices.len()];
    let mut row_count = 0;

    for result in rdr.records() {
        if row_count >= 3100 {
            break; 
        }
        let record = result?;
        
        if record.len() < *indices.iter().max().unwrap_or(&0) + 1 {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Fewer columns than expected (error)")));
        }
    
        for (i, &idx) in indices.iter().enumerate() {
            match record.get(idx) {
                Some(value) => {
                    if let Ok(parsed_value) = value.parse::<f64>() {
                        columns[i].push(parsed_value);
                    } else {
                        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "error")));
                    }
                },
                None => {
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Column index out of bounds")));
                }
            }
        }
        row_count += 1;
    }

    let mut normalized_data = Vec::new();
    for column_data in columns {
        let filtered_data = filter_outliers(&column_data, threshold);
        let normalized_column = zscore(&filtered_data);
        normalized_data.push(normalized_column);
    }

    let row_count = normalized_data.get(0).map_or(0, |col| col.len());
    let transposed_data = (0..row_count).filter_map(|i| {
        if normalized_data.iter().all(|col| i < col.len()) {
            Some(normalized_data.iter().map(|col| col[i]).collect::<Vec<f64>>())
        } else {
            None 
        }
    }).collect();

    Ok(transposed_data)
}
