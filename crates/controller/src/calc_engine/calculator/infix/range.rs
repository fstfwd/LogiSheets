use logisheets_parser::ast;

use super::super::calc_vertex::{CalcReference, CalcVertex, ColRange, Reference, RowRange};
use logisheets_base::Addr as Address;

pub fn get_range(lhs: CalcVertex, rhs: CalcVertex) -> CalcVertex {
    match (lhs, rhs) {
        (CalcVertex::Reference(lhs_ref), CalcVertex::Reference(rhs_ref)) => {
            let lhs_prefix = lhs_ref.sheet;
            let rhs_prefix = rhs_ref.sheet;
            if lhs_prefix != rhs_prefix {
                return CalcVertex::from_error(ast::Error::Value);
            }
            let result = get_range_without_prefix(lhs_ref.reference, rhs_ref.reference);
            match result {
                Some(r) => CalcVertex::Reference(CalcReference {
                    from_sheet: None,
                    sheet: lhs_prefix,
                    reference: r,
                }),
                None => CalcVertex::from_error(ast::Error::Value),
            }
        }
        _ => CalcVertex::from_error(ast::Error::Value),
    }
}

pub fn get_range_without_prefix(l_ref: Reference, r_ref: Reference) -> Option<Reference> {
    match (l_ref, r_ref) {
        (Reference::Addr(la), Reference::Addr(ra)) => get_range_of_addresses(la, ra),
        (Reference::Addr(addr), Reference::ColumnRange(cr)) => {
            get_range_of_addr_and_col_range(addr, cr)
        }
        (Reference::Addr(addr), Reference::RowRange(rr)) => {
            get_range_of_addr_and_row_range(addr, rr)
        }
        (Reference::Addr(addr), Reference::Range(start, end)) => {
            get_range_of_addr_and_range(addr, start, end)
        }
        (Reference::ColumnRange(cr), Reference::Addr(addr)) => {
            get_range_of_addr_and_col_range(addr, cr)
        }
        (Reference::ColumnRange(lcr), Reference::ColumnRange(rcr)) => {
            get_range_of_col_ranges(lcr, rcr)
        }
        (Reference::ColumnRange(cr), Reference::RowRange(rr)) => {
            get_range_of_col_range_and_row_range(cr, rr)
        }
        (Reference::ColumnRange(cr), Reference::Range(start, end)) => {
            get_range_of_range_and_col_range(start, end, cr)
        }
        (Reference::RowRange(rr), Reference::Addr(addr)) => {
            get_range_of_addr_and_row_range(addr, rr)
        }
        (Reference::RowRange(rr), Reference::ColumnRange(cr)) => {
            get_range_of_col_range_and_row_range(cr, rr)
        }
        (Reference::RowRange(lrr), Reference::RowRange(rrr)) => get_range_of_row_ranges(lrr, rrr),
        (Reference::RowRange(rr), Reference::Range(start, end)) => {
            get_range_of_range_and_row_range(start, end, rr)
        }
        (Reference::Range(start, end), Reference::Addr(addr)) => {
            get_range_of_addr_and_range(addr, start, end)
        }
        (Reference::Range(start, end), Reference::ColumnRange(cr)) => {
            get_range_of_range_and_col_range(start, end, cr)
        }
        (Reference::Range(start, end), Reference::RowRange(rr)) => {
            get_range_of_range_and_row_range(start, end, rr)
        }
        (Reference::Range(left_start, left_end), Reference::Range(right_start, right_end)) => {
            get_range_of_ranges(left_start, left_end, right_start, right_end)
        }
    }
}

fn get_range_of_addresses(la: Address, ra: Address) -> Option<Reference> {
    Some(Reference::Range(la, ra))
}

fn get_range_of_ranges(
    lr_start: Address,
    lr_end: Address,
    rr_start: Address,
    rr_end: Address,
) -> Option<Reference> {
    let (col_start, col_end) =
        get_range_of_intervals((lr_start.col, lr_end.col), (rr_start.col, rr_end.col));
    let (row_start, row_end) =
        get_range_of_intervals((lr_start.row, lr_end.row), (rr_start.row, rr_end.row));
    Some(Reference::Range(
        Address {
            col: col_start,
            row: row_start,
        },
        Address {
            col: col_end,
            row: row_end,
        },
    ))
}

fn get_range_of_col_ranges(lcr: ColRange, rcr: ColRange) -> Option<Reference> {
    let (start, end) = get_range_of_intervals((lcr.start, lcr.end), (rcr.start, rcr.end));
    Some(Reference::ColumnRange(ColRange { start, end }))
}

fn get_range_of_row_ranges(lrr: RowRange, rrr: RowRange) -> Option<Reference> {
    let (start, end) = get_range_of_intervals((lrr.start, lrr.end), (rrr.start, rrr.end));
    Some(Reference::RowRange(RowRange { start, end }))
}

fn get_range_of_addr_and_range(
    addr: Address,
    range_start: Address,
    range_end: Address,
) -> Option<Reference> {
    let (col_start, col_end) =
        get_range_of_point_and_interval(addr.col, (range_start.col, range_end.col));
    let (row_start, row_end) =
        get_range_of_point_and_interval(addr.row, (range_start.row, range_end.row));
    Some(Reference::Range(
        Address {
            col: col_start,
            row: row_start,
        },
        Address {
            col: col_end,
            row: row_end,
        },
    ))
}

fn get_range_of_addr_and_col_range(addr: Address, cr: ColRange) -> Option<Reference> {
    let (start, end) = get_range_of_point_and_interval(addr.col, (cr.start, cr.end));
    Some(Reference::ColumnRange(ColRange { start, end }))
}

fn get_range_of_addr_and_row_range(addr: Address, rr: RowRange) -> Option<Reference> {
    let (start, end) = get_range_of_point_and_interval(addr.row, (rr.start, rr.end));
    Some(Reference::RowRange(RowRange { start, end }))
}

fn get_range_of_range_and_col_range(
    range_start: Address,
    range_end: Address,
    cr: ColRange,
) -> Option<Reference> {
    let (start, end) = get_range_of_intervals((range_start.col, range_end.col), (cr.start, cr.end));
    Some(Reference::ColumnRange(ColRange { start, end }))
}
fn get_range_of_range_and_row_range(
    range_start: Address,
    range_end: Address,
    rr: RowRange,
) -> Option<Reference> {
    let (start, end) = get_range_of_intervals((range_start.row, range_end.row), (rr.start, rr.end));
    Some(Reference::RowRange(RowRange { start, end }))
}

fn get_range_of_col_range_and_row_range(_cr: ColRange, _rr: RowRange) -> Option<Reference> {
    None
}

#[inline]
fn get_range_of_intervals(
    l_interval: (usize, usize),
    r_interval: (usize, usize),
) -> (usize, usize) {
    let l_ordered = order(l_interval.0, l_interval.1);
    let r_ordered = order(r_interval.0, r_interval.1);
    (
        order(l_ordered.0, r_ordered.0).0,
        order(l_ordered.1, r_ordered.1).1,
    )
}

#[inline]
fn get_range_of_point_and_interval(p: usize, interval: (usize, usize)) -> (usize, usize) {
    let ordered = order(interval.0, interval.1);
    (order(p, ordered.0).0, order(p, ordered.1).1)
}

#[inline]
fn order(l: usize, r: usize) -> (usize, usize) {
    if l < r {
        (l, r)
    } else {
        (r, l)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{CalcValue, Value};
    use super::{
        ast, get_range, get_range_without_prefix, Address, CalcReference, CalcVertex, ColRange,
        Reference, RowRange,
    };

    #[test]
    fn addr_test() {
        let addr = Reference::Addr(Address { row: 2, col: 3 });

        let r = get_range_without_prefix(addr.clone(), Reference::Addr(Address { row: 4, col: 1 }));
        assert!(matches!(
            r,
            Some(Reference::Range(
                Address { row: 2, col: 3 },
                Address { row: 4, col: 1 }
            )),
        ));

        let r = get_range_without_prefix(
            addr.clone(),
            Reference::Range(Address { row: 4, col: 2 }, Address { row: 6, col: 5 }),
        );
        assert!(matches!(
            r,
            Some(Reference::Range(
                Address { row: 2, col: 2 },
                Address { row: 6, col: 5 }
            )),
        ));

        let r = get_range_without_prefix(
            addr.clone(),
            Reference::ColumnRange(ColRange { start: 1, end: 3 }),
        );
        assert!(matches!(
            r,
            Some(Reference::ColumnRange(ColRange { start: 1, end: 3 })),
        ));

        let r = get_range_without_prefix(
            addr.clone(),
            Reference::RowRange(RowRange { start: 5, end: 6 }),
        );
        assert!(matches!(
            r,
            Some(Reference::RowRange(RowRange { start: 2, end: 6 })),
        ));
    }

    #[test]
    fn range_test() {
        let range = Reference::Range(Address { row: 6, col: 3 }, Address { row: 4, col: 6 });

        let r = get_range_without_prefix(
            range.clone(),
            Reference::Range(Address { row: 3, col: 2 }, Address { row: 5, col: 8 }),
        );
        assert!(matches!(
            r,
            Some(Reference::Range(
                Address { row: 3, col: 2 },
                Address { row: 6, col: 8 }
            )),
        ));

        let r = get_range_without_prefix(
            range.clone(),
            Reference::ColumnRange(ColRange { start: 1, end: 3 }),
        );
        assert!(matches!(
            r,
            Some(Reference::ColumnRange(ColRange { start: 1, end: 6 })),
        ));

        let r = get_range_without_prefix(
            Reference::RowRange(RowRange { start: 8, end: 9 }),
            range.clone(),
        );
        assert!(matches!(
            r,
            Some(Reference::RowRange(RowRange { start: 4, end: 9 })),
        ));
    }

    #[test]
    fn col_range_test() {
        let cr = Reference::ColumnRange(ColRange { start: 3, end: 6 });

        let r = get_range_without_prefix(
            cr.clone(),
            Reference::ColumnRange(ColRange { start: 1, end: 4 }),
        );
        assert!(matches!(
            r,
            Some(Reference::ColumnRange(ColRange { start: 1, end: 6 })),
        ));

        let r = get_range_without_prefix(
            Reference::RowRange(RowRange { start: 8, end: 9 }),
            cr.clone(),
        );
        assert!(matches!(r, None));
    }

    #[test]
    fn row_range_test() {
        let cr = Reference::RowRange(RowRange { start: 3, end: 6 });

        let r = get_range_without_prefix(
            Reference::RowRange(RowRange { start: 1, end: 7 }),
            cr.clone(),
        );
        assert!(matches!(
            r,
            Some(Reference::RowRange(RowRange { start: 1, end: 7 })),
        ));
    }
    #[test]
    fn prefix_test() {
        let cv1 = CalcVertex::Reference(CalcReference {
            from_sheet: None,
            sheet: 1,
            reference: Reference::Addr(Address { row: 1, col: 1 }),
        });
        let cv2 = CalcVertex::Reference(CalcReference {
            from_sheet: None,
            sheet: 1,
            reference: Reference::Addr(Address { row: 2, col: 2 }),
        });
        let r = get_range(cv1, cv2);
        assert!(matches!(
            r,
            CalcVertex::Reference(CalcReference {
                from_sheet: None,
                sheet: 1,
                reference: Reference::Range(Address { row: 1, col: 1 }, Address { row: 2, col: 2 },)
            })
        ));

        let cv1 = CalcVertex::Reference(CalcReference {
            from_sheet: None,
            sheet: 1,
            reference: Reference::Addr(Address { row: 1, col: 1 }),
        });
        let cv2 = CalcVertex::Reference(CalcReference {
            from_sheet: None,
            sheet: 2,
            reference: Reference::Addr(Address { row: 1, col: 1 }),
        });
        let r = get_range(cv1, cv2);
        assert!(matches!(
            r,
            CalcVertex::Value(CalcValue::Scalar(Value::Error(ast::Error::Value))),
        ))
    }
}
