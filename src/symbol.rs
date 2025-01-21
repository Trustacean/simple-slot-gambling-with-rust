pub enum Symbol {
    Cherry,
    Lemon,
    Orange,
    Plum,
    Bell,
    Bar,
    Seven,
}

impl Symbol {
    pub fn to_string(&self) -> &str {
        match self {
            Symbol::Cherry => "🍒",
            Symbol::Lemon => "🍋",
            Symbol::Orange => "🍊",
            Symbol::Plum => "🍇",
            Symbol::Bell => "🔔",
            Symbol::Bar => "🍫",
            Symbol::Seven => "7️⃣"
        }
    }
}