use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{alpha1, alphanumeric1, digit1, multispace0, multispace1},
    combinator::recognize,
    multi::{fold_many0, many0, separated_list0},
    sequence::{delimited, pair, terminated},
    IResult,
};

use crate::ast;
static mut FLG: bool = false;

//字符串转换成AST(抽象语法树)
pub fn to_ast(input: &str) -> IResult<&str, ast::Tree> {
    let (a, root) = many0(del_space(switch_get))(input)?;
    Ok((a, ast::Tree { root }))
}

//方法 + 全局变量
fn switch_get(input: &str) -> IResult<&str, ast::Program> {
    alt((function_definition, global_variable_definition))(input)
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

//剔除回车空格
fn del_block<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(tag("("), f, tag(")"))
}

fn loops(input: &str) -> IResult<&str, ast::Token> {
    terminated(
        alt((block_get, println_get, assignment_get, express_get)),
        multispace0,
    )(input)
}

fn block_get(input: &str) -> IResult<&str, ast::Token> {
    let (input, elements) = delimited(tag("{"), del_space(many0(loops)), tag("}"))(input)?;
    Ok((input, ast::block(elements)))
}

fn assignment_get(input: &str) -> IResult<&str, ast::Token> {
    let (input, name) = terminated(del_space(get_params), tag("="))(input)?;
    let (input, val) = del_space(alt((get_dig_numbesr, get_float_numbesr)))(input)?;
    let (_, val) = take_until(";")(val)?;
    Ok((
        input,
        ast::assignment(
            name,
            ast::set_zval(val.parse::<f64>().unwrap(), "", "number".to_string()),
        ),
    ))
}

fn println_get(input: &str) -> IResult<&str, ast::Token> {
    let (input, _) = terminated(tag("println"), multispace0)(input)?;
    let (input, d) = terminated(
        delimited(
            tag("("),
            del_space(alt((get_float_number, alpha1, digit1, get_string))),
            tag(")"),
        ),
        tag(";"),
    )(input)?;
    //数字的场合
    if d.parse::<f64>().is_ok() {
        Ok((
            input,
            ast::ast_println(ast::set_zval(
                d.parse::<f64>().unwrap(),
                "",
                "number".to_string(),
            )),
        ))
    } else {
        //字符串的场合
        if unsafe { FLG } {
            Ok((
                input,
                ast::ast_println(ast::set_zval(
                    0.0,
                    &d.replace('"', ""),
                    "string".to_string(),
                )),
            ))
        } else {
            Ok((input, ast::ast_println(ast::set_flg(d))))
        }
    }
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
        separated_list0(delimited(multispace0, tag(","), multispace0), get_params),
        tag(")"),
    ))(args)?;
    let (input, ex) = block_get(input)?;
    Ok((input, ast::define_function(name, &b, ex)))
}

//提取全局变量
fn global_variable_definition(input: &str) -> IResult<&str, ast::Program> {
    let (input, _) = tag("global")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = terminated(del_space(get_params), tag("="))(input)?;
    let (input, val) = del_space(alt((get_dig_numbesr, get_float_numbesr)))(input)?;
    let (_, val) = take_until(";")(val)?;
    Ok((
        input,
        ast::difine_global_variable(
            name,
            ast::set_zval(val.parse::<f64>().unwrap(), "", "number".to_string()),
        ),
    ))
}

fn get_float_number(input: &str) -> IResult<&str, &str> {
    let (inptu1, input2) = recognize(delimited(digit1, tag("."), digit1))(input)?;
    Ok((inptu1, input2))
}

fn get_float_numbesr(input: &str) -> IResult<&str, &str> {
    let (inptu1, input2) = recognize(delimited(
        digit1,
        tag("."),
        delimited(multispace0, digit1, tag(";")),
    ))(input)?;
    Ok((inptu1, input2))
}

fn get_dig_numbesr(input: &str) -> IResult<&str, &str> {
    let (inptu1, input2) = recognize(pair(digit1, tag(";")))(input)?;
    Ok((inptu1, input2))
}

fn get_string(input: &str) -> IResult<&str, &str> {
    unsafe { FLG = true };
    let (inptu1, input2) = recognize(delimited(
        tag("\""),
        take_while1(|f| {
            if f == '"' {
                return false;
            } else {
                return true;
            }
        }),
        tag("\""),
    ))(input)?;
    Ok((inptu1, input2))
}

fn get_params(input: &str) -> IResult<&str, &str> {
    let (input1, input2) = recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)?;
    Ok((input1, input2))
}

pub fn express_get(input: &str) -> IResult<&str, ast::Token> {
    let (input, name) = terminated(del_space(get_params), tag("="))(input)?;
    let (other, input) = take_until(";")(input)?;

    let (_, res) = calcal_get(input)?;
    let (other, _) = tag(";")(other)?;
    Ok((other, ast::assignment(name, res)))
}

fn get_tt(input: &str) -> IResult<&str, ast::Token> {
    alt((del_block(calcal_get), set_value))(input)
}

fn calcal_get(input: &str) -> IResult<&str, ast::Token> {
    let (input, lefthandle) = get_tt(input)?;

    let result = fold_many0(
        pair(del_space(alt((tag("+"), tag("-")))), get_tt),
        || lefthandle.clone(),
        |acc, (operator, right_operand)| match operator {
            "+" => ast::add(acc, right_operand),
            "-" => ast::subtract(acc, right_operand),
            _ => unreachable!(),
        },
    )(input)?;
    Ok(result)
}

fn set_value(input: &str) -> IResult<&str, ast::Token> {
    let (a, b) = alt((digit1, get_float_number))(input)?;
    Ok((
        a,
        ast::set_zval(b.parse::<f64>().unwrap(), "", "number".to_string()),
    ))
}
