#![allow(dead_code)]

/// Gets the index (into Int. Crys. Handbook Vol A 2016) for the given Herman Mauguin symbol
pub fn get_index_for_symbol(symbol: &str) -> Option<usize> {
    let mut counter = 1;
    for item in HERMANN_MAUGUIN_SYMBOL {
        if *item == symbol {
            return Some(counter);
        }
        counter += 1;
    }
    None
}

/// Gets the Herman Mauguin symbol for the given index (into Int. Crys. Handbook Vol A 2016)
pub fn get_symbol_for_index(index: usize) -> Option<&'static str> {
    match HERMANN_MAUGUIN_SYMBOL.get(index-1) {
        Some(n) => Some(*n),
        None => None,
    }
}

/// Gets the transformations given an index (into Int. Crys. Handbook Vol A 2016) for the given space group
pub fn get_transformation(index: usize) -> Option<&'static [(usize, &'static str)]> {
    match SYMBOL_TRANSFORMATION.get(index-1) {
        Some(n) => Some(*n),
        None => None,
    }
}

/// Gets the atomic number for the given element
pub fn get_atomic_number(element: &str) -> Option<usize> {
    let mut counter = 1;
    for item in ELEMENT_SYMBOLS {
        if *item == element {
            return Some(counter);
        }
        counter += 1;
    }
    None
}

/// Gets the atomic radius for the given atomic number (defined up until 'Cm')
/// Source: Martin Rahm, Roald Hoffmann, and N. W. Ashcroft. Atomic and Ionic Radii of Elements 1-96. Chemistry - A European Journal, 22(41):14625–14632, oct 2016. URL: http://doi.wiley.com/10.1002/chem.201602949, doi:10.1002/chem.201602949.
pub fn get_atomic_radius(atomic_number: usize) -> Option<f64> {
    match ELEMENT_RADII.get(atomic_number) {
        Some(n) => Some(*n),
        None => None,
    }
}

/// Gets the amino acid number into the table, effectively providing the recognition of it being an amino acid or not
pub fn get_amino_acid_number(aa: &str) -> Option<usize> {
    let mut counter = 1;
    for item in AMINO_ACIDS {
        if *item == aa {
            return Some(counter);
        }
        counter += 1;
    }
    None
}

const SYMBOL_TRANSFORMATION: &[&[(usize,&str)]] = &[
    &[],
    &[ (1,"1/2,1/2,1/2"), (1,"0,1/2,1/2"), (1,"1/2,0,1/2"), (1,"1/2,1/2,0"), (1,"1/2,0,0"), (1,"0,1/2,0"), (1,"0,0,1/2"), (1,"0,0,0")],
    &[ (1,"1/2,y,1/2"), (1,"1/2,y,0"), (1,"0,y,1/2"), (1,"0,y,0")],
    &[],
    &[ (2,"0,y,1/2"), (2,"0,y,0")],
    &[ (1,"x,1/2,z"), (1,"x,0,z")],
    &[],
    &[ (2,"x,0,z")],
    &[],
    &[ (2,"x,1/2,z"), (2,"x,0,z"), (2,"1/2,y,1/2"), (2,"0,y,1/2"), (2,"1/2,y,0"), (2,"0,y,0"), (1,"1/2,1/2,1/2"), (1,"1/2,0,1/2"), (1,"0,1/2,1/2"), (1,"1/2,1/2,0"), (1,"1/2,0,0"), (1,"0,0,1/2"), (1,"0,1/2,0"), (1,"0,0,0")],
    &[ (2,"x,1/4,z"), (2,"1/2,0,1/2"), (2,"0,0,1/2"), (2,"1/2,0,0"), (2,"0,0,0")],
    &[ (4,"x,0,z"), (4,"0,y,1/2"), (4,"0,y,0"), (4,"1/4,1/4,1/2"), (4,"1/4,1/4,0"), (2,"0,1/2,1/2"), (2,"0,0,1/2"), (2,"0,1/2,0"), (2,"0,0,0")],
    &[ (2,"1/2,y,1/4"), (2,"0,y,1/4"), (2,"1/2,0,0"), (2,"0,1/2,0"), (2,"1/2,1/2,0"), (2,"0,0,0")],
    &[ (2,"1/2,0,1/2"), (2,"0,0,1/2"), (2,"1/2,0,0"), (2,"0,0,0")],
    &[ (4,"0,y,1/4"), (4,"1/4,1/4,1/2"), (4,"1/4,1/4,0"), (4,"0,1/2,0"), (4,"0,0,0")],
    &[ (2,"1/2,1/2,z"), (2,"0,1/2,z"), (2,"1/2,0,z"), (2,"0,0,z"), (2,"1/2,y,1/2"), (2,"1/2,y,0"), (2,"0,y,1/2"), (2,"0,y,0"), (2,"x,1/2,1/2"), (2,"x,1/2,0"), (2,"x,0,1/2"), (2,"x,0,0"), (1,"1/2,1/2,1/2"), (1,"0,1/2,1/2"), (1,"1/2,0,1/2"), (1,"1/2,1/2,0"), (1,"0,0,1/2"), (1,"0,1/2,0"), (1,"1/2,0,0"), (1,"0,0,0")],
    &[ (2,"1/2,y,1/4"), (2,"0,y,1/4"), (2,"x,1/2,0"), (2,"x,0,0")],
    &[ (2,"0,1/2,z"), (2,"0,0,z")],
    &[],
    &[ (4,"0,y,1/4"), (4,"x,0,0")],
    &[ (4,"1/4,1/4,z"), (4,"0,1/2,z"), (4,"0,0,z"), (4,"0,y,1/2"), (4,"0,y,0"), (4,"x,0,1/2"), (4,"x,0,0"), (2,"0,0,1/2"), (2,"1/2,0,1/2"), (2,"0,1/2,0"), (2,"0,0,0")],
    &[ (8,"x,1/4,1/4"), (8,"1/4,y,1/4"), (8,"1/4,1/4,z"), (8,"0,0,z"), (8,"0,y,0"), (8,"x,0,0"), (4,"1/4,1/4,-1/4"), (4,"1/4,1/4,1/4"), (4,"0,0,1/2"), (4,"0,0,0")],
    &[ (4,"0,1/2,z"), (4,"0,0,z"), (4,"1/2,y,0"), (4,"0,y,0"), (4,"x,0,1/2"), (4,"x,0,0"), (2,"0,1/2,0"), (2,"0,0,1/2"), (2,"1/2,0,0"), (2,"0,0,0")],
    &[ (4,"0,1/4,z"), (4,"1/4,y,0"), (4,"x,0,1/4")],
    &[ (2,"1/2,y,z"), (2,"0,y,z"), (2,"x,1/2,z"), (2,"x,0,z"), (1,"1/2,1/2,z"), (1,"1/2,0,z"), (1,"0,1/2,z"), (1,"0,0,z")],
    &[ (2,"1/2,y,z"), (2,"0,y,z")],
    &[ (2,"1/2,1/2,z"), (2,"1/2,0,z"), (2,"0,1/2,z"), (2,"0,0,z")],
    &[ (2,"1/4,y,z"), (2,"0,1/2,z"), (2,"0,0,z")],
    &[],
    &[ (2,"1/2,0,z"), (2,"0,0,z")],
    &[ (2,"0,y,z")],
    &[],
    &[],
    &[],
    &[ (4,"0,y,z"), (4,"x,0,z"), (4,"1/4,1/4,z"), (2,"0,1/2,z"), (2,"0,0,z")],
    &[ (4,"0,y,z")],
    &[ (4,"1/4,1/4,z"), (4,"0,1/2,z"), (4,"0,0,z")],
    &[ (4,"1/2,y,z"), (4,"0,y,z"), (4,"x,0,z"), (2,"1/2,0,z"), (2,"0,0,z")],
    &[ (4,"x,1/4,z"), (4,"1/2,0,z"), (4,"0,0,z")],
    &[ (4,"1/4,y,z"), (4,"0,0,z")],
    &[ (4,"0,0,z")],
    &[ (8,"x,0,z"), (8,"0,y,z"), (8,"1/4,1/4,z"), (4,"0,0,z")],
    &[ (8,"0,0,z")],
    &[ (4,"0,y,z"), (4,"x,0,z"), (2,"0,1/2,z"), (2,"0,0,z")],
    &[ (4,"0,1/2,z"), (4,"0,0,z")],
    &[],
    &[ (4,"x,y,1/2"), (4,"x,y,0"), (4,"x,1/2,z"), (4,"x,0,z"), (4,"1/2,y,z"), (4,"0,y,z"), (2,"1/2,1/2,z"), (2,"1/2,0,z"), (2,"0,1/2,z"), (2,"0,0,z"), (2,"1/2,y,1/2"), (2,"1/2,y,0"), (2,"0,y,1/2"), (2,"0,y,0"), (2,"x,1/2,1/2"), (2,"x,1/2,0"), (2,"x,0,1/2"), (2,"x,0,0"), (1,"1/2,1/2,1/2"), (1,"0,1/2,1/2"), (1,"1/2,1/2,0"), (1,"0,1/2,0"), (1,"1/2,0,1/2"), (1,"0,0,1/2"), (1,"1/2,0,0"), (1,"0,0,0")],
    &[ (4,"1/4,-1/4,z"), (4,"1/4,1/4,z"), (4,"-1/4,y,1/4"), (4,"1/4,y,1/4"), (4,"x,1/4,-1/4"), (4,"x,1/4,1/4"), (4,"0,0,0"), (4,"1/2,1/2,1/2"), (2,"1/4,-1/4,1/4"), (2,"1/4,1/4,-1/4"), (2,"-1/4,1/4,1/4"), (2,"1/4,1/4,1/4")],
    &[ (4,"x,y,0"), (4,"1/2,0,z"), (4,"0,1/2,z"), (4,"1/2,1/2,z"), (4,"0,0,z"), (4,"1/2,y,1/4"), (4,"0,y,1/4"), (4,"x,1/2,1/4"), (4,"x,0,1/4"), (2,"1/2,1/2,1/4"), (2,"0,1/2,1/4"), (2,"1/2,0,1/4"), (2,"0,0,1/4"), (2,"1/2,0,0"), (2,"0,1/2,0"), (2,"1/2,1/2,0"), (2,"0,0,0")],
    &[ (4,"1/4,-1/4,z"), (4,"1/4,1/4,z"), (4,"1/4,y,1/2"), (4,"1/4,y,0"), (4,"x,1/4,1/2"), (4,"x,1/4,0"), (4,"0,0,1/2"), (4,"0,0,0"), (2,"1/4,1/4,1/2"), (2,"-1/4,1/4,1/2"), (2,"-1/4,1/4,0"), (2,"1/4,1/4,0")],
    &[ (4,"1/4,y,z"), (4,"x,1/2,z"), (4,"x,0,z"), (4,"0,y,1/2"), (4,"0,y,0"), (2,"1/4,1/2,z"), (2,"1/4,0,z"), (2,"0,1/2,1/2"), (2,"0,0,1/2"), (2,"0,1/2,0"), (2,"0,0,0")],
    &[ (4,"x,1/4,1/4"), (4,"1/4,0,z"), (4,"0,0,1/2"), (4,"0,0,0")],
    &[ (4,"0,y,z"), (4,"1/4,y,1/4"), (4,"x,1/2,0"), (4,"x,0,0"), (2,"0,1/2,0"), (2,"1/2,1/2,0"), (2,"1/2,0,0"), (2,"0,0,0")],
    &[ (4,"1/4,1/2,z"), (4,"1/4,0,z"), (4,"0,y,1/4"), (4,"0,1/2,0"), (4,"0,0,0")],
    &[ (4,"x,y,1/2"), (4,"x,y,0"), (4,"0,1/2,z"), (4,"0,0,z"), (2,"0,1/2,1/2"), (2,"0,1/2,0"), (2,"0,0,1/2"), (2,"0,0,0")],
    &[ (4,"1/4,-1/4,z"), (4,"1/4,1/4,z"), (4,"0,0,1/2"), (4,"0,0,0")],
    &[ (4,"x,y,1/4"), (4,"x,1/4,0"), (4,"1/2,0,0"), (4,"0,0,0")],
    &[ (4,"x,y,0"), (4,"0,1/2,z"), (4,"0,0,z"), (2,"0,1/2,1/2"), (2,"0,1/2,0"), (2,"0,0,1/2"), (2,"0,0,0")],
    &[ (4,"x,1/4,z"), (4,"1/4,y,z"), (4,"0,0,1/2"), (4,"0,0,0"), (2,"1/4,-1/4,z"), (2,"1/4,1/4,z")],
    &[ (4,"0,y,1/4"), (4,"0,1/2,0"), (4,"0,0,0")],
    &[ (4,"0,0,1/2"), (4,"0,0,0")],
    &[ (4,"x,1/4,z"), (4,"0,0,1/2"), (4,"0,0,0")],
    &[ (8,"x,y,1/4"), (8,"0,y,z"), (8,"x,0,0"), (8,"1/4,1/4,0"), (4,"0,y,1/4"), (4,"0,1/2,0"), (4,"0,0,0")],
    &[ (8,"0,y,z"), (8,"1/4,y,1/4"), (8,"x,0,0"), (8,"1/4,1/4,0"), (4,"1/2,0,0"), (4,"0,0,0")],
    &[ (8,"x,y,1/2"), (8,"x,y,0"), (8,"x,0,z"), (8,"0,y,z"), (8,"1/4,1/4,z"), (4,"0,1/2,z"), (4,"0,0,z"), (4,"0,y,1/2"), (4,"0,y,0"), (4,"x,0,1/2"), (4,"x,0,0"), (4,"1/4,1/4,1/2"), (4,"1/4,1/4,0"), (2,"0,0,1/2"), (2,"1/2,0,1/2"), (2,"1/2,0,0"), (2,"0,0,0")],
    &[ (8,"x,y,0"), (8,"1/4,1/4,z"), (8,"0,1/2,z"), (8,"0,0,z"), (8,"0,y,1/4"), (8,"x,0,1/4"), (4,"1/4,-1/4,0"), (4,"1/4,1/4,0"), (4,"0,1/2,0"), (4,"0,0,0"), (4,"0,1/2,1/4"), (4,"0,0,1/4")],
    &[ (8,"x,1/4,z"), (8,"0,y,z"), (8,"1/4,0,z"), (8,"1/4,y,1/2"), (8,"1/4,y,0"), (8,"x,0,1/2"), (8,"x,0,0"), (4,"0,1/4,z"), (4,"1/4,1/4,1/2"), (4,"1/4,1/4,0"), (4,"0,0,1/2"), (4,"0,0,0"), (4,"1/4,0,1/2"), (4,"1/4,0,0")],
    &[ (8,"1/4,0,z"), (8,"0,1/4,z"), (8,"0,y,1/4"), (8,"x,1/4,1/4"), (8,"0,0,0"), (8,"1/4,-1/4,0"), (4,"0,1/4,-1/4"), (4,"0,1/4,1/4")],
    &[ (16,"x,y,0"), (16,"x,0,z"), (16,"0,y,z"), (16,"x,1/4,1/4"), (16,"1/4,y,1/4"), (16,"1/4,1/4,z"), (8,"0,0,z"), (8,"0,y,0"), (8,"x,0,0"), (8,"1/4,1/4,1/4"), (8,"1/4,1/4,0"), (8,"1/4,0,1/4"), (8,"0,1/4,1/4"), (4,"0,0,1/2"), (4,"0,0,0")],
    &[ (16,"1/8,1/8,z"), (16,"1/8,y,1/8"), (16,"x,1/8,1/8"), (16,"1/2,1/2,1/2"), (16,"0,0,0"), (8,"1/8,1/8,-3/8"), (8,"1/8,1/8,1/8")],
    &[ (8,"x,y,0"), (8,"x,0,z"), (8,"0,y,z"), (8,"1/4,1/4,1/4"), (4,"1/2,0,z"), (4,"0,0,z"), (4,"0,y,1/2"), (4,"0,y,0"), (4,"x,1/2,0"), (4,"x,0,0"), (2,"1/2,0,1/2"), (2,"1/2,1/2,0"), (2,"0,1/2,1/2"), (2,"0,0,0")],
    &[ (8,"x,y,0"), (8,"0,1/2,z"), (8,"0,0,z"), (8,"0,y,1/4"), (8,"x,0,1/4"), (8,"1/4,1/4,1/4"), (4,"1/2,0,0"), (4,"0,0,0"), (4,"1/2,0,1/4"), (4,"0,0,1/4")],
    &[ (8,"0,1/4,z"), (8,"1/4,y,0"), (8,"x,0,1/4"), (8,"1/4,1/4,1/4"), (8,"0,0,0")],
    &[ (8,"x,1/4,z"), (8,"0,y,z"), (8,"1/4,y,1/4"), (8,"x,0,0"), (4,"0,1/4,z"), (4,"1/4,1/4,-1/4"), (4,"1/4,1/4,1/4"), (4,"0,0,1/2"), (4,"0,0,0")],
    &[ (2,"0,1/2,z"), (1,"1/2,1/2,z"), (1,"0,0,z")],
    &[],
    &[ (2,"0,1/2,z"), (2,"1/2,1/2,z"), (2,"0,0,z")],
    &[],
    &[ (4,"0,1/2,z"), (2,"0,0,z")],
    &[],
    &[ (2,"0,1/2,z"), (2,"1/2,1/2,z"), (2,"0,0,z"), (1,"1/2,1/2,1/2"), (1,"1/2,1/2,0"), (1,"0,0,1/2"), (1,"0,0,0")],
    &[ (4,"0,1/2,z"), (4,"0,0,z"), (2,"0,1/2,-1/4"), (2,"0,1/2,1/4"), (2,"0,0,1/2"), (2,"0,0,0")],
    &[ (4,"x,y,1/2"), (4,"x,y,0"), (4,"0,1/2,z"), (2,"1/2,1/2,z"), (2,"0,0,z"), (2,"0,1/2,1/2"), (2,"0,1/2,0"), (1,"1/2,1/2,1/2"), (1,"1/2,1/2,0"), (1,"0,0,1/2"), (1,"0,0,0")],
    &[ (4,"x,y,0"), (4,"0,1/2,z"), (4,"1/2,1/2,z"), (4,"0,0,z"), (2,"1/2,1/2,1/4"), (2,"0,0,1/4"), (2,"0,1/2,1/2"), (2,"0,1/2,0"), (2,"1/2,1/2,0"), (2,"0,0,0")],
    &[ (4,"1/4,-1/4,z"), (4,"0,0,1/2"), (4,"0,0,0"), (2,"1/4,1/4,z"), (2,"1/4,-1/4,1/2"), (2,"1/4,-1/4,0")],
    &[ (4,"1/4,1/4,z"), (4,"-1/4,1/4,z"), (4,"0,0,1/2"), (4,"0,0,0"), (2,"1/4,1/4,-1/4"), (2,"1/4,1/4,1/4")],
    &[ (8,"x,y,0"), (8,"0,1/2,z"), (8,"1/4,1/4,1/4"), (4,"0,0,z"), (4,"0,1/2,1/4"), (4,"0,1/2,0"), (2,"0,0,1/2"), (2,"0,0,0")],
    &[ (8,"0,1/4,z"), (8,"0,0,1/2"), (8,"0,0,0"), (4,"0,1/4,-3/8"), (4,"0,1/4,1/8")],
    &[ (4,"x,1/2,0"), (4,"x,0,1/2"), (4,"x,1/2,1/2"), (4,"x,0,0"), (4,"1/2*x+1/2*y,1/2*x+1/2*y,1/2"), (4,"1/2*x+1/2*y,1/2*x+1/2*y,0"), (4,"0,1/2,z"), (2,"1/2,1/2,z"), (2,"0,0,z"), (2,"1/2,0,1/2"), (2,"1/2,0,0"), (1,"1/2,1/2,1/2"), (1,"1/2,1/2,0"), (1,"0,0,1/2"), (1,"0,0,0")],
    &[ (4,"1/2*x+1/2*y,1/2*x+1/2*y,1/2"), (4,"1/2*x+1/2*y,1/2*x+1/2*y,0"), (4,"0,0,z"), (2,"0,1/2,z"), (2,"0,0,1/2"), (2,"0,0,0")],
    &[ (4,"1/2*x+1/2*y,1/2*x+1/2*y,3/8"), (4,"1/2,y,0"), (4,"0,y,0")],
    &[ (4,"1/2*x+1/2*y,1/2*x+1/2*y,0")],
    &[ (4,"1/2*x+1/2*y,1/2*x+1/2*y,-1/4"), (4,"1/2*x+1/2*y,1/2*x+1/2*y,1/4"), (4,"x,1/2,0"), (4,"x,0,1/2"), (4,"x,1/2,1/2"), (4,"x,0,0"), (4,"0,1/2,z"), (4,"1/2,1/2,z"), (4,"0,0,z"), (2,"1/2,1/2,1/4"), (2,"0,0,1/4"), (2,"0,1/2,1/2"), (2,"0,1/2,0"), (2,"1/2,1/2,0"), (2,"0,0,0")],
    &[ (4,"1/2*x+1/2*y,1/2*x+1/2*y,1/2"), (4,"1/2*x+1/2*y,1/2*x+1/2*y,0"), (4,"0,1/2,z"), (4,"0,0,z"), (2,"0,0,1/2"), (2,"0,0,0")],
    &[ (4,"1/2*x+1/2*y,1/2*x+1/2*y,-3/8"), (4,"1/2,y,0"), (4,"0,y,0")],
    &[],
    &[ (8,"1/2*x+1/2*y-1/4,1/2*x+1/2*y+1/4,1/4"), (8,"x,0,1/2"), (8,"x,0,0"), (8,"1/2*x+1/2*y,1/2*x+1/2*y,0"), (8,"0,1/2,z"), (4,"0,0,z"), (4,"0,1/2,1/4"), (4,"0,1/2,0"), (2,"0,0,1/2"), (2,"0,0,0")],
    &[ (8,"x,1/4,1/8"), (8,"1/2*x-1/2*y,-1/2*x+1/2*y,0"), (8,"1/2*x+1/2*y,1/2*x+1/2*y,0"), (8,"0,0,z"), (4,"0,0,1/2"), (4,"0,0,0")],
    &[ (4,"x,1/2,z"), (4,"x,0,z"), (4,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (2,"1/2,0,z"), (1,"1/2,1/2,z"), (1,"0,0,z")],
    &[ (4,"1/2*x+1/2*y-1/4,1/2*x+1/2*y+1/4,z"), (2,"1/2,0,z"), (2,"0,0,z")],
    &[ (4,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (4,"0,1/2,z"), (2,"1/2,1/2,z"), (2,"0,0,z")],
    &[ (4,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (4,"0,1/2,z"), (2,"0,0,z")],
    &[ (4,"0,1/2,z"), (2,"1/2,1/2,z"), (2,"0,0,z")],
    &[],
    &[ (4,"x,1/2,z"), (4,"x,0,z"), (2,"0,1/2,z"), (2,"1/2,1/2,z"), (2,"0,0,z")],
    &[],
    &[ (8,"x,0,z"), (8,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (4,"0,1/2,z"), (2,"0,0,z")],
    &[ (8,"1/2*x+1/2*y-1/4,1/2*x+1/2*y+1/4,z"), (4,"1/2,0,z"), (4,"0,0,z")],
    &[ (8,"0,y,z"), (4,"0,0,z")],
    &[],
    &[ (4,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (4,"0,1/2,z"), (4,"x,1/2,0"), (4,"x,0,1/2"), (4,"x,1/2,1/2"), (4,"x,0,0"), (2,"1/2,1/2,z"), (2,"0,0,z"), (2,"1/2,0,1/2"), (2,"1/2,0,0"), (1,"1/2,1/2,0"), (1,"0,0,1/2"), (1,"1/2,1/2,1/2"), (1,"0,0,0")],
    &[ (4,"0,1/2,z"), (4,"1/2,1/2,z"), (4,"0,0,z"), (4,"0,y,1/4"), (4,"x,1/2,1/4"), (4,"1/2,y,1/4"), (4,"x,0,1/4"), (2,"1/2,1/2,0"), (2,"0,0,0"), (2,"0,1/2,1/4"), (2,"1/2,1/2,1/4"), (2,"1/2,0,1/4"), (2,"0,0,1/4")],
    &[ (4,"1/2*x+1/2*y-1/4,1/2*x+1/2*y+1/4,z"), (4,"0,0,z"), (2,"0,1/2,z"), (2,"0,0,1/2"), (2,"0,0,0")],
    &[ (4,"0,1/2,z"), (4,"0,0,z"), (2,"0,0,1/2"), (2,"0,0,0")],
    &[ (4,"x,1/2,z"), (4,"x,0,z"), (4,"1/2*x+1/2*y,1/2*x+1/2*y,1/2"), (4,"1/2*x+1/2*y,1/2*x+1/2*y,0"), (2,"0,1/2,z"), (2,"1/2,1/2,z"), (2,"0,0,z"), (1,"0,0,1/2"), (1,"1/2,1/2,1/2"), (1,"1/2,1/2,0"), (1,"0,0,0")],
    &[ (4,"0,1/2,z"), (4,"1/2,1/2,z"), (4,"0,0,z"), (4,"1/2*x+1/2*y,1/2*x+1/2*y,-1/4"), (4,"1/2*x+1/2*y,1/2*x+1/2*y,1/4"), (2,"1/2,1/2,0"), (2,"0,0,0"), (2,"1/2,1/2,1/4"), (2,"0,0,1/4")],
    &[ (4,"1/2*x+1/2*y-1/4,1/2*x+1/2*y+1/4,1/2"), (4,"1/2*x+1/2*y-1/4,1/2*x+1/2*y+1/4,0"), (4,"0,1/2,z"), (4,"0,0,z"), (2,"0,1/2,1/2"), (2,"0,1/2,0"), (2,"0,0,1/2"), (2,"0,0,0")],
    &[ (4,"0,1/2,z"), (4,"1/2*x+1/2*y-1/4,1/2*x+1/2*y+1/4,1/4"), (4,"1/2*x-1/2*y+1/4,-1/2*x+1/2*y+1/4,1/4"), (4,"0,0,z"), (2,"0,1/2,-1/4"), (2,"0,1/2,1/4"), (2,"0,0,1/2"), (2,"0,0,0")],
    &[ (8,"x,0,z"), (8,"1/2*x+1/2*y-1/4,1/2*x+1/2*y+1/4,1/4"), (8,"1/2*x+1/2*y,1/2*x+1/2*y,0"), (4,"0,1/2,z"), (4,"0,0,z"), (2,"0,1/2,-1/4"), (2,"0,1/2,1/4"), (2,"0,0,1/2"), (2,"0,0,0")],
    &[ (8,"1/2*x+1/2*y-1/4,1/2*x+1/2*y+1/4,0"), (8,"0,1/2,z"), (8,"0,0,z"), (8,"1/2*x+1/2*y,1/2*x+1/2*y,1/4"), (4,"0,1/2,0"), (4,"0,1/2,1/4"), (4,"0,0,0"), (4,"0,0,1/4")],
    &[ (8,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (8,"0,1/2,z"), (8,"x,0,1/2"), (8,"x,0,0"), (4,"0,0,z"), (4,"0,1/2,1/4"), (4,"0,1/2,0"), (2,"0,0,1/2"), (2,"0,0,0")],
    &[ (8,"x,1/4,1/8"), (8,"0,0,z"), (4,"0,0,1/2"), (4,"0,0,0")],
    &[ (8,"x,1/2,z"), (8,"x,0,z"), (8,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (8,"x,y,1/2"), (8,"x,y,0"), (4,"x,1/2,1/2"), (4,"x,1/2,0"), (4,"x,0,1/2"), (4,"x,0,0"), (4,"1/2*x+1/2*y,1/2*x+1/2*y,1/2"), (4,"1/2*x+1/2*y,1/2*x+1/2*y,0"), (4,"0,1/2,z"), (2,"1/2,1/2,z"), (2,"0,0,z"), (2,"0,1/2,0"), (2,"0,1/2,1/2"), (1,"1/2,1/2,1/2"), (1,"1/2,1/2,0"), (1,"0,0,1/2"), (1,"0,0,0")],
    &[ (8,"x,y,0"), (8,"x,1/2,1/4"), (8,"x,0,1/4"), (8,"1/2*x+1/2*y,1/2*x+1/2*y,1/4"), (8,"0,1/2,z"), (4,"1/2,1/2,z"), (4,"0,0,z"), (4,"0,1/2,1/4"), (4,"0,1/2,0"), (2,"1/2,1/2,0"), (2,"1/2,1/2,1/4"), (2,"0,0,0"), (2,"0,0,1/4")],
    &[ (8,"1/2*x-1/2*y,-1/2*x+1/2*y,z"), (8,"x,1/4,1/2"), (8,"x,1/4,0"), (8,"1/2*x+1/2*y,1/2*x+1/2*y,1/2"), (8,"1/2*x+1/2*y,1/2*x+1/2*y,0"), (4,"-1/4,1/4,z"), (4,"1/4,1/4,z"), (4,"0,0,1/2"), (4,"0,0,0"), (2,"-1/4,1/4,1/2"), (2,"-1/4,1/4,0"), (2,"1/4,1/4,1/2"), (2,"1/4,1/4,0")],
    &[ (8,"x,-1/4,1/4"), (8,"x,1/4,1/4"), (8,"1/2*x+1/2*y,1/2*x+1/2*y,1/4"), (8,"1/4,-1/4,z"), (8,"0,0,0"), (4,"1/4,1/4,z"), (4,"1/4,-1/4,0"), (4,"1/4,-1/4,-1/4"), (2,"1/4,1/4,-1/4"), (2,"1/4,1/4,1/4")],
    &[ (8,"1/2*x+1/2*y-1/4,1/2*x+1/2*y+1/4,z"), (8,"x,y,1/2"), (8,"x,y,0"), (4,"1/2*x+1/2*y-1/4,1/2*x+1/2*y+1/4,1/2"), (4,"1/2*x+1/2*y-1/4,1/2*x+1/2*y+1/4,0"), (4,"0,1/2,z"), (4,"0,0,z"), (2,"0,1/2,0"), (2,"0,1/2,1/2"), (2,"0,0,1/2"), (2,"0,0,0")],
    &[ (8,"x,y,0"), (8,"1/2*x+1/2*y-1/4,1/2*x+1/2*y+1/4,1/4"), (8,"0,1/2,z"), (4,"0,0,z"), (4,"0,1/2,1/4"), (4,"0,1/2,0"), (2,"0,0,1/2"), (2,"0,0,0")],
    &[ (8,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (8,"1/4,y,z"), (8,"1/2*x-1/2*y,-1/2*x+1/2*y,1/2"), (8,"1/2*x-1/2*y,-1/2*x+1/2*y,0"), (4,"-1/4,1/4,z"), (4,"0,0,1/2"), (4,"0,0,0"), (2,"1/4,1/4,z"), (2,"-1/4,1/4,1/2"), (2,"-1/4,1/4,0")],
    &[ (8,"1/2*x-1/2*y,-1/2*x+1/2*y,1/4"), (8,"-1/4,1/4,z"), (8,"0,0,0"), (4,"1/4,1/4,z"), (4,"-1/4,1/4,0"), (4,"-1/4,1/4,1/4")],
    &[ (8,"x,y,0"), (8,"1/2,y,z"), (8,"0,y,z"), (8,"1/2*x+1/2*y,1/2*x+1/2*y,1/4"), (4,"x,1/2,0"), (4,"x,0,1/2"), (4,"x,1/2,1/2"), (4,"x,0,0"), (4,"0,1/2,z"), (4,"1/2,1/2,z"), (4,"0,0,z"), (2,"1/2,1/2,1/4"), (2,"0,0,1/4"), (2,"0,1/2,1/2"), (2,"0,1/2,0"), (2,"1/2,1/2,0"), (2,"0,0,0")],
    &[ (8,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (8,"x,y,0"), (8,"x,1/2,1/4"), (8,"x,0,1/4"), (8,"0,1/2,z"), (4,"1/2*x+1/2*y,1/2*x+1/2*y,1/2"), (4,"1/2*x+1/2*y,1/2*x+1/2*y,0"), (4,"1/2,1/2,z"), (4,"0,0,z"), (4,"0,1/2,0"), (4,"0,1/2,1/4"), (2,"1/2,1/2,1/4"), (2,"1/2,1/2,0"), (2,"0,0,1/4"), (2,"0,0,0")],
    &[ (8,"1/2*x+1/2*y,1/2*x+1/2*y,1/4"), (8,"x,1/4,1/2"), (8,"x,1/4,0"), (8,"-1/4,1/4,z"), (8,"1/4,1/4,z"), (8,"0,0,0"), (4,"-1/4,1/4,-1/4"), (4,"1/4,1/4,1/4"), (4,"-1/4,1/4,0"), (4,"1/4,1/4,0")],
    &[ (8,"1/2*x-1/2*y,-1/2*x+1/2*y,z"), (8,"1/2*x+1/2*y,1/2*x+1/2*y,1/2"), (8,"1/2*x+1/2*y,1/2*x+1/2*y,0"), (8,"x,1/4,1/4"), (8,"x,1/4,-1/4"), (8,"1/4,1/4,z"), (4,"-1/4,1/4,z"), (4,"0,0,0"), (4,"0,0,1/2"), (4,"1/4,1/4,0"), (4,"1/4,1/4,1/4"), (2,"-1/4,1/4,1/4"), (2,"1/4,-1/4,1/4")],
    &[ (8,"x,y,0"), (8,"1/2*x+1/2*y-1/4,1/2*x+1/2*y+1/4,1/4"), (8,"0,1/2,z"), (8,"0,0,z"), (4,"0,1/2,1/4"), (4,"0,1/2,0"), (4,"0,0,1/4"), (4,"0,0,0")],
    &[ (8,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (8,"x,y,0"), (8,"0,1/2,z"), (4,"1/2*x-1/2*y,-1/2*x+1/2*y,0"), (4,"1/2*x+1/2*y,1/2*x+1/2*y,0"), (4,"0,0,z"), (4,"0,1/2,1/4"), (4,"0,1/2,0"), (2,"0,0,1/2"), (2,"0,0,0")],
    &[ (8,"1/4,y,z"), (8,"1/2*x-1/2*y,-1/2*x+1/2*y,1/4"), (8,"0,0,0"), (4,"1/4,1/4,z"), (4,"-1/4,1/4,z"), (2,"-1/4,1/4,1/4"), (2,"-1/4,1/4,-1/4")],
    &[ (8,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (8,"1/2*x-1/2*y,-1/2*x+1/2*y,0"), (8,"1/2*x-1/2*y,-1/2*x+1/2*y,1/2"), (8,"-1/4,1/4,z"), (4,"1/4,1/4,z"), (4,"0,0,0"), (4,"0,0,1/2"), (4,"-1/4,1/4,-1/4"), (4,"-1/4,1/4,0")],
    &[ (16,"0,y,z"), (16,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (16,"x,y,0"), (16,"1/2*x+1/2*y-1/4,1/2*x+1/2*y+1/4,1/4"), (8,"x,1/2,0"), (8,"x,0,0"), (8,"1/2*x+1/2*y,1/2*x+1/2*y,0"), (8,"0,1/2,z"), (8,"1/4,1/4,1/4"), (4,"0,0,z"), (4,"0,1/2,1/4"), (4,"0,1/2,0"), (2,"0,0,1/2"), (2,"0,0,0")],
    &[ (16,"1/2*x+1/2*y-1/4,1/2*x+1/2*y+1/4,z"), (16,"x,y,0"), (16,"x,0,1/4"), (16,"1/2*x+1/2*y,1/2*x+1/2*y,1/4"), (8,"1/2*x+1/2*y-1/4,1/2*x+1/2*y+1/4,0"), (8,"0,1/2,z"), (8,"0,0,z"), (8,"1/4,1/4,1/4"), (4,"0,1/2,0"), (4,"0,0,0"), (4,"0,1/2,1/4"), (4,"0,0,1/4")],
    &[ (16,"0,y,z"), (16,"1/2*x+1/2*y-1/8,1/2*x+1/2*y+1/8,-1/8"), (16,"x,0,0"), (8,"0,1/4,z"), (8,"0,0,1/2"), (8,"0,0,0"), (4,"0,1/4,3/8"), (4,"0,-1/4,1/8")],
    &[ (16,"1/2*x+1/2*y-1/8,1/2*x+1/2*y+1/8,1/8"), (16,"x,0,1/4"), (16,"0,1/4,z"), (16,"0,0,0"), (8,"0,1/4,1/8"), (8,"0,1/4,3/8")],
    &[ (1,"-1/3,1/3,z"), (1,"1/3,-1/3,z"), (1,"0,0,z")],
    &[],
    &[],
    &[ (3,"0,0,z")],
    &[ (3,"1/2,0,1/2"), (3,"1/2,0,0"), (2,"1/3,-1/3,z"), (2,"0,0,z"), (1,"0,0,1/2"), (1,"0,0,0")],
    &[ (9,"1/2,0,0"), (9,"1/2,0,1/2"), (6,"0,0,z"), (3,"0,0,1/2"), (3,"0,0,0")],
    &[ (3,"1/2*x-1/2*y,-1/2*x+1/2*y,1/2"), (3,"1/2*x-1/2*y,-1/2*x+1/2*y,0"), (2,"-1/3,1/3,z"), (2,"1/3,-1/3,z"), (2,"0,0,z"), (1,"-1/3,1/3,1/2"), (1,"-1/3,1/3,0"), (1,"1/3,-1/3,1/2"), (1,"1/3,-1/3,0"), (1,"0,0,1/2"), (1,"0,0,0")],
    &[ (3,"x-1/2*y,0,1/2"), (3,"x-1/2*y,0,0"), (2,"1/3,-1/3,z"), (2,"0,0,z"), (1,"0,0,1/2"), (1,"0,0,0")],
    &[ (3,"1/2*x-1/2*y,-1/2*x+1/2*y,-1/6"), (3,"1/2*x-1/2*y,-1/2*x+1/2*y,1/3")],
    &[ (3,"x-1/2*y,0,-1/6"), (3,"x-1/2*y,0,1/3")],
    &[ (3,"1/2*x-1/2*y,-1/2*x+1/2*y,1/6"), (3,"1/2*x-1/2*y,-1/2*x+1/2*y,-1/3")],
    &[ (3,"x-1/2*y,0,1/6"), (3,"x-1/2*y,0,-1/3")],
    &[ (9,"x-1/2*y,0,1/2"), (9,"x-1/2*y,0,0"), (6,"0,0,z"), (3,"0,0,1/2"), (3,"0,0,0")],
    &[ (3,"1/2*x-1/2*y,-1/2*x+1/2*y,z"), (1,"-1/3,1/3,z"), (1,"1/3,-1/3,z"), (1,"0,0,z")],
    &[ (3,"x-1/2*y,0,z"), (2,"1/3,-1/3,z"), (1,"0,0,z")],
    &[ (2,"-1/3,1/3,z"), (2,"1/3,-1/3,z"), (2,"0,0,z")],
    &[ (2,"1/3,-1/3,z"), (2,"0,0,z")],
    &[ (9,"1/2*x-1/2*y,-1/2*x+1/2*y,z"), (3,"0,0,z")],
    &[ (6,"0,0,z")],
    &[ (6,"x-1/2*y,0,z"), (6,"1/2*x-1/2*y,-1/2*x+1/2*y,1/2"), (6,"1/2*x-1/2*y,-1/2*x+1/2*y,0"), (4,"1/3,-1/3,z"), (3,"1/2,0,1/2"), (3,"1/2,0,0"), (2,"0,0,z"), (2,"1/3,-1/3,1/2"), (2,"1/3,-1/3,0"), (1,"0,0,1/2"), (1,"0,0,0")],
    &[ (6,"1/2*x-1/2*y,-1/2*x+1/2*y,1/4"), (6,"1/2,0,0"), (4,"1/3,-1/3,z"), (4,"0,0,z"), (2,"-1/3,1/3,1/4"), (2,"1/3,-1/3,1/4"), (2,"0,0,0"), (2,"0,0,1/4")],
    &[ (6,"1/2*x-1/2*y,-1/2*x+1/2*y,z"), (6,"x-1/2*y,0,1/2"), (6,"x-1/2*y,0,0"), (3,"1/2,0,1/2"), (3,"1/2,0,0"), (2,"1/3,-1/3,z"), (2,"0,0,z"), (1,"0,0,1/2"), (1,"0,0,0")],
    &[ (6,"x-1/2*y,0,1/4"), (6,"1/2,0,0"), (4,"1/3,-1/3,z"), (4,"0,0,z"), (2,"0,0,0"), (2,"0,0,1/4")],
    &[ (18,"1/2*x-1/2*y,-1/2*x+1/2*y,z"), (18,"x-1/2*y,0,1/2"), (18,"x-1/2*y,0,0"), (9,"1/2,0,0"), (9,"1/2,0,1/2"), (6,"0,0,z"), (3,"0,0,1/2"), (3,"0,0,0")],
    &[ (18,"x-1/2*y,0,1/4"), (18,"1/2,0,0"), (12,"0,0,z"), (6,"0,0,0"), (6,"0,0,1/4")],
    &[ (3,"1/2,0,z"), (2,"1/3,-1/3,z"), (1,"0,0,z")],
    &[],
    &[],
    &[ (3,"1/2,1/2,z"), (3,"0,0,z")],
    &[],
    &[],
    &[ (3,"x,y,1/2"), (3,"x,y,0"), (2,"-1/3,1/3,z"), (2,"1/3,-1/3,z"), (2,"0,0,z"), (1,"-1/3,1/3,1/2"), (1,"-1/3,1/3,0"), (1,"1/3,-1/3,1/2"), (1,"1/3,-1/3,0"), (1,"0,0,1/2"), (1,"0,0,0")],
    &[ (6,"x,y,1/2"), (6,"x,y,0"), (6,"1/2,0,z"), (4,"1/3,-1/3,z"), (3,"1/2,0,1/2"), (3,"1/2,0,0"), (2,"0,0,z"), (2,"1/3,-1/3,1/2"), (2,"1/3,-1/3,0"), (1,"0,0,1/2"), (1,"0,0,0")],
    &[ (6,"x,y,1/4"), (6,"1/2,0,0"), (4,"1/3,-1/3,z"), (4,"0,0,z"), (2,"-1/3,1/3,1/4"), (2,"1/3,-1/3,1/4"), (2,"0,0,0"), (2,"0,0,1/4")],
    &[ (6,"1/2*x-1/2*y,-1/2*x+1/2*y,1/2"), (6,"1/2*x-1/2*y,-1/2*x+1/2*y,0"), (6,"x-1/2*y,0,1/2"), (6,"x-1/2*y,0,0"), (6,"1/2,0,z"), (4,"1/3,-1/3,z"), (3,"1/2,0,1/2"), (3,"1/2,0,0"), (2,"0,0,z"), (2,"1/3,-1/3,1/2"), (2,"1/3,-1/3,0"), (1,"0,0,1/2"), (1,"0,0,0")],
    &[ (6,"1/2*y,y,1/4"), (6,"x-1/2*y,0,0")],
    &[ (6,"1/2*y,y,-1/4"), (6,"x-1/2*y,0,0")],
    &[ (6,"1/2*y,y,1/2"), (6,"1/2*y,y,0"), (6,"x-1/2*y,0,1/2"), (6,"x-1/2*y,0,0"), (6,"1/2,0,z"), (6,"0,0,z"), (3,"1/2,0,1/2"), (3,"1/2,0,0"), (3,"0,0,1/2"), (3,"0,0,0")],
    &[],
    &[ (6,"1/2*y,y,1/4"), (6,"x-1/2*y,0,0"), (4,"1/3,-1/3,z"), (4,"0,0,z"), (2,"1/3,-1/3,-1/4"), (2,"1/3,-1/3,1/4"), (2,"0,0,1/4"), (2,"0,0,0")],
    &[ (6,"1/2*x-1/2*y,-1/2*x+1/2*y,z"), (6,"x-1/2*y,0,z"), (3,"1/2,0,z"), (2,"1/3,-1/3,z"), (1,"0,0,z")],
    &[ (6,"1/2,0,z"), (4,"1/3,-1/3,z"), (2,"0,0,z")],
    &[ (6,"x-1/2*y,0,z"), (4,"1/3,-1/3,z"), (2,"0,0,z")],
    &[ (6,"1/2*x-1/2*y,-1/2*x+1/2*y,z"), (2,"1/3,-1/3,z"), (2,"0,0,z")],
    &[ (6,"1/2*x-1/2*y,-1/2*x+1/2*y,z"), (6,"x,y,1/2"), (6,"x,y,0"), (3,"1/2*x-1/2*y,-1/2*x+1/2*y,1/2"), (3,"1/2*x-1/2*y,-1/2*x+1/2*y,0"), (2,"-1/3,1/3,z"), (2,"1/3,-1/3,z"), (2,"0,0,z"), (1,"-1/3,1/3,1/2"), (1,"-1/3,1/3,0"), (1,"1/3,-1/3,1/2"), (1,"1/3,-1/3,0"), (1,"0,0,1/2"), (1,"0,0,0")],
    &[ (6,"x,y,1/4"), (6,"1/2*x-1/2*y,-1/2*x+1/2*y,0"), (4,"-1/3,1/3,z"), (4,"1/3,-1/3,z"), (4,"0,0,z"), (2,"-1/3,1/3,1/4"), (2,"-1/3,1/3,0"), (2,"1/3,-1/3,1/4"), (2,"1/3,-1/3,0"), (2,"0,0,1/4"), (2,"0,0,0")],
    &[ (6,"x,y,1/2"), (6,"x,y,0"), (6,"x-1/2*y,0,z"), (4,"1/3,-1/3,z"), (3,"x-1/2*y,0,1/2"), (3,"x-1/2*y,0,0"), (2,"0,0,z"), (2,"1/3,-1/3,1/2"), (2,"1/3,-1/3,0"), (1,"0,0,1/2"), (1,"0,0,0")],
    &[ (6,"x,y,1/4"), (6,"x-1/2*y,0,0"), (4,"1/3,-1/3,z"), (4,"0,0,z"), (2,"-1/3,1/3,1/4"), (2,"1/3,-1/3,1/4"), (2,"0,0,1/4"), (2,"0,0,0")],
    &[ (12,"x,y,1/2"), (12,"x,y,0"), (12,"1/2*y,y,z"), (12,"x-1/2*y,0,z"), (6,"1/2*y,y,1/2"), (6,"1/2*y,y,0"), (6,"x-1/2*y,0,1/2"), (6,"x-1/2*y,0,0"), (6,"1/2,0,z"), (4,"1/3,-1/3,z"), (3,"1/2,0,1/2"), (3,"1/2,0,0"), (2,"0,0,z"), (2,"1/3,-1/3,1/2"), (2,"1/3,-1/3,0"), (1,"0,0,1/2"), (1,"0,0,0")],
    &[ (12,"x,y,0"), (12,"1/2*y,y,1/4"), (12,"x-1/2*y,0,1/4"), (12,"1/2,0,z"), (8,"1/3,-1/3,z"), (6,"1/2,0,0"), (6,"1/2,0,1/4"), (4,"0,0,z"), (4,"1/3,-1/3,0"), (4,"1/3,-1/3,1/4"), (2,"0,0,0"), (2,"0,0,1/4")],
    &[ (12,"x-1/2*y,0,z"), (12,"x,y,1/4"), (12,"1/2*y,y,0"), (8,"1/3,-1/3,z"), (6,"x-1/2*y,0,1/4"), (6,"1/2,0,0"), (4,"0,0,z"), (4,"1/3,-1/3,0"), (4,"1/3,-1/3,1/4"), (2,"0,0,0"), (2,"0,0,1/4")],
    &[ (12,"1/2*y,y,z"), (12,"x,y,1/4"), (12,"x-1/2*y,0,0"), (6,"1/2*y,y,1/4"), (6,"1/2,0,0"), (4,"1/3,-1/3,z"), (4,"0,0,z"), (2,"1/3,-1/3,-1/4"), (2,"1/3,-1/3,1/4"), (2,"0,0,1/4"), (2,"0,0,0")],
    &[ (6,"x,1/2,1/2"), (6,"x,1/2,0"), (6,"x,0,1/2"), (6,"x,0,0"), (4,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (3,"1/2,0,0"), (3,"0,1/2,1/2"), (1,"1/2,1/2,1/2"), (1,"0,0,0")],
    &[ (24,"x,1/4,1/4"), (24,"x,0,0"), (16,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (4,"-1/4,-1/4,-1/4"), (4,"1/4,1/4,1/4"), (4,"1/2,1/2,1/2"), (4,"0,0,0")],
    &[ (12,"x,1/2,0"), (12,"x,0,0"), (8,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (6,"0,1/2,1/2"), (2,"0,0,0")],
    &[ (4,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z")],
    &[ (12,"x,0,1/4"), (8,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z")],
    &[ (12,"1/2,y,z"), (12,"0,y,z"), (8,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (6,"x,1/2,1/2"), (6,"x,1/2,0"), (6,"x,0,1/2"), (6,"x,0,0"), (3,"1/2,0,0"), (3,"0,1/2,1/2"), (1,"1/2,1/2,1/2"), (1,"0,0,0")],
    &[ (12,"x,-1/4,1/4"), (12,"x,1/4,1/4"), (8,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (6,"1/4,-1/4,-1/4"), (4,"1/2,1/2,1/2"), (4,"0,0,0"), (2,"1/4,1/4,1/4")],
    &[ (48,"0,y,z"), (48,"x,1/4,1/4"), (32,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (24,"x,0,0"), (24,"0,1/4,1/4"), (8,"1/4,1/4,1/4"), (4,"1/2,1/2,1/2"), (4,"0,0,0")],
    &[ (48,"x,1/8,1/8"), (32,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (16,"1/2,1/2,1/2"), (16,"0,0,0"), (8,"-3/8,-3/8,-3/8"), (8,"1/8,1/8,1/8")],
    &[ (24,"0,y,z"), (16,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (12,"x,0,1/2"), (12,"x,0,0"), (8,"1/4,1/4,1/4"), (6,"0,1/2,1/2"), (2,"0,0,0")],
    &[ (8,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (4,"1/2,1/2,1/2"), (4,"0,0,0")],
    &[ (24,"x,0,1/4"), (16,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (8,"1/4,1/4,1/4"), (8,"0,0,0")],
    &[ (12,"1/2,1/2*y+1/2*z,1/2*y+1/2*z"), (12,"0,1/2*y+1/2*z,1/2*y+1/2*z"), (12,"x,1/2,0"), (8,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (6,"x,1/2,1/2"), (6,"x,0,0"), (3,"1/2,0,0"), (3,"0,1/2,1/2"), (1,"1/2,1/2,1/2"), (1,"0,0,0")],
    &[ (12,"1/4,1/2*y+1/2*z-1/4,1/2*y+1/2*z+1/4"), (12,"1/4,1/2*y-1/2*z+1/4,-1/2*y+1/2*z+1/4"), (12,"x,1/2,0"), (12,"x,0,1/2"), (12,"x,0,0"), (8,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (6,"1/4,1/2,0"), (6,"1/4,0,1/2"), (6,"0,1/2,1/2"), (4,"-1/4,-1/4,-1/4"), (4,"1/4,1/4,1/4"), (2,"0,0,0")],
    &[ (48,"x,1/4,1/4"), (48,"1/2,1/2*y+1/2*z,1/2*y+1/2*z"), (48,"0,1/2*y+1/2*z,1/2*y+1/2*z"), (32,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (24,"x,0,0"), (24,"0,1/4,1/4"), (8,"1/4,1/4,1/4"), (4,"1/2,1/2,1/2"), (4,"0,0,0")],
    &[ (48,"1/8,1/2*y-1/2*z+1/8,-1/2*y+1/2*z+1/8"), (48,"x,0,0"), (32,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (16,"-3/8,-3/8,-3/8"), (16,"1/8,1/8,1/8"), (8,"1/2,1/2,1/2"), (8,"0,0,0")],
    &[ (24,"1/4,1/2*y-1/2*z+1/4,-1/2*y+1/2*z+1/4"), (24,"0,1/2*y+1/2*z,1/2*y+1/2*z"), (24,"x,1/2,0"), (16,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (12,"x,0,0"), (12,"1/4,1/2,0"), (8,"1/4,1/4,1/4"), (6,"0,1/2,1/2"), (2,"0,0,0")],
    &[ (12,"1/8,1/2*y-1/2*z+1/8,-1/2*y+1/2*z+1/8"), (8,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (4,"-3/8,-3/8,-3/8"), (4,"1/8,1/8,1/8")],
    &[ (12,"1/8,1/2*y+1/2*z-1/8,1/2*y+1/2*z+1/8"), (8,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (4,"-1/8,-1/8,-1/8"), (4,"3/8,3/8,3/8")],
    &[ (24,"1/8,1/2*y-1/2*z+1/8,-1/2*y+1/2*z+1/8"), (24,"1/8,1/2*y+1/2*z-1/8,1/2*y+1/2*z+1/8"), (24,"x,0,1/4"), (16,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (12,"-3/8,0,1/4"), (12,"1/8,0,1/4"), (8,"-1/8,-1/8,-1/8"), (8,"1/8,1/8,1/8")],
    &[ (12,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (12,"x,1/2,0"), (6,"x,1/2,1/2"), (6,"x,0,0"), (4,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (3,"1/2,0,0"), (3,"0,1/2,1/2"), (1,"1/2,1/2,1/2"), (1,"0,0,0")],
    &[ (48,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (24,"x,1/4,1/4"), (24,"x,0,0"), (16,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (4,"-1/4,-1/4,-1/4"), (4,"1/4,1/4,1/4"), (4,"1/2,1/2,1/2"), (4,"0,0,0")],
    &[ (24,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (24,"x,1/2,0"), (12,"x,0,0"), (12,"1/4,1/2,0"), (8,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (6,"0,1/2,1/2"), (2,"0,0,0")],
    &[ (12,"x,0,1/2"), (12,"x,1/2,0"), (12,"x,0,0"), (8,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (6,"1/4,0,1/2"), (6,"1/4,1/2,0"), (6,"0,1/2,1/2"), (2,"0,0,0")],
    &[ (48,"x,1/4,1/4"), (48,"x,0,0"), (32,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (24,"1/4,0,0"), (24,"0,1/4,1/4"), (8,"1/4,1/4,1/4"), (8,"0,0,0")],
    &[ (24,"x,0,1/4"), (16,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (12,"-1/8,0,1/4"), (12,"3/8,0,1/4")],
    &[ (24,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (24,"1/2,y,z"), (24,"0,y,z"), (12,"1/2,1/2*y+1/2*z,1/2*y+1/2*z"), (12,"0,1/2*y+1/2*z,1/2*y+1/2*z"), (12,"x,1/2,0"), (8,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (6,"x,1/2,1/2"), (6,"x,0,0"), (3,"1/2,0,0"), (3,"0,1/2,1/2"), (1,"1/2,1/2,1/2"), (1,"0,0,0")],
    &[ (24,"1/4,1/2*y+1/2*z,1/2*y+1/2*z"), (24,"x,-1/4,1/4"), (16,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (12,"x,1/4,1/4"), (12,"0,-1/4,1/4"), (8,"0,0,0"), (6,"-1/4,1/4,1/4"), (2,"1/4,1/4,1/4")],
    &[ (24,"0,y,z"), (24,"1/4,1/2*y+1/2*z-1/4,1/2*y+1/2*z+1/4"), (16,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (12,"x,1/2,0"), (12,"x,0,1/2"), (12,"x,0,0"), (8,"1/4,1/4,1/4"), (6,"1/4,1/2,0"), (6,"1/4,0,1/2"), (6,"0,1/2,1/2"), (2,"0,0,0")],
    &[ (24,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (24,"1/2,1/2*y+1/2*z-1/4,1/2*y+1/2*z+1/4"), (24,"1/2,1/2*y-1/2*z,-1/2*y+1/2*z"), (24,"x,1/4,-1/4"), (12,"x,1/4,1/4"), (12,"1/2,1/4,-1/4"), (8,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (6,"1/4,-1/4,-1/4"), (4,"1/2,1/2,1/2"), (4,"0,0,0"), (2,"1/4,1/4,1/4")],
    &[ (96,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (96,"0,y,z"), (48,"1/2,1/2*y+1/2*z,1/2*y+1/2*z"), (48,"0,1/2*y+1/2*z,1/2*y+1/2*z"), (48,"x,1/4,1/4"), (32,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (24,"x,0,0"), (24,"0,1/4,1/4"), (8,"1/4,1/4,1/4"), (4,"1/2,1/2,1/2"), (4,"0,0,0")],
    &[ (96,"0,y,z"), (96,"1/4,1/2*y+1/2*z,1/2*y+1/2*z"), (64,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (48,"x,1/4,1/4"), (48,"x,0,0"), (24,"0,1/4,1/4"), (24,"1/4,0,0"), (8,"0,0,0"), (8,"1/4,1/4,1/4")],
    &[ (96,"0,1/2*y-1/2*z,-1/2*y+1/2*z"), (96,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (48,"x,1/8,1/8"), (32,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (16,"1/2,1/2,1/2"), (16,"0,0,0"), (8,"3/8,3/8,3/8"), (8,"1/8,1/8,1/8")],
    &[ (96,"1/4,1/2*y-1/2*z,-1/2*y+1/2*z"), (96,"x,1/8,1/8"), (64,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (48,"-1/8,1/8,1/8"), (32,"0,0,0"), (32,"1/4,1/4,1/4"), (16,"1/8,1/8,1/8")],
    &[ (48,"1/2*x+1/2*y,1/2*x+1/2*y,z"), (48,"0,y,z"), (48,"1/4,1/2*y-1/2*z+1/4,-1/2*y+1/2*z+1/4"), (24,"0,1/2*y+1/2*z,1/2*y+1/2*z"), (24,"x,0,1/2"), (16,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (12,"x,0,0"), (12,"1/4,0,1/2"), (8,"1/4,1/4,1/4"), (6,"0,1/2,1/2"), (2,"0,0,0")],
    &[ (48,"1/8,1/2*y-1/2*z+1/8,-1/2*y+1/2*z+1/8"), (48,"x,0,1/4"), (32,"1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z,1/3*x+1/3*y+1/3*z"), (24,"3/8,0,1/4"), (24,"1/8,0,1/4"), (16,"1/8,1/8,1/8"), (16,"0,0,0")],
];

const ELEMENT_SYMBOLS: &[&str] = &[
    "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mg", "Al", "Si", "P", "S", "Cl",
    "Ar", "K", "Ca", "Sc", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga", "Ge", "As",
    "Se", "Br", "Kr", "Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pd", "Ag", "Cd", "In",
    "Sn", "Sb", "Te", "I", "Xe", "Cs", "Ba", "La", "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd", "Tb",
    "Dy", "Ho", "Er", "Tm", "Yb", "Lu", "Hf", "Ta", "W", "Re", "Os", "Ir", "Pt", "Au", "Hg", "Tl",
    "Pb", "Bi", "Po", "At", "Rn", "Fr", "Ra", "Ac", "Th", "Pa", "U", "Np", "Pu", "Am", "Cm",
];
const ELEMENT_RADII: &[f64] = &[
    1.54, 1.34, 2.20, 2.19, 2.05, 1.90, 1.79, 1.71, 1.63, 1.56, 2.25, 2.40, 2.39, 2.32, 2.23, 2.14,
    2.06, 1.97, 2.34, 2.70, 2.63, 2.57, 2.52, 2.33, 2.42, 2.26, 2.22, 2.19, 2.17, 2.22, 2.33, 2.34,
    2.31, 2.24, 2.19, 2.12, 2.40, 2.79, 2.74, 2.68, 2.51, 2.44, 2.41, 2.37, 2.33, 2.15, 2.25, 2.38,
    2.46, 2.48, 2.46, 2.42, 2.38, 2.32, 2.49, 2.93, 2.84, 2.82, 2.86, 2.84, 2.83, 2.80, 2.80, 2.77,
    2.76, 2.75, 2.73, 2.72, 2.71, 2.77, 2.70, 2.64, 2.58, 2.53, 2.49, 2.44, 2.33, 2.30, 2.26, 2.29,
    2.42, 2.49, 2.50, 2.50, 2.47, 2.43, 2.58, 2.92, 2.93, 2.89, 2.85, 2.83, 2.80, 2.78, 2.76, 2.64,
];
const AMINO_ACIDS: &[&str] = &[
    "ALA", "ARG", "ASN", "ASP", "CYS", "GLN", "GLU", "GLY", "HIS", "ILE", "LEU", "LYS",
    "MET", "PHE", "PRO", "SER", "THR", "TRP", "TYR", "VAL",
];

const HERMANN_MAUGUIN_SYMBOL: &[&str] = &[
    /* 001 */ "P 1",
    /* 002 */ "P -1",
    /* 003 */ "P 1 2 1",
    /* 004 */ "P 1 21 1",
    /* 005 */ "C 1 2 1",
    /* 006 */ "P 1 m 1",
    /* 007 */ "P 1 c 1",
    /* 008 */ "C 1 m 1",
    /* 009 */ "C 1 c 1",
    /* 010 */ "P 1 2/m 1",
    /* 011 */ "P 1 21/m 1",
    /* 012 */ "C 1 2/m 1",
    /* 013 */ "P 1 2/c 1",
    /* 014 */ "P 1 21/c 1",
    /* 015 */ "C 1 2/c 1",
    /* 016 */ "P 2 2 2",
    /* 017 */ "P 2 2 21",
    /* 018 */ "P 21 21 2",
    /* 019 */ "P 21 21 21",
    /* 020 */ "C 2 2 21",
    /* 021 */ "C 2 2 2",
    /* 022 */ "F 2 2 2",
    /* 023 */ "I 2 2 2",
    /* 024 */ "I 21 21 21",
    /* 025 */ "P m m 2",
    /* 026 */ "P m c 21",
    /* 027 */ "P c c 2",
    /* 028 */ "P m a 2",
    /* 029 */ "P c a 21",
    /* 030 */ "P n c 2",
    /* 031 */ "P m n 21",
    /* 032 */ "P b a 2",
    /* 033 */ "P n a 21",
    /* 034 */ "P n n 2",
    /* 035 */ "C m m 2",
    /* 036 */ "C m c 21",
    /* 037 */ "C c c 2",
    /* 038 */ "A m m 2",
    /* 039 */ "A b m 2",
    /* 040 */ "A m a 2",
    /* 041 */ "A b a 2",
    /* 042 */ "F m m 2",
    /* 043 */ "F d d 2",
    /* 044 */ "I m m 2",
    /* 045 */ "I b a 2",
    /* 046 */ "I m a 2",
    /* 047 */ "P m m m",
    /* 048 */ "P n n n :2",
    /* 049 */ "P c c m",
    /* 050 */ "P b a n :2",
    /* 051 */ "P m m a",
    /* 052 */ "P n n a",
    /* 053 */ "P m n a",
    /* 054 */ "P c c a",
    /* 055 */ "P b a m",
    /* 056 */ "P c c n",
    /* 057 */ "P b c m",
    /* 058 */ "P n n m",
    /* 059 */ "P m m n :2",
    /* 060 */ "P b c n",
    /* 061 */ "P b c a",
    /* 062 */ "P n m a",
    /* 063 */ "C m c m",
    /* 064 */ "C m c a",
    /* 065 */ "C m m m",
    /* 066 */ "C c c m",
    /* 067 */ "C m m a",
    /* 068 */ "C c c a :2",
    /* 069 */ "F m m m",
    /* 070 */ "F d d d :2",
    /* 071 */ "I m m m",
    /* 072 */ "I b a m",
    /* 073 */ "I b c a",
    /* 074 */ "I m m a",
    /* 075 */ "P 4",
    /* 076 */ "P 41",
    /* 077 */ "P 42",
    /* 078 */ "P 43",
    /* 079 */ "I 4",
    /* 080 */ "I 41",
    /* 081 */ "P -4",
    /* 082 */ "I -4",
    /* 083 */ "P 4/m",
    /* 084 */ "P 42/m",
    /* 085 */ "P 4/n :2",
    /* 086 */ "P 42/n :2",
    /* 087 */ "I 4/m",
    /* 088 */ "I 41/a :2",
    /* 089 */ "P 4 2 2",
    /* 090 */ "P 4 21 2",
    /* 091 */ "P 41 2 2",
    /* 092 */ "P 41 21 2",
    /* 093 */ "P 42 2 2",
    /* 094 */ "P 42 21 2",
    /* 095 */ "P 43 2 2",
    /* 096 */ "P 43 21 2",
    /* 097 */ "I 4 2 2",
    /* 098 */ "I 41 2 2",
    /* 099 */ "P 4 m m",
    /* 100 */ "P 4 b m",
    /* 101 */ "P 42 c m",
    /* 102 */ "P 42 n m",
    /* 103 */ "P 4 c c",
    /* 104 */ "P 4 n c",
    /* 105 */ "P 42 m c",
    /* 106 */ "P 42 b c",
    /* 107 */ "I 4 m m",
    /* 108 */ "I 4 c m",
    /* 109 */ "I 41 m d",
    /* 110 */ "I 41 c d",
    /* 111 */ "P -4 2 m",
    /* 112 */ "P -4 2 c",
    /* 113 */ "P -4 21 m",
    /* 114 */ "P -4 21 c",
    /* 115 */ "P -4 m 2",
    /* 116 */ "P -4 c 2",
    /* 117 */ "P -4 b 2",
    /* 118 */ "P -4 n 2",
    /* 119 */ "I -4 m 2",
    /* 120 */ "I -4 c 2",
    /* 121 */ "I -4 2 m",
    /* 122 */ "I -4 2 d",
    /* 123 */ "P 4/m m m",
    /* 124 */ "P 4/m c c",
    /* 125 */ "P 4/n b m :2",
    /* 126 */ "P 4/n n c :2",
    /* 127 */ "P 4/m b m",
    /* 128 */ "P 4/m n c",
    /* 129 */ "P 4/n m m :2",
    /* 130 */ "P 4/n c c :2",
    /* 131 */ "P 42/m m c",
    /* 132 */ "P 42/m c m",
    /* 133 */ "P 42/n b c :2",
    /* 134 */ "P 42/n n m :2",
    /* 135 */ "P 42/m b c",
    /* 136 */ "P 42/m n m",
    /* 137 */ "P 42/n m c :2",
    /* 138 */ "P 42/n c m :2",
    /* 139 */ "I 4/m m m",
    /* 140 */ "I 4/m c m",
    /* 141 */ "I 41/a m d :2",
    /* 142 */ "I 41/a c d :2",
    /* 143 */ "P 3",
    /* 144 */ "P 31",
    /* 145 */ "P 32",
    /* 146 */ "R 3 :h",
    /* 147 */ "P -3",
    /* 148 */ "R -3 :h",
    /* 149 */ "P 3 1 2",
    /* 150 */ "P 3 2 1",
    /* 151 */ "P 31 1 2",
    /* 152 */ "P 31 2 1",
    /* 153 */ "P 32 1 2",
    /* 154 */ "P 32 2 1",
    /* 155 */ "R 3 2 :h",
    /* 156 */ "P 3 m 1",
    /* 157 */ "P 3 1 m",
    /* 158 */ "P 3 c 1",
    /* 159 */ "P 3 1 c",
    /* 160 */ "R 3 m :h",
    /* 161 */ "R 3 c :h",
    /* 162 */ "P -3 1 m",
    /* 163 */ "P -3 1 c",
    /* 164 */ "P -3 m 1",
    /* 165 */ "P -3 c 1",
    /* 166 */ "R -3 m :h",
    /* 167 */ "R -3 c :h",
    /* 168 */ "P 6",
    /* 169 */ "P 61",
    /* 170 */ "P 65",
    /* 171 */ "P 62",
    /* 172 */ "P 64",
    /* 173 */ "P 63",
    /* 174 */ "P -6",
    /* 175 */ "P 6/m",
    /* 176 */ "P 63/m",
    /* 177 */ "P 6 2 2",
    /* 178 */ "P 61 2 2",
    /* 179 */ "P 65 2 2",
    /* 180 */ "P 62 2 2",
    /* 181 */ "P 64 2 2",
    /* 182 */ "P 63 2 2",
    /* 183 */ "P 6 m m",
    /* 184 */ "P 6 c c",
    /* 185 */ "P 63 c m",
    /* 186 */ "P 63 m c",
    /* 187 */ "P -6 m 2",
    /* 188 */ "P -6 c 2",
    /* 189 */ "P -6 2 m",
    /* 190 */ "P -6 2 c",
    /* 191 */ "P 6/m m m",
    /* 192 */ "P 6/m c c",
    /* 193 */ "P 63/m c m",
    /* 194 */ "P 63/m m c",
    /* 195 */ "P 2 3",
    /* 196 */ "F 2 3",
    /* 197 */ "I 2 3",
    /* 198 */ "P 21 3",
    /* 199 */ "I 21 3",
    /* 200 */ "P m -3",
    /* 201 */ "P n -3 :2",
    /* 202 */ "F m -3",
    /* 203 */ "F d -3 :2",
    /* 204 */ "I m -3",
    /* 205 */ "P a -3",
    /* 206 */ "I a -3",
    /* 207 */ "P 4 3 2",
    /* 208 */ "P 42 3 2",
    /* 209 */ "F 4 3 2",
    /* 210 */ "F 41 3 2",
    /* 211 */ "I 4 3 2",
    /* 212 */ "P 43 3 2",
    /* 213 */ "P 41 3 2",
    /* 214 */ "I 41 3 2",
    /* 215 */ "P -4 3 m",
    /* 216 */ "F -4 3 m",
    /* 217 */ "I -4 3 m",
    /* 218 */ "P -4 3 n",
    /* 219 */ "F -4 3 c",
    /* 220 */ "I -4 3 d",
    /* 221 */ "P m -3 m",
    /* 222 */ "P n -3 n :2",
    /* 223 */ "P m -3 n",
    /* 224 */ "P n -3 m :2",
    /* 225 */ "F m -3 m",
    /* 226 */ "F m -3 c",
    /* 227 */ "F d -3 m :2",
    /* 228 */ "F d -3 c :2",
    /* 229 */ "I m -3 m",
    /* 230 */ "I a -3 d",
];
