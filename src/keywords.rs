use phf::phf_map;
use crate::token::TokenType::{ self, * };

pub static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "nest" => NEST,
    "of" => OF,
    "Leatherback" => LEATHERBACK,
    "Green" => GREEN,
    "turtles" => TURTLES,
    "~~~" => SHORELINE,
    "---" => HORIZON,
};
