use std::collections::HashSet;

pub fn test_hashset_basic () {
    let planets_list:HashSet<&str> = HashSet::from(["Mercury", "Venus", "Earth"]);

    let planets_list_more:HashSet<&str> = HashSet::from(["Neptune", "Pluto", "Uranus", "Earth"]);

    let symdifference_planets = planets_list.symmetric_difference(& planets_list_more);

    println!("{:?}", symdifference_planets)
}