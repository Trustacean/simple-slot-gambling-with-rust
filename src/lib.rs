#[derive(Debug, Clone, Copy)]
pub enum Symbols {
    Diamond,
    Clover,
    Lemon,
    Cherry,
    Orange,
}

pub struct Outcome {
    pub symbol: Symbols,
    pub display: String,
    pub weight: u32,
    pub payout: u32,
}

impl Outcome {
    pub fn new(symbol: Symbols, weight: u32, payout: u32) -> Self {
        let display = match symbol {
            Symbols::Diamond => "💎",
            Symbols::Clover => "🍀",
            Symbols::Lemon => "🍋",
            Symbols::Cherry => "🍒",
            Symbols::Orange => "🍊",
        };

        Self {
            symbol,
            display: display.to_string(),
            weight,
            payout,
        }
    }

    pub fn get_symbol(&self) -> &Symbols {
        &self.symbol
    }

    pub fn display(&self) -> &str {
        &self.display
    }

    pub fn get_weight(&self) -> u32 {
        self.weight
    }

    pub fn get_payout(&self) -> u32 {
        self.payout
    }

    pub fn set_weight(&mut self, weight: u32) {
        self.weight = weight;
    }
}
