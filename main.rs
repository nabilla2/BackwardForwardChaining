use std::io;
use std::fs;   
use std::collections::HashMap;

#[derive(Clone)]
struct Rule {
    positive_clause: String,
    negative_clauses: Vec<String>,
}

#[allow(dead_code)]
impl Rule {
    pub fn new() -> Rule {
        Rule {
            positive_clause: String::new(),
            negative_clauses: vec!(),
        }
    }

    pub fn display(&self) {
        
        for (index, negative) in self.negative_clauses.iter().enumerate() {
            print!("{}", negative);
            if index != self.negative_clauses.len() - 1 {
                print!(" ^ ");
            }
        }

        println!(" => {}", self.positive_clause);
    }

    pub fn add_positive_clause(&mut self, literal: String) {
        self.positive_clause = literal;
    }

    pub fn add_negative_clause(&mut self, literal: String) {
        self.negative_clauses.push(literal);
    }

    pub fn get_negative_clauses(&self) -> Vec<String> {
        self.negative_clauses.clone()
    }

    pub fn get_positive_clause(&self) -> String {
        self.positive_clause.clone()
    }
}


fn read_answers() -> Result<Vec<Rule>, String> {

    let mut new_rules = vec!();

    loop {
        let mut buffer = String::new();
        println!("What is patient temperature?(answer is a number)");
        io::stdin().read_line(&mut buffer).expect("Could not read from stdin!");

        if buffer.trim() == "stop" {
            return Err("Stop was pressed.".to_string());
        }

        match buffer.trim().parse::<u8>() {
            Ok(temp) => {
                if temp > 38 {
                    let mut rule = Rule::new();
                    rule.add_positive_clause("temperature".to_string());
                    new_rules.push(rule);
                }
                break;
            }
            Err(e) => println!("Please insert a valid temperature : {}", e),
        };
    }
    println!();

    loop {
        let mut buffer = String::new();
        println!("For how many days has the patient been sick?(answer is a number)");
        io::stdin().read_line(&mut buffer).expect("Could not read from stdin!");

        if buffer.trim() == "stop" {
            return Err("Stop was pressed.".to_string());
        }

        match buffer.trim().parse::<u8>() {
            Ok(sick_days) => {
                if sick_days > 2 {
                    let mut rule = Rule::new();
                    rule.add_positive_clause("sick".to_string());
                    new_rules.push(rule);
                }
                break;
            }
            Err(e) => println!("Please insert a valid number of days : {}", e),
        };
    }
    println!();

    loop {
        let mut buffer = String::new();
        println!("Has patient cough?(answer is yes/no)");
        io::stdin().read_line(&mut buffer).expect("Could not read from stdin!");

        if buffer.trim() == "stop" {
            return Err("Stop was pressed.".to_string());
        }

        if buffer.trim() == "yes" {
            let mut rule = Rule::new();
            rule.add_positive_clause("cough".to_string());
            new_rules.push(rule);
            break;
        } else if buffer.trim() == "no" {
            break;
        } else {
            println!("Not a valid answers. Only yes/no accepted!");
        }
    }
    println!();

    Ok(new_rules)
}

fn parse_kb(mut kb: String) -> Vec<Rule> {
    
    let mut parsed_rules: Vec<Rule> = vec!();

    // Remove first [
    kb.remove(0);
    // Remove last ]
    kb.remove(kb.len() - 1);

    // Take all the rules
    let rules = kb.split(",");
    for r in rules {
        let mut rule = Rule::new();
        // Remove first [
        let r = &r[1..];
        // Remove last ]
        let r = &r[..(r.len() - 1)];
        // Split atoms in each rule
        let literals = r.split(".");
        for literal in literals {
            // This is a condition
            if literal.starts_with("n(") {
                rule.add_negative_clause(literal[2..(literal.len() - 1)].to_string());
            // This is the concluson
            } else {
                rule.add_positive_clause(literal.to_string());
            }
        }
        // Add the obtained rule
        parsed_rules.push(rule);
    }

    parsed_rules
}

fn backward_solve(q: &Vec<String>, kb: &Vec<Rule>) -> bool {

    // If no goal left we can return
    if q.len() == 0 {
        return true;
    }

    for c in kb.iter() {
        // Create a new clause so that if c = [q1, p2, .., pm] => new_clause = [p1,..pm,q2,..,qn]
        let mut new_q = q.clone();
        new_q.remove(0);
        new_q.extend(c.get_negative_clauses());
        if c.positive_clause == q[0] && backward_solve(&new_q, &kb) {
            return true;
        }
    }

    return false;
}

fn forward_solve(q: &Vec<String>, kb: &Vec<Rule>, mut solved: HashMap<String, bool>) -> bool {

    loop {
        let mut completed = true;
        // Check if all the goals are solved
        for goal in q {
            match solved.get(&goal[..]) {
                // There is an unsolved goal
                Some(false) => {
                    completed = false;
                    break;
                },
                Some(true) => (),
                None => panic!("Invalid atom"),
            };
        }
        
        // If all completed return true 
        if completed == true {
            return true;
        }
        
        let mut solved_clause = false;
        // If not completed look for a clause which may resolve(turn to solved)
        for c in kb {
            // If resulting atom is already solved, continue
            match solved.get(&c.positive_clause[..]) {
                Some(false) => (),
                Some(true) => {
                    continue;
                },
                None => panic!("Invalid atom")
            };
            let mut atoms_solved = true;
            // Check if all the atoms in the rule are true
            for atom in c.get_negative_clauses().iter() {
                match solved.get(&atom[..]) {
                    // If at least one is unsolved break
                    Some(false) => {
                        atoms_solved = false;
                        break;
                    },
                    Some(true) => continue,
                    None => panic!("Invalid atom")
                };
            }
            // If at least one clause solved a goal, we can go back to step 1
            solved_clause |= atoms_solved;
            if atoms_solved == true {
                solved.insert(c.get_positive_clause(), true);
            }
        }
        // If no solving found it means the patient hasn't pneumonia
        if solved_clause == false {
            return false;
        }
    }

}

fn create_solved_map() -> HashMap<String, bool> {
    let mut solved = HashMap::new();
    solved.insert("cough".to_string(), false);
    solved.insert("infection".to_string(), false);
    solved.insert("temperature".to_string(), false);
    solved.insert("sick".to_string(), false);
    solved.insert("pneumonia".to_string(), false);
    solved.insert("fever".to_string(), false);

    solved
}

fn main() {
    
    // Read the Knowledge Base
    let mut kb = fs::read_to_string("input.txt").expect("Could not read file");
    // Remove white spaces
    kb.retain(|c| !c.is_whitespace());
    // Parse the initial kb
    let parsed_kb = parse_kb(kb);
    
    // Get new rules out of answers
    while let Ok(mut new_kb_rules) = read_answers() {
        // Clone the initial kb
        let mut final_kb = parsed_kb.to_vec();
        // Append the user's new rules
        final_kb.append(&mut new_kb_rules);
        // Create a goals array
        let q = vec!("pneumonia".to_string());
        // Perform the backward chaining
        println!("Backward chaining : {}", backward_solve(&q, &final_kb));
        let solved = create_solved_map();
        println!("Forward chaining : {}\n", forward_solve(&q, &final_kb, solved));
        println!();
    }
}
