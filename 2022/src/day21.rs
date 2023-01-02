use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{alpha1, i64},
    combinator::map,
    sequence::{delimited, terminated, tuple},
    IResult,
};

pub(crate) fn part_1(input: &str) -> String {
    let formulas = input
        .lines()
        .map(|l| parse_formula(l).unwrap().1)
        .collect::<Vec<_>>();
    ExpressionTree::from_formulas(&formulas, "root", None, None)
        .flatten()
        .solve()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let formulas = input
        .lines()
        .map(|l| parse_formula(l).unwrap().1)
        .collect::<Vec<_>>();
    ExpressionTree::from_formulas(&formulas, "root", Some("root"), Some("humn"))
        .flatten()
        .solve()
        .to_string()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Formula<'a> {
    name: &'a str,
    expression: Expression<'a>,
}

impl<'a> From<(&'a str, Expression<'a>)> for Formula<'a> {
    fn from((name, expression): (&'a str, Expression<'a>)) -> Self {
        Formula { name, expression }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Expression<'a> {
    Number(i64),
    Sum(&'a str, &'a str),
    Difference(&'a str, &'a str),
    Product(&'a str, &'a str),
    Quotient(&'a str, &'a str),
}

impl<'a> Expression<'a> {
    fn operands(&self) -> (&str, &str) {
        match self {
            Expression::Number(_) => unreachable!(),
            Expression::Sum(a, b) => (a, b),
            Expression::Difference(a, b) => (a, b),
            Expression::Product(a, b) => (a, b),
            Expression::Quotient(a, b) => (a, b),
        }
    }
}

impl<'a> From<i64> for Expression<'a> {
    fn from(value: i64) -> Self {
        Expression::Number(value)
    }
}

impl<'a> From<(&'a str, &'a str, &'a str)> for Expression<'a> {
    fn from((operand1, operator, operand2): (&'a str, &'a str, &'a str)) -> Self {
        match operator {
            "+" => Expression::Sum(operand1, operand2),
            "-" => Expression::Difference(operand1, operand2),
            "*" => Expression::Product(operand1, operand2),
            "/" => Expression::Quotient(operand1, operand2),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ExpressionTree {
    Number(i64),
    Variable,
    Equation(Box<ExpressionTree>, Box<ExpressionTree>),
    Sum(Box<ExpressionTree>, Box<ExpressionTree>),
    Difference(Box<ExpressionTree>, Box<ExpressionTree>),
    Product(Box<ExpressionTree>, Box<ExpressionTree>),
    Quotient(Box<ExpressionTree>, Box<ExpressionTree>),
}

impl ExpressionTree {
    fn from_formulas(
        formulas: &[Formula],
        current: &str,
        equation: Option<&str>,
        variable: Option<&str>,
    ) -> Self {
        let current = formulas.iter().find(|f| f.name == current).unwrap();
        if Some(current.name) == equation && !matches!(current.expression, Expression::Number(_)) {
            let operands = current.expression.operands();
            return ExpressionTree::Equation(
                Box::new(ExpressionTree::from_formulas(
                    formulas, operands.0, equation, variable,
                )),
                Box::new(ExpressionTree::from_formulas(
                    formulas, operands.1, equation, variable,
                )),
            );
        }
        if Some(current.name) == variable {
            return ExpressionTree::Variable;
        }
        match current.expression {
            Expression::Number(value) => ExpressionTree::Number(value),
            Expression::Sum(a, b) => ExpressionTree::Sum(
                Box::new(ExpressionTree::from_formulas(
                    formulas, a, equation, variable,
                )),
                Box::new(ExpressionTree::from_formulas(
                    formulas, b, equation, variable,
                )),
            ),
            Expression::Difference(a, b) => ExpressionTree::Difference(
                Box::new(ExpressionTree::from_formulas(
                    formulas, a, equation, variable,
                )),
                Box::new(ExpressionTree::from_formulas(
                    formulas, b, equation, variable,
                )),
            ),
            Expression::Product(a, b) => ExpressionTree::Product(
                Box::new(ExpressionTree::from_formulas(
                    formulas, a, equation, variable,
                )),
                Box::new(ExpressionTree::from_formulas(
                    formulas, b, equation, variable,
                )),
            ),
            Expression::Quotient(a, b) => ExpressionTree::Quotient(
                Box::new(ExpressionTree::from_formulas(
                    formulas, a, equation, variable,
                )),
                Box::new(ExpressionTree::from_formulas(
                    formulas, b, equation, variable,
                )),
            ),
        }
    }

    fn flatten(self) -> ExpressionTree {
        match self {
            ExpressionTree::Number(_) | ExpressionTree::Variable => self,
            ExpressionTree::Equation(a, b) => {
                ExpressionTree::Equation(Box::new(a.flatten()), Box::new(b.flatten()))
            }
            ExpressionTree::Sum(a, b) => {
                let a = a.flatten();
                let b = b.flatten();
                if let ExpressionTree::Number(a) = a && let ExpressionTree::Number(b) = b
                {
                    ExpressionTree::Number(a + b)
                } else {
                    ExpressionTree::Sum(Box::new(a), Box::new(b))
                }
            }
            ExpressionTree::Difference(a, b) => {
                let a = a.flatten();
                let b = b.flatten();
                if let ExpressionTree::Number(a) = a && let ExpressionTree::Number(b) = b
                {
                    ExpressionTree::Number(a - b)
                } else {
                    ExpressionTree::Difference(Box::new(a), Box::new(b))
                }
            }
            ExpressionTree::Product(a, b) => {
                let a = a.flatten();
                let b = b.flatten();
                if let ExpressionTree::Number(a) = a && let ExpressionTree::Number(b) = b
                {
                    ExpressionTree::Number(a * b)
                } else {
                    ExpressionTree::Product(Box::new(a), Box::new(b))
                }
            }
            ExpressionTree::Quotient(a, b) => {
                let a = a.flatten();
                let b = b.flatten();
                if let ExpressionTree::Number(a) = a && let ExpressionTree::Number(b) = b
                {
                    ExpressionTree::Number(a / b)
                } else {
                    ExpressionTree::Quotient(Box::new(a), Box::new(b))
                }
            }
        }
    }

    fn solve(&self) -> i64 {
        match self {
            ExpressionTree::Equation(a, b) => a.solve_equals(b),
            ExpressionTree::Number(value) => *value,
            _ => unreachable!(),
        }
    }

    fn solve_equals(&self, other: &ExpressionTree) -> i64 {
        match (self, other) {
            (ExpressionTree::Number(value), ExpressionTree::Variable)
            | (ExpressionTree::Variable, ExpressionTree::Number(value)) => *value,
            (ExpressionTree::Number(value), ExpressionTree::Sum(a, b))
            | (ExpressionTree::Sum(a, b), ExpressionTree::Number(value)) => {
                if let ExpressionTree::Number(value2) = &**a {
                    b.solve_equals(&ExpressionTree::Number(value - value2))
                } else if let ExpressionTree::Number(value2) = &**b {
                    a.solve_equals(&ExpressionTree::Number(value - value2))
                } else {
                    unreachable!()
                }
            }
            (ExpressionTree::Number(value), ExpressionTree::Difference(a, b))
            | (ExpressionTree::Difference(a, b), ExpressionTree::Number(value)) => {
                if let ExpressionTree::Number(value2) = &**a {
                    // v2-b=v => -b=v-v2 => b=-(v-v2)=-v+v2=v2-v
                    b.solve_equals(&ExpressionTree::Number(value2 - value))
                } else if let ExpressionTree::Number(value2) = &**b {
                    // a-v2=v => a=v2+v
                    a.solve_equals(&ExpressionTree::Number(value2 + value))
                } else {
                    unreachable!()
                }
            }
            (ExpressionTree::Number(value), ExpressionTree::Product(a, b))
            | (ExpressionTree::Product(a, b), ExpressionTree::Number(value)) => {
                if let ExpressionTree::Number(value2) = &**a {
                    b.solve_equals(&ExpressionTree::Number(value / value2))
                } else if let ExpressionTree::Number(value2) = &**b {
                    a.solve_equals(&ExpressionTree::Number(value / value2))
                } else {
                    unreachable!()
                }
            }
            (ExpressionTree::Number(value), ExpressionTree::Quotient(a, b))
            | (ExpressionTree::Quotient(a, b), ExpressionTree::Number(value)) => {
                if let ExpressionTree::Number(value2) = &**a {
                    // v2/b=v => 1/b=v/v2 => b=v2/v
                    b.solve_equals(&ExpressionTree::Number(value2 / value))
                } else if let ExpressionTree::Number(value2) = &**b {
                    // a/v2=v => a=v2*v
                    a.solve_equals(&ExpressionTree::Number(value2 * value))
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }
}

fn parse_formula(input: &str) -> IResult<&str, Formula> {
    map(
        tuple((
            terminated(alpha1, tag(": ")),
            alt((
                map(i64, Expression::from),
                map(
                    tuple((alpha1, delimited(tag(" "), take(1u8), tag(" ")), alpha1)),
                    Expression::from,
                ),
            )),
        )),
        Formula::from,
    )(input)
}
