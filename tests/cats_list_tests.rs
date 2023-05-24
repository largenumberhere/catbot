use std::collections::HashSet;

static cat_file:&str = include_str!("..\\src\\cats.json");

#[test]
pub fn cat_list_makes_valid_hashmap(){

    let cats:HashSet<String> = serde_json::de::from_str(cat_file).expect("failed to parse file");
    assert_ne!(cats.len(), 0);
}


#[test]
pub fn cat_list_is_lowercase(){
    let cats:HashSet<String> = serde_json::de::from_str(cat_file).expect("failed to parse file");
    cats.iter().for_each(|cat|{assert_eq!(cat.to_lowercase(), cat.as_str()) });
}
