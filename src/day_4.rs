//! This module contains the code
//! for the solution of the fourth day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/4).
use std::collections::HashMap;

/// Check how many passports contain the required fields.
///
/// `data` is the content of the input batch file, which contains
/// the data of all passports, where the batches are separated by
/// blank lines.
pub fn task_1(data: &str) -> usize {
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    data.split("\n\n")
        .map(|p| passport_has_valid_fields(p, &required_fields))
        .filter(|b| *b)
        .count()
}

/// Check how many passports are valid.
///
/// `data` is the content of the input batch file, which contains
/// the data of all passports, where the batches are separated by
/// blank lines.
pub fn task_2(data: &str) -> usize {
    data.split("\n\n")
        .map(|p| get_passport_fields(p))
        .map(|hm| passport_is_valid(&hm))
        .filter(|b| b.is_some())
        .count()
}

#[derive(Debug, PartialEq)]
/// Representation of a height, either in units of inches or centimeters.
pub enum Height {
    /// The height in units of inches.
    Inch(u32),
    /// The height in units of centimeters.
    Cm(u32),
}

#[derive(Debug, PartialEq)]
/// A valid passport.
pub struct Passport<'a> {
    /// The birth year of the holder.
    pub byr: u32,
    /// The issue year of the holder.
    pub iyr: u32,
    /// The expiration year of the passport.
    pub eyr: u32,
    /// The height of the passport holder.
    pub hgt: Height,
    /// The hair color of the passport holder.
    pub hcl: &'a str,
    /// The eye color of the passport holder.
    pub ecl: &'a str,
    /// The passport identification number.
    pub pid: &'a str, // we use a string because of the leading zeros
    /// The country id.
    pub cid: Option<i64>,
}

/// Extract the passport fields and their corresponding data.
///
/// The input `passport_batch` is the string format of the
/// passport taken from the batch file.
///
/// The content of this batch is of the format `key:value`, separated
/// by spaces or new lines and these key-value pairs are returned in
/// form of a hash map.
///
/// Notice that no processing (e.g. parsing & validation) is performed
/// on the values and only their raw string representation is returned.
fn get_passport_fields(passport_batch: &str) -> HashMap<&str, &str> {
    passport_batch
        .split_ascii_whitespace()
        .map(|s| {
            let split: Vec<_> = s.split(':').collect();
            assert_eq!(split.len(), 2);
            (split[0], split[1])
        })
        .collect()
}

/// Check whether a password is valid.
///
/// The content of the password is provided in `field_data` which contains the
/// raw (i.e. unparsed string form) fields of the passport and their corresponding values.
///
/// If the passport is valid, it will be returned, wrapped in an `Option`. If any of the
/// fields does not satisfy the format specified in the problem statement, `None` is
/// returned.
fn passport_is_valid<'a>(field_data: &HashMap<&'a str, &'a str>) -> Option<Passport<'a>> {
    let byr = match get_year(field_data, "byr", 1920, 2002) {
        Some(value) => value,
        None => return None,
    };

    let iyr = match get_year(field_data, "iyr", 2010, 2020) {
        Some(value) => value,
        None => return None,
    };

    let eyr = match get_year(field_data, "eyr", 2020, 2030) {
        Some(value) => value,
        None => return None,
    };

    let hgt = match get_hgt(field_data) {
        Some(value) => value,
        None => return None,
    };

    let hcl = match get_hcl(field_data) {
        Some(value) => value,
        None => return None,
    };

    let ecl = match get_ecl(field_data) {
        Some(value) => value,
        None => return None,
    };

    let pid = match get_pid(field_data) {
        Some(value) => value,
        None => return None,
    };

    let cid = match field_data.get("cid") {
        Some(val) => val.parse::<i64>().ok(),
        None => None,
    };

    Some(Passport {
        byr,
        iyr,
        eyr,
        hgt,
        hcl,
        ecl,
        pid,
        cid,
    })
}

/// Check if a password has all required fields.
///
/// The first argument `raw_data` is the string representation of the passport
/// as taken from the input batch file. It is processed by the function
/// [`get_passport_fields`](crate::day_4::get_passport_fields).
///
/// The second argument `required_fields` contains a list of entries that are
/// required on the passport for it to be considered valid.
fn passport_has_valid_fields(raw_data: &str, required_fields: &[&str]) -> bool {
    let data = get_passport_fields(raw_data);
    for elem in required_fields {
        if !data.contains_key(elem) {
            return false;
        }
    }
    true
}

/// Extract the year `key` from the `passport_data`.
///
/// If the value cannot be correctly parsed from the string or if it does not satisfy
/// min <= year <= max, `None` is returned.
fn get_year(passport_data: &HashMap<&str, &str>, key: &str, min: u32, max: u32) -> Option<u32> {
    match passport_data.get(key) {
        Some(value) => match value.parse::<u32>() {
            Ok(year) if year >= min && year <= max => Some(year),
            _ => None,
        },
        None => None,
    }
}

/// Get the height of the passport holder.
fn get_hgt(passport_data: &HashMap<&str, &str>) -> Option<Height> {
    lazy_static::lazy_static! {
        static ref RE_HEIGHT: regex::Regex = regex::Regex::new(r"^(\d*)(in|cm)$").unwrap();
    }
    match passport_data.get("hgt") {
        Some(value) => match RE_HEIGHT.captures(value) {
            Some(v) if v.len() == 3 => match (v[1].parse::<u32>(), &v[2]) {
                (Ok(val @ 59..=76), "in") => Some(Height::Inch(val)),
                (Ok(val @ 150..=193), "cm") => Some(Height::Cm(val)),
                _ => None,
            },
            _ => None,
        },
        None => None,
    }
}

/// Get the hair color of the passport holder.
fn get_hcl<'a>(passport_data: &HashMap<&'a str, &'a str>) -> Option<&'a str> {
    lazy_static::lazy_static! {
        static ref RE_HCL: regex::Regex = regex::Regex::new(r"#[0-9,a-f]{6}$").unwrap();
    }
    match passport_data.get("hcl") {
        Some(&val) if RE_HCL.is_match(val) => Some(val),
        _ => None,
    }
}

/// Get the eye color of the passport holder.
fn get_ecl<'a>(passport_data: &HashMap<&'a str, &'a str>) -> Option<&'a str> {
    lazy_static::lazy_static! {
        static ref RE_ECL: regex::Regex = regex::Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    }

    match passport_data.get("ecl") {
        Some(&val) if RE_ECL.is_match(val) => Some(val),
        _ => None,
    }
}

/// Get the passport id..
fn get_pid<'a>(passport_data: &HashMap<&'a str, &'a str>) -> Option<&'a str> {
    lazy_static::lazy_static! {
        static ref RE_PID: regex::Regex = regex::Regex::new(r"^(\d{9})$").unwrap();
    }

    match passport_data.get("pid") {
        Some(&val) if RE_PID.is_match(val) => Some(val),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_passport_fields() {
        let input = "eyr:2039 hgt:64
        ecl:#ab45a8 byr:2009
        iyr:2025 pid:182cm hcl:d1614a cid:103";

        let target: HashMap<&str, &str> = vec![
            ("eyr", "2039"),
            ("hgt", "64"),
            ("ecl", "#ab45a8"),
            ("byr", "2009"),
            ("iyr", "2025"),
            ("pid", "182cm"),
            ("hcl", "d1614a"),
            ("cid", "103"),
        ]
        .into_iter()
        .collect();

        assert_eq!(target, get_passport_fields(&input));
    }

    #[test]
    fn test_passport_has_valid_fields() {
        let valid_input = "eyr:2039 hgt:64
        ecl:#ab45a8 byr:2009
        iyr:2025 pid:182cm hcl:d1614a cid:103";

        let invalid_input = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929";

        let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

        assert_eq!(
            passport_has_valid_fields(valid_input, &required_fields),
            true
        );
        assert_eq!(
            passport_has_valid_fields(invalid_input, &required_fields),
            false
        );
    }

    #[test]
    fn test_get_byr() {
        let valid_1 = vec![("byr", "1991"), ("cid", "39849")]
            .into_iter()
            .collect();
        let valid_2 = vec![("byr", "2002"), ("cid", "39849")]
            .into_iter()
            .collect();
        let valid_3 = vec![("byr", "1980"), ("cid", "39849")]
            .into_iter()
            .collect();
        let invalid_1 = vec![("byr", "2020"), ("cid", "39849")]
            .into_iter()
            .collect();
        let invalid_2 = vec![("cid", "39849")].into_iter().collect();
        let invalid_3 = vec![("byr", "2003"), ("cid", "39849")]
            .into_iter()
            .collect();

        assert_eq!(get_year(&valid_1, "byr", 1920, 2002), Some(1991));
        assert_eq!(get_year(&valid_2, "byr", 1920, 2002), Some(2002));
        assert_eq!(get_year(&valid_3, "byr", 1920, 2002), Some(1980));
        assert_eq!(get_year(&invalid_1, "byr", 1920, 2002), None);
        assert_eq!(get_year(&invalid_2, "byr", 1920, 2002), None);
        assert_eq!(get_year(&invalid_3, "byr", 1920, 2002), None);
    }

    #[test]
    fn test_get_iyr() {
        let valid_1 = vec![("iyr", "2015"), ("cid", "39849")]
            .into_iter()
            .collect();
        let valid_2 = vec![("iyr", "2012"), ("cid", "39849")]
            .into_iter()
            .collect();
        let invalid_1 = vec![("iyr", "1822"), ("cid", "39849")]
            .into_iter()
            .collect();
        let invalid_2 = vec![("cid", "39849")].into_iter().collect();

        assert_eq!(get_year(&valid_1, "iyr", 2010, 2020), Some(2015));
        assert_eq!(get_year(&valid_2, "iyr", 2010, 2020), Some(2012));
        assert_eq!(get_year(&invalid_1, "iyr", 2010, 2020), None);
        assert_eq!(get_year(&invalid_2, "iyr", 2010, 2020), None);
    }

    #[test]
    fn test_get_eyr() {
        let valid_1 = vec![("eyr", "2025"), ("cid", "39849")]
            .into_iter()
            .collect();
        let valid_2 = vec![("eyr", "2030"), ("cid", "39849")]
            .into_iter()
            .collect();
        let invalid_1 = vec![("eyr", "2050"), ("cid", "39849")]
            .into_iter()
            .collect();
        let invalid_2 = vec![("cid", "39849")].into_iter().collect();

        assert_eq!(get_year(&valid_1, "eyr", 2020, 2030), Some(2025));
        assert_eq!(get_year(&valid_2, "eyr", 2020, 2030), Some(2030));
        assert_eq!(get_year(&invalid_1, "eyr", 2020, 2030), None);
        assert_eq!(get_year(&invalid_2, "eyr", 2020, 2030), None);
    }

    #[test]
    fn test_get_hgt() {
        let valid_cm = vec![("hgt", "180cm"), ("cid", "993483")]
            .into_iter()
            .collect();
        let invalid_cm = vec![("hgt", "215cm"), ("cid", "993483")]
            .into_iter()
            .collect();
        let valid_in = vec![("hgt", "65in"), ("cid", "993483")]
            .into_iter()
            .collect();
        let valid_1 = vec![("hgt", "60in"), ("cid", "993483")]
            .into_iter()
            .collect();
        let valid_2 = vec![("hgt", "190cm"), ("cid", "993483")]
            .into_iter()
            .collect();
        let valid_3 = vec![("hgt", "74in"), ("cid", "993483")]
            .into_iter()
            .collect();

        let invalid_in = vec![("hgt", "35in"), ("cid", "993483")]
            .into_iter()
            .collect();
        let invalid_1 = vec![("hgt", "1035mm"), ("cid", "993483")]
            .into_iter()
            .collect();
        let invalid_2 = vec![("cid", "993483")].into_iter().collect();
        let invalid_3 = vec![("hgt", "190in"), ("cid", "993483")]
            .into_iter()
            .collect();
        let invalid_4 = vec![("hgt", "190"), ("cid", "993483")]
            .into_iter()
            .collect();

        assert_eq!(get_hgt(&valid_cm), Some(Height::Cm(180)));
        assert_eq!(get_hgt(&valid_in), Some(Height::Inch(65)));
        assert_eq!(get_hgt(&valid_1), Some(Height::Inch(60)));
        assert_eq!(get_hgt(&valid_2), Some(Height::Cm(190)));
        assert_eq!(get_hgt(&valid_3), Some(Height::Inch(74)));

        assert_eq!(get_hgt(&invalid_cm), None);
        assert_eq!(get_hgt(&invalid_in), None);
        assert_eq!(get_hgt(&invalid_1), None);
        assert_eq!(get_hgt(&invalid_2), None);
        assert_eq!(get_hgt(&invalid_3), None);
        assert_eq!(get_hgt(&invalid_4), None);
    }

    #[test]
    fn test_get_hcl() {
        let valid_1 = vec![("hcl", "#123abc"), ("cid", "993483")]
            .into_iter()
            .collect();
        let valid_2 = vec![("hcl", "#623a2f"), ("cid", "993483")]
            .into_iter()
            .collect();
        let invalid_1 = vec![("hcl", "#123abz"), ("cid", "993483")]
            .into_iter()
            .collect();
        let invalid_2 = vec![("hcl", "123abc"), ("cid", "993483")]
            .into_iter()
            .collect();
        let invalid_3 = vec![("cid", "993483")].into_iter().collect();

        assert_eq!(get_hcl(&valid_1), Some("#123abc"));
        assert_eq!(get_hcl(&valid_2), Some("#623a2f"));
        assert_eq!(get_hcl(&invalid_1), None);
        assert_eq!(get_hcl(&invalid_2), None);
        assert_eq!(get_hcl(&invalid_3), None);
    }

    #[test]
    fn test_get_ecl() {
        let valid_1 = vec![("ecl", "brn"), ("cid", "993483")]
            .into_iter()
            .collect();
        let valid_2 = vec![("ecl", "grn"), ("cid", "993483")]
            .into_iter()
            .collect();
        let valid_3 = vec![("ecl", "amb"), ("cid", "993483")]
            .into_iter()
            .collect();
        let valid_4 = vec![("ecl", "blu"), ("cid", "993483")]
            .into_iter()
            .collect();
        let valid_5 = vec![("ecl", "gry"), ("cid", "993483")]
            .into_iter()
            .collect();
        let valid_6 = vec![("ecl", "hzl"), ("cid", "993483")]
            .into_iter()
            .collect();
        let valid_7 = vec![("ecl", "oth"), ("cid", "993483")]
            .into_iter()
            .collect();
        let invalid_1 = vec![("ecl", "wat"), ("cid", "993483")]
            .into_iter()
            .collect();
        let invalid_2 = vec![("cid", "993483")].into_iter().collect();

        assert_eq!(get_ecl(&valid_1), Some("brn"));
        assert_eq!(get_ecl(&valid_2), Some("grn"));
        assert_eq!(get_ecl(&valid_3), Some("amb"));
        assert_eq!(get_ecl(&valid_4), Some("blu"));
        assert_eq!(get_ecl(&valid_5), Some("gry"));
        assert_eq!(get_ecl(&valid_6), Some("hzl"));
        assert_eq!(get_ecl(&valid_7), Some("oth"));
        assert_eq!(get_ecl(&invalid_1), None);
        assert_eq!(get_ecl(&invalid_2), None);
    }

    #[test]
    fn test_get_pid() {
        let valid_1 = vec![("pid", "000000001"), ("cid", "993483")]
            .into_iter()
            .collect();
        let valid_2 = vec![("pid", "087499704"), ("cid", "993483")]
            .into_iter()
            .collect();
        let invalid_1 = vec![("pid", "0123456789"), ("cid", "993483")]
            .into_iter()
            .collect();
        let invalid_2 = vec![("cid", "993483")].into_iter().collect();

        assert_eq!(get_pid(&valid_1), Some("000000001"));
        assert_eq!(get_pid(&valid_2), Some("087499704"));
        assert_eq!(get_pid(&invalid_1), None);
        assert_eq!(get_pid(&invalid_2), None);
    }

    #[test]
    fn test_valid_passport() {
        let valid_input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f";
        let fields = get_passport_fields(valid_input);
        assert!(fields.contains_key("byr"));
        assert!(fields.contains_key("iyr"));
        assert!(fields.contains_key("eyr"));
        assert!(fields.contains_key("hgt"));
        assert!(fields.contains_key("hcl"));
        assert!(fields.contains_key("ecl"));
        assert!(fields.contains_key("pid"));

        assert_eq!(get_year(&fields, "byr", 1920, 2002).unwrap(), 1980);
        assert_eq!(get_year(&fields, "iyr", 2010, 2020).unwrap(), 2012);
        assert_eq!(get_year(&fields, "eyr", 2020, 2030).unwrap(), 2030);
        assert_eq!(get_hgt(&fields).unwrap(), Height::Inch(74));
        assert_eq!(get_hcl(&fields).unwrap(), "#623a2f");
        assert_eq!(get_ecl(&fields).unwrap(), "grn");
        assert_eq!(get_pid(&fields).unwrap(), "087499704");

        let passport = passport_is_valid(&fields);
        assert_eq!(
            passport,
            Some(Passport {
                byr: 1980,
                iyr: 2012,
                eyr: 2030,
                hgt: Height::Inch(74),
                hcl: "#623a2f",
                ecl: "grn",
                pid: "087499704",
                cid: None
            })
        );
    }

    #[test]
    fn test_invalid_passport() {
        let valid_input = "hgt:97 byr:1990 iyr:2019 ecl:grn pid:587580330 hcl:#341e13 eyr:2022 ";
        let fields = get_passport_fields(valid_input);
        assert!(fields.contains_key("byr"));
        assert!(fields.contains_key("iyr"));
        assert!(fields.contains_key("eyr"));
        assert!(fields.contains_key("hgt"));
        assert!(fields.contains_key("hcl"));
        assert!(fields.contains_key("ecl"));
        assert!(fields.contains_key("pid"));
        assert!(!fields.contains_key("cid"));

        assert_eq!(get_year(&fields, "byr", 1920, 2002).unwrap(), 1990);
        assert_eq!(get_year(&fields, "iyr", 2010, 2020).unwrap(), 2019);
        assert_eq!(get_year(&fields, "eyr", 2020, 2030).unwrap(), 2022);
        assert_eq!(get_hgt(&fields), None);
        assert_eq!(get_hcl(&fields).unwrap(), "#341e13");
        assert_eq!(get_ecl(&fields).unwrap(), "grn");
        assert_eq!(get_pid(&fields).unwrap(), "587580330");

        let passport = passport_is_valid(&fields);
        assert_eq!(passport, None);
    }

    #[test]
    fn test_day_4_task_2() {
        let valid = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f

        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022

        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let invalid = "eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
        
        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946
        
        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
        
        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007";

        assert_eq!(task_2(valid), 4);
        assert_eq!(task_2(invalid), 0);
    }
}
