use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Rule {
    pub id: usize,
    pub rule: String,
}

pub fn expand_rule(rule: &Rule, rules: &HashMap<usize, Rule>) -> String {
    let mut all = String::new();
    for (i, set) in rule.rule.split('|').map(|r| String::from(r.trim())).enumerate() {
        if i == 0 {
            all.push('(');
        } else {
            all.push('|');
        }
        for sub_rule in set.split_whitespace() {
            if let Ok(n) = sub_rule.parse::<usize>() {
                // if n == 8 {
                //     // println!("42: {}", expand_rule_v2(rules.get(&42).unwrap(), rules));
                //     // all.push_str(&format!("{}{}{}", "(", expand_rule_v2(rules.get(&42).unwrap(), rules), ")+"));
                //     all.push_str(".+");
                // } else if n == 11 {
                //     // println!("31: {}", expand_rule_v2(rules.get(&42).unwrap(), rules));
                //     // all.push_str(&format!("{}{}{}{}{}", "(", expand_rule_v2(rules.get(&42).unwrap(), rules), ")+(", expand_rule_v2(rules.get(&31).unwrap(), rules), ")+"));
                //     all.push_str(".+");
                // }
                all.push_str(&expand_rule(rules.get(&n).unwrap(), rules));            
            } else {
                all.push_str(sub_rule);
            }
        }
    }
    all.push(')');
    all    
}