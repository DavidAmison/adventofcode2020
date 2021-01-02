use std::collections::HashMap;

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
pub enum RuleType {
    Normal,
    Compound,
    End,
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub rule_type: RuleType,
    pub rule: String,
}

impl Rule {

    /// Evaluate a string against a rule-set (recursively calls into itself to evaluate each rule)
    /// 
    /// # Arguments
    /// 
    /// * `s` the string to evaluate
    /// * `rules` HashMap of all rules
    /// 
    /// # Returns
    /// 
    /// * A list of strings remaining after evaluation over all trees
    pub fn evaluate(&self, s: &str, rules: &HashMap<usize, Rule>) -> Vec< Option<String> > {
        // println!("Exectuing rule {} on \"{}\"", self.rule, s);
        if s.is_empty() {
            return vec!(None);
        }

        match self.rule_type {
            RuleType::Normal => Rule::evaluate_normal_rule(&self.rule, s, rules),
            RuleType::Compound => Rule::evaluate_compound_rule(&self.rule, s, rules),
            RuleType::End => Rule::evaluate_end_rule(&self.rule, s),
        }
    }

    /// Evaluates a normal rule list (space separated rules)
    /// 
    /// # Arguments
    /// 
    /// * `r` the rule string to run
    /// * `s` the string to evaluate against
    /// * `rules` HashMap of the rules
    /// 
    /// # Returns
    /// 
    /// * A list of possible remaining strings after evaluating all rules
    fn evaluate_normal_rule(r: &str, s: &str, rules: &HashMap<usize, Rule>) -> Vec< Option<String> > {
        let ids: Vec<usize> = r.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
        let mut sub_strings = vec!(Some(String::from(s)));
        for id in ids {
            let mut new_strings = Vec::new();
            let r = rules.get(&id).unwrap();
            for string in &sub_strings {
                if let Some(s) = string {
                    new_strings.append(&mut r.evaluate(&s.clone(), rules));
                } else {
                    new_strings.push(None);
                }
            }
            sub_strings = new_strings;
        }
        sub_strings
    }

    /// Evaluates a compound rule split by '|' - calls into Rule::evaluate_rule_list for each compound section
    /// 
    /// # Arguments
    /// 
    /// * `r` the rule string to run
    /// * `s` the string to evaluate against
    /// * `rules` HashMap of the rules
    /// 
    /// # Returns
    /// 
    /// * A list of possible remaining strings after evaluating all rules
    fn evaluate_compound_rule(r: &str, s: &str, rules: &HashMap<usize, Rule>) -> Vec< Option<String> > {
        let mut return_strings = Vec::new();
        let options: Vec<String> = r.split('|').map(|s| String::from(s.trim())).collect();
        for option in options {                 
            return_strings.append(&mut Rule::evaluate_normal_rule(&option, s, rules));
        }
        return_strings
    }

    /// Evaluates an end rule (i.e. single charachter)
    /// 
    /// # Arguments
    /// 
    /// * `r` the rule to check
    /// * `s` the string to evaluate against
    /// 
    /// # Returns
    /// 
    /// * A list of possible remaining strings after evaluating all rules (in this case either None or the original string with the first character removed)
    fn evaluate_end_rule(r: &str, s: &str) -> Vec< Option<String> > {
        if s.chars().nth(0) == r.chars().nth(0) {
            vec!(Some(String::from(&s[1..])))
        } else {
            vec!(None)
        }
    }
}

pub fn read_in_rules(filename: &str) -> HashMap<usize, Rule> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    
    
    let mut rules = HashMap::new();
    for line in reader.lines().flatten().collect::<Vec<String>>() {
        let temp: Vec<String> = line.split(':').map(|x| String::from(x.trim())).collect();
        let id = temp[0].parse::<usize>().unwrap();
        let rule = String::from(temp[1].trim_matches('"'));
        // What type of rule do we have?
        let rule_type;
        if rule.contains('|') {
            rule_type = RuleType::Compound;
        } else if rule == "a" || rule == "b" {
            rule_type = RuleType::End;
        } else {
            rule_type = RuleType::Normal;
        }
        // Add to the hashmap of rules
        rules.insert(id, Rule {rule_type, rule});
    }
    rules
}