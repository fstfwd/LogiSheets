pub fn calc_sln(cost: f64, salvage: f64, life: u32) -> f64 {
    (cost - salvage) / (life as f64)
}

pub fn calc_syd(cost: f64, salvage: f64, life: u32, per: f64) -> f64 {
    let life = life as f64;
    (cost - salvage) * (life - per + 1.) * 2. / (life * (life + 1.))
}

#[cfg(test)]
mod tests {
    use super::calc_sln;

    #[test]
    fn test1() {
        let res = calc_sln(104000., 4000., 10);
        assert_eq!(res, 10000.00);
        let res = calc_sln(5000., 500., 5);
        assert_eq!(res, 900.00);
    }
}
