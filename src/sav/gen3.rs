use std::collections::HashMap;

pub struct Save {
    save: Vec<u8>,
    hof: Vec<u8>,
    mg: Vec<u8>,
    rb: Vec<u8>
}

impl Save {
    fn read_file(file: Vec<u8>) -> Save {
        let block = Self::get_recent_slot(file);

        Save {
            save: Vec::new(),
            hof: Vec::new(),
            mg: Vec::new(),
            rb: Vec::new()
        }
    }

    fn get_recent_slot(file: Vec<u8>) -> Vec<u8> {
        let a = u32::from_le_bytes(*file.clone()[57340..57344]);
        let b = u32::from_le_bytes(*file.clone()[(57344 * 2)-4..(57344 * 2)]);

        if a > b {
            *file.clone()[0..57344]
        } else {
            *file.clone()[57344..(57344 * 2)]
        }
    }
}

struct Section<T> {
    data: T,
    id: u8,
    checksum: u32,
    signature: Vec<u8>,
    index: u32
}

impl<T> Section<T> {
    fn new(data: T, id: u8, checksum: u32, signature: Vec<u8>, index: u32) -> Section<T> {
        match {

        }
    }

    fn get_section_raw_data(data: Vec<u8>, id: u8) -> Vec<u8> {
        match id {
            0  => *data.clone()[0..3884],
            4  => *data.clone()[0..3848],
            13 => *data.clone()[0..2000],
            _  => *data.clone()[0..3968],
        }
    }

    fn get_section_data(data: Vec<u8>, id: u8) -> T {
        match id {
            0 => TrainerInfo::new_trainer_info(data),
            1 => Bag::new_bag(data),
            2 => GameState::new(data),
            3 => GameSpecific::new(data),
            4 =>
        }
    }
}

struct TrainerInfo {
    name: String,
    gender: char,
    id: HashMap<String, u16>,
    time: String,
    options: Vec<u8>,
    game_code: Option<u8>,
    security: Option<u32>
}

impl TrainerInfo {
    fn new_trainer_info(data: Vec<u8>) -> TrainerInfo {

    }
}

struct Bag {
    team_size: u8,
    team: Vec<u8>, /// TODO: Change
    money: u8,
    coins: u8,
    pc_items: Vec<(u32, u32)>,
    bag_items: Vec<(u32, u32)>,
    key_items: Vec<(u32, u32)>,
    balls: Vec<(u32, u32)>,
    tms: Vec<(u32, u32)>,
    berries: Vec<(u32, u32)>
}

impl Bag {
    fn new_bag(data: Vec<u8>) -> Bag {

    }
}

struct GameState {
    data: Vec<u8>,
}

impl GameState {
    fn new(data: Vec<u8>) -> GameState {
        GameState { data }
    }
}

struct GameSpecific {
    data: Vec<u8>,
}

impl GameSpecific {
    fn new(data: Vec<u8>) -> GameSpecific {
        GameSpecific { data }
    }
}

struct RivalInfo {
    data: Vec<u8>
}

impl RivalInfo {
    fn new(data: Vec<u8>) -> RivalInfo {
        RivalInfo { data }
    }
}

struct PCBox {
    pokemon: Vec<u8>, /// TODO: Change
    current: bool,
    name: String,
    wallpaper: u8
}

impl PCBox {}

struct PokemonFull {

}

impl PokemonFull {}


struct PokemonPC {

}

impl PokemonPC {}

struct PokemonStats<T> {
    stats: Vec<T>
}

impl<T> PokemonStats<T> {

}

struct Growth {
    species: u16,
    item: u16,
    experience: u8,
    pp_bonuses: Vec<u8>,
    friendship: u8
}

impl Growth {}

struct Attacks {
    move1: u16,
    move2: u16,
    move3: u16,
    move4: u16,
    pp1: u8,
    pp2: u8,
    pp3: u8,
    pp4: u8
}

impl Attacks {}

struct EVs {
    hp: u8,
    attack: u8,
    defense: u8,
    speed: u8,
    sp_attack: u8,
    sp_defense: u8,
    coolness: u8,
    beauty: u8,
    cuteness: u8,
    smartness: u8,
    toughness: u8,
    feel: u8
}

impl EVs {}

struct Misc {
    pokerus: u8,
    met: u16,
    origin: u8,
    ivs: HashMap<String, u8>,
    egg: u8,
    ability: u8
}

impl Misc {}

fn byte_to_char_map() -> HashMap<u8, char> {
    return HashMap::from([
        (0xA1, '0'),
        (0xA2, '1'),
        (0xA3, '2'),
        (0xA4, '3'),
        (0xA5, '4'),
        (0xA6, '5'),
        (0xA7, '6'),
        (0xA8, '7'),
        (0xA9, '8'),
        (0xAA, '9'),
        (0xAB, '!'),
        (0xAC, '?'),
        (0xAD, '.'),
        (0xAE, '-'),
        (0xB0, '‥'),
        (0xB1, '“'),
        (0xB2, '”'),
        (0xB3, '‘'),
        (0xB4,'\''),
        (0xB5, '♂'),
        (0xB6, '♀'),
        (0xB8, ')'),
        (0xBA, '/'),
        (0xBB, 'A'),
        (0xBC, 'B'),
        (0xBD, 'C'),
        (0xBE, 'D'),
        (0xBF, 'E'),
        (0xC0, 'F'),
        (0xC1, 'G'),
        (0xC2, 'H'),
        (0xC3, 'I'),
        (0xC4, 'J'),
        (0xC5, 'K'),
        (0xC6, 'L'),
        (0xC7, 'M'),
        (0xC8, 'N'),
        (0xC9, 'O'),
        (0xCA, 'P'),
        (0xCB, 'Q'),
        (0xCC, 'R'),
        (0xCD, 'S'),
        (0xCE, 'T'),
        (0xCF, 'U'),
        (0xD0, 'V'),
        (0xD1, 'W'),
        (0xD2, 'X'),
        (0xD3, 'Y'),
        (0xD4, 'Z'),
        (0xD5, 'a'),
        (0xD6, 'b'),
        (0xD7, 'c'),
        (0xD8, 'd'),
        (0xD9, 'e'),
        (0xDA, 'f'),
        (0xDB, 'g'),
        (0xDC, 'h'),
        (0xDD, 'i'),
        (0xDE, 'j'),
        (0xDF, 'k'),
        (0xE0, 'l'),
        (0xE1, 'm'),
        (0xE2, 'n'),
        (0xE3, 'o'),
        (0xE4, 'p'),
        (0xE5, 'q'),
        (0xE6, 'r'),
        (0xE7, 's'),
        (0xE8, 't'),
        (0xE9, 'u'),
        (0xEA, 'v'),
        (0xEB, 'w'),
        (0xEC, 'x'),
        (0xED, 'y'),
        (0xEE, 'z'),
        (0xF1, 'Ä'),
        (0xF2, 'Ö'),
        (0xF3, 'Ü'),
        (0xF4, 'ä'),
        (0xF5, 'ö'),
        (0xF6, 'ü'),
    ])
}