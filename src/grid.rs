use nalgebra::Vector3;
use nom::{
    IResult, Parser,
    bytes::{complete::tag, take_while},
    character::complete::{self as nomchar, newline, space0, space1},
    combinator::map,
    multi::{count, many0},
    sequence::{delimited, terminated},
};

#[derive(Debug)]
pub struct StructuredGrid {
    pub shape: (u32, u32, u32),
    pub dimensions: u8,
    pub vertices: Vec<Vector3<f32>>,
}

fn parse_header(input: &str) -> IResult<&str, ((u32, u32, u32), u8)> {
    let (input, _) = terminated(tag("structured_grid"), space1).parse(input)?;
    let (input, _) = terminated(tag("1.1"), newline).parse(input)?;
    let (input, _label) = delimited(tag("label: "), take_while(|c| c != '\n'), newline)
        .parse(input)
        .map(|(remaining, label)| (remaining, label.to_string()))?;
    let (input, dimensions) = delimited(tag("dimensions: "), nomchar::u8, newline).parse(input)?;
    let (input, niv) = delimited(tag("niv: "), nomchar::u32, newline).parse(input)?;
    let (input, njv) = delimited(tag("njv: "), nomchar::u32, newline).parse(input)?;
    let (input, nkv) = delimited(tag("nkv: "), nomchar::u32, newline).parse(input)?;

    Ok((input, ((niv, njv, nkv), dimensions)))
}

fn parse_points(input: &str) -> IResult<&str, Vec<Vector3<f32>>> {
    many0(terminated(
        map(
            count(terminated(nom::number::complete::float, space0), 3),
            Vector3::from_vec,
        ),
        newline,
    ))
    .parse(input)
}

pub fn parse_grid(source: &str) -> StructuredGrid {
    // Making the choice here that a malformed input file is *unrecoverable*
    let (remaining, (shape, dimensions)) = parse_header(&source).expect("Failed to parse header\n");
    let (_, vertices) = parse_points(&remaining).expect("Failed to parse vertices\n");

    let number_of_vertices = shape.0 * shape.1 * shape.2;
    if vertices.len() != number_of_vertices as usize {
        panic!("The number of verticies does not match the defined shape");
    }

    StructuredGrid {
        shape,
        dimensions,
        vertices,
    }
}
