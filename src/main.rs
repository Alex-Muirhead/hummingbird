use anyhow::{Context, Result, bail};
use clap::Parser;
use flate2::read::GzDecoder;
use std::{
    fs::{self, File},
    io::Read,
};

mod grid;
use crate::grid::parse_grid;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    dir: String,
}

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

    for grid_num in 0..num_grids {
        let grid_path = grid_directory
            .path()
            .join(format!("grid-{:04}.gz", grid_num));
        if !grid_path.try_exists()? {
            bail!("Could not read grid-{:04} file", grid_num);
        }
        let grid_file = File::open(grid_path)?;
        let mut grid_data = GzDecoder::new(grid_file);
        let mut grid_string = String::new();
        grid_data.read_to_string(&mut grid_string)?;
        let grid = parse_grid(&grid_string);
        println!("Grid {:04} data: {:?}", grid_num, grid);
    }

    Ok(())
}
