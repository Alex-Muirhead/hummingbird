use anyhow::{Context, Result, bail};
use clap::Parser;
use flate2::read::GzDecoder;
use nalgebra::Vector3;
use nom::{
    IResult,
    sequence::{preceded, separated_pair},
};
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
    Two,
    Three,
}

struct StructuredGrid {
    shape: (u32, u32, u32),
    dimensions: u8,
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
        println!("Grid {:04} data", grid_num);
        parse_grid(&grid_string);
    }

    Ok(())
}

fn parse_grid(source: &str) -> IResult<&str, StructuredGrid> {
    // -> Result<StructuredGrid>
    use nom::Parser;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{space1, u8, u32};
    use nom::number::complete::float;

    let header: Vec<&str> = source.lines().take(6).collect();

    let (_, (grid_type, version)) = separated_pair(
        alt((tag("structured_grid"), tag("unstructured_grid"))),
        space1,
        separated_pair(u32, tag("."), u32),
    )
    .parse(header[0])?;

    let (label, _) = tag("label: ").parse(header[1])?;
    let (_, dimensions) = preceded(tag("dimensions: "), u8).parse(header[2])?;
    let (_, i_length) = preceded(tag("niv: "), u32).parse(header[3])?;
    let (_, j_length) = preceded(tag("njv: "), u32).parse(header[4])?;
    let (_, k_length) = preceded(tag("nkv: "), u32).parse(header[5])?;
    let shape = (i_length, j_length, k_length);

    println!(
        "The grid is size ({}, {}, {}) with {} dimensions",
        i_length, j_length, k_length, dimensions,
    );

    let mut vertices = Vec::new();
    for line in source.lines().skip(6) {
        // Manual way of ensuring there are 3 floats
        let (_, (x, y, z)) =
            (float, preceded(space1, float), preceded(space1, float)).parse(line)?;
        vertices.push(Vector3::new(x, y, z));
    }

    Ok((
        "",
        StructuredGrid {
            shape,
            dimensions,
            vertices,
        },
    ))
}
