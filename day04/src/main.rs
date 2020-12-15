/*
--- Day 4: Passport Processing ---

You arrive at the airport only to realize that you grabbed your North Pole Credentials instead of your passport. While these documents are extremely similar, North Pole Credentials aren't issued by a country and therefore aren't actually valid documentation for travel in most of the world.

It seems like you're not the only one having problems, though; a very long line has formed for the automatic passport scanners, and the delay could upset your travel itinerary.

Due to some questionable network security, you realize you might be able to solve both of these problems at the same time.

The automatic passport scanners are slow because they're having trouble detecting which passports have all required fields. The expected fields are as follows:

    byr (Birth Year)
    iyr (Issue Year)
    eyr (Expiration Year)
    hgt (Height)
    hcl (Hair Color)
    ecl (Eye Color)
    pid (Passport ID)
    cid (Country ID)

Passport data is validated in batch files (your puzzle input). Each passport is represented as a sequence of key:value pairs separated by spaces or newlines. Passports are separated by blank lines.

Here is an example batch file containing four passports:

ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in

The first passport is valid - all eight fields are present. The second passport is invalid - it is missing hgt (the Height field).

The third passport is interesting; the only missing field is cid, so it looks like data from North Pole Credentials, not a passport at all! Surely, nobody would mind if you made the system temporarily ignore missing cid fields. Treat this "passport" as valid.

The fourth passport is missing two fields, cid and byr. Missing cid is fine, but missing any other field is not, so this passport is invalid.

According to the above rules, your improved system would report 2 valid passports.

Count the number of valid passports - those that have all required fields. Treat cid as optional. In your batch file, how many passports are valid?

--- Part Two ---

The line is moving more quickly now, but you overhear airport security talking about how passports with invalid data are getting through. Better add some data validation, quick!

You can continue to ignore the cid field, but each other field has strict rules about what values are valid for automatic validation:

    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.

Your job is to count the passports where all required fields are both present and valid according to the above rules. Here are some example values:

byr valid:   2002
byr invalid: 2003

hgt valid:   60in
hgt valid:   190cm
hgt invalid: 190in
hgt invalid: 190

hcl valid:   #123abc
hcl invalid: #123abz
hcl invalid: 123abc

ecl valid:   brn
ecl invalid: wat

pid valid:   000000001
pid invalid: 0123456789

Here are some invalid passports:

eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

Here are some valid passports:

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719

Count the number of valid passports - those that have all required fields and valid values. Continue to treat cid as optional. In your batch file, how many passports are valid?

*/


use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let passports = read_in_passports("src/passports.txt");
    println!("Passports with required fields = {}", check_required_fields(&passports));

    let mut valid_passports = 0;
    for passport in passports {
        if validate_passport(&passport) {
            valid_passports += 1;
        }
    }
    println!("Valid passports = {}", valid_passports);
}



// struct Passport {
//     byr: Option<u32>,
//     iyr: Option<u32>,
//     eyr: Option<u32>,
//     hgt: Option<Height>,
//     hcl: Option<String>,
//     ecl: Option<String>,
//     pid: Option<String>,
//     cid: Option<String>,
// }

// impl Passport {
//     fn update_field<T>(&self, field: &str, value: T) {
//         match field {
//             "byr" => self.byr = Option::from(value),
//             "iyr"
//         }     
//     }

//     fn check_required_fields(&self) -> bool {
//         self.byr.is_some() || self.iyr.is_some() || self.eyr.is_some() ||
//         self.hgt.is_some() || self.hcl.is_some() || self.ecl.is_some() ||
//         self.pid.is_some()
//     }

//     fn validate() {
        
//     }
// }

// struct Height {
//     h: u32,
//     unit: String,
// }

/// Read in the passports from the defined file and return as a vector of
/// HashMap
/// 
/// # Arguments
/// 
/// * `filename` - the filename to read the passports from
fn read_in_passports(filename: &str) -> Vec< HashMap<String, String> > {
    let mut passports: Vec< HashMap<String, String> > = Vec::new();

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // iterate over lines until end of file
    loop {
        let mut line = lines.next();
        if line.is_none() {
            break;
        }
        let mut entry = HashMap::new();
        loop {
            let lin = String::from(line.unwrap().unwrap().trim());
            // let l = lin.trim();
            if lin.is_empty() {
                break;
            }
            for item in lin.split_whitespace() {
                let mut pair = item.split(':');
                let key = String::from(pair.next().unwrap());
                let value = String::from(pair.next().unwrap());
                entry.insert(key, value);
            }
            line = lines.next();
            if line.is_none() {
                break;
            }
        }
        passports.push(entry);
    }
    passports
}

fn check_required_fields(passports: &Vec< HashMap<String, String> >) -> u32 {
    // Check that all passports contain the required fields
    // byr - iyr - eyr - hgt - hcl - ecl -pid
    let required_fields = vec!(
        String::from("byr"),
        String::from("iyr"),
        String::from("eyr"),
        String::from("hgt"),
        String::from("hcl"),
        String::from("ecl"),
        String::from("pid"));

    let mut valid_passports_total = 0;
    for passport in passports {
        let mut valid = true;
        for field in &required_fields {
            if passport.contains_key(field) == false {
                valid = false;
                break;
            }
        }
        valid_passports_total += if valid { 1 } else { 0 }; 
    }
    valid_passports_total
}

fn validate_passport(passport: &HashMap<String, String>) -> bool {
    let mut result = true;

    // Birth year
    if let Some(byr) = passport.get("byr") {
        result = result && validate_byr(byr);
    } else {
        result = false;
    }

    // Issue Year
    if let Some(iyr) = passport.get("iyr") {
        result = result && validate_iyr(iyr);
    } else {
        result = false;
    }

    // Expiration Year
    if let Some(eyr) = passport.get("eyr") {
        result = result && validate_eyr(eyr);
    } else {
        result = false;
    }

    // Height
    if let Some(hgt) = passport.get("hgt") {
        result = result && validate_hgt(hgt);
    } else {
        result = false;
    }

    // Hair Colour
    if let Some(hcl) = passport.get("hcl") {
        result = result && validate_hcl(hcl);
    } else {
        result = false;
    }

    // Eye Colour
    if let Some(ecl) = passport.get("ecl") {
        result = result && validate_ecl(ecl);
    } else {
        result = false;
    }

    // Passport ID
    if let Some(pid) = passport.get("pid") {
        result = result && validate_pid(pid);
    } else {
        result = false;
    }

    // Country ID - Ignored

    result
}

fn validate_byr(field: &String) -> bool {
    // Requirement are:
    //    - four digits
    //    - at least 1920 and at most 2002.

    // Convert to number
    if let Ok(year) = field.parse::< u32 >() {
        year >= 1920 && year <= 2002
    }
    else {
        false
    }
}

fn validate_iyr(field: &String) -> bool {
    // Requirement are:
    //    - four digits
    //    - at least 2010 and at most 2020.

    // Convert to number
    if let Ok(year) = field.parse::< u32 >() {
        year >= 2010 && year <= 2020
    }
    else {
        false
    }
}

fn validate_eyr(field: &String) -> bool {
    // Requirement are:
    //    - four digits
    //    - at least 2010 and at most 2020.

    // Convert to number
    if let Ok(year) = field.parse::< u32 >() {
        year >= 2020 && year <= 2030
    }
    else {
        false
    }
}

fn validate_hgt(field: &String) -> bool {
    // Requirement are:
    //    - a number followed by cm or in
    //    - if cm -> 150-193 inclusive
    //    - if in -> 59-76 inclusive

    // Remove last two letters
    let chars = field.chars().count();
    if chars > 2 {
        let measurement: String = field.chars().take(chars - 2).collect();
        let measurement_type: String = field.chars().rev().take(2).collect();
        // Convert to number
        if let Ok(value) = measurement.parse::< u32 >() {
            match measurement_type.as_str() {
                // Strings are reversed!!!
                "mc" => value >= 150 && value <= 193,
                "ni" => value >= 59 && value <= 76,
                _ => false
            }
        }
        else {
            false
        }
    } else {
        false
    }
}

fn validate_hcl(field: &String) -> bool {
    // Requirement are:
    //    - starts with #
    //    - followed by 6 0-9 or a-f charachters

    let valid_chars = vec!('0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f');
    let validate_chars = | hcl: &String | -> bool {
        let mut result = true;
        for c in hcl.chars() {
            result = result && valid_chars.contains(&c);
        }
        result        
    };

    let chars = field.chars().count();
    if chars == 7 {
        if field.chars().next().unwrap() == '#' {
            let hcl: String = field.chars().rev().take(6).collect();
            validate_chars(&hcl)
        } else {
            false
        }
    } else {
        false
    }
}

fn validate_ecl(field: &String) -> bool {
    // Requirement are:
    //    - exactly one of: amb blu brn gry grn hzl oth

    let valid_colors = vec!(
        String::from("amb"),
        String::from("blu"),
        String::from("brn"),
        String::from("gry"),
        String::from("grn"),
        String::from("hzl"),
        String::from("oth") );
    
    valid_colors.contains(field)
}

fn validate_pid(field: &String) -> bool {
    // Requirement are:
    //    - nine-digit number, including leading zeroes

    let valid_chars = vec!('0', '1', '2', '3', '4', '5', '6', '7', '8', '9');
    let validate_chars = | hcl: &String | -> bool {
        let mut result = true;
        for c in hcl.chars() {
            result = result && valid_chars.contains(&c);
        }
        result        
    };

    let chars = field.chars().count();
    if chars == 9 {
            validate_chars(field)
    } else {
        false
    }
}

