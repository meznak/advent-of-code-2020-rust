use crate::RunError;

use regex::Regex;

#[derive(Debug, PartialEq)]
enum Unit {
    Cm,
    Inch,
}

#[derive(Debug, PartialEq)]
struct Passport <'a> {
    byr: Option<u16>, // Birth Year
    iyr: Option<u16>, // Issue Year
    eyr: Option<u16>, // Expiration Year
    hgt: Option<u16>, // Height
    unit: Option<Unit>, // Height unit
    hcl: Option<&'a str>, // Hair Color
    ecl: Option<&'a str>, // Eye Color
    pid: Option<&'a str>, // Passport ID
    cid: Option<u16>, // Country ID
}

impl <'a> Default for Passport <'a> {
    fn default() -> Passport <'a> {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            unit: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
}

pub fn main(part: u8, data: &str) -> Result<usize, RunError> {
    let parsed_data = parse_data(&data)?;

    match part {
        1 => part1(&parsed_data),
        2 => part2(&parsed_data),
        _ => Err(RunError::BadPartNum)
    }
}

fn parse_data<'a>(data: &str) -> Result<Vec<Passport>, RunError> {
    let re_height = Regex::new(r"(?P<height>\d+)(<?Punit>\w{2})").unwrap();

    let lines: Vec<&str> = data[..].split('\n').collect();

    let mut passports: Vec<Passport> = vec![];
    let mut passport = Passport::default();

    for line in lines {
        if line == "" {
            passports.push(passport);
            passport = Passport::default();
        }
        let kv_pairs: Vec<&str> = line.split(' ').collect();

        for pair in kv_pairs {
            let kv: Vec<&str> = pair.split(':').collect();
            match kv[0] {
                "byr" => passport.byr = Some(kv[1].parse::<u16>()?),
                "iyr" => passport.iyr = Some(kv[1].parse::<u16>()?),
                "eyr" => passport.eyr = Some(kv[1].parse::<u16>()?),
                "hgt" => {
                    let cap = re_height.captures(kv[1]).ok_or_else(|| {Err(RunError::Regex(pair.to_string()))})?;
                    match cap.len() {
                        2 => {
                            passport.hgt = cap.name("height")
                                .map_or(None,
                                    |x| Some(x.as_str().parse::<u16>().ok()?));
                            passport.unit = cap.name("unit")
                                .map_or(None,
                                    |x| match x.as_str() {
                                        "cm" => Some(Unit::Cm),
                                        "in" => Some(Unit::Inch),
                                        _ => None
                                    }
                                )
                        },
                        _ => return Err(RunError::ParseString(pair.to_string()))
                    }
                },
                "hcl" => passport.hcl = Some(kv[1]),
                "ecl" => passport.ecl = Some(kv[1]),
                "pid" => passport.pid = Some(kv[1]),
                "cid" => passport.cid = Some(kv[1].parse::<u16>()?),
                "" => {},
                _ => {return Err(RunError::ParseString(pair.to_string()))}
            }
        }
    }

    // last line of file isn't empty so the
    // last record doesn't get pushed in the loop
    passports.push(passport);

    Ok(passports)
}

fn part1(values: &[Passport]) -> Result<usize, RunError> {
    // Count valid passports: has all fields, ignoring cid

    Ok(values.iter().filter(|passport|
        passport.byr != None &&
        passport.iyr != None &&
        passport.eyr != None &&
        passport.hgt != None &&
        passport.hcl != None &&
        passport.ecl != None &&
        passport.pid != None).count())
}

fn part2(values: &[Passport]) -> Result<usize, RunError> {
    // Count valid passports: has all fields with valid values, ignoring cid

    let re_color = Regex::new(r"#[[:alpha:]0-9]{6}").unwrap();
    let re_pid = Regex::new(r"[0-9]{9}").unwrap();

    Ok(values.iter().filter(|passport|
        passport.byr.map_or(false, |x| x >= 1920 && x <= 2002) &&
        passport.iyr.map_or(false, |x| x >= 2010 && x <= 2020) &&
        passport.eyr.map_or(false, |x| x >= 2020 && x <= 2030) &&
        passport.hgt.map_or(false, |x|
            match passport.unit {
                Some(Unit::Cm) => x >= 150 && x <= 193,
                Some(Unit::Inch) => x >= 59 && x <= 76,
                _ => false
        }) &&
        passport.hcl.map_or(false, |x| re_color.is_match(x)) &&
        passport.ecl.map_or(false, |x| re_color.is_match(x)) &&
        passport.pid.map_or(false, |x| re_pid.is_match(x))).count())
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

    static SAMPLE_INPUT_INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

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
        Passport {byr:Some(1937), iyr:Some(2017), eyr:Some(2020), hgt:Some(183), unit:Some(Unit::Cm), hcl:Some("#fffffd"), ecl:Some("gry"), pid:Some("860033327"), cid:Some(147)},
        Passport {byr:Some(1929), iyr:Some(2013), eyr:Some(2023), hgt:None, unit:None, hcl:Some("#cfa07d"), ecl:Some("amb"), pid:Some("028048884"), cid:Some(350)},
        Passport {byr:Some(1931), iyr:Some(2013), eyr:Some(2024), hgt:Some(179), unit:Some(Unit::Cm), hcl:Some("#ae17e1"), ecl:Some("brn"), pid:Some("760753108"), cid:None},
        Passport {byr:None, iyr:Some(2011), eyr:Some(2025), hgt:Some(59), unit:Some(Unit::Inch), hcl:Some("#cfa07d"), ecl:Some("brn"), pid:Some("166559648"), cid:None},
    ];

    static SAMPLE_GOALS: [usize; 3] = [2, 0, 4];

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_data(SAMPLE_INPUT).unwrap(),
            SAMPLE_DATA);
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
            SAMPLE_GOALS[1]);
    }
}
