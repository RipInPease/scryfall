pub struct URI;
pub struct UUID;


/// Color of mana
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless
}

/// The mana value for each color
pub struct ManaValue {
    pub white       : Option<u8>,
    pub blue        : Option<u8>,
    pub black       : Option<u8>,
    pub red         : Option<u8>,
    pub green       : Option<u8>,
    pub colorless   : Option<u8>
}

pub enum Language {
    /// English
    EN,
    /// Spanish
    ES,
    /// French
    FR,
    /// German
    DE,
    /// Italian
    IT,
    /// Japanese
    PT,
    /// Portuguese
    JA,
    /// Korean
    KO,
    /// Russian
    RU,
    /// Simplified chinese
    ZHS,
    /// Traditional chinese
    ZHT,
    /// Hebrew
    HE,
    /// Latin
    LA,
    /// Ancient greek
    GRC,
    /// Arabic
    AR,
    /// Sanskrit
    SA,
    /// Phyrexian
    PH,
    /// Quenya
    QYA
}

/// The layout property categorizes the arrangement of card parts, 
/// faces, and other bounded regions on cards. 
/// 
/// The layout can be used to programmatically determine 
/// which other properties on a card you can expect
pub enum Layout {
    /// A standard Magic card with one face
    Normal,
    /// A split-faced card
    Split,
    /// Cards that invert vertically with the flip keyword
    Flip,
    /// Double-sided cards that transform
    Transform,
    /// Double-sided cards that can be played either-side
    ModalDFC,
    /// Cards with meld parts printed on the back
    Meld,
    /// Cards with Level Up
    Leveler,
    /// Class-type enchantment cards
    Class,
    /// Case-type enchantment cards
    Case,
    /// Saga-type cards
    Saga,
    /// Cards with an Adventure spell part
    Adventure,
    /// Cards with a prepared spell part
    Prepare,
    /// Cards with Mutate
    Mutate,
    /// Cards with Prototype
    Prototype,
    /// Battle-type cards
    Battle,
    /// Plane and Phenomenon-type cards
    Planar,
    /// Scheme-type cards
    Scheme,
    /// Vanguard-type cards
    Vanguard,
    /// Token cards
    Token,
    /// Tokens with another token printed on the back
    DoubleFacedToken,
    /// Emblem cards
    Emblem,
    /// Cards with Augment
    Augment,
    /// Host-type cards
    Host,
    /// Art Series collectable double-faced cards
    ArtSeries,
    /// A Magic card with two sides that are unrelated
    ReversibleCard
}

/// An object providing URIs to imagery for this face
pub struct ImageURIs {
    pub small       : Option<URI>,
    pub normal      : Option<URI>,
    pub large       : Option<URI>,
    pub png         : Option<URI>,
    pub art_crop    : Option<URI>,
    pub border_crop : Option<URI>,
    pub thumb       : Option<URI>,
    pub grid        : Option<URI>,
    pub display     : Option<URI>,
}

/// Multiface cards have a card_faces property 
/// containing at least two [`CardFace`] objects. 
/// Those objects have these properties
pub struct CardFace {
    /// The name of the illustrator of this card face. 
    /// Newly spoiled cards may not have this field yet
    pub artist          : Option<String>,

    /// The ID of the illustrator of this card face. 
    /// Newly spoiled cards may not have this field yet
    pub artist_id       : Option<UUID>,

    /// The mana value of this particular face, 
    /// if the card is reversible
    pub cmc             : Option<f32>,

    /// The colors in this face’s color indicator, if any.
    pub color_indicator : Option<Vec<Color>>,

    /// This face’s colors, if the game defines colors for the 
    /// individual face of this card
    pub colors          : Option<Vec<Color>>,

    /// This face’s defense, if any
    pub defense         : Option<String>,

    /// The flavor text printed on this face, if any
    pub flavor_text     : Option<String>,

    /// A unique identifier for the card face artwork 
    /// that remains consistent across reprints. 
    /// Newly spoiled cards may not have this field yet.
    pub illustration_id : Option<UUID>,

    /// An object providing URIs to imagery for this face, 
    /// if this is a double-sided card. 
    /// If this card is not double-sided, 
    /// then the image_uris property will be part of the 
    /// parent object instead
    pub image_uruis     : Option<ImageURIs>,

    /// The [`Layout`] of this card face, 
    /// if the card is reversible
    pub layout          : Option<Layout>,

    /// This face’s loyalty, if any
    pub loyalty         : Option<String>,

    /// The mana cost for this face. 
    /// This value will be any empty string "" 
    /// if the cost is absent. 
    /// Remember that per the game rules, a missing mana cost 
    /// and a mana cost of {0} are different values
    pub mana_cost           : String,

    /// The name of this particular face
    pub name                : String,

    /// The Oracle ID of this particular face, 
    /// if the card is reversible
    pub oracle_id           : Option<UUID>,

    /// The Oracle text for this face, if any
    pub oracle_text         : Option<String>,

    /// This face’s power, if any. 
    /// Note that some cards have powers that are not numeric, 
    /// such as *
    pub power               : Option<String>,

    /// The localized name printed on this face, if any
    pub printed_name        : Option<String>,

    /// The localized text printed on this face, if any
    pub printed_text        : Option<String>,

    /// The localized type line printed on this face, if any
    pub printed_type_line   : Option<String>,

    /// This face’s toughness, if any
    pub toughness           : Option<String>,

    /// The type line of this particular face, 
    /// if the card is reversible
    pub type_line           : Option<String>,

    /// The watermark on this particulary card face, if any
    pub watermark           : Option<String>
}

/// Cards that are closely related to other cards 
/// (because they call them by name, or generate a token, 
/// or meld, etc) have a all_parts property that contains 
/// Related Card objects. 
/// Those objects have the following properties
pub struct RelatedCard {
    /// An unique ID for this card in Scryfall’s database
    pub id          : UUID,
    
    /// A field explaining what role this card plays in this 
    /// relationship, one of token, meld_part, meld_result, 
    /// or combo_piece
    pub component   : String,

    /// The name of this particular related card
    pub name        : String,

    /// The type line of this card
    pub type_line   : String,

    /// A URI where you can retrieve a full object 
    /// describing this card on Scryfall’s API
    pub uri         : URI
}

/// How legal a card is in a single format
pub enum Legality {
    Legal,
    NotLegal,
    Restricted,
    Banned,
}

/// How legal a card is in all formats
pub struct Legalities {
    pub standard            : Legality,
    pub future              : Legality,
    pub historic            : Legality,
    pub timeless            : Legality,
    pub gladiator           : Legality,
    pub pioneer             : Legality,
    pub modern              : Legality,
    pub legacy              : Legality,
    pub pauper              : Legality,
    pub vintage             : Legality,
    pub penny               : Legality,
    pub commander           : Legality,
    pub oathbreaker         : Legality,
    pub standardbrawl       : Legality,
    pub brawl               : Legality,
    pub competitivebrawl    : Legality,
    pub alchemy             : Legality,
    pub paupercommander     : Legality,
    pub duel                : Legality,
    pub oldschool           : Legality,
    pub premodern           : Legality,
    pub predh               : Legality,
    pub tlr                 : Legality
}

/// The color a card's border can be
pub enum BorderColor {
    Black,
    White,
    Borderless,
    Yellow,
    Silver,
    Gold
}

/// A finish a card can come in
pub enum Finish {
    Foil,
    Nonfoil,
    Etched
} 

/// All finishes a card can come in
pub struct Finishes {
    pub foil    : bool,
    pub nonfoil : bool,
    pub etched  : bool
}

/// Tracis additional frame artwork applied 
/// over a particular frame
pub struct FrameEffects {
    pub legendary               : bool,    
    pub miracle                 : bool,
    pub enchantment             : bool,    
    pub draft                   : bool,
    pub devoid                  : bool,
    pub tombstone               : bool,    
    pub colorshifted            : bool,        
    pub inverted                : bool,    
    pub sunmoondfc              : bool,    
    pub compasslanddfc          : bool,        
    pub originpwdfc             : bool,    
    pub mooneldrazidfc          : bool,        
    pub waxingandwaningmoondfc  : bool,                
    pub showcase                : bool,    
    pub extendedart             : bool,    
    pub companion               : bool,    
    pub etched                  : bool,
    pub snow                    : bool,
    pub lesson                  : bool,
    pub shatteredglass          : bool,        
    pub convertdfc              : bool,    
    pub fandfc                  : bool,
    pub upsidedowndfc           : bool,    
    pub spree                   : bool
}

/// The frame field tracks the edition of the card 
/// frame of used for the re/print in question
pub enum Frame {
    /// 1993
    Original,
    /// 1997
    UpdatedClassic,
    /// 2003
    Modern,
    /// 2015
    HoloFoilStamp,
    /// The frame used on cards from the future
    Future,
}

/// All games a card's print is available in
pub struct Games {
    pub paper   : bool,
    pub arena   : bool,
    pub astral  : bool,
    pub sega    : bool,
}

/// The state of this card's image
pub enum ImageStatus {
    Missing,
    Placeholder,
    Lowres,
    HighRes
}

/// Daily price information for this card
pub struct Prices {
    pub usd         : Option<String>,
    pub usd_foil    : Option<String>,
    pub usd_etched  : Option<String>,
    pub eur         : Option<String>,
    pub eur_foil    : Option<String>,
    pub tix         : Option<String>
}

/// URIs to this card’s listing on major marketplaces
pub struct PurchaseURIs {
    pub tcgplayer       : Option<URI>,
    pub cardmarket      : Option<URI>,
    pub cardhoarder     : Option<URI>,
}

/// A card's rarity
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Special,
    Mythic,
    Bonus
}

/// URIs to this card’s listing on other 
/// Magic: The Gathering online resources
pub struct RelatedURIs {
    pub tcgplayer_infinite_articles : Option<URI>,
    pub tcgplayer_infinite_decks    : Option<URI>,
    pub edhrec                      : Option<URI>
}

/// The security stamp on this card
pub enum SecurityStamp {
    Oval,
    Triangle,
    Acorn,
    Circle,
    Arena,
    Heart,
    None
}

pub struct List {
    pub has_more    : bool,
    pub data        : Vec<Data>,
    pub next_page   : Option<URI>,

    /// If this is a list of card object this field will 
    /// be [`Some`]
    pub total_cards : Option<i32>,

    /// Human readable warning returned to your request
    pub warnings    : Option<String>
}

/// The different kinds of data a [`List`] can contain
pub enum Data {
    Cards,
}

pub struct Card {
    pub core_fields : CardCore, 
    pub gameplay    : CardGameplay,
    pub print_fields: CardPrint
}

/// Cards have these core properties
pub struct CardCore {
    /// This card's arena ID, if any
    pub arena_id        : Option<i32>,

    /// A unique ID for this card in Scryfall’s database
    pub id                  : UUID,

    /// A [`Language`] code for this printing
    pub lang                : Language,
    
    /// This card's Magic Online ID, if any
    pub mtgo_id             : Option<i32>,

    /// This card's foil Magic Online ID, if any
    pub mtgo_foil_id        : Option<i32>,

    /// This card's multiverse IDs on Gatherer, if any
    pub multiverse_ids      : Option<Vec<String>>,

    /// This card’s Resource ID on Gatherer, if any
    pub resource_id         : Option<String>,

    /// This card’s ID on TCGplayer’s API, also known as the productId
    pub tcgplayer_id        : Option<i32>,

    /// This card’s ID on TCGplayer’s API, 
    /// for its etched version if that version is a separate product
    pub tcgplayer_etched_id : Option<i32>,

    /// This card’s ID on Cardmarket’s API, also known as the idProduct
    pub cardmarket_id       : Option<i32>,

    /// A code for this card’s [`Layout`].
    pub layout              : Layout,

    /// A unique ID for this card’s oracle identity
    pub oracle_id           : Option<UUID>,

    /// A link to where you can begin paginating 
    /// all re/prints for this card on Scryfall’s API
    pub prints_search_uri   : URI,

    ///  link to this card’s rulings list on Scryfall’s API
    pub rulings_uri         : URI,

    /// A link to this card’s permapage on Scryfall’s website
    pub scryfall_uri        : URI,

    /// A link to this card object on Scryfall’s API
    pub uri                 : URI,
}

/// Cards have these properties relevant to the game rules
pub struct CardGameplay {
    /// If this card is closely related to other cards, 
    /// this property will be an array with [`RelatedCard`]
    pub all_parts           : Option<Vec<RelatedCard>>,

    /// An array of Card Face objects, 
    /// if this card is multifaced
    pub card_faces          : Option<Vec<CardFace>>,

    /// The card’s mana value. 
    /// Note that some funny cards have fractional mana costs
    pub cmc                 : f32,

    /// This card’s color identity
    pub color_identity      : Vec<Color>,

    /// The colors in this card’s color indicator, if any. 
    /// A `None` value for this field indicates the card 
    /// does not have one
    pub color_indicator     : Option<Vec<Color>>,

    /// This card’s colors, if the overall card has colors 
    /// defined by the rules. 
    /// Otherwise the colors will be on the [`CardFace`] objects
    pub colors              : Option<Vec<Color>>,

    /// This face’s defense, if any
    pub defense             : Option<String>,

    /// This card’s overall rank/popularity on EDHREC. 
    /// Not all cards are ranked
    pub edhrec_rank         : Option<i32>,

    /// True if this card is on the Commander Game Changer list
    pub game_changes        : Option<bool>,

    /// This card’s hand modifier, if it is Vanguard card. 
    /// This value will contain a delta, such as -1
    pub hand_modifier       : Option<String>,

    /// An array of keywords that this card uses, 
    /// such as 'Flying' and 'Cumulative upkeep'
    pub keywords            : Vec<String>,

    /// An object describing the legality of this card 
    /// across play formats. 
    /// Possible legalities are legal, not_legal, restricted, 
    /// and banned
    pub legalities          : Legalities,

    /// This card’s life modifier, if it is Vanguard card. 
    /// This value will contain a delta, such as +2
    pub life_modifier       : Option<String>,

    /// This loyalty if any. 
    /// Note that some cards have loyalties that are not numeric,
    /// such as X
    pub loyalty             : Option<String>,

    /// The mana cost for this card. 
    /// This value will be any empty string "" if the cost is 
    /// absent. 
    /// Remember that per the game rules, a missing mana cost 
    /// and a mana cost of {0} are different values. 
    /// Multi-faced cards will report this value in [`CardFace`]
    pub mana_cost           : Option<String>,

    /// The name of this card. 
    /// If this card has multiple faces, 
    /// this field will contain both names separated by ␣//␣
    pub name                : String,

    /// The Oracle text for this card, if any
    pub oracle_text         : Option<String>,

    /// This card’s rank/popularity on Penny Dreadful. 
    /// Not all cards are ranked
    pub penny_rank          : Option<i32>,

    /// This card’s power, if any. 
    /// Note that some cards have powers that are not numeric, 
    /// such as *
    pub power               : Option<String>,

    /// Colors of mana that this card could produce
    pub produced_mana       : Option<Vec<Color>>,

    /// True if this card is on the Reserved List
    pub reserved            : bool,

    /// This card’s toughness, if any. 
    /// Note that some cards have toughnesses that are 
    /// not numeric, such as *
    pub toughness           : Option<String>,

    /// The type line of this card
    pub type_line           : String,
}

/// Cards have the following properties 
/// unique to their particular re/print
pub struct CardPrint {
    /// The name of the illustrator of this card. 
    /// Newly spoiled cards may not have this field yet
    pub artist              : Option<String>,

    /// The IDs of the artists that illustrated this card. 
    /// Newly spoiled cards may not have this field yet
    pub artist_ids          : Option<Vec<UUID>>,

    /// The lit Unfinity attractions lights on this card, if any
    pub attraction_lights   : Option<Vec<i32>>,

    /// Whether this card is found in boosters
    pub booster             : bool,

    /// This card’s [`BorderColor`]
    pub border_color        : BorderColor,

    /// The Scryfall ID for the card back design 
    /// present on this card
    pub card_back_id        : UUID,

    /// This card’s collector number. 
    /// Note that collector numbers can contain non-numeric 
    /// characters, such as letters or ★.
    pub collector_number    : String,

    /// True if you should consider avoiding use of 
    /// this print downstream
    pub content_warning     : Option<bool>,

    /// True if this card was only released in a video game
    pub digital             : bool,

    /// All [`Finishes`] a card can come in
    pub finishes            : Finishes,

    /// The just-for-fun name printed on the card 
    /// (such as for Godzilla series cards)
    pub flavor_name         : Option<String>,

    /// The flavor text, if any
    pub flavor_text         : Option<String>,

    /// This card’s [`FrameEffects`]
    pub frame_effects       : FrameEffects,

    /// The card's [`Frame`] layout
    pub frame               : Frame,

    /// True if this card’s artwork is larger than normal
    pub full_art            : bool,

    /// A list of [`Games`] that this card print is available in
    pub games               : Games,

    /// True if this card’s imagery is high resolution
    pub highres_image       : bool,

    /// A unique identifier for the card artwork that 
    /// remains consistent across reprints. 
    /// Newly spoiled cards may not have this field yet
    pub illustration_id     : Option<UUID>,

    /// The [`ImageStatus`] of this card's image
    pub image_status        : ImageStatus,

    /// An object providing URIs to imagery for this face, 
    /// if this is a double-sided card. 
    /// If this card is not double-sided, 
    /// then the image_uris property will be part of the 
    /// parent object instead
    pub image_uris          : Option<ImageURIs>,

    /// True if this card is oversized
    pub oversized           : bool,

    /// Daily [`Prices`] information for this card, including
    pub prices              : Prices,

    /// The localized name printed on this card, if any
    pub printed_name        : Option<String>,

    /// The localized text printed on this card, if any
    pub printed_text        : Option<String>,

    /// The localized type line printed on this card, if any
    pub printed_type_line   : Option<String>,

    /// True if this card is a promotional card
    pub promo               : bool,

    /// An array of strings describing what categories of 
    /// promo cards this card falls into.
    pub promo_types         : Option<Vec<String>>,

    /// [`PurchaseURIs`] to this card’s listing on major marketplaces. 
    /// Omitted if the card is unpurchaseable
    pub purchase_uris       : PurchaseURIs,

    /// This card's [`Rarity`]
    pub rarity              : Rarity,

    /// This card's [`RelatedURIs`] on other 
    /// Magic: The Gathering online resources
    pub related_uris        : RelatedURIs,

    /// The date this card was first released,
    pub released_at         : String,

    /// True if this card is a reprint
    pub reprint             : bool,

    /// A link to this card’s set on Scryfall’s website
    pub scryfall_set_uri    : URI,

    /// This card’s full set name
    pub set_name            : String,

    /// A link to where you can begin paginating this 
    /// card’s set on the Scryfall API
    pub set_search_uri      : URI,

    /// The type of set this printing is in
    pub set_type            : String,

    /// A link to this card’s set object on Scryfall’s API
    pub set_uri             : URI,

    /// This card’s set code
    pub set                 : String,

    /// This card’s Set object [`UUID`]
    pub set_id              : UUID,

    /// True if this card is a Story Spotlight
    pub story_spotlight     : bool,

    /// True if the card is printed without text
    pub textless            : bool,

    /// Whether this card is a variation of another printing
    pub variation           : bool,

    /// The printing ID of the printing this card is a 
    /// variation of
    pub variation_of        : Option<UUID>,

    /// The [`SecurityStamp`] on this card
    pub security_stamp      : SecurityStamp,

    /// This card’s watermark, if any
    pub watermark           : Option<String>,

    /// The date this card was previewed
    pub previewed_at        : Option<String>,

    /// A link to the preview for this card
    pub preview_source_uri  : Option<URI>,

    /// The name of the source that previewed this card
    pub preview_source      : Option<String>
}