use colored::Colorize;

#[derive(Clone)]
pub enum Terrain {
    Player,
    Wall,
    Floor,
    Target,
}

impl std::fmt::Display for Terrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Terrain::Player => write!(f, "{}", "██".cyan()),
            Terrain::Wall => write!(f, "{}", "██".white()),
            Terrain::Floor => write!(f, "{}", "██".bright_black()),
            Terrain::Target => write!(f, "{}", "██".yellow()),
        }
    }
}
