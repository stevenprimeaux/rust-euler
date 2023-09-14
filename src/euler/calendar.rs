use std::ops::RangeInclusive;

const MONTH_STARTS: [u16; 12] = [1, 32, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];
const MONTH_STARTS_LEAP: [u16; 12] = [1, 32, 61, 92, 122, 153, 183, 214, 245, 275, 306, 336];

#[derive(Clone, Debug, PartialEq)]
pub struct Year {
    is_leap: bool,
}

impl Year {
    fn days(&self) -> u16 {
        let mut days: u16 = 365;
        if self.is_leap {
            days = 366;
        }

        days
    }

    fn month_starts(&self) -> [u16; 12] {
        let mut starts: [u16; 12] = MONTH_STARTS;
        if self.is_leap {
            starts = MONTH_STARTS_LEAP;
        }

        starts
    }
}

pub fn cal_get_years(years_range: RangeInclusive<u16>) -> Vec<Year> {
    let mut years: Vec<Year> = vec![Year { is_leap: false }; years_range.len()];
    for (i, year) in years_range.enumerate() {
        if year % 100 == 0 {
            if year % 400 == 0 {
                years[i].is_leap = true;
            }
        } else if year % 4 == 0 {
            years[i].is_leap = true;
        }
    }

    years
}

pub fn cal_count_sundays(year_start: u16, year_end: u16, dayweek_start: u64) -> u64 {
    let mut count_days: u64 = dayweek_start;
    let mut count_first_sundays: u64 = 0;
    let years: Vec<Year> = cal_get_years(year_start..=year_end);
    for y in years {
        for d in 1..=y.days() {
            if count_days % 7 == 0 {
                if y.month_starts().contains(&d) {
                    count_first_sundays += 1;
                }
            }
            count_days += 1;
        }
    }

    count_first_sundays
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cal_get_years() {
        let years_1900 = vec![
            Year { is_leap: false },
            Year { is_leap: false },
            Year { is_leap: false },
            Year { is_leap: false },
            Year { is_leap: true },
        ];
        let years_2000 = vec![
            Year { is_leap: true },
            Year { is_leap: false },
            Year { is_leap: false },
            Year { is_leap: false },
            Year { is_leap: true },
        ];

        assert_eq!(cal_get_years(1900..=1904), years_1900);
        assert_eq!(cal_get_years(2000..=2004), years_2000)
    }

    #[test]
    fn test_cal_count_sundays() {
        assert_eq!(cal_count_sundays(1900, 1900, 0), 2);
        assert_eq!(cal_count_sundays(2000, 2000, 6), 1);
        assert_eq!(cal_count_sundays(1901, 2000, 2), 171);
    }
}
