use crate::{parse_separator, split_line_by};
use nom::combinator::opt;
use nom::multi::{many0, many_till};
use nom::sequence::tuple;
use nom::IResult;
use std::str::FromStr;
use std::time::Duration;
use nom::bytes::complete::{tag, take_until};
use size::Size;

#[derive(Debug)]
pub enum TreeKind {
    Empty,
    Detailed,
    Peak,
}

#[derive(Debug)]
pub struct ParseTreeKindError;

impl FromStr for TreeKind {
    type Err = ParseTreeKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = match s {
            "empty" => Self::Empty,
            "detailed" => Self::Detailed,
            "peak" => Self::Peak,
            _ => unimplemented!("TreeKind {s} is not impl"),
        };

        Ok(kind)
    }
}

#[derive(Debug)]
pub struct Snapshot {
    pub num: u32,
    pub time: Duration,
    pub mem_heap: Size,
    pub mem_heap_extra: Size,
    pub mem_stacks: Size,
    pub heap_tree: TreeKind,
}

fn get_snapshot_num(input: &str) -> IResult<&str, u32> {
    let (rem, (_, num)) = split_line_by(input, "snapshot=")?;
    let num = num.parse().unwrap();

    Ok((rem, num))
}

fn parse_snapshot_header(input: &str) -> IResult<&str, u32> {
    // dbg!(input);
    let (rem, (_, num, _)) = tuple((parse_separator, get_snapshot_num, parse_separator))(input)?;

    Ok((rem, num))
}

fn parse_time(input: &str) -> IResult<&str, Duration> {
    let (rem, (_, time)) = split_line_by(input, "time=")?;
    let time = time.parse::<u64>().unwrap();
    let time = Duration::from_micros(time);

    Ok((rem, time))
}

fn parse_mem_heap(input: &str) -> IResult<&str, Size> {
    let (rem, (_, mem_heap)) = split_line_by(input, "mem_heap_B=")?;
    let mem_heap = mem_heap.parse::<u32>().unwrap();
    let mem_heap = Size::from_bytes(mem_heap);

    Ok((rem, mem_heap))
}

fn parse_mem_heap_extra(input: &str) -> IResult<&str, Size> {
    let (rem, (_, mem_heap_extra)) = split_line_by(input, "mem_heap_extra_B=")?;
    let mem_heap_extra = mem_heap_extra.parse::<u32>().unwrap();
    let mem_heap_extra = Size::from_bytes(mem_heap_extra);

    Ok((rem, mem_heap_extra))
}

fn parse_mem_stacks(input: &str) -> IResult<&str, Size> {
    let (rem, (_, mem_stacks)) = split_line_by(input, "mem_stacks_B=")?;
    let mem_stacks = mem_stacks.parse::<u32>().unwrap();
    let mem_stacks = Size::from_bytes(mem_stacks);

    Ok((rem, mem_stacks))
}

fn parse_heap_tree(input: &str) -> IResult<&str, TreeKind> {
    let (rem, (_, kind)) = split_line_by(input, "heap_tree=")?;
    let kind = kind.parse().unwrap();

    Ok((rem, kind))
}

fn parse_snapshot_data(input: &str) -> IResult<&str, Snapshot> {
    let (rem, (num, time, mem_heap, mem_heap_extra, mem_stacks, heap_tree, _)) = tuple((
        parse_snapshot_header,
        parse_time,
        parse_mem_heap,
        parse_mem_heap_extra,
        parse_mem_stacks,
        parse_heap_tree,
        opt(take_until("#-----------")),
    ))(input)?;

    Ok((
        rem,
        Snapshot {
            num,
            time,
            mem_heap,
            mem_heap_extra,
            mem_stacks,
            heap_tree,
        },
    ))
}

pub(crate) fn parse_snapshots(input: &str) -> IResult<&str, Vec<Snapshot>> {
    let (rem, snapshots) = many0(parse_snapshot_data)(input)?;

    Ok((rem, snapshots))
}
