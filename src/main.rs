

use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;
fn main(){

    //let (n, transformations, alphabet) = read_from_console();
    let (n, transformations, alphabet) = read_from_file("data1.txt".to_string());
    let  variables = find_variables(&transformations);
    
    let mut transformations_with_variables: Vec<HashMap<char, VariableSituation>> = create_matrix(&variables, n);
    fill_matrix_with_variables_status(&transformations, &mut transformations_with_variables);
    // for (i, transformation) in transformations.iter().enumerate() {
    //     println!("Równanie nr {}: {}", i+1, transformation);
    //     for (variable, situation) in &transformations_with_variables[i] {
    //         println!("Zmienna: {}, sytuacja: {:?}", variable, situation);
    //     }
    // }
    let (D, I) = create_sets(&transformations_with_variables, &alphabet);
    println!("Zbiór D: {:?}", D);
    println!("Zbiór I: {:?}", I);
    


}

fn read_file_name() -> String {
    println!("Podaj nazwę pliku z danymi: ");
    let mut filename = String::new();
    std::io::stdin().read_line(&mut filename).expect("Failed to read filename");
    let filename = filename.trim();
    filename.to_string()
}
fn read_from_file(mut filename: String) ->(i32, Vec<String>, Vec<char>) {
    if filename.is_empty() {
        filename = read_file_name(); 
    }
    let content = fs::read_to_string(filename).expect("Failed to read file");
    let mut lines = content.lines();
    let n: i32 = lines.next().expect("Failed to read n").parse().expect("Failed to parse n");
    let mut transformations = Vec::new();
    for _ in 0..n {
        transformations.push(lines.next().expect("Failed to read transformation").to_string());   
    }
    let alphabet = parse_alphabet(&lines.next().expect("Failed to read alphabet").to_string());
    (n, transformations, alphabet)
}

#[doc = "Test sprawdzający czy funkcja read_from_file zwraca poprawne dane"]
#[test]
fn test_read_from_file() {
    let (n, transformations, alphabet) = read_from_file("data1.txt".to_string());
    println!("n: {}, transformations: {:?}, alphabet: {:?}", n, transformations, alphabet);
    assert_eq!(n, 4);
    assert_eq!(transformations, vec!["x <= x+y", "y <= y+2z", "x <= 3x+z", "z <= y-z"]);
    assert_eq!(alphabet, vec!['a', 'b', 'c', 'd']);
}
fn read_from_console() ->(i32, Vec<String>, Vec<char>) {
    println!("Podaj liczbę równań, które chcesz wprowadzić: ");
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: i32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("This must be a number!"),
    };
    let transformations = read_transformations(n);
    let alphabet = get_alphabet_from_input();
    if alphabet.len() != n as usize {
        panic!("Alphabet size has to be equal to number of transformations!");
    }
    (n, transformations, alphabet)
}

fn get_alphabet_from_input() -> Vec<char> {
    println!("Podaj alfabet: ");
    let mut alphabet = String::new();
    std::io::stdin().read_line(&mut alphabet).expect("Failed to read alphabet");
    let alphabet: Vec<char> = parse_alphabet(&alphabet);
    alphabet
}
fn parse_alphabet(alphabet: &String) -> Vec<char> {
    let alphabet: Vec<char> = alphabet.chars().filter(|&c| c.is_alphanumeric()).collect();
    alphabet


}

#[doc = "Test sprawdzający czy funkcja read_alphabet zwraca poprawny alfabet, trzeba wpisać \"abcd\""]
#[test]
fn test_read_alphabet() {
    let alphabet = get_alphabet_from_input();
    assert_eq!(alphabet, vec!['a', 'b', 'c', 'd']);
}
fn create_sets(transformations_with_variables: &Vec<HashMap<char, VariableSituation>>, alphabet: &Vec<char>) -> (HashSet<(char, char)>, HashSet<(char, char)>) {
    let mut D = HashSet::new();
    let mut I = HashSet::new();
    let mut has_been_added_to_D = false;
    let twv_len = transformations_with_variables.len();
    for i in 0..twv_len {
        for j in i..twv_len{
            if i == j {
                D.insert((alphabet[i], alphabet[j]));
                continue;
            }
            for (variable, situation_first) in &transformations_with_variables[i] {
                match transformations_with_variables[j].get(variable) {
                    Some(situation_second) => {
                        if situation_first.is_depend(situation_second) {
                            D.insert((alphabet[i], alphabet[j]));
                            D.insert((alphabet[j], alphabet[i]));
                            has_been_added_to_D = true;
                            break;
                        }
                    },
                    None => panic!("Variable not found in hashmap")
                }
            }
            if !has_been_added_to_D {
                I.insert((alphabet[i], alphabet[j]));
                I.insert((alphabet[j], alphabet[i]));
            }
            has_been_added_to_D = false;
    }
    
    }
    (D, I)
}
fn read_transformations(n: i32) -> Vec<String> {
    let mut transformations: Vec<String> = Vec::new();
    for i in 0..n {
        println!("Podaj równanie nr {}: ", i+1);
        let mut equation = String::new();
        std::io::stdin().read_line(&mut equation).expect("Failed to read line");
        transformations.push(equation);
    }
    transformations    
}

fn find_variables(transformations: &Vec<String>) -> HashSet<char> {
    let mut variables:HashSet<char> = HashSet::new();
    for transformation in transformations {
        for c in transformation.chars() {
            if c.is_alphabetic() {
                variables.insert(c);
            }
        }
    }
    variables
}

fn create_matrix(variables: &HashSet<char>, n: i32) -> Vec<HashMap<char, VariableSituation>> {
    let mut transformation_with_variables: Vec<HashMap<char, VariableSituation>> = Vec::new();
    for _ in 0..n {
        transformation_with_variables.push(HashMap::new());
        for variable in variables {
            transformation_with_variables.last_mut().expect("Error creating matrix!").insert(*variable, VariableSituation::Neither);
        }
    }
    transformation_with_variables
}

fn fill_matrix_with_variables_status(transformations: &Vec<String>, transformations_with_variables: &mut Vec<HashMap<char, VariableSituation>>)  {
    for (i, transformation) in transformations.iter().enumerate() {
        let mut last_char = ' ';
        let mut current_site = CurrentSite::Left;
        for c in transformation.chars() {
            if last_char == '<' && c == '=' {
                current_site = CurrentSite::Right;
            }
            last_char = c;
             
            if c.is_alphabetic() {
                match transformations_with_variables[i].get_mut(&c) {
                    Some(situation) => {
                        match situation {
                            VariableSituation::Neither => {
                                match current_site {
                                    CurrentSite::Left => {
                                        *situation = VariableSituation::Left;
                                    },
                                    CurrentSite::Right => {
                                        *situation = VariableSituation::Right;
                                    }
                                }
                            },
                            VariableSituation::Left => {
                                match current_site {
                                    CurrentSite::Left => {
                                        *situation = VariableSituation::Left;
                                    },
                                    CurrentSite::Right => {
                                        *situation = VariableSituation::Both;
                                    }
                                    
                                }
                                *situation = VariableSituation::Both;
                            },
                            VariableSituation::Right => {
                                //*situation = VariableSituation::Both;
                            },
                            VariableSituation::Both => {
                                //*situation = VariableSituation::Both;
                            }
                        }
                    },
                    None => {
                        panic!("Variable not found in hashmap");
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum VariableSituation {
    Left,
    Right,
    Both,
    Neither 
}

impl VariableSituation {
    fn is_depend(&self, other:  &VariableSituation) -> bool {
        match (self, other) {
            (VariableSituation::Left | VariableSituation::Both, other)  if *other != VariableSituation::Neither => true,
            (VariableSituation::Right, VariableSituation::Left | VariableSituation::Both) => true,
            (_, _) => false
        }
    }
}
#[doc = "Test sprawdzający wszystkie możliwe przypadki zależności zmiennych"]
#[test]
fn test_is_depend() {
    assert_eq!(VariableSituation::Left.is_depend(&VariableSituation::Right), true);
    assert_eq!(VariableSituation::Left.is_depend(&VariableSituation::Left), true); 
    assert_eq!(VariableSituation::Left.is_depend(&VariableSituation::Both), true); 
    assert_eq!(VariableSituation::Left.is_depend(&VariableSituation::Neither), false);
    assert_eq!(VariableSituation::Right.is_depend(&VariableSituation::Left), true);
    assert_eq!(VariableSituation::Right.is_depend(&VariableSituation::Right), false);
    assert_eq!(VariableSituation::Right.is_depend(&VariableSituation::Both), true);
    assert_eq!(VariableSituation::Right.is_depend(&VariableSituation::Neither), false);
    assert_eq!(VariableSituation::Both.is_depend(&VariableSituation::Left), true);
    assert_eq!(VariableSituation::Both.is_depend(&VariableSituation::Right), true);
    assert_eq!(VariableSituation::Both.is_depend(&VariableSituation::Both), true);
    assert_eq!(VariableSituation::Both.is_depend(&VariableSituation::Neither), false);
    assert_eq!(VariableSituation::Neither.is_depend(&VariableSituation::Left), false);
    assert_eq!(VariableSituation::Neither.is_depend(&VariableSituation::Right), false);
    assert_eq!(VariableSituation::Neither.is_depend(&VariableSituation::Both), false);
    assert_eq!(VariableSituation::Neither.is_depend(&VariableSituation::Neither), false);
}
enum CurrentSite {
    Left,
    Right
}