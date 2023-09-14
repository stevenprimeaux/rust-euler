pub struct Year {
    year: u16,
    days: u16,
    is_leap: bool,
}

impl Year {
    fn month_starts(&self) -> Vec<u16> {
        let mut starts: Vec<u16> = vec![1, 32, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];
        if self.is_leap {
            starts = vec![1, 32, 61, 92, 122, 153, 183, 214, 245, 275, 306, 336];
        }

        starts
    }
}

pub fn cal_get_years() -> Vec<Year> {
    let mut years: Vec<Year> = vec![];
    for year in 1900..=2000 {
        let mut days: u16 = 365;
        let mut is_leap: bool = false;
        if year % 100 == 0 {
            if year % 400 == 0 {
                is_leap = true;
            }
        } else if year % 4 == 0 {
            is_leap = true;
        }

        if is_leap {
            days = 366;
        }

        years.push(Year {
            year,
            days,
            is_leap,
        });
    }

    years
}

pub fn cal_count_sundays() -> u64 {
    let mut count_days: u64 = 0;
    let mut count_first_sundays: u64 = 0;
    let years: Vec<Year> = cal_get_years();
    for y in years {
        for d in 1..=y.days {
            count_days += 1;
            if count_days % 7 == 0 {
                if y.month_starts().contains(&d) && y.year != 1900 {
                    count_first_sundays += 1;
                }
            }
        }
    }

    count_first_sundays
}
