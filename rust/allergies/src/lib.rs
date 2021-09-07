pub struct Allergies(u32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

const ALL_ALLERGENS: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

impl Allergen {
    pub fn score(&self) -> u32 {
        *self as u32
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        &self.0 & allergen.score() != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALL_ALLERGENS
            .iter()
            .filter(|&a| self.is_allergic_to(a))
            .copied()
            .collect()
    }
}
