pub mod events;
pub mod models;

pub mod utils {
    use std::fmt;

    pub static YEAR: u64 = 365;
    pub static MONTH: u64 = 30;
    pub static DAY: u64 = 1;

    pub struct Date {
        pub year: u64,
        pub month: u64,
        pub day: u64,
    }

    /// Custom Date
    ///
    impl Date {
        pub fn new(day: u64) -> Date {
            let year = if day >= YEAR { day / YEAR + 1 } else { 1 };

            let day = day - (year - 1) * YEAR;

            let month = if day >= MONTH { day / MONTH + 1 } else { 1 };

            let day = day - (month - 1) * MONTH;

            Date { year, month, day }
        }

        /// # leap year condition
        /// - year % 4 == 0
        /// - year % 400 == 0
        ///
        /// # common year condition
        /// - year % 100 == 0
        pub fn is_leap_year(year: u64) -> bool {
            if year % 4 == 0 {
                if year % 100 == 0 {
                    return year % 400 == 0;
                }
                return true;
            }
            return false;
        }

        pub fn count_leap_year(year: u64) -> u64 {
            let leap_year_count = year / 4;
            let leap_year_count = leap_year_count - year / 100;
            let leap_year_count = leap_year_count + year / 400;

            leap_year_count
        }

        pub fn to_day(&self) -> u64 {
            (self.year - 1) * YEAR + (self.month - 1) * MONTH + self.day
        }
    }

    impl fmt::Display for Date {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}/{}/{}", self.year, self.month, self.day)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::Date;

    #[test]
    fn year() {
        let date = Date::new(36544);

        assert_eq!(format!("{}", date), "101/2/14");
    }

    #[test]
    fn to_day() {
        let date = Date::new(36544);

        assert_eq!(date.to_day(), 36544);
    }

    #[test]
    fn count_leap_year() {
        assert_eq!(Date::count_leap_year(3), 0);
        assert_eq!(Date::count_leap_year(4), 1);
        assert_eq!(Date::count_leap_year(957), 232);
    }
}
