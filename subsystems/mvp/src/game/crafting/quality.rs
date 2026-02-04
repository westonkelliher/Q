/// Quality tiers for tools and items
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
pub enum Quality {
    Makeshift = 0,
    Crude = 1,
    Common = 2,
    Uncommon = 3,
    Rare = 4,
    Epic = 5,
    Legendary = 6,
}
