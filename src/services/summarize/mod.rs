use crate::{models, services};
use chrono::{Datelike, NaiveDate};
use std::collections::{BTreeMap, BTreeSet};

pub fn run(file_path: &str) {
    println!("家計簿の集計を行います");

    let data = services::io::read_data_or_panic(file_path);

    let target_dates: BTreeSet<NaiveDate> = get_target_dates(&data);
    let mut result_table: BTreeMap<NaiveDate, i32> = BTreeMap::new();

    for date in target_dates {
        let filtered_data = get_filtered_date(&data, date);
        let sum = summarize_data(&filtered_data);
        result_table.insert(date, sum);
    }

    print_table(result_table);
}

fn get_target_dates(date: &Vec<models::Item>) -> BTreeSet<NaiveDate> {
    let target_dates: BTreeSet<_> = date.iter().map(|item| item.get_first_day()).collect();

    target_dates
}

fn get_filtered_date(date: &Vec<models::Item>, filtered_date: NaiveDate) -> Vec<&models::Item> {
    let filtered_date: Vec<&models::Item> = date
        .iter()
        .filter(|item| {
            (item.get_year() == filtered_date.year()) && (item.get_month() == filtered_date.month())
        })
        .collect();

    filtered_date
}

fn summarize_data(data: &Vec<&models::Item>) -> i32 {
    let mut sum = 0;
    for item in data {
        sum += item.get_price_for_summary();
    }

    sum
}

fn format_date(date: NaiveDate) -> String {
    format!("{}/{}", date.year(), date.month())
}

fn format_price(price: i32) -> String {
    if price > 0 {
        format!("+{}", price)
    } else {
        format!("{}", price)
    }
}

fn print_table(result_table: BTreeMap<NaiveDate, i32>) {
    for result in result_table {
        let data = format_date(result.0);
        let price = format_price(result.1);
        println!("{}の収支は{}円でした", data, price)
    }
}

#[cfg(test)]
mod summarize_text {
    use super::*;

    fn get_test_data() -> Vec<models::Item> {
        vec![
            super::models::Item::new(
                "新年会".to_string(),
                models::Category::Expense(models::ExpenseCategory::Food),
                5000,
                NaiveDate::from_ymd(2022, 1, 10),
            ),
            super::models::Item::new(
                "給料".to_string(),
                models::Category::Income(models::IncomeCategory::Salary),
                300000,
                NaiveDate::from_ymd(2022, 1, 20),
            ),
            super::models::Item::new(
                "旅行".to_string(),
                models::Category::Expense(models::ExpenseCategory::Hobby),
                100000,
                NaiveDate::from_ymd(2022, 1, 30),
            ),
            super::models::Item::new(
                "外食".to_string(),
                models::Category::Expense(models::ExpenseCategory::Food),
                3000,
                NaiveDate::from_ymd(2022, 2, 15),
            ),
            super::models::Item::new(
                "歓迎会".to_string(),
                models::Category::Expense(models::ExpenseCategory::Other),
                10000,
                NaiveDate::from_ymd(2022, 4, 15),
            ),
        ]
    }

    #[test]
    fn test_target_dates() {
        let test_data = get_test_data();
        let mut expected = BTreeSet::new();
        expected.insert(NaiveDate::from_ymd(2022, 1, 1));
        expected.insert(NaiveDate::from_ymd(2022, 2, 1));
        expected.insert(NaiveDate::from_ymd(2022, 4, 1));

        assert_eq!(get_target_dates(&test_data), expected)
    }
}
