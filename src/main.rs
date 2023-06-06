// Import the csv crate
use csv::{ ReaderBuilder, WriterBuilder };
use serde::{ Deserialize, Serialize };

const G: f64 = 9.81;

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    a: f64, // acceleration in m/s^2
    uk: f64, // coefficient of friction
    angle: Option<f64>, // angle in degrees
}

fn calculate_angle(data: &Data) -> f64 {
    let angle_rad = (data.a / G + data.uk).atan();
    angle_rad.to_degrees()
}

fn read_data(filename: &str) -> Vec<Data> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_path(filename)
        .expect("Cannot open file");

    let mut data_vec = Vec::new();

    for result in reader.deserialize() {
        let data: Data = result.expect("Cannot parse record");
        data_vec.push(data);
    }

    data_vec
}

fn write_data(filename: &str, data_vec: &[Data]) {
    let mut writer = WriterBuilder::new()
        .delimiter(b';')
        .from_path(filename)
        .expect("Cannot open or create file");

    for data in data_vec {
        writer.serialize(data).expect("Cannot write record");
    }
}

fn main() {
    let mut data_vec = read_data("input.csv");

    for data in &mut data_vec {
        let angle = calculate_angle(&data);
        data.angle = Some(angle);
    }

    write_data("output.csv", &data_vec);
    println!("Finished writing to output.csv");
}
