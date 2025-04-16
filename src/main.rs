use anyhow::{Context, Result, bail};
use clap::Parser;
use nalgebra::Vector3;
use std::{
    fs::{self, File},
    io::Read,
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    dir: String,
}

enum Dimensions {
    TWO,
    THREE,
}

struct StructuredGrid {
    shape: (i32, i32, i32),
    dimensions: Dimensions,
    vertices: Vec<Vector3<f32>>,
}

// fn read_grid(file: File) -> StructuredGrid {
//     let shape = (0, 0, 0);
// }

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Args: {:?}", args);

    let sim_directory = fs::read_dir("./".to_owned() + &args.dir)?
        .flatten() // Remove all errors
        .find(|e| e.file_name() == "lmrsim" && e.path().is_dir())
        .context("Directory does not contain a valid lmrsim folder")?;

    let grid_directory = sim_directory
        .path()
        .read_dir()?
        .flatten()
        .find(|e| e.file_name() == "grid" && e.path().is_dir())
        .context("lmrsim directory does not contain grid folder")?;

    let snapshots_directory = sim_directory
        .path()
        .read_dir()?
        .flatten()
        .find(|e| e.file_name() == "snapshots" && e.path().is_dir())
        .context("lmrsim directory does not contain snapshots folder")?;

    println!("Dir: {:?}", sim_directory);
    println!("Grid Dir: {:?}", grid_directory);
    println!("Snapshots Dir: {:?}", snapshots_directory);

    let grid_metadata_path = grid_directory.path().join("grid.metadata");
    println!(
        "Does the grid-metadata file exist? {:?}",
        grid_metadata_path.try_exists()
    );
    let mut grid_metadata_file = File::open(grid_metadata_path)?;
    let mut grid_metadata = String::new();
    grid_metadata_file.read_to_string(&mut grid_metadata)?;

    let grid_metadata = json::parse(&grid_metadata).context("Could not parse grid metadata")?;
    println!("Grid metadata: {:?}", grid_metadata);

    let Some(num_grids) = grid_metadata["ngrids"].as_u8() else {
        bail!("Number of grids is poorly formed");
    };

    println!("Number of grids: {}", num_grids);

    for grid_num in 0..num_grids {}

    Ok(())
}
