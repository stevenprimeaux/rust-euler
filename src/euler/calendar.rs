use std::ops::RangeInclusive;

const MONTH_STARTS: [u16; 12] = [1, 32, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];
const MONTH_STARTS_LEAP: [u16; 12] = [1, 32, 61, 92, 122, 153, 183, 214, 245, 275, 306, 336];

#[derive(Debug, PartialEq)]
pub struct Year {
    year: u16,
    is_leap: bool,
    days: u16,
    month_starts: [u16; 12],
}

impl Year {
    fn new(year: u16) -> Self {
        let mut is_leap: bool = false;
        let mut days: u16 = 365;
        let mut month_starts: [u16; 12] = MONTH_STARTS;

        if year % 100 == 0 {
            if year % 400 == 0 {
                is_leap = true;
            }
        } else if year % 4 == 0 {
            is_leap = true;
        }

        if is_leap {
            days = 366;
            month_starts = MONTH_STARTS_LEAP;
        }

        Self {
            year,
            is_leap,
            days,
            month_starts,
        }
    }
}

pub fn cal_years(years_range: RangeInclusive<u16>) -> Vec<Year> {
    years_range.map(|x: u16| Year::new(x)).collect()
}

pub fn cal_firstsundays_count(year_start: u16, year_end: u16, dayweek_start: u64) -> u64 {
    let mut count_firstsundays: u64 = 0;
    let mut count_days: u64 = dayweek_start;
    let years: Vec<Year> = cal_years(year_start..=year_end);
    for y in years {
        for d in 1..=y.days {
            if count_days % 7 == 0 {
                if y.month_starts.contains(&d) {
                    count_firstsundays += 1;
                }
            }
            count_days += 1;
        }
    }

    count_firstsundays
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cal_get_years() {
        let years_1900: [Year; 5] = [
            Year::new(1900),
            Year::new(1901),
            Year::new(1902),
            Year::new(1903),
            Year::new(1904),
        ];
        let years_2000: [Year; 5] = [
            Year::new(2000),
            Year::new(2001),
            Year::new(2002),
            Year::new(2003),
            Year::new(2004),
        ];

        assert_eq!(cal_years(1900..=1904), years_1900);
        assert_eq!(cal_years(2000..=2004), years_2000)
    }

    #[test]
    fn test_cal_count_sundays() {
        assert_eq!(cal_firstsundays_count(1901, 2000, 2), 171);
    }
}
