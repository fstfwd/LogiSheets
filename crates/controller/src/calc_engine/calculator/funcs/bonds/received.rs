use crate::calc_engine::calculator::math::bond::received;
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
    assert_or_return!(args.len() >= 4 && args.len() <= 5, ast::Error::Unspecified);
    let mut args_iter = args.into_iter();

    let first = fetcher.get_calc_value(args_iter.next().unwrap());
    assert_f64_from_calc_value!(settlement, first);
    assert_or_return!(settlement > 0., ast::Error::Value);

    let second = fetcher.get_calc_value(args_iter.next().unwrap());
    assert_f64_from_calc_value!(maturity, second);
    assert_or_return!(maturity > 0., ast::Error::Value);

    let third = fetcher.get_calc_value(args_iter.next().unwrap());
    assert_f64_from_calc_value!(investment, third);
    assert_or_return!(investment > 0., ast::Error::Num);

    let fourth = fetcher.get_calc_value(args_iter.next().unwrap());
    assert_f64_from_calc_value!(discount, fourth);
    assert_or_return!(discount > 0., ast::Error::Num);

    assert_or_return!(settlement < maturity, ast::Error::Num);

    let settle = settlement.floor() as u32;
    let maturity = maturity.floor() as u32;

    let result = if let Some(arg) = args_iter.next() {
        assert_f64_from_calc_value!(b, fetcher.get_calc_value(arg));
        assert_or_return!(b >= 0. && b <= 4., ast::Error::Num);
        match b.floor() as u8 {
            0 => received::<UsPsa30_360>(settle, maturity, investment, discount),
            1 => received::<ActualActual>(settle, maturity, investment, discount),
            2 => received::<Actual360>(settle, maturity, investment, discount),
            3 => received::<Actual365>(settle, maturity, investment, discount),
            4 => received::<Europe30_360>(settle, maturity, investment, discount),
            _ => unreachable!(),
        }
    } else {
        received::<UsPsa30_360>(settle, maturity, investment, discount)
    };
    CalcVertex::from_number(result)
}
