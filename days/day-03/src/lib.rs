use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, u32},
    multi::fold_many0,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct Mul(u32, u32);

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Mul(Mul),
    Do,
    Dont,
    Junk,
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    let inner = separated_pair(u32, tag(","), u32);
    let (rest, (x, y)) = delimited(tag("mul("), inner, tag(")"))(input)?;
    Ok((rest, Instruction::Mul(Mul(x, y))))
}

fn parse_junk(input: &str) -> IResult<&str, Instruction> {
    let (rest, _) = anychar(input)?;
    Ok((rest, Instruction::Junk))
}

fn parse_do(input: &str) -> IResult<&str, Instruction> {
    let (rest, _) = tag("do()")(input)?;
    Ok((rest, Instruction::Do))
}

fn parse_dont(input: &str) -> IResult<&str, Instruction> {
    let (rest, _) = tag("don't()")(input)?;
    Ok((rest, Instruction::Dont))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_mul, parse_dont, parse_do, parse_junk))(input)
}

fn parse_full_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    fold_many0(parse_instruction, Vec::new, |mut acc: Vec<_>, item| {
        acc.push(item);
        acc
    })(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Mul>> {
    fold_many0(parse_instruction, Vec::new, |mut acc: Vec<_>, item| {
        if let Instruction::Mul(mul) = item {
            acc.push(mul);
        }
        acc
    })(input)
}

fn multiply(xs: &[Mul]) -> u32 {
    xs.iter().fold(0, |acc, m| acc + (m.0 * m.1))
}

pub fn uncorrupted(input: &str) -> u32 {
    let (_, xs) = parse_instructions(input).unwrap();
    multiply(&xs)
}
pub fn uncorrupted_enabled(input: &str) -> u32 {
    let (_, res) = parse_full_instructions(input).unwrap();
    let mut enabled = true;
    let mut acc = 0;
    for cmd in res.into_iter() {
        match cmd {
            Instruction::Mul(mul) if enabled => acc += mul.0 * mul.1,
            Instruction::Do => {
                enabled = true;
            }
            Instruction::Dont => {
                enabled = false;
            }
            _ => continue,
        }
    }
    acc
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_mul() {
        let s = "mul(10,20)";
        assert_eq!(parse_mul(s), Ok(("", Instruction::Mul(Mul(10, 20)))));
    }

    #[test]
    fn test_uncorrupted() {
        let s = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(
            parse_instructions(s).unwrap().1,
            vec![Mul(2, 4), Mul(5, 5), Mul(11, 8), Mul(8, 5)]
        );
        assert_eq!(uncorrupted(s), 161);
    }

    #[test]
    fn test_parse_full_instructions() {
        let s = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(
            parse_full_instructions(s)
                .unwrap()
                .1
                .into_iter()
                .filter(|x| !matches!(x, Instruction::Junk))
                .collect::<Vec<_>>(),
            vec![
                Instruction::Mul(Mul(2, 4)),
                Instruction::Dont,
                Instruction::Mul(Mul(5, 5)),
                Instruction::Mul(Mul(11, 8)),
                Instruction::Do,
                Instruction::Mul(Mul(8, 5))
            ]
        );
        assert_eq!(uncorrupted_enabled(s), 48);
    }
}
