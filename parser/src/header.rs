use crate::split_line_by;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, Default)]
pub struct Header {
    description: Option<String>,
    command: String,
    time_unit: String,
}

fn get_desc(input: &str) -> IResult<&str, Option<&str>> {
    let (rem, (_, desc)) = split_line_by(input, "desc: ")?;
    let desc = match desc {
        "(none)" => None,
        _ => Some(desc),
    };

    Ok((rem, desc))
}

fn get_cmd(input: &str) -> IResult<&str, &str> {
    let (rem, (_, cmd)) = split_line_by(input, "cmd: ")?;
    Ok((rem, cmd))
}

fn get_time_unit(input: &str) -> IResult<&str, &str> {
    let (rem, (_, unit)) = split_line_by(input, "time_unit: ")?;
    Ok((rem, unit))
}

pub(crate) fn read_header(input: &str) -> IResult<&str, Header> {
    let (rem, (desc, cmd, unit)) = tuple((get_desc, get_cmd, get_time_unit))(input)?;

    Ok((
        rem,
        Header {
            description: desc.map(|d| d.to_string()),
            command: cmd.to_string(),
            time_unit: unit.to_string(),
        },
    ))
}
