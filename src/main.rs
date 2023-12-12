use regex::Regex;
use serde_json::json;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::{thread, time};
use urlencoding::encode;

fn main() {
    let rust_output = get_command_output("cargo", vec!["bench"], None);

    // cooldown
    thread::sleep(time::Duration::from_secs(10));

    let mut benchmarks = extract_benchmarks(&rust_output);
    benchmarks.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let chart_url = generate_quickchart_url(&benchmarks);
    let readme_content = generate_readme(&benchmarks, &chart_url, &rust_output);
    write_to_file("README.md", &readme_content).expect("Failed to write README.md");
}

fn get_command_output(command: &str, args: Vec<&str>, dir: Option<&str>) -> String {
    let mut command = Command::new(command);
    command.args(args);
    if let Some(directory) = dir {
        command.current_dir(directory);
    }

    let output = command.output().expect("Failed to execute command").stdout;
    String::from_utf8_lossy(&output).into_owned()
}

fn extract_benchmarks(rust_output: &str) -> Vec<(String, f64)> {
    let mut benchmarks = Vec::new();
    extract_time_from_rust_output(rust_output, &mut benchmarks);
    benchmarks
}

fn generate_readme(benchmarks: &[(String, f64)], chart_url: &str, rust_output: &str) -> String {
    format!(
        "# JSON Transformations - Native vs VRL vs Rhai \n\n\
        > Note: benchmarks are ran within GitHub CI which might introduce a bit of noise, though, we can make a bold assumption that noise that will affect one will affect others. Since the benchmark only lasts a few seconds each.
        \n\n\
        | Benchmark | Time (in µs) |\n\
        |-----------|------|\n\
        {}\n\n\
        ![Benchmark Bar Chart]({})\n\n\
        <details><summary>Click to expand logs</summary>\n\n\
        Rust Benchmark Output:\n\n\
        ```shell\n\
        {}\n\
        ```\n\n\
        \n\n\
        </details>\n",
        benchmarks
            .iter()
            .map(|(benchmark, time)| format!("| {} | {} |", benchmark, time))
            .collect::<Vec<String>>()
            .join("\n"),
        chart_url,
        rust_output,
    )
}

fn write_to_file(file_name: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())
}

fn generate_quickchart_url(benchmarks: &[(String, f64)]) -> String {
    // Collect the labels and data from the benchmarks
    let labels: Vec<String> = benchmarks.iter().map(|(name, _)| name.clone()).collect();
    let data: Vec<f64> = benchmarks.iter().map(|&(_, time)| time).collect();

    // Serialize the chart data to JSON
    let chart_data = json!({
        "type": "bar",
        "data": {
            "labels": labels,
            "datasets": [{
                "label": "Benchmark Results",
                "data": data
            }]
        },
        "options": {
            "title": {
                "display": true,
                "text": "Lower is Better"
            },
            "scales": {
                "yAxes": [{
                    "ticks": {
                        "beginAtZero": true
                    }
                }]
            }
        }
    });

    // URL-encode the JSON string
    let data_str = chart_data.to_string();
    let encoded_chart_data = encode(&data_str);

    // Generate the full QuickChart URL
    format!(
        "https://quickchart.io/chart?bkg=white&c={}",
        encoded_chart_data
    )
}
fn extract_time_from_rust_output(output: &str, benchmarks: &mut Vec<(String, f64)>) {
    // extract time in microseconds (µs) from Rust output
    let re =
        Regex::new(r"(\w+)\s+time:\s+\[\d+\.\d+ (µs|ns)\s+(\d+\.\d+) (µs|ns)\s+\d+\.\d+ (µs|ns)\]")
            .unwrap();

    for captures in re.captures_iter(output) {
        let benchmark = captures.get(1).unwrap().as_str();
        let time_unit = captures.get(2).unwrap().as_str();
        let time = captures
            .get(3)
            .unwrap()
            .as_str()
            .parse::<f64>()
            .unwrap_or(f64::MAX);

        // convert time from µs to ns if necessary
        let time_ns = if time_unit == "µs" {
            time * 1000.0
        } else {
            time
        };
        let rounded_time_ns = format!("{:.1}", time_ns).parse::<f64>().unwrap();

        benchmarks.push((format!("{}", benchmark), rounded_time_ns));
    }
}
