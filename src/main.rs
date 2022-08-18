mod ast;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, digit1, multispace0, multispace1},
    combinator::recognize,
    multi::{many0, separated_list0},
    sequence::{delimited, pair, terminated},
    IResult,
};

fn main() {
    let programs = r#"
    global wh = 12;
    fn hell(n,d) {
        w =n;
        s = d;
        println(12321);
    }
    fn main()
    {
        a = 12;
        println(12321);
        println(wh);
    }
    "#;
    let (_, b) = to_ast(programs).unwrap();
    println!("{:#?}", b);
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

fn loops(input: &str) -> IResult<&str, ast::Expression> {
    terminated(alt((block_get, println_get, assignment_get)), multispace0)(input)
}

fn block_get(input: &str) -> IResult<&str, ast::Expression> {
    let (input, elements) = delimited(tag("{"), del_space(many0(loops)), tag("}"))(input)?;
    Ok((input, ast::block(elements)))
}

fn assignment_get(input: &str) -> IResult<&str, ast::Expression> {
    let (input, name) = terminated(
        del_space(recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        ))),
        tag("="),
    )(input)?;
    let (input, data) = terminated(del_space(digit1), tag(";"))(input)?;
    Ok((
        input,
        ast::assignment(name, ast::integer(data.parse::<i64>().unwrap())),
    ))
}

fn println_get(input: &str) -> IResult<&str, ast::Expression> {
    let (input, _) = terminated(tag("println"), multispace0)(input)?;
    let (input, d) = terminated(
        delimited(tag("("), del_space(alt((digit1, alpha1))), tag(")")),
        tag(";"),
    )(input)?;
    Ok((input, ast::ast_println(d)))
}

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
    let (input, b) = del_space(delimited(
        tag("("),
        separated_list0(
            delimited(multispace0, tag(","), multispace0),
            recognize(pair(
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_")))),
            )),
        ),
        tag(")"),
    ))(args)?;
    let (input, ex) = block_get(input)?;
    Ok((input, ast::define_function(name, &b, ex)))
}

//提取全局变量
fn global_variable_definition(input: &str) -> IResult<&str, ast::Program> {
    let (input, _) = tag("global")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = terminated(
        del_space(recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        ))),
        tag("="),
    )(input)?;
    let (input, val) = terminated(del_space(digit1), tag(";"))(input)?;
    Ok((
        input,
        ast::difine_global_variable(name, ast::integer(val.parse::<i64>().unwrap())),
    ))
}
