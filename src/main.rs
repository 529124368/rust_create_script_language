mod ast;
use nom::{
    branch::{alt, permutation},
    bytes::complete::{is_not, tag, take, take_till, take_until, take_while, take_while_m_n},
    character::complete::{
        alpha1, alphanumeric1, anychar, char, digit1, multispace0, multispace1, one_of,
    },
    combinator::recognize,
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, separated_pair, terminated, tuple},
    IResult,
};

fn main() {
    let programs = r#"
    fn test(a,b)
    {
        
    }
    "#;
    to_ast(programs).unwrap();
}

//剔除回车空格
fn del_space<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, f, multispace0)
}
///     expression_line;
// fn line(input: &str) -> IResult<&str, Vec<&str>> {
//     // *terminated multispace0* is important!
//     terminated(multispace0, multispace0)(input)
// }

//字符串转换成AST(抽象语法树)
fn to_ast(input: &str) -> IResult<&str, ast::Tree> {
    let (a, root) = many0(del_space(switch_get))(input)?;
    Ok((a, ast::Tree { root }))
}

//switch 方法 or 全局变量
fn switch_get(input: &str) -> IResult<&str, ast::Program> {
    alt((function_definition, global_variable_definition))(input)
}

//提取方法
fn function_definition(input: &str) -> IResult<&str, ast::Program> {
    let (input, _) = tag("fn")(input)?;
    let (input, _) = multispace1(input)?;
    let (args, name) = recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)?;
    let (input, b) = delimited(
        tag("("),
        separated_list0(
            delimited(multispace0, tag(","), multispace0),
            recognize(pair(
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_")))),
            )),
        ),
        tag(")"),
    )(args)?;
    println!("{:?}->{:?}->{:?}", input, b, name);
    Ok(("", todo!()))
}

//提取全局变量
fn global_variable_definition(input: &str) -> IResult<&str, ast::Program> {
    let (input, _) = tag("global")(input)?;
    let (input, _) = multispace1(input)?;
    Ok(("", todo!()))
}
