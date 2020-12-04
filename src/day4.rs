use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Passport {
    fields: HashMap<String, String>,
}

const REQUIRED_FIELDS: &[&str] = &[
    "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid",
    /*"cid"*/ // optional
];

impl Passport {
    pub fn is_valid_part1(&self) -> bool {
        REQUIRED_FIELDS
            .iter()
            .all(|&key| self.fields.contains_key(key))
    }

    pub fn is_valid_part2(&self) -> bool {
        if !self.is_valid_part1() {
            return false;
        }
        let byr = self.fields.get("byr").unwrap().parse::<i32>();
        let iyr = self.fields.get("iyr").unwrap().parse::<i32>();
        let eyr = self.fields.get("eyr").unwrap().parse::<i32>();
        let hgt = self.fields.get("hgt").unwrap();
        let hcl = self.fields.get("hcl").unwrap();
        let ecl = self.fields.get("ecl").unwrap();
        let pid = self.fields.get("pid").unwrap();
        matches!(byr, Ok(1920..=2002))
            && matches!(iyr, Ok(2010..=2020))
            && matches!(eyr, Ok(2020..=2030))
            && is_height(hgt)
            && is_hex_color(hcl)
            && is_eye_color(ecl)
            && is_password_id(pid)
    }
}

impl FromStr for Passport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s
            .split(&[' ', '\n'][..])
            .map(|field| {
                let mut parts = field.split(':');
                let key = parts.next().unwrap().to_owned();
                let value = parts.next().unwrap().to_owned();
                (key, value)
            })
            .collect::<HashMap<String, String>>();
        Ok(Passport { fields })
    }
}

fn is_height(s: &str) -> bool {
    if let Some(cm) = s.strip_suffix("cm") {
        return matches!(cm.parse::<u32>(), Ok(150..=193));
    }
    if let Some(inch) = s.strip_suffix("in") {
        return matches!(inch.parse::<u32>(), Ok(59..=76));
    }
    false
}

fn is_hex_color(s: &str) -> bool {
    match s.strip_prefix('#') {
        Some(hex) => hex.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')),
        None => false,
    }
}

fn is_eye_color(s: &str) -> bool {
    matches!(s, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

fn is_password_id(s: &str) -> bool {
    s.len() == 9 && s.chars().all(|c| c.is_ascii_digit())
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|x| Passport::from_str(x).unwrap())
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|passport| passport.is_valid_part1())
        .count()
}

#[aoc(day4, part2)]
pub fn part2(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|passport| passport.is_valid_part2())
        .count()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::day4::Passport;

    #[test]
    fn part1_valid() {
        let inputs = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
            "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm",
        ];
        for input in inputs {
            let passport = Passport::from_str(input).unwrap();
            assert_eq!(true, passport.is_valid_part1())
        }
    }

    #[test]
    fn part1_invalid() {
        let inputs = vec![
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929",
            "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in",
        ];
        for input in inputs {
            let passport = Passport::from_str(input).unwrap();
            assert_eq!(false, passport.is_valid_part1())
        }
    }

    #[test]
    fn part2_valid() {
        let inputs = vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        ];
        for input in inputs {
            let passport = Passport::from_str(input).unwrap();
            assert_eq!(true, passport.is_valid_part2())
        }
    }

    #[test]
    fn part2_invalid() {
        let inputs = vec![
            "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946",
            "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007",
        ];
        for input in inputs {
            let passport = Passport::from_str(input).unwrap();
            assert_eq!(false, passport.is_valid_part2())
        }
    }
}
