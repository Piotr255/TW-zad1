#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;
fn main(){

    //let (n, transformations, alphabet) = read_from_console();
    let (n, transformations, alphabet, word) = read_from_file("data3.txt".to_string());
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
    let foata = create_foata_normal_form(&word, &I, &alphabet);
    for (i, set) in foata.iter().enumerate() {
        println!("Zbiór nr {}: {:?}", i+1, set);
    }
    


}

#[doc = "Funkcja odpowiada za wyznaczenie zbiorów Foaty. Korzystam z algorytmu z kopcami z książki podanej w treści zadania."]
fn create_foata_normal_form(word: &String, I: &HashSet<(char,char)>,  alphabet: &Vec<char>) -> Vec<HashSet<char>> {
    let mut foata: Vec<HashSet<char>> = Vec::new();
    let mut stacks: HashMap<char, Vec<char>> = alphabet.iter().map(|&c| (c, Vec::new())).collect();
    fill_stacks(word, &mut stacks, I);
    fill_foata(&mut stacks, &mut foata, &I, &alphabet);
    foata
}


fn fill_foata(stacks: &mut HashMap<char, Vec<char>>, foata: &mut Vec<HashSet<char>>, I: &HashSet<(char, char)>, alphabet: &Vec<char>) {  
    let mut to_be_popped = Vec::new();
    while !stacks.values().all(|stack| stack.is_empty()) {
        let mut set = HashSet::new();
        for element in &to_be_popped {
            let stack = stacks.get_mut(element).unwrap();
            stack.pop();
        }
        to_be_popped.clear();
        for stack_el in stacks.iter_mut() {
            let (_letter, stack) = stack_el;
            let current_sign = *stack.last().unwrap_or(&'*'); // '*' has here second meaning, it's a sign that stack is empty (normally is a special stack sign)
            if current_sign != '*' {
                set.insert(current_sign);
                stack.pop();
                for alphabet_letter in alphabet {
                    if !I.contains(&(current_sign, *alphabet_letter)) {
                        if current_sign == *alphabet_letter {
                            continue;
                        }
                            to_be_popped.push(*alphabet_letter);
                        }
                    }
                }  
            }
        
        if set.is_empty() {
            for stack_el in stacks.iter_mut() {
                let (letter, stack) = stack_el;
                stack.pop();
            }
        } else {
            foata.push(set);
        }
    }
    }
    

fn fill_stacks(word: &String, stacks: &mut HashMap<char, Vec<char>>, I: &HashSet<(char, char)>) {
    for c in word.chars().rev() {
        for stack_el in stacks.iter_mut() {
            let (letter, stack) = stack_el;
            if !I.contains(&(c, *letter)) {
                if c == *letter {
                    stack.push(c);
                } else {
                    stack.push('*');
                }
            }
    
        }

    }
}


fn read_file_name() -> String {
    println!("Podaj nazwę pliku z danymi: ");
    let mut filename = String::new();
    std::io::stdin().read_line(&mut filename).expect("Failed to read filename");
    let filename = filename.trim();
    filename.to_string()
}
fn read_from_file(mut filename: String) ->(i32, Vec<String>, Vec<char>, String) {
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
    let word = lines.next().expect("Failed to read word");
    (n, transformations, alphabet, word.to_string())
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

enum CurrentSite {
    Left,
    Right
}