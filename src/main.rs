

use std::collections::HashSet;
use std::collections::HashMap;

fn main(){
    println!("Podaj liczbę równań, które chcesz wprowadzić: ");
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: i32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Podana wartość nie jest liczbą całkowitą!"),
    };
    let transformations = read_transformations(n);

    let  variables = find_variables(&transformations);
    
    let mut transformations_with_variables: Vec<HashMap<char, VariableSituation>> = create_matrix(&variables, n);
    fill_matrix_with_variables_status(&transformations, &mut transformations_with_variables);
    for (i, transformation) in transformations.iter().enumerate() {
        println!("Równanie nr {}: {}", i+1, transformation);
        for (variable, situation) in &transformations_with_variables[i] {
            println!("Zmienna: {}, sytuacja: {:?}", variable, situation);
        }
    }


}

#[doc = "test read_transformations"]
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
enum VariableSituation {
    Left,
    Right,
    Both,
    Neither 
}

enum CurrentSite {
    Left,
    Right
}