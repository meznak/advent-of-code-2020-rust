use crate::RunError;

#[derive(Debug, PartialEq)]
struct Passport <'a> {
    byr: Option<u16>, // Birth Year
    iyr: Option<u16>, // Issue Year
    eyr: Option<u16>, // Expiration Year
    hgt: Option<&'a str>, // Height
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
                "hgt" => passport.hgt = Some(kv[1]),
                "hcl" => passport.hcl = Some(kv[1]),
                "ecl" => passport.ecl = Some(kv[1]),
                "pid" => passport.pid = Some(kv[1]),
                "cid" => passport.cid = Some(kv[1].parse::<u16>()?),
                "" => {},
                _ => {return Err(RunError::ParseString(pair.to_string()))}
            }
        }
    }

    // last line of file isn't empty so the last record doesn't get pushed in the loop
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
    // What's the goal?

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str ="ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    static SAMPLE_DATA: &'static [Passport] = &[
        Passport {byr:Some(1937), iyr:Some(2017), eyr:Some(2020), hgt:Some("183cm"), hcl:Some("#fffffd"), ecl:Some("gry"), pid:Some(860033327), cid:Some(147)},
        Passport {byr:Some(1929), iyr:Some(2013), eyr:Some(2023), hgt:None, hcl:Some("#cfa07d"), ecl:Some("amb"), pid:Some(028048884), cid:Some(350)},
        Passport {byr:Some(1931), iyr:Some(2013), eyr:Some(2024), hgt:Some("179cm"), hcl:Some("#ae17e1"), ecl:Some("brn"), pid:Some(760753108), cid:None},
        Passport {byr:None, iyr:Some(2011), eyr:Some(2025), hgt:Some("59in"), hcl:Some("#cfa07d"), ecl:Some("brn"), pid:Some(166559648), cid:None},
    ];

    static SAMPLE_GOALS: [usize; 2] = [2, 0];

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
    fn test_part2() {
        assert_eq!(
            part2(SAMPLE_DATA).unwrap(),
            SAMPLE_GOALS[1]);
    }
}
