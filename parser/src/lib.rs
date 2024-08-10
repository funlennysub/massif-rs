mod header;
mod snapshot;

use crate::header::{read_header, Header};
use crate::snapshot::{parse_snapshots, Snapshot};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric0, anychar, line_ending, not_line_ending};
use nom::multi::many0;
use nom::sequence::terminated;
use nom::{Finish, IResult};
use std::fs;

#[derive(Debug, Default)]
pub struct Output {
    pub header: Header,
    pub snapshots: Vec<Snapshot>,
}

/// Splits string by `\n`. Everything after new line will be on the left.
fn get_line(input: &str) -> IResult<&str, &str> {
    terminated(not_line_ending, line_ending)(input)
}

fn split_by<'a>(input: &'a str, kw: &'a str) -> IResult<&'a str, &'a str> {
    let (r, l) = tag(kw)(input)?;

    Ok((l, r))
}

fn split_line_by<'a>(input: &'a str, kw: &'a str) -> IResult<&'a str, (&'a str, &'a str)> {
    let (rem, line) = get_line(input)?;

    Ok((rem, split_by(line, kw)?))
}

fn parse_separator(input: &str) -> IResult<&str, &str> {
    let (rem, line) = get_line(input)?;
    let (_, sep) = tag("#-----------")(line)?;

    Ok((rem, sep))
}

pub fn read_file(file: &str) -> Output {
    let input = fs::read_to_string(file).unwrap();
    let (rem, header) = read_header(&input).unwrap();
    let (_, snapshots) = parse_snapshots(rem).finish().unwrap();

    Output { header, snapshots }
}
