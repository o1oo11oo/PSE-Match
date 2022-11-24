const SYMBOLS: [&str; 119] = [
    "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mg", "Al", "Si", "P", "S", "Cl",
    "Ar", "K", "Ca", "Sc", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga", "Ge", "As",
    "Se", "Br", "Kr", "Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pd", "Ag", "Cd", "In",
    "Sn", "Sb", "Te", "I", "Xe", "Cs", "Ba", "La", "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd", "Tb",
    "Dy", "Ho", "Er", "Tm", "Yb", "Lu", "Hf", "Ta", "W", "Re", "Os", "Ir", "Pt", "Au", "Hg", "Tl",
    "Pb", "Bi", "Po", "At", "Rn", "Fr", "Ra", "Ac", "Th", "Pa", "U", "Np", "Pu", "Am", "Cm", "Bk",
    "Cf", "Es", "Fm", "Md", "No", "Lr", "Rf", "Db", "Sg", "Bh", "Hs", "Mt", "Ds", "Rg", "Cn", "Nh",
    "Fl", "Mc", "Lv", "Ts", "Og", "Uue",
];

const SYMBOLS_LOWER: [&str; 119] = [
    "h", "he", "li", "be", "b", "c", "n", "o", "f", "ne", "na", "mg", "al", "si", "p", "s", "cl",
    "ar", "k", "ca", "sc", "ti", "v", "cr", "mn", "fe", "co", "ni", "cu", "zn", "ga", "ge", "as",
    "se", "br", "kr", "rb", "sr", "y", "zr", "nb", "mo", "tc", "ru", "rh", "pd", "ag", "cd", "in",
    "sn", "sb", "te", "i", "xe", "cs", "ba", "la", "ce", "pr", "nd", "pm", "sm", "eu", "gd", "tb",
    "dy", "ho", "er", "tm", "yb", "lu", "hf", "ta", "w", "re", "os", "ir", "pt", "au", "hg", "tl",
    "pb", "bi", "po", "at", "rn", "fr", "ra", "ac", "th", "pa", "u", "np", "pu", "am", "cm", "bk",
    "cf", "es", "fm", "md", "no", "lr", "rf", "db", "sg", "bh", "hs", "mt", "ds", "rg", "cn", "nh",
    "fl", "mc", "lv", "ts", "og", "uue",
];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    for arg in &args[1..] {
        println!("Match for {}: {:?}", arg, find_match(arg));
    }
}

#[derive(Debug)]
enum MatchResult {
    None,
    Found(Vec<usize>, String),
}

#[derive(Debug)]
enum ElementMatch {
    None,
    Found(Vec<usize>),
}

fn find_match(word: &str) -> MatchResult {
    let mut indexes = match find_match_recursive(&word.to_lowercase()) {
        ElementMatch::None => return MatchResult::None,
        ElementMatch::Found(indexes) => indexes,
    };
    indexes.reverse();

    let composition = indexes.iter().map(|&idx| SYMBOLS[idx]).collect();
    for ele in indexes.iter_mut() {
        *ele += 1;
    }

    MatchResult::Found(indexes, composition)
}

fn find_match_recursive(word: &str) -> ElementMatch {
    //println!("Checking '{}' with {}", word, word.len());

    if word.is_empty() {
        return ElementMatch::Found(Vec::new());
    }

    let max_search_length = std::cmp::min(word.len(), 3);
    for i in (0..=max_search_length).rev() {
        //println!("Word part is {}", &word[..i]);
        let index = SYMBOLS_LOWER.iter().position(|&r| *r == word[..i]);
        if let Some(index) = index {
            let substring_result = find_match_recursive(&word[i..]);
            match substring_result {
                ElementMatch::None => continue,
                ElementMatch::Found(mut indexes) => {
                    indexes.push(index);
                    return ElementMatch::Found(indexes);
                }
            }
        }
    }

    ElementMatch::None
}
