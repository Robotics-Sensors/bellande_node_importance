// Copyright (C) 2024 Bellande Robotics Sensors Research Innovation Center, Ronaldson Bellande

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use reqwest;
use serde_json::{json, Value};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "bellande_node_importance", about = "Bellande Node Importance Tool")]
struct Opt {
    #[structopt(long, help = "Node to evaluate as JSON object with coordinates and segment")]
    node: String,

    #[structopt(long, help = "List of recent nodes as JSON array")]
    recent_nodes: String,

    #[structopt(long, help = "Dictionary of important nodes by segment as JSON object")]
    important_nodes: String,

    #[structopt(long, help = "Dictionary of adjacent segments as JSON object")]
    adjacent_segments: String,

    #[structopt(long, help = "Grid steps for each dimension as JSON array")]
    grid_steps: String,

    #[structopt(long, help = "Minimum required segment coverage ratio", default_value = "0.5")]
    min_segment_coverage: f64,

    #[structopt(long, help = "Use local executable instead of API")]
    use_executable: bool,
}

async fn make_node_importance_request(
    node: Value,
    recent_nodes: Value,
    important_nodes: Value,
    adjacent_segments: Value,
    grid_steps: Value,
    min_segment_coverage: f64,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = "https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Node_Importance/node_importance";

    let payload = json!({
        "node": node,
        "nodes": recent_nodes,
        "important_nodes": important_nodes,
        "adjacent_segments": adjacent_segments,
        "grid_steps": grid_steps,
        "min_segment_coverage": min_segment_coverage,
        "auth": {
            "authorization_key": "bellande_web_api_opensource"
        }
    });

    let response = client
        .post(url)
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(response)
}

fn get_executable_path() -> PathBuf {
    if cfg!(target_os = "windows") {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("Bellande_Node_Importance.exe")
    } else {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("Bellande_Node_Importance")
    }
}

fn run_node_importance_executable(
    node: &str,
    recent_nodes: &str,
    important_nodes: &str,
    adjacent_segments: &str,
    grid_steps: &str,
    min_segment_coverage: f64,
) -> Result<(), Box<dyn Error>> {
    let executable_path = get_executable_path();
    let passcode = "bellande_node_importance_executable_access_key";

    // Parse and validate input
    let node_obj: Value = serde_json::from_str(node)?;
    let grid_steps_list: Value = serde_json::from_str(grid_steps)?;

    // Validate dimensions
    if let (Some(coords), Some(steps)) = (
        node_obj["coords"].as_array(),
        grid_steps_list.as_array()
    ) {
        if coords.len() != steps.len() {
            return Err(format!(
                "Node coordinates must match grid steps dimensions ({})",
                steps.len()
            ).into());
        }
    }

    // Prepare and run command
    let output = Command::new(executable_path)
        .args(&[
            passcode,
            node,
            recent_nodes,
            important_nodes,
            adjacent_segments,
            grid_steps,
            &min_segment_coverage.to_string(),
        ])
        .output()?;

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    } else {
        Err(format!(
            "Process failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ).into())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    // Parse JSON strings to Values for validation
    let node: Value = serde_json::from_str(&opt.node)
        .map_err(|e| format!("Error parsing node: {}", e))?;
    let recent_nodes: Value = serde_json::from_str(&opt.recent_nodes)
        .map_err(|e| format!("Error parsing recent nodes: {}", e))?;
    let important_nodes: Value = serde_json::from_str(&opt.important_nodes)
        .map_err(|e| format!("Error parsing important nodes: {}", e))?;
    let adjacent_segments: Value = serde_json::from_str(&opt.adjacent_segments)
        .map_err(|e| format!("Error parsing adjacent segments: {}", e))?;
    let grid_steps: Value = serde_json::from_str(&opt.grid_steps)
        .map_err(|e| format!("Error parsing grid steps: {}", e))?;

    if opt.use_executable {
        // Run using local executable
        if let Err(e) = run_node_importance_executable(
            &opt.node,
            &opt.recent_nodes,
            &opt.important_nodes,
            &opt.adjacent_segments,
            &opt.grid_steps,
            opt.min_segment_coverage,
        ) {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    } else {
        // Run using API
        match make_node_importance_request(
            node,
            recent_nodes,
            important_nodes,
            adjacent_segments,
            grid_steps,
            opt.min_segment_coverage,
        ).await {
            Ok(result) => {
                println!("{}", serde_json::to_string_pretty(&result)?);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    }

    Ok(())
}
