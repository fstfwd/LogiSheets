use crate::calc_engine::calculator::math::bond::yieldmat;
use crate::calc_engine::calculator::math::day_count::{
    Actual360, Actual365, ActualActual, Europe30_360, UsPsa30_360,
};
use logisheets_parser::ast;

use super::super::{CalcValue, CalcVertex, Value};
use crate::calc_engine::connector::Connector;

pub fn calc<C>(args: Vec<CalcVertex>, fetcher: &mut C) -> CalcVertex
where
    C: Connector,
{
    assert_or_return!(args.len() >= 5 && args.len() <= 6, ast::Error::Unspecified);
    let mut args_iter = args.into_iter();

    let first = fetcher.get_calc_value(args_iter.next().unwrap());
    assert_f64_from_calc_value!(settlement, first);
    assert_or_return!(settlement > 0., ast::Error::Value);

    let second = fetcher.get_calc_value(args_iter.next().unwrap());
    assert_f64_from_calc_value!(maturity, second);
    assert_or_return!(maturity > 0., ast::Error::Value);

    let third = fetcher.get_calc_value(args_iter.next().unwrap());
    assert_f64_from_calc_value!(issue, third);
    assert_or_return!(issue > 0., ast::Error::Num);

    let fourth = fetcher.get_calc_value(args_iter.next().unwrap());
    assert_f64_from_calc_value!(rate, fourth);
    assert_or_return!(rate >= 0., ast::Error::Num);

    let fifth = fetcher.get_calc_value(args_iter.next().unwrap());
    assert_f64_from_calc_value!(pr, fifth);
    assert_or_return!(pr >= 0., ast::Error::Num);

    assert_or_return!(settlement < maturity, ast::Error::Num);

    let settle = settlement.floor() as u32;
    let maturity = maturity.floor() as u32;
    let issue = issue.floor() as u32;

    let result = if let Some(arg) = args_iter.next() {
        assert_f64_from_calc_value!(b, fetcher.get_calc_value(arg));
        assert_or_return!(b >= 0. && b <= 4., ast::Error::Num);
        match b.floor() as u8 {
            0 => yieldmat::<UsPsa30_360>(settle, maturity, issue, rate, pr),
            1 => yieldmat::<ActualActual>(settle, maturity, issue, rate, pr),
            2 => yieldmat::<Actual360>(settle, maturity, issue, rate, pr),
            3 => yieldmat::<Actual365>(settle, maturity, issue, rate, pr),
            4 => yieldmat::<Europe30_360>(settle, maturity, issue, rate, pr),
            _ => unreachable!(),
        }
    } else {
        yieldmat::<UsPsa30_360>(settle, maturity, issue, rate, pr)
    };
    CalcVertex::from_number(result)
}
