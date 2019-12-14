use crate::utils::prelude::*;

//5 B, 7 C => 1 BC
#[derive(PartialEq, Debug)]
pub struct Input<'a> {
    qty: usize,
    ingredient: &'a str,
}

#[derive(PartialEq, Debug)]
pub struct Recipe<'a> {
    output: &'a str,
    qty_output: &'a str,
    inputs: Vec<Input<'a>>,
}

type RecipeBook<'a> = HashMap<&'a str, (usize, Vec<(usize, &'a str)>)>;
pub fn mk_rb(input: &str) -> RecipeBook {
    input
        .trim()
        .lines()
        .map(|l| {
            let a = l
                .split("=>")
                .map(|side| {
                    side.trim()
                        .split(',')
                        .map(|item| {
                            let b = item.trim().split(' ').map(str::trim).collect_vec();
                            assert_eq!(b.len(), 2);
                            (
                                b[0].parse::<usize>()
                                    .unwrap_or_else(|x| panic!("Not an int {}: {}", b[0], x)),
                                b[1],
                            )
                        })
                        .collect_vec()
                })
                .collect_vec();
            let (q, output) = a[1][0];
            (output, (q, a[0].clone()))
        })
        .collect()
}

pub fn ore_for_n_fuel(recipes: &RecipeBook, n: usize) -> usize {
    let mut required: HashMap<&str, usize> = HashMap::new();
    let mut hold: HashMap<&str, usize> = HashMap::new();
    required.insert("FUEL", n);
    let mut ore_used = 0;
    while !required.is_empty() {
        let (&lets_make, qty_needed) = required.iter().nth(0).unwrap();
        let (qty_per, ingredients) = &recipes[lets_make];
        let left_over = qty_needed % qty_per;
        let iterations_required = (qty_needed / qty_per) + if left_over > 0 { 1 } else { 0 };
        for (q, reagent) in ingredients {
            let needed = q * iterations_required;
            if *reagent == "ORE" {
                ore_used += needed;
            } else {
                let available = *hold.get(reagent).unwrap_or(&0);
                if available > needed {
                    *hold.get_mut(reagent).unwrap() -= needed;
                } else {
                    hold.remove(reagent);
                    *required.entry(reagent).or_insert(0) += needed - available;
                }
            }
        }
        if left_over > 0 {
            *hold.entry(lets_make).or_default() += qty_per - left_over;
        }
        required.remove(lets_make);
    }
    ore_used
}

#[aoc(day14, part1)]
pub fn p1(input: &str) -> usize {
    ore_for_n_fuel(&mk_rb(input), 1)
}
#[aoc(day14, part2)]
pub fn p2(input: &str) -> usize {
    let recipes = mk_rb(input);
    unbounded_bin_search(&|x| ore_for_n_fuel(&recipes, x), 1_000_000_000_000)
}
