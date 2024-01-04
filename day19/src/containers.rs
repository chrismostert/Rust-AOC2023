use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct Workflow<'a> {
    pub rules: Vec<Rule<'a>>,
}

impl Workflow<'_> {
    pub fn run_on(&self, part: &Part) -> Output {
        let mut rules = self.rules.iter();
        let mut result = None;
        while result.is_none() {
            result = rules.next().unwrap().apply_to(part);
        }
        result.unwrap()
    }
}

pub struct Part {
    pub x: usize,
    pub m: usize,
    pub a: usize,
    pub s: usize,
}

impl Part {
    fn get(&self, field: &str) -> usize {
        match field {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Ranges {
    pub x: RangeInclusive<usize>,
    pub m: RangeInclusive<usize>,
    pub a: RangeInclusive<usize>,
    pub s: RangeInclusive<usize>,
}

pub enum RangeValue {
    Lower,
    Upper,
}

impl Ranges {
    pub fn set(&self, field: &str, range_value: RangeValue, value: usize) -> Ranges {
        let mut result = self.clone();
        let field_to_mutate = match field {
            "x" => &mut result.x,
            "m" => &mut result.m,
            "a" => &mut result.a,
            "s" => &mut result.s,
            _ => panic!(),
        };
        match range_value {
            RangeValue::Lower => *field_to_mutate = value..=*field_to_mutate.end(),
            RangeValue::Upper => *field_to_mutate = *field_to_mutate.start()..=value,
        }
        result
    }

    pub fn get_valid_amount(&self) -> usize {
        (self.x.end() - self.x.start() + 1)
            * (self.m.end() - self.m.start() + 1)
            * (self.a.end() - self.a.start() + 1)
            * (self.s.end() - self.s.start() + 1)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Output<'a> {
    Accept,
    Reject,
    Redirect(&'a str),
}

#[derive(Clone, Debug)]
pub enum Operator<'a> {
    Less(&'a str, usize, Output<'a>),
    Greater(&'a str, usize, Output<'a>),
    Return(Output<'a>),
}

#[derive(Debug, Clone)]
pub struct Rule<'a> {
    pub operation: Operator<'a>,
}

impl Rule<'_> {
    fn apply_to(&self, part: &Part) -> Option<Output> {
        match &self.operation {
            Operator::Less(field, compare, t) => {
                if part.get(field) < *compare {
                    Some(t.to_owned())
                } else {
                    None
                }
            }
            Operator::Greater(field, compare, t) => {
                if part.get(field) > *compare {
                    Some(t.to_owned())
                } else {
                    None
                }
            }
            Operator::Return(output) => Some(output.to_owned()),
        }
    }
}
