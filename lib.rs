use std::collections::HashMap;

pub fn evaluate_propositional_string(prop: &str) -> bool {
    let proposition = parse_proposition_string(prop);
    evaluate(proposition)
}


fn evaluate(expression: Proposition) -> bool {
    match expression {
        Proposition::Atom(Atom::True) => true,
        Proposition::Atom(Atom::False) => false,
        Proposition::Connective(Connective::And(left, right)) => evaluate(*left) && evaluate(*right),
        Proposition::Connective(Connective::Or(left, right)) => evaluate(*left) || evaluate(*right),
        Proposition::Connective(Connective::IfThen(left, right)) => !evaluate(*left) || evaluate(*right),
        Proposition::Connective(Connective::Iff(left, right)) => evaluate(*left) == evaluate(*right),
        Proposition::Connective(Connective::Not(prop)) => !evaluate(*prop),
        Proposition::Connective(Connective::Possibly(prop)) => {
            // TODO: Possibly
            // for some world related to the actual, the proposition is true
            evaluate(*prop)
        },
        Proposition::Connective(Connective::Necessarily(prop)) => {
            // TODO: Necessarily
            // for all worlds related to the actual, the proposition is true
            evaluate(*prop)
        }
        Proposition::Parenthesised(prop) => evaluate(*prop),
    }
}

#[derive(Debug, Clone)]
enum Proposition {
    Atom(Atom),
    Connective(Connective),
    Parenthesised(Box<Proposition>),
}

#[derive(Debug, Clone, Copy)]
enum Atom {
    True,
    False,
}

#[derive(Debug, Clone)]
enum Connective {
    And(Box<Proposition>, Box<Proposition>),
    Or(Box<Proposition>, Box<Proposition>),
    IfThen(Box<Proposition>, Box<Proposition>),
    Iff(Box<Proposition>, Box<Proposition>),
    Not(Box<Proposition>),
    Possibly(Box<Proposition>),
    Necessarily(Box<Proposition>),
}

fn parse_proposition_string(prop: &str) -> Proposition {
    let mut prop = prop.to_string();
    prop.retain(|c| !c.is_whitespace());
    let mut prop = prop.split(';');
    let proposition = prop.next().unwrap();
    let mut atoms = prop.next().unwrap().split(',');
    let mut atom_map: HashMap<char, char> = HashMap::new();
    for atom in atoms {
        let mut atom = atom.split('=');
        let atom_name = atom.next().unwrap().chars().next().unwrap();
        let atom_value = atom.next().unwrap().chars().next().unwrap();
        atom_map.insert(atom_name, atom_value);
    }
    parse_proposition(proposition, &atom_map)
}

fn parse_proposition(prop: &str, atom_map: &HashMap<char, char>) -> Proposition {
    let mut prop = prop.to_string();
    prop.retain(|c| !c.is_whitespace());
    let mut prop = prop.chars();
    let mut current_char = prop.next();
    let mut current_prop = None;
    while let Some(c) = current_char {
        match c {
            'P' | 'Q' | 'R' | 'S' | 'T' => {
                let atom = atom_map.get(&c as &char).unwrap();
                let atom = match atom {
                    'T' => Atom::True,
                    'F' => Atom::False,
                    _ => panic!("Invalid atom value"),
                };
                current_prop = Some(Proposition::Atom(atom));
            }
            '∧' => {
                let left = current_prop.unwrap();
                let right = parse_proposition(&prop.collect::<String>(), atom_map);
                current_prop = Some(Proposition::Connective(Connective::And(Box::new(left), Box::new(right))));
                break;
            }
            '∨' => {
                let left = current_prop.unwrap();
                let right = parse_proposition(&prop.collect::<String>(), atom_map);
                current_prop = Some(Proposition::Connective(Connective::Or(Box::new(left), Box::new(right))));
                break;
            }
            '→' => {
                let left = current_prop.unwrap();
                let right = parse_proposition(&prop.collect::<String>(), atom_map);
                current_prop = Some(Proposition::Connective(Connective::IfThen(Box::new(left), Box::new(right))));
                break;
            }
            '↔' => {
                let left = current_prop.unwrap();
                let right = parse_proposition(&prop.collect::<String>(), atom_map);
                current_prop = Some(Proposition::Connective(Connective::Iff(Box::new(left), Box::new(right))));
                break;
            }
            '¬' => {
                let left = parse_proposition(&prop.collect::<String>(), atom_map);
                current_prop = Some(Proposition::Connective(Connective::Not(Box::new(left))));
                break;
            }
            '(' => {
                let mut paren_count = 1;
                let mut paren_prop = String::new();
                while let Some(c) = prop.next() {
                    match c {
                        '(' => paren_count += 1,
                        ')' => paren_count -= 1,
                        _ => (),
                    }
                    if paren_count == 0 {
                        break;
                    }
                    paren_prop.push(c);
                }
                current_prop = Some(Proposition::Parenthesised(Box::new(parse_proposition(&paren_prop, atom_map))));
            }
            _ => {
                print!("{} ", c);
                panic!("Invalid character");
            },
        }
        current_char = prop.next();
    }
    current_prop.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_propositional_atoms() {
        let true_atom = Proposition::Atom(Atom::True);
        let false_atom = Proposition::Atom(Atom::False);
        assert_eq!(evaluate(true_atom), true);
        assert_eq!(evaluate(false_atom), false);
    }

    #[test]
    fn evaluates_propositional_connectives() {
        let true_atom = Proposition::Atom(Atom::True);
        let false_atom = Proposition::Atom(Atom::False);
        let and = Proposition::Connective(Connective::And(Box::new(true_atom.clone()), Box::new(false_atom.clone())));
        let or = Proposition::Connective(Connective::Or(Box::new(true_atom.clone()), Box::new(false_atom.clone())));
        let if_then = Proposition::Connective(Connective::IfThen(Box::new(true_atom.clone()), Box::new(false_atom.clone())));
        let iff = Proposition::Connective(Connective::Iff(Box::new(true_atom.clone()), Box::new(false_atom.clone())));
        let not = Proposition::Connective(Connective::Not(Box::new(true_atom.clone())));
        assert_eq!(evaluate(and), false);
        assert_eq!(evaluate(or), true);
        assert_eq!(evaluate(if_then), false);
        assert_eq!(evaluate(iff), false);
        assert_eq!(evaluate(not), false);
    }

    #[test]
    fn evaluates_parenthesised_propositions() {
        let true_atom = Proposition::Atom(Atom::True);
        let false_atom = Proposition::Atom(Atom::False);
        let and = Proposition::Connective(Connective::And(Box::new(true_atom), Box::new(false_atom)));
        let parenthesised = Proposition::Parenthesised(Box::new(and));
        assert_eq!(evaluate(parenthesised), false); 
    }

    #[test]
    fn parses_proposition_strings() {
        let test_str = "P ∧ Q;P=T,Q=F";
        let another_test_str = "¬(P ∨ Q);P=F,Q=F";
        let parsed = parse_proposition_string(test_str);
        let also_parsed = parse_proposition_string(another_test_str);
        assert_eq!(evaluate(parsed), false);
        assert_eq!(evaluate(also_parsed), true);
    }

    #[test]
    fn parses_complex_propositions() {
        let test_str = "P ∨ (Q ∧ R) ↔ (P ∨ Q) ∧ (P ∨ R);P=F,Q=T,R=T";
        let another_test_str = "P ∨ (Q ∧ R);P=F,Q=F,R=T";
        let parsed = parse_proposition_string(test_str);
        let also_parsed = parse_proposition_string(another_test_str);
        assert_eq!(evaluate(parsed), true);
        assert_eq!(evaluate(also_parsed), false);
    }

    #[test]
    fn parses_propositions() {
        let mut atom_map: HashMap<char, char> = HashMap::new();
        atom_map.insert('P', 'T');
        atom_map.insert('Q', 'F');
        let proposition = "P∧Q";
        let parsed = parse_proposition(proposition, &atom_map);
        assert_eq!(evaluate(parsed), false);
    }
}