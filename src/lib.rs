/// The list object returned from scryfall
pub struct List {
    pub total_cards : u32,
    pub has_more    : bool,

    /// Contains the link to the next result page
    /// 
    /// Is [`Some`] if `Self.has_more` is `true`
    pub next_page   : Option<String>,
    pub data        : Vec<Card>
}


/// Data for a single card returned from scryfall
pub struct Card {
    pub id: String,
    pub oracle_id: String,
    pub multiverse_ids: Vec<String>,
    pub mtgo_id: u32,
    pub arena_id: u32,
    pub tcgplayer_id: u32,
    pub cardmarket_id: u32,
    pub name: String,
    pub lang: String,
    pub released_at: String,
    pub uri: String,
    pub scryfall_uri: String,
    pub layout: String,
    pub highres_image: bool,
    pub image_status: String,
    pub image_updated_at: String,
    pub image_uris: ImageUris,
    pub mana_cost: ManaCost,
    pub cmc: u8,
    pub type_line: String,
    pub oractle_text: String,
    pub colors: Vec<Color>,
    pub color_idenity: Vec<Color>,
    pub keywords: Vec<String>,
    pub legalities: Legalities,
    pub games: Vec<String>,
    pub reserved: bool,
    pub game_changer: bool,
    pub foil: bool,
    pub nonfoil: bool,
    pub finishes: Vec<String>,
    pub oversized: bool,
    pub promo: bool,
    pub variation: bool,
    pub set_id: String,
    pub set: String,
    pub set_name: String,
    pub set_type: String,
    pub set_url: String,
    pub set_search_url: String,
    pub scryfall_set_uri: String,
    pub rulings_uri: String,
    pub prints_search_uri: String,
    pub collector_number: String,
    pub digital: bool,
    pub rarity: String,
    pub flavor_text: String,
    pub card_back_id: String,
    pub artist: String,
    pub artist_ids: Vec<String>,
    pub illustration_id: String,
    pub border_color: String,
    pub frame: String,
    pub full_art: bool,
    pub textless: bool,
    pub booster: bool,
    pub story_spotlight: bool,
    pub edhrec_rank: u32,
    pub preview: PreviewData,
    pub prices: Prices,
    pub related_uris: RelatedUris,
    pub purchase_uris: PurchaseUris,
}

pub struct ImageUris {
    pub small       : String,
    pub normal      : String,
    pub large       : String,
    pub png         : String,
    pub art_crop    : String,
    pub border_crop : String,
}

pub struct ManaCost {
    pub white   : Option<u8>,
    pub black   : Option<u8>,
    pub red     : Option<u8>,
    pub green   : Option<u8>,
    pub blue    : Option<u8>,
}

pub enum Color {
    White,
    Black,
    Red,
    Green,
    Blue
}

pub struct Legalities {
    pub standard            : bool,
    pub future              : bool,
    pub historic            : bool,
    pub timeless            : bool,
    pub gladiator           : bool,
    pub pioneer             : bool,
    pub modern              : bool,
    pub legacy              : bool,
    pub pauper              : bool,
    pub vintage             : bool,
    pub penny               : bool,
    pub commander           : bool,
    pub oathbreaker         : bool,
    pub standardbrawl       : bool,
    pub brawl               : bool,
    pub competitivebrawl    : bool,
    pub alchemy             : bool,
    pub paupercommander     : bool,
    pub duel                : bool,
    pub oldschool           : bool,
    pub premodern           : bool,
    pub predh               : bool,
    pub tlr                 : bool,
}

pub struct PreviewData {
    pub source          : String,
    pub source_url      : String,
    pub previewed_at    : String,
}

pub struct Prices {
    pub usd         : Option<f32>,
    pub usd_foil    : Option<f32>,
    pub eur         : Option<f32>,
    pub eur_foil    : Option<f32>,
    pub tix         : Option<f32>  
}

pub struct RelatedUris {
    pub tcgplayer_infinite_articles : String,
    pub tcgplayer_infinite_decks    : String,
    pub edhrec                      : String,
}

pub struct PurchaseUris {
    pub tcgplayer   : String,
    pub cardmarket  : String,
    pub cardhoarder : String
}