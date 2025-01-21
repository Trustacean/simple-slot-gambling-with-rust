pub enum Symbol {
    Cherry,
    Lemon,
    Grape,
    Star,
    Bell,
    Diamond,
    Seven,
}

impl Symbol {
    pub fn to_string(&self) -> &str {
        match self {
            Symbol::Cherry => "🍒",
            Symbol::Lemon => "🍋",
            Symbol::Grape => "🍇",
            Symbol::Star => "⭐",
            Symbol::Bell => "🔔",
            Symbol::Diamond => "💎",
            Symbol::Seven => "7️⃣",
        }
    }
}