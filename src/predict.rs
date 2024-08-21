use std::error::Error;
use std::fs;
use ndarray::{Array2, ArrayView2};
use tch::{nn, nn::Module, Device, Tensor};
use csv::ReaderBuilder;

const SEQ_LENGTH: usize = 10;

struct Model {
    lstm1: nn::LSTM,
    lstm2: nn::LSTM,
    fc: nn::Linear,
}

impl Model {
    fn new(vs: &nn::Path) -> Self {
        let lstm1 = nn::lstm(vs, 2, 50, Default::default());
        let lstm2 = nn::lstm(vs, 50, 50, Default::default());
        let fc = nn::linear(vs, 50, 2, Default::default());
        Self { lstm1, lstm2, fc }
    }
}

impl Module for Model {
    fn forward(&self, xs: &Tensor) -> Tensor {
        let (out, _) = self.lstm1.seq(xs);
        let (out, _) = self.lstm2.seq(&out);
        self.fc.forward(&out.select(1, -1))
    }
}

fn load_data(file_path: &str) -> Result<Array2<f32>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().has_headers(false).from_path(file_path)?;
    let mut data = Vec::new();

    for result in reader.records() {
        let record = result?;
        let x: f32 = record[0].parse()?;
        let y: f32 = record[1].parse()?;
        data.push(vec![x, y]);
    }

    Ok(Array2::from_shape_vec((data.len(), 2), data.into_iter().flatten().collect())?)
}

fn create_sequences(data: &Array2<f32>) -> (Array2<f32>, Array2<f32>) {
    let mut x = Vec::new();
    let mut y = Vec::new();

    for i in 0..(data.nrows() - SEQ_LENGTH) {
        x.push(data.slice(s![i..i+SEQ_LENGTH, ..]).to_owned().into_raw_vec());
        y.push(data.row(i + SEQ_LENGTH).to_vec());
    }

    (Array2::from_shape_vec((x.len(), SEQ_LENGTH, 2), x.into_iter().flatten().collect()).unwrap(),
     Array2::from_shape_vec((y.len(), 2), y.into_iter().flatten().collect()).unwrap())
}

fn train_model(x: &Array2<f32>, y: &Array2<f32>) -> Model {
    let vs = nn::VarStore::new(Device::Cpu);
    let model = Model::new(&vs.root());

    let x_tensor = Tensor::of_slice(&x.as_slice().unwrap()).view([-1, SEQ_LENGTH as i64, 2]);
    let y_tensor = Tensor::of_slice(&y.as_slice().unwrap()).view([-1, 2]);

    let mut opt = nn::Adam::default().build(&vs, 1e-3).unwrap();

    for epoch in 1..=10 {
        let loss = model.forward(&x_tensor).mse_loss(&y_tensor, tch::Reduction::Mean);
        opt.backward_step(&loss);
        println!("Epoch: {}, Loss: {}", epoch, f64::from(&loss));
    }

    model
}

fn predict_next_position(model: &Model, last_positions: ArrayView2<f32>) -> [f32; 2] {
    let input = Tensor::of_slice(&last_positions.as_slice().unwrap()).view([1, SEQ_LENGTH as i64, 2]);
    let output = model.forward(&input);
    let result: Vec<f32> = Vec::from(output);
    [result[0], result[1]]
}

pub fn run_prediction() -> Result<(), Box<dyn Error>> {
    let mut all_data = Vec::new();

    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("csv") {
            println!("Loading data from: {:?}", path);
            let data = load_data(path.to_str().unwrap())?;
            all_data.extend(data.rows().into_iter().map(|row| row.to_vec()));
        }
    }

    let all_data = Array2::from_shape_vec((all_data.len(), 2), all_data.into_iter().flatten().collect())?;
    let (x, y) = create_sequences(&all_data);
    let model = train_model(&x, &y);

    println!("Enter 'q' to quit.");
    loop {
        println!("Enter current X coordinate:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim() == "q" {
            break;
        }
        let current_x: f32 = input.trim().parse()?;

        println!("Enter current Y coordinate:");
        input.clear();
        std::io::stdin().read_line(&mut input)?;
        if input.trim() == "q" {
            break;
        }
        let current_y: f32 = input.trim().parse()?;

        let mut last_positions = all_data.slice(s![-9.., ..]).to_owned();
        last_positions.push_row(ndarray::arr1(&[current_x, current_y]).view()).unwrap();

        let predicted_position = predict_next_position(&model, last_positions.view());
        println!("Predicted next position: X={:.2}, Y={:.2}", predicted_position[0], predicted_position[1]);
        println!();
    }

    Ok(())
}