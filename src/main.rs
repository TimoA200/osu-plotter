// TODO: create legend

use plotters::prelude::*;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug, Clone)]
struct Result {
    name: String,
    sizes: Vec<u32>,
    values: Vec<f32>,
}

impl Result {
    fn new() -> Result {
        Result {
            name: String::new(),
            sizes: Vec::new(),
            values: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Test {
    name: String,
    result: Result,
}

impl Test {
    // Create new list of tests from string
    fn create_new_from_data(data: String) -> Vec<Test> {
        // Start iterating over data string
        let mut tests: Vec<Test> = Vec::new();
        let mut name: String = String::new();
        let mut result: Result = Result::new();

        let mut stage_flag = false;
        for line in data.lines() {
            if line.starts_with("# OSU") {
                if stage_flag {
                    tests.push(Test {
                        name: name,
                        result: result,
                    });
                    result = Result::new(); // Create completely new result, to reset sizes and values
                }
                stage_flag = true;
                name = line.to_string();
                println!("Test name: {}", name);
            } else if line.starts_with("# Size") {
                result.name = line.split("# Size").collect::<Vec<&str>>()[1]
                    .trim()
                    .to_string();
                println!("Value name: {}", result.name);
            } else {
                let res_tmp = line.split_whitespace().collect::<Vec<&str>>(); // Temporary result, in order to not modify the original data
                if res_tmp.len() != 0 && res_tmp[0].chars().all(char::is_numeric) {
                    println!("size: {}, value: {}", res_tmp[0], res_tmp[1]);
                    result.sizes.push(res_tmp[0].parse::<u32>().unwrap());
                    result.values.push(res_tmp[1].parse::<f32>().unwrap());
                }
            }
        }

        tests.push(Test {
            name: name,
            result: result,
        });

        tests
    }
}

fn main() -> std::io::Result<()> {
    let mut test_pairs: Vec<Vec<Test>> = Vec::new();
    for arg in env::args().skip(1) {
        let file = File::open(arg)?;
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content)?;
        let tests = Test::create_new_from_data(content);
        'outer: for test in &tests {
            for tp in &mut test_pairs {
                if test.name == tp[0].name {
                    tp.push(test.clone());
                    continue 'outer;
                }
            }
            test_pairs.push(Vec::from([test.clone()]));
        }
    }

    println!("\n\n\nPrint test pairs:\n\n");
    for tp in &test_pairs {
        println!("{:?}", tp);
        create_plot1(tp);
    }

    Ok(())
}

fn create_plot1(tests: &Vec<Test>) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let name = tests[0].name.as_str();
    let file_name = format!("{}.png", name);

    let root = BitMapBackend::new(&file_name, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(name, ("sans-serif", 20).into_font())
        .margin(30)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            (*tests[0].result.sizes.first().unwrap()..*tests[0].result.sizes.last().unwrap())
                .log_scale()
                .with_key_points(tests[0].result.sizes.clone()),
            *tests[0].result.values.first().unwrap()..*tests[0].result.values.last().unwrap(),
        )?;

    chart.configure_mesh().draw()?;

    tests.iter().for_each(|test| {
        chart
            .draw_series(LineSeries::new(
                test.result
                    .sizes
                    .iter()
                    .zip(test.result.values.iter())
                    .map(|(size, value)| (*size, *value)),
                &RED,
            ))
            .expect("Failed to draw series");

        chart
            .draw_series(
                test.result
                    .sizes
                    .iter()
                    .zip(test.result.values.iter())
                    .map(|(size, value)| Cross::new((*size, *value), 4, &RED)),
            )
            .expect("Failed to draw series");
    });

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
