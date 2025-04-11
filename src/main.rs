use clap::Parser;
use nalgebra::Vector3;
use std::{
    fs::{self, File},
    io::{self, Error, ErrorKind},
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

fn main() -> io::Result<()> {
    let args = Args::parse();

    println!("Args: {:?}", args);

    let sim_directory = fs::read_dir("./".to_owned() + &args.dir)?
        .flatten() // Remove all errors
        .find(|e| e.file_name() == "lmrsim" && e.path().is_dir());

    let Some(sim_directory) = sim_directory else {
        return Err(Error::new(
            ErrorKind::Other,
            "Directory does not contain a valid lmrsim folder",
        ));
    };

    let grid_directory = sim_directory
        .path()
        .read_dir()?
        .flatten()
        .find(|e| e.file_name() == "grid" && e.path().is_dir());

    let Some(grid_directory) = grid_directory else {
        return Err(Error::new(
            ErrorKind::Other,
            "lmrsim directory does not contain grid folder",
        ));
    };

    let snapshots_directory = sim_directory
        .path()
        .read_dir()?
        .flatten()
        .find(|e| e.file_name() == "snapshots" && e.path().is_dir());

    let Some(snapshots_directory) = snapshots_directory else {
        return Err(Error::new(
            ErrorKind::Other,
            "lmrsim directory does not contain snapshots folder",
        ));
    };

    println!("Dir: {:?}", sim_directory);
    println!("Grid Dir: {:?}", grid_directory);
    println!("Snapshots Dir: {:?}", snapshots_directory);

    Ok(())
}
