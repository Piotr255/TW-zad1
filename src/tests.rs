
use super::*;


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



#[doc = "Test sprawdzający czy funkcja read_from_file zwraca poprawne dane"]
#[test]
fn test_read_from_file() {
    let (n, transformations, alphabet,word ) = read_from_file("data1.txt".to_string());
    assert_eq!(n, 4);
    assert_eq!(transformations, vec!["x <= x+y", "y <= y+2z", "x <= 3x+z", "z <= y-z"]);
    assert_eq!(alphabet, vec!['a', 'b', 'c', 'd']);
    assert_eq!(word, "baadcb".to_string());
}