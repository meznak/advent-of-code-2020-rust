use crate::RunError;

use regex::Regex;

#[derive(Debug, PartialEq)]
enum Unit {
    Cm,
    Inch,
    None,
}

impl TryFrom<&str> for Unit {
    type Error = RunError;

    fn try_from(input: &str) -> Result<Unit, RunError> {
        match input {
        "cm" => Ok(Unit::Cm),
        "in" => Ok(Unit::Inch),
        "" => Ok(Unit::None),
        _ => Err(RunError::Regex(input.to_string()))
        }
    }
}

#[derive(Debug, PartialEq)]
struct Height {
    value: u16,
    unit: Unit,
}

#[derive(Debug, Default, PartialEq)]
struct Passport <'a> {
    byr: Option<u16>, // Birth Year
    iyr: Option<u16>, // Issue Year
    eyr: Option<u16>, // Expiration Year
    hgt: Option<Height>, // Height
    hcl: Option<&'a str>, // Hair Color
    ecl: Option<&'a str>, // Eye Color
    pid: Option<&'a str>, // Passport ID
    cid: Option<u16>, // Country ID
}

pub fn main(part: u8, data: &str) -> Result<usize, RunError> {
    let parsed_data = parse_data(data)?;

    match part {
        1 => part1(&parsed_data),
        2 => part2(&parsed_data),
        _ => Err(RunError::BadPartNum)
    }
}

fn parse_data(data: &str) -> Result<Vec<Passport>, RunError> {
    let mut passports: Vec<Passport> = vec![];
    let mut passport: Passport;

    for line in data.split("\n\n").collect::<Vec<&str>>() {
        passport = Passport::default();

        let kv_pairs: Vec<&str> = line.split(&['\n', ' ']).collect();

        for pair in kv_pairs {
            let kv: Vec<&str> = pair.split(':').collect();
            match kv[0] {
                "byr" => passport.byr = Some(kv[1].parse::<u16>()?),
                "iyr" => passport.iyr = Some(kv[1].parse::<u16>()?),
                "eyr" => passport.eyr = Some(kv[1].parse::<u16>()?),
                "hgt" => passport.hgt = parse_height(kv[1]).unwrap_or(None),
                "hcl" => passport.hcl = Some(kv[1]),
                "ecl" => passport.ecl = Some(kv[1]),
                "pid" => passport.pid = Some(kv[1]),
                "cid" => passport.cid = Some(kv[1].parse::<u16>()?),
                "" => {},
                _ => {return Err(RunError::ParseString(pair.to_string()))}
            }
        }

       passports.push(passport);
    }

    Ok(passports)
}

fn parse_height(text: &str) -> Result<Option<Height>, RunError> {
    let re_height = Regex::new(r"(?P<value>\d+)(?P<unit>\w{0,2})").unwrap();

    let cap = re_height.captures(text)
        .ok_or_else(|| RunError::Regex(text.to_string()))?;

    let value = cap.name("value")
        .ok_or(RunError::Regex(text.to_string()))?
        .as_str()
        .parse()?;
    let unit = cap.name("unit")
        .ok_or(RunError::Regex(text.to_string()))?
        .as_str()
        .try_into()?;

    Ok(Some(Height { value, unit }))
}

fn part1(values: &[Passport]) -> Result<usize, RunError> {
    // Count valid passports: has all fields, ignoring cid

    Ok(values.iter().filter(|passport|
        passport.byr.is_some() &&
        passport.iyr.is_some() &&
        passport.eyr.is_some() &&
        passport.hgt.is_some() &&
        passport.hcl.is_some() &&
        passport.ecl.is_some() &&
        passport.pid.is_some())
        .count())
}

fn part2(values: &[Passport]) -> Result<usize, RunError> {
    // Count valid passports: has all fields with valid values, ignoring cid

    let re_hcl = Regex::new(r"#[0-9a-f]{6}").unwrap();
    let re_ecl = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
    let re_pid = Regex::new(r"^[0-9]{9}$").unwrap();

    Ok(values.iter().filter(|passport|
        passport.byr.map_or(false, |x| (1920..2002).contains(&x)) &&
        passport.iyr.map_or(false, |x| (2010..2020).contains(&x)) &&
        passport.eyr.map_or(false, |x| (2020..2030).contains(&x)) &&
        passport.hgt.as_ref().map_or(false, |x|
            match x.unit {
                Unit::Cm => (150..193).contains(&x.value),
                Unit::Inch => (59..76).contains(&x.value
                ),
                Unit::None => false,
        }) &&
        passport.hcl.map_or(false, |x| re_hcl.is_match(x)) &&
        passport.ecl.map_or(false, |x| re_ecl.is_match(x)) &&
        passport.pid.map_or(false, |x| re_pid.is_match(x)))
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    static SAMPLE_INPUT_INVALID: &str = "pid:087499704 hgt:740in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1789
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:201 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b662a ecl:blu byr:1944 eyr:2021 pid:093154719

pid:087499704 hgt:74 ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f";

    static SAMPLE_INPUT_VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    static SAMPLE_DATA: &'static [Passport] = &[
        Passport {byr:Some(1937), iyr:Some(2017), eyr:Some(2020), hgt:Some(Height{ value: 183, unit: Unit::Cm }), hcl:Some("#fffffd"), ecl:Some("gry"), pid:Some("860033327"), cid:Some(147)},
        Passport {byr:Some(1929), iyr:Some(2013), eyr:Some(2023), hgt:None, hcl:Some("#cfa07d"), ecl:Some("amb"), pid:Some("028048884"), cid:Some(350)},
        Passport {byr:Some(1931), iyr:Some(2013), eyr:Some(2024), hgt:Some(Height { value: 179, unit: Unit::Cm }), hcl:Some("#ae17e1"), ecl:Some("brn"), pid:Some("760753108"), cid:None},
        Passport {byr:None, iyr:Some(2011), eyr:Some(2025), hgt:Some(Height{ value: 59, unit: Unit::Inch }), hcl:Some("#cfa07d"), ecl:Some("brn"), pid:Some("166559648"), cid:None},
    ];

    static SAMPLE_GOALS: [usize; 3] = [2, 0, 4];

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_data(SAMPLE_INPUT).unwrap(),
            SAMPLE_DATA);
    }

    #[test]
    fn test_parse_height() {
        assert_eq!(
            parse_height("100cm").unwrap(),
            Some(Height{ value: 100, unit: Unit::Cm}));
        assert_eq!(
            parse_height("60in").unwrap(),
            Some(Height{ value: 60, unit: Unit::Inch}));
        assert_eq!(
            parse_height("100").unwrap(),
            Some(Height{ value: 100, unit: Unit::None}));
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&SAMPLE_DATA).unwrap(),
            SAMPLE_GOALS[0]);
    }

    #[test]
    fn test_part2_invalid() {
        assert_eq!(
            part2(&parse_data(SAMPLE_INPUT_INVALID).unwrap()).unwrap(),
            SAMPLE_GOALS[1]);
    }

    #[test]
    fn test_part2_valid() {
        assert_eq!(
            part2(&parse_data(SAMPLE_INPUT_VALID).unwrap()).unwrap(),
            SAMPLE_GOALS[2]);
    }
}
