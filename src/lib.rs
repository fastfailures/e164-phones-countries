use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::{NonZeroU32, NonZeroU64};

///
/// All the ISO 3166 territorial codes. Most of these are country codes,
/// but some territories still exist in the world today.
///
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive] // New territories may want independence
pub enum TerritoryCode {
    US,
    CA,
    AC,
    AD,
    AE,
    AF,
    AG,
    AI,
    AL,
    AM,
    AO,
    AQ,
    AR,
    AS,
    AT,
    AU,
    AW,
    AX,
    AZ,
    BA,
    BB,
    BD,
    BE,
    BF,
    BG,
    BH,
    BI,
    BJ,
    BM,
    BN,
    BO,
    BQ,
    BR,
    BS,
    BT,
    BW,
    BY,
    BZ,
    CC,
    CD,
    CF,
    CG,
    CH,
    CI,
    CK,
    CL,
    CM,
    CN,
    CO,
    CR,
    CU,
    CV,
    CW,
    CX,
    CY,
    CZ,
    DE,
    DJ,
    DK,
    DM,
    DO,
    DZ,
    EC,
    EE,
    EG,
    ER,
    ES,
    ET,
    FI,
    FJ,
    FK,
    FM,
    FO,
    FR,
    GA,
    GB,
    GD,
    GE,
    GF,
    GG,
    GH,
    GI,
    GL,
    GM,
    GN,
    GP,
    GQ,
    GR,
    GT,
    GU,
    GW,
    GY,
    HK,
    HN,
    HR,
    HT,
    HU,
    ID,
    IE,
    IL,
    IM,
    IN,
    IO,
    IQ,
    IR,
    IS,
    IT,
    JE,
    JM,
    JO,
    JP,
    KE,
    KG,
    KH,
    KI,
    KM,
    KN,
    KP,
    KR,
    KW,
    KY,
    KZ,
    LA,
    LB,
    LC,
    LI,
    LK,
    LR,
    LS,
    LT,
    LU,
    LV,
    LY,
    MA,
    MC,
    MD,
    ME,
    MG,
    MH,
    MK,
    ML,
    MM,
    MN,
    MO,
    MP,
    MQ,
    MR,
    MS,
    MT,
    MU,
    MV,
    MW,
    MX,
    MY,
    MZ,
    NA,
    NC,
    NE,
    NF,
    NG,
    NI,
    NL,
    NO,
    NP,
    NR,
    NU,
    NZ,
    OM,
    PA,
    PE,
    PF,
    PG,
    PH,
    PK,
    PL,
    PM,
    PR,
    PS,
    PT,
    PW,
    PY,
    QA,
    RE,
    RO,
    RS,
    RU,
    RW,
    SA,
    SB,
    SC,
    SD,
    SE,
    SG,
    SH,
    SI,
    SJ,
    SK,
    SL,
    SM,
    SN,
    SO,
    SR,
    SS,
    ST,
    SV,
    SX,
    SY,
    SZ,
    TC,
    TD,
    TG,
    TH,
    TJ,
    TK,
    TL,
    TM,
    TN,
    TO,
    TR,
    TT,
    TV,
    TW,
    TZ,
    UA,
    UG,
    UY,
    UZ,
    VA,
    VC,
    VE,
    VG,
    VI,
    VN,
    VU,
    WF,
    WS,
    XG,
    XN,
    XP,
    XS,
    XT,
    XV,
    YE,
    YT,
    ZA,
    ZM,
    ZW,
}

impl Display for TerritoryCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl TerritoryCode {
    /// Yields the associated calling codes. Most countries have only one code, in which case
    /// [`CallingCodes::primary`] will suffice.
    pub fn calling_codes(&self) -> CallingCodes {
        macro_rules! single_calling_code {
            ($value:literal) => {
                CallingCodes(CallingCodesInner::Single(
                    [NonZeroU32::new($value).unwrap()],
                ))
            };
        }
        macro_rules! cc {
            ($value:literal) => {
                NonZeroU32::new($value).unwrap()
            };
        }
        match *self {
            Self::US => single_calling_code!(1),
            Self::CA => single_calling_code!(1),
            Self::AC => single_calling_code!(247),
            Self::AD => single_calling_code!(376),
            Self::AE => single_calling_code!(971),
            Self::AF => single_calling_code!(93),
            Self::AG => single_calling_code!(1),
            Self::AI => single_calling_code!(1),
            Self::AL => single_calling_code!(355),
            Self::AM => single_calling_code!(374),
            Self::AO => single_calling_code!(244),
            Self::AQ => single_calling_code!(672),
            Self::AR => single_calling_code!(54),
            Self::AS => single_calling_code!(1),
            Self::AT => single_calling_code!(43),
            Self::AU => single_calling_code!(61),
            Self::AW => single_calling_code!(297),
            Self::AX => single_calling_code!(35818),
            Self::AZ => single_calling_code!(994),
            Self::BA => single_calling_code!(387),
            Self::BB => single_calling_code!(1),
            Self::BD => single_calling_code!(880),
            Self::BE => single_calling_code!(32),
            Self::BF => single_calling_code!(226),
            Self::BG => single_calling_code!(359),
            Self::BH => single_calling_code!(973),
            Self::BI => single_calling_code!(257),
            Self::BJ => single_calling_code!(229),
            Self::BM => single_calling_code!(1),
            Self::BN => single_calling_code!(673),
            Self::BO => single_calling_code!(591),
            Self::BQ => CallingCodes(CallingCodesInner::Three([cc!(5993), cc!(5994), cc!(5997)])),
            Self::BR => single_calling_code!(55),
            Self::BS => single_calling_code!(1),
            Self::BT => single_calling_code!(975),
            Self::BW => single_calling_code!(267),
            Self::BY => single_calling_code!(375),
            Self::BZ => single_calling_code!(501),
            Self::CC => single_calling_code!(6189162),
            Self::CD => single_calling_code!(243),
            Self::CF => single_calling_code!(236),
            Self::CG => single_calling_code!(242),
            Self::CH => single_calling_code!(41),
            Self::CI => single_calling_code!(225),
            Self::CK => single_calling_code!(682),
            Self::CL => single_calling_code!(56),
            Self::CM => single_calling_code!(237),
            Self::CN => single_calling_code!(86),
            Self::CO => single_calling_code!(57),
            Self::CR => single_calling_code!(506),
            Self::CU => single_calling_code!(53),
            Self::CV => single_calling_code!(238),
            Self::CW => single_calling_code!(5999),
            Self::CX => single_calling_code!(6189164),
            Self::CY => single_calling_code!(357),
            Self::CZ => single_calling_code!(420),
            Self::DE => single_calling_code!(49),
            Self::DJ => single_calling_code!(253),
            Self::DK => single_calling_code!(45),
            Self::DM => single_calling_code!(1),
            Self::DO => single_calling_code!(1),
            Self::DZ => single_calling_code!(213),
            Self::EC => single_calling_code!(593),
            Self::EE => single_calling_code!(372),
            Self::EG => single_calling_code!(20),
            Self::ER => single_calling_code!(291),
            Self::ES => single_calling_code!(34),
            Self::ET => single_calling_code!(251),
            Self::FI => single_calling_code!(358),
            Self::FJ => single_calling_code!(679),
            Self::FK => single_calling_code!(500),
            Self::FM => single_calling_code!(691),
            Self::FO => single_calling_code!(298),
            Self::FR => single_calling_code!(33),
            Self::GA => single_calling_code!(241),
            Self::GB => single_calling_code!(44),
            Self::GD => single_calling_code!(1),
            Self::GE => single_calling_code!(995),
            Self::GF => single_calling_code!(594),
            Self::GG => single_calling_code!(441481),
            Self::GH => single_calling_code!(233),
            Self::GI => single_calling_code!(350),
            Self::GL => single_calling_code!(299),
            Self::GM => single_calling_code!(220),
            Self::GN => single_calling_code!(224),
            Self::GP => single_calling_code!(590),
            Self::GQ => single_calling_code!(240),
            Self::GR => single_calling_code!(30),
            Self::GT => single_calling_code!(502),
            Self::GU => single_calling_code!(1),
            Self::GW => single_calling_code!(245),
            Self::GY => single_calling_code!(592),
            Self::HK => single_calling_code!(852),
            Self::HN => single_calling_code!(504),
            Self::HR => single_calling_code!(385),
            Self::HT => single_calling_code!(509),
            Self::HU => single_calling_code!(36),
            Self::ID => single_calling_code!(62),
            Self::IE => single_calling_code!(353),
            Self::IL => single_calling_code!(972),
            Self::IM => single_calling_code!(441624),
            Self::IN => single_calling_code!(91),
            Self::IO => single_calling_code!(246),
            Self::IQ => single_calling_code!(964),
            Self::IR => single_calling_code!(98),
            Self::IS => single_calling_code!(354),
            Self::IT => single_calling_code!(39),
            Self::JE => single_calling_code!(441534),
            Self::JM => single_calling_code!(1),
            Self::JO => single_calling_code!(962),
            Self::JP => single_calling_code!(81),
            Self::KE => single_calling_code!(254),
            Self::KG => single_calling_code!(996),
            Self::KH => single_calling_code!(855),
            Self::KI => single_calling_code!(686),
            Self::KM => single_calling_code!(269),
            Self::KN => single_calling_code!(1),
            Self::KP => single_calling_code!(850),
            Self::KR => single_calling_code!(82),
            Self::KW => single_calling_code!(965),
            Self::KY => single_calling_code!(1),
            Self::KZ => single_calling_code!(7),
            Self::LA => single_calling_code!(856),
            Self::LB => single_calling_code!(961),
            Self::LC => single_calling_code!(1),
            Self::LI => single_calling_code!(423),
            Self::LK => single_calling_code!(94),
            Self::LR => single_calling_code!(231),
            Self::LS => single_calling_code!(266),
            Self::LT => single_calling_code!(370),
            Self::LU => single_calling_code!(352),
            Self::LV => single_calling_code!(371),
            Self::LY => single_calling_code!(218),
            Self::MA => single_calling_code!(212),
            Self::MC => single_calling_code!(377),
            Self::MD => single_calling_code!(373),
            Self::ME => single_calling_code!(382),
            Self::MG => single_calling_code!(261),
            Self::MH => single_calling_code!(692),
            Self::MK => single_calling_code!(389),
            Self::ML => single_calling_code!(223),
            Self::MM => single_calling_code!(95),
            Self::MN => single_calling_code!(976),
            Self::MO => single_calling_code!(853),
            Self::MP => single_calling_code!(1),
            Self::MQ => single_calling_code!(596),
            Self::MR => single_calling_code!(222),
            Self::MS => single_calling_code!(1),
            Self::MT => single_calling_code!(356),
            Self::MU => single_calling_code!(230),
            Self::MV => single_calling_code!(960),
            Self::MW => single_calling_code!(265),
            Self::MX => single_calling_code!(52),
            Self::MY => single_calling_code!(60),
            Self::MZ => single_calling_code!(258),
            Self::NA => single_calling_code!(264),
            Self::NC => single_calling_code!(687),
            Self::NE => single_calling_code!(227),
            Self::NF => single_calling_code!(6723),
            Self::NG => single_calling_code!(234),
            Self::NI => single_calling_code!(505),
            Self::NL => single_calling_code!(31),
            Self::NO => single_calling_code!(47),
            Self::NP => single_calling_code!(977),
            Self::NR => single_calling_code!(674),
            Self::NU => single_calling_code!(683),
            Self::NZ => single_calling_code!(64),
            Self::OM => single_calling_code!(968),
            Self::PA => single_calling_code!(507),
            Self::PE => single_calling_code!(51),
            Self::PF => single_calling_code!(689),
            Self::PG => single_calling_code!(675),
            Self::PH => single_calling_code!(63),
            Self::PK => single_calling_code!(92),
            Self::PL => single_calling_code!(48),
            Self::PM => single_calling_code!(508),
            Self::PR => single_calling_code!(1),
            Self::PS => single_calling_code!(970),
            Self::PT => single_calling_code!(351),
            Self::PW => single_calling_code!(680),
            Self::PY => single_calling_code!(595),
            Self::QA => single_calling_code!(974),
            Self::RE => single_calling_code!(262),
            Self::RO => single_calling_code!(40),
            Self::RS => single_calling_code!(381),
            Self::RU => single_calling_code!(7),
            Self::RW => single_calling_code!(250),
            Self::SA => single_calling_code!(966),
            Self::SB => single_calling_code!(677),
            Self::SC => single_calling_code!(248),
            Self::SD => single_calling_code!(249),
            Self::SE => single_calling_code!(46),
            Self::SG => single_calling_code!(65),
            Self::SH => single_calling_code!(290),
            Self::SI => single_calling_code!(386),
            Self::SJ => single_calling_code!(4779),
            Self::SK => single_calling_code!(421),
            Self::SL => single_calling_code!(232),
            Self::SM => single_calling_code!(378),
            Self::SN => single_calling_code!(221),
            Self::SO => single_calling_code!(252),
            Self::SR => single_calling_code!(597),
            Self::SS => single_calling_code!(211),
            Self::ST => single_calling_code!(239),
            Self::SV => single_calling_code!(503),
            Self::SX => single_calling_code!(1),
            Self::SY => single_calling_code!(963),
            Self::SZ => single_calling_code!(268),
            Self::TC => single_calling_code!(1),
            Self::TD => single_calling_code!(235),
            Self::TG => single_calling_code!(228),
            Self::TH => single_calling_code!(66),
            Self::TJ => single_calling_code!(992),
            Self::TK => single_calling_code!(690),
            Self::TL => single_calling_code!(670),
            Self::TM => single_calling_code!(993),
            Self::TN => single_calling_code!(216),
            Self::TO => single_calling_code!(676),
            Self::TR => single_calling_code!(90),
            Self::TT => single_calling_code!(1),
            Self::TV => single_calling_code!(688),
            Self::TW => single_calling_code!(886),
            Self::TZ => single_calling_code!(255),
            Self::UA => single_calling_code!(380),
            Self::UG => single_calling_code!(256),
            Self::UY => single_calling_code!(598),
            Self::UZ => single_calling_code!(998),
            Self::VA => single_calling_code!(379),
            Self::VC => single_calling_code!(1),
            Self::VE => single_calling_code!(58),
            Self::VG => single_calling_code!(1),
            Self::VI => single_calling_code!(1),
            Self::VN => single_calling_code!(84),
            Self::VU => single_calling_code!(678),
            Self::WF => single_calling_code!(681),
            Self::WS => single_calling_code!(685),
            Self::XG => single_calling_code!(881),
            Self::XN => single_calling_code!(870),
            Self::XP => single_calling_code!(878),
            Self::XS => single_calling_code!(808),
            Self::XT => single_calling_code!(800),
            Self::XV => CallingCodes(CallingCodesInner::Two([cc!(882), cc!(883)])),
            Self::YE => single_calling_code!(967),
            Self::YT => single_calling_code!(262),
            Self::ZA => single_calling_code!(27),
            Self::ZM => single_calling_code!(260),
            Self::ZW => single_calling_code!(263),
        }
    }

    /// Yields a territory code by name. I.e.:
    /// ```
    /// use e164_phones_countries::TerritoryCode;
    /// assert_eq!(TerritoryCode::IN, TerritoryCode::from_name("IN").unwrap());
    /// ```
    pub fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "US" => Self::US,
            "CA" => Self::CA,
            "AC" => Self::AC,
            "AD" => Self::AD,
            "AE" => Self::AE,
            "AF" => Self::AF,
            "AG" => Self::AG,
            "AI" => Self::AI,
            "AL" => Self::AL,
            "AM" => Self::AM,
            "AO" => Self::AO,
            "AQ" => Self::AQ,
            "AR" => Self::AR,
            "AS" => Self::AS,
            "AT" => Self::AT,
            "AU" => Self::AU,
            "AW" => Self::AW,
            "AX" => Self::AX,
            "AZ" => Self::AZ,
            "BA" => Self::BA,
            "BB" => Self::BB,
            "BD" => Self::BD,
            "BE" => Self::BE,
            "BF" => Self::BF,
            "BG" => Self::BG,
            "BH" => Self::BH,
            "BI" => Self::BI,
            "BJ" => Self::BJ,
            "BM" => Self::BM,
            "BN" => Self::BN,
            "BO" => Self::BO,
            "BQ" => Self::BQ,
            "BR" => Self::BR,
            "BS" => Self::BS,
            "BT" => Self::BT,
            "BW" => Self::BW,
            "BY" => Self::BY,
            "BZ" => Self::BZ,
            "CC" => Self::CC,
            "CD" => Self::CD,
            "CF" => Self::CF,
            "CG" => Self::CG,
            "CH" => Self::CH,
            "CI" => Self::CI,
            "CK" => Self::CK,
            "CL" => Self::CL,
            "CM" => Self::CM,
            "CN" => Self::CN,
            "CO" => Self::CO,
            "CR" => Self::CR,
            "CU" => Self::CU,
            "CV" => Self::CV,
            "CW" => Self::CW,
            "CX" => Self::CX,
            "CY" => Self::CY,
            "CZ" => Self::CZ,
            "DE" => Self::DE,
            "DJ" => Self::DJ,
            "DK" => Self::DK,
            "DM" => Self::DM,
            "DO" => Self::DO,
            "DZ" => Self::DZ,
            "EC" => Self::EC,
            "EE" => Self::EE,
            "EG" => Self::EG,
            "ER" => Self::ER,
            "ES" => Self::ES,
            "ET" => Self::ET,
            "FI" => Self::FI,
            "FJ" => Self::FJ,
            "FK" => Self::FK,
            "FM" => Self::FM,
            "FO" => Self::FO,
            "FR" => Self::FR,
            "GA" => Self::GA,
            "GB" => Self::GB,
            "GD" => Self::GD,
            "GE" => Self::GE,
            "GF" => Self::GF,
            "GG" => Self::GG,
            "GH" => Self::GH,
            "GI" => Self::GI,
            "GL" => Self::GL,
            "GM" => Self::GM,
            "GN" => Self::GN,
            "GP" => Self::GP,
            "GQ" => Self::GQ,
            "GR" => Self::GR,
            "GT" => Self::GT,
            "GU" => Self::GU,
            "GW" => Self::GW,
            "GY" => Self::GY,
            "HK" => Self::HK,
            "HN" => Self::HN,
            "HR" => Self::HR,
            "HT" => Self::HT,
            "HU" => Self::HU,
            "ID" => Self::ID,
            "IE" => Self::IE,
            "IL" => Self::IL,
            "IM" => Self::IM,
            "IN" => Self::IN,
            "IO" => Self::IO,
            "IQ" => Self::IQ,
            "IR" => Self::IR,
            "IS" => Self::IS,
            "IT" => Self::IT,
            "JE" => Self::JE,
            "JM" => Self::JM,
            "JO" => Self::JO,
            "JP" => Self::JP,
            "KE" => Self::KE,
            "KG" => Self::KG,
            "KH" => Self::KH,
            "KI" => Self::KI,
            "KM" => Self::KM,
            "KN" => Self::KN,
            "KP" => Self::KP,
            "KR" => Self::KR,
            "KW" => Self::KW,
            "KY" => Self::KY,
            "KZ" => Self::KZ,
            "LA" => Self::LA,
            "LB" => Self::LB,
            "LC" => Self::LC,
            "LI" => Self::LI,
            "LK" => Self::LK,
            "LR" => Self::LR,
            "LS" => Self::LS,
            "LT" => Self::LT,
            "LU" => Self::LU,
            "LV" => Self::LV,
            "LY" => Self::LY,
            "MA" => Self::MA,
            "MC" => Self::MC,
            "MD" => Self::MD,
            "ME" => Self::ME,
            "MG" => Self::MG,
            "MH" => Self::MH,
            "MK" => Self::MK,
            "ML" => Self::ML,
            "MM" => Self::MM,
            "MN" => Self::MN,
            "MO" => Self::MO,
            "MP" => Self::MP,
            "MQ" => Self::MQ,
            "MR" => Self::MR,
            "MS" => Self::MS,
            "MT" => Self::MT,
            "MU" => Self::MU,
            "MV" => Self::MV,
            "MW" => Self::MW,
            "MX" => Self::MX,
            "MY" => Self::MY,
            "MZ" => Self::MZ,
            "NA" => Self::NA,
            "NC" => Self::NC,
            "NE" => Self::NE,
            "NF" => Self::NF,
            "NG" => Self::NG,
            "NI" => Self::NI,
            "NL" => Self::NL,
            "NO" => Self::NO,
            "NP" => Self::NP,
            "NR" => Self::NR,
            "NU" => Self::NU,
            "NZ" => Self::NZ,
            "OM" => Self::OM,
            "PA" => Self::PA,
            "PE" => Self::PE,
            "PF" => Self::PF,
            "PG" => Self::PG,
            "PH" => Self::PH,
            "PK" => Self::PK,
            "PL" => Self::PL,
            "PM" => Self::PM,
            "PR" => Self::PR,
            "PS" => Self::PS,
            "PT" => Self::PT,
            "PW" => Self::PW,
            "PY" => Self::PY,
            "QA" => Self::QA,
            "RE" => Self::RE,
            "RO" => Self::RO,
            "RS" => Self::RS,
            "RU" => Self::RU,
            "RW" => Self::RW,
            "SA" => Self::SA,
            "SB" => Self::SB,
            "SC" => Self::SC,
            "SD" => Self::SD,
            "SE" => Self::SE,
            "SG" => Self::SG,
            "SH" => Self::SH,
            "SI" => Self::SI,
            "SJ" => Self::SJ,
            "SK" => Self::SK,
            "SL" => Self::SL,
            "SM" => Self::SM,
            "SN" => Self::SN,
            "SO" => Self::SO,
            "SR" => Self::SR,
            "SS" => Self::SS,
            "ST" => Self::ST,
            "SV" => Self::SV,
            "SX" => Self::SX,
            "SY" => Self::SY,
            "SZ" => Self::SZ,
            "TC" => Self::TC,
            "TD" => Self::TD,
            "TG" => Self::TG,
            "TH" => Self::TH,
            "TJ" => Self::TJ,
            "TK" => Self::TK,
            "TL" => Self::TL,
            "TM" => Self::TM,
            "TN" => Self::TN,
            "TO" => Self::TO,
            "TR" => Self::TR,
            "TT" => Self::TT,
            "TV" => Self::TV,
            "TW" => Self::TW,
            "TZ" => Self::TZ,
            "UA" => Self::UA,
            "UG" => Self::UG,
            "UY" => Self::UY,
            "UZ" => Self::UZ,
            "VA" => Self::VA,
            "VC" => Self::VC,
            "VE" => Self::VE,
            "VG" => Self::VG,
            "VI" => Self::VI,
            "VN" => Self::VN,
            "VU" => Self::VU,
            "WF" => Self::WF,
            "WS" => Self::WS,
            "XG" => Self::XG,
            "XN" => Self::XN,
            "XP" => Self::XP,
            "XS" => Self::XS,
            "XT" => Self::XT,
            "XV" => Self::XV,
            "YE" => Self::YE,
            "YT" => Self::YT,
            "ZA" => Self::ZA,
            "ZM" => Self::ZM,
            "ZW" => Self::ZW,
            _ => return None,
        })
    }

    /// Gets the name of this territory code. The value is always uppercase.
    /// ```
    /// use e164_phones_countries::TerritoryCode;
    /// assert_eq!("FR", TerritoryCode::FR.name())
    /// ```
    pub fn name(&self) -> &'static str {
        match *self {
            Self::US => "US",
            Self::CA => "CA",
            Self::AC => "AC",
            Self::AD => "AD",
            Self::AE => "AE",
            Self::AF => "AF",
            Self::AG => "AG",
            Self::AI => "AI",
            Self::AL => "AL",
            Self::AM => "AM",
            Self::AO => "AO",
            Self::AQ => "AQ",
            Self::AR => "AR",
            Self::AS => "AS",
            Self::AT => "AT",
            Self::AU => "AU",
            Self::AW => "AW",
            Self::AX => "AX",
            Self::AZ => "AZ",
            Self::BA => "BA",
            Self::BB => "BB",
            Self::BD => "BD",
            Self::BE => "BE",
            Self::BF => "BF",
            Self::BG => "BG",
            Self::BH => "BH",
            Self::BI => "BI",
            Self::BJ => "BJ",
            Self::BM => "BM",
            Self::BN => "BN",
            Self::BO => "BO",
            Self::BQ => "BQ",
            Self::BR => "BR",
            Self::BS => "BS",
            Self::BT => "BT",
            Self::BW => "BW",
            Self::BY => "BY",
            Self::BZ => "BZ",
            Self::CC => "CC",
            Self::CD => "CD",
            Self::CF => "CF",
            Self::CG => "CG",
            Self::CH => "CH",
            Self::CI => "CI",
            Self::CK => "CK",
            Self::CL => "CL",
            Self::CM => "CM",
            Self::CN => "CN",
            Self::CO => "CO",
            Self::CR => "CR",
            Self::CU => "CU",
            Self::CV => "CV",
            Self::CW => "CW",
            Self::CX => "CX",
            Self::CY => "CY",
            Self::CZ => "CZ",
            Self::DE => "DE",
            Self::DJ => "DJ",
            Self::DK => "DK",
            Self::DM => "DM",
            Self::DO => "DO",
            Self::DZ => "DZ",
            Self::EC => "EC",
            Self::EE => "EE",
            Self::EG => "EG",
            Self::ER => "ER",
            Self::ES => "ES",
            Self::ET => "ET",
            Self::FI => "FI",
            Self::FJ => "FJ",
            Self::FK => "FK",
            Self::FM => "FM",
            Self::FO => "FO",
            Self::FR => "FR",
            Self::GA => "GA",
            Self::GB => "GB",
            Self::GD => "GD",
            Self::GE => "GE",
            Self::GF => "GF",
            Self::GG => "GG",
            Self::GH => "GH",
            Self::GI => "GI",
            Self::GL => "GL",
            Self::GM => "GM",
            Self::GN => "GN",
            Self::GP => "GP",
            Self::GQ => "GQ",
            Self::GR => "GR",
            Self::GT => "GT",
            Self::GU => "GU",
            Self::GW => "GW",
            Self::GY => "GY",
            Self::HK => "HK",
            Self::HN => "HN",
            Self::HR => "HR",
            Self::HT => "HT",
            Self::HU => "HU",
            Self::ID => "ID",
            Self::IE => "IE",
            Self::IL => "IL",
            Self::IM => "IM",
            Self::IN => "IN",
            Self::IO => "IO",
            Self::IQ => "IQ",
            Self::IR => "IR",
            Self::IS => "IS",
            Self::IT => "IT",
            Self::JE => "JE",
            Self::JM => "JM",
            Self::JO => "JO",
            Self::JP => "JP",
            Self::KE => "KE",
            Self::KG => "KG",
            Self::KH => "KH",
            Self::KI => "KI",
            Self::KM => "KM",
            Self::KN => "KN",
            Self::KP => "KP",
            Self::KR => "KR",
            Self::KW => "KW",
            Self::KY => "KY",
            Self::KZ => "KZ",
            Self::LA => "LA",
            Self::LB => "LB",
            Self::LC => "LC",
            Self::LI => "LI",
            Self::LK => "LK",
            Self::LR => "LR",
            Self::LS => "LS",
            Self::LT => "LT",
            Self::LU => "LU",
            Self::LV => "LV",
            Self::LY => "LY",
            Self::MA => "MA",
            Self::MC => "MC",
            Self::MD => "MD",
            Self::ME => "ME",
            Self::MG => "MG",
            Self::MH => "MH",
            Self::MK => "MK",
            Self::ML => "ML",
            Self::MM => "MM",
            Self::MN => "MN",
            Self::MO => "MO",
            Self::MP => "MP",
            Self::MQ => "MQ",
            Self::MR => "MR",
            Self::MS => "MS",
            Self::MT => "MT",
            Self::MU => "MU",
            Self::MV => "MV",
            Self::MW => "MW",
            Self::MX => "MX",
            Self::MY => "MY",
            Self::MZ => "MZ",
            Self::NA => "NA",
            Self::NC => "NC",
            Self::NE => "NE",
            Self::NF => "NF",
            Self::NG => "NG",
            Self::NI => "NI",
            Self::NL => "NL",
            Self::NO => "NO",
            Self::NP => "NP",
            Self::NR => "NR",
            Self::NU => "NU",
            Self::NZ => "NZ",
            Self::OM => "OM",
            Self::PA => "PA",
            Self::PE => "PE",
            Self::PF => "PF",
            Self::PG => "PG",
            Self::PH => "PH",
            Self::PK => "PK",
            Self::PL => "PL",
            Self::PM => "PM",
            Self::PR => "PR",
            Self::PS => "PS",
            Self::PT => "PT",
            Self::PW => "PW",
            Self::PY => "PY",
            Self::QA => "QA",
            Self::RE => "RE",
            Self::RO => "RO",
            Self::RS => "RS",
            Self::RU => "RU",
            Self::RW => "RW",
            Self::SA => "SA",
            Self::SB => "SB",
            Self::SC => "SC",
            Self::SD => "SD",
            Self::SE => "SE",
            Self::SG => "SG",
            Self::SH => "SH",
            Self::SI => "SI",
            Self::SJ => "SJ",
            Self::SK => "SK",
            Self::SL => "SL",
            Self::SM => "SM",
            Self::SN => "SN",
            Self::SO => "SO",
            Self::SR => "SR",
            Self::SS => "SS",
            Self::ST => "ST",
            Self::SV => "SV",
            Self::SX => "SX",
            Self::SY => "SY",
            Self::SZ => "SZ",
            Self::TC => "TC",
            Self::TD => "TD",
            Self::TG => "TG",
            Self::TH => "TH",
            Self::TJ => "TJ",
            Self::TK => "TK",
            Self::TL => "TL",
            Self::TM => "TM",
            Self::TN => "TN",
            Self::TO => "TO",
            Self::TR => "TR",
            Self::TT => "TT",
            Self::TV => "TV",
            Self::TW => "TW",
            Self::TZ => "TZ",
            Self::UA => "UA",
            Self::UG => "UG",
            Self::UY => "UY",
            Self::UZ => "UZ",
            Self::VA => "VA",
            Self::VC => "VC",
            Self::VE => "VE",
            Self::VG => "VG",
            Self::VI => "VI",
            Self::VN => "VN",
            Self::VU => "VU",
            Self::WF => "WF",
            Self::WS => "WS",
            Self::XG => "XG",
            Self::XN => "XN",
            Self::XP => "XP",
            Self::XS => "XS",
            Self::XT => "XT",
            Self::XV => "XV",
            Self::YE => "YE",
            Self::YT => "YT",
            Self::ZA => "ZA",
            Self::ZM => "ZM",
            Self::ZW => "ZW",
        }
    }

    fn lookup_table(prefix: u64) -> Option<TerritoryCode> {
        Some(match prefix {
            1201 => Self::US,
            1202 => Self::US,
            1203 => Self::US,
            1204 => Self::CA,
            1205 => Self::US,
            1206 => Self::US,
            1207 => Self::US,
            1208 => Self::US,
            1209 => Self::US,
            1210 => Self::US,
            1212 => Self::US,
            1213 => Self::US,
            1214 => Self::US,
            1215 => Self::US,
            1216 => Self::US,
            1217 => Self::US,
            1218 => Self::US,
            1219 => Self::US,
            1224 => Self::US,
            1225 => Self::US,
            1226 => Self::CA,
            1228 => Self::US,
            1229 => Self::US,
            1231 => Self::US,
            1234 => Self::US,
            1236 => Self::CA,
            1239 => Self::US,
            1240 => Self::US,
            1242 => Self::BS,
            1246 => Self::BB,
            1248 => Self::US,
            1249 => Self::CA,
            1250 => Self::CA,
            1251 => Self::US,
            1252 => Self::US,
            1253 => Self::US,
            1254 => Self::US,
            1256 => Self::US,
            1260 => Self::US,
            1262 => Self::US,
            1264 => Self::AI,
            1267 => Self::US,
            1268 => Self::AG,
            1269 => Self::US,
            1270 => Self::US,
            1272 => Self::US,
            1274 => Self::US,
            1276 => Self::US,
            1281 => Self::US,
            1284 => Self::VG,
            1289 => Self::CA,
            1301 => Self::US,
            1302 => Self::US,
            1303 => Self::US,
            1304 => Self::US,
            1305 => Self::US,
            1306 => Self::CA,
            1307 => Self::US,
            1308 => Self::US,
            1309 => Self::US,
            1310 => Self::US,
            1312 => Self::US,
            1313 => Self::US,
            1314 => Self::US,
            1315 => Self::US,
            1316 => Self::US,
            1317 => Self::US,
            1318 => Self::US,
            1319 => Self::US,
            1320 => Self::US,
            1321 => Self::US,
            1323 => Self::US,
            1325 => Self::US,
            1330 => Self::US,
            1331 => Self::US,
            1334 => Self::US,
            1336 => Self::US,
            1337 => Self::US,
            1339 => Self::US,
            1340 => Self::VI,
            1343 => Self::CA,
            1345 => Self::KY,
            1346 => Self::US,
            1347 => Self::US,
            1351 => Self::US,
            1352 => Self::US,
            1360 => Self::US,
            1361 => Self::US,
            1364 => Self::US,
            1365 => Self::CA,
            1385 => Self::US,
            1386 => Self::US,
            1401 => Self::US,
            1402 => Self::US,
            1403 => Self::CA,
            1404 => Self::US,
            1405 => Self::US,
            1406 => Self::US,
            1407 => Self::US,
            1408 => Self::US,
            1409 => Self::US,
            1410 => Self::US,
            1412 => Self::US,
            1413 => Self::US,
            1414 => Self::US,
            1415 => Self::US,
            1416 => Self::CA,
            1417 => Self::US,
            1418 => Self::CA,
            1419 => Self::US,
            1423 => Self::US,
            1424 => Self::US,
            1425 => Self::US,
            1430 => Self::US,
            1431 => Self::CA,
            1432 => Self::US,
            1434 => Self::US,
            1435 => Self::US,
            1437 => Self::CA,
            1438 => Self::CA,
            1440 => Self::US,
            1441 => Self::BM,
            1442 => Self::US,
            1443 => Self::US,
            1450 => Self::CA,
            1457 => Self::CA,
            1458 => Self::US,
            1469 => Self::US,
            1470 => Self::US,
            1473 => Self::GD,
            1475 => Self::US,
            1478 => Self::US,
            1479 => Self::US,
            1480 => Self::US,
            1484 => Self::US,
            1500 => Self::US,
            1501 => Self::US,
            1502 => Self::US,
            1503 => Self::US,
            1504 => Self::US,
            1505 => Self::US,
            1506 => Self::CA,
            1507 => Self::US,
            1508 => Self::US,
            1509 => Self::US,
            1510 => Self::US,
            1512 => Self::US,
            1513 => Self::US,
            1514 => Self::CA,
            1515 => Self::US,
            1516 => Self::US,
            1517 => Self::US,
            1518 => Self::US,
            1519 => Self::CA,
            1520 => Self::US,
            1530 => Self::US,
            1531 => Self::US,
            1533 => Self::US,
            1534 => Self::US,
            1539 => Self::US,
            1540 => Self::US,
            1541 => Self::US,
            1544 => Self::US,
            1551 => Self::US,
            1559 => Self::US,
            1561 => Self::US,
            1562 => Self::US,
            1563 => Self::US,
            1566 => Self::US,
            1567 => Self::US,
            1570 => Self::US,
            1571 => Self::US,
            1573 => Self::US,
            1574 => Self::US,
            1575 => Self::US,
            1577 => Self::US,
            1579 => Self::CA,
            1580 => Self::US,
            1581 => Self::CA,
            1585 => Self::US,
            1586 => Self::US,
            1587 => Self::CA,
            1600 => Self::CA,
            1601 => Self::US,
            1602 => Self::US,
            1603 => Self::US,
            1604 => Self::CA,
            1605 => Self::US,
            1606 => Self::US,
            1607 => Self::US,
            1608 => Self::US,
            1609 => Self::US,
            1610 => Self::US,
            1612 => Self::US,
            1613 => Self::CA,
            1614 => Self::US,
            1615 => Self::US,
            1616 => Self::US,
            1617 => Self::US,
            1618 => Self::US,
            1619 => Self::US,
            1620 => Self::US,
            1623 => Self::US,
            1626 => Self::US,
            1628 => Self::US,
            1629 => Self::US,
            1630 => Self::US,
            1631 => Self::US,
            1636 => Self::US,
            1639 => Self::CA,
            1641 => Self::US,
            1646 => Self::US,
            1647 => Self::CA,
            1649 => Self::TC,
            1650 => Self::US,
            1651 => Self::US,
            1657 => Self::US,
            1660 => Self::US,
            1661 => Self::US,
            1662 => Self::US,
            1664 => Self::MS,
            1667 => Self::US,
            1669 => Self::US,
            1670 => Self::MP,
            1671 => Self::GU,
            1678 => Self::US,
            1681 => Self::US,
            1682 => Self::US,
            1684 => Self::AS,
            1700 => Self::US,
            1701 => Self::US,
            1702 => Self::US,
            1703 => Self::US,
            1704 => Self::US,
            1705 => Self::CA,
            1706 => Self::US,
            1707 => Self::US,
            1708 => Self::US,
            1709 => Self::CA,
            1710 => Self::US,
            1712 => Self::US,
            1713 => Self::US,
            1714 => Self::US,
            1715 => Self::US,
            1716 => Self::US,
            1717 => Self::US,
            1718 => Self::US,
            1719 => Self::US,
            1720 => Self::US,
            1721 => Self::SX,
            1724 => Self::US,
            1725 => Self::US,
            1727 => Self::US,
            1731 => Self::US,
            1732 => Self::US,
            1734 => Self::US,
            1737 => Self::US,
            1740 => Self::US,
            1747 => Self::US,
            1754 => Self::US,
            1757 => Self::US,
            1758 => Self::LC,
            1760 => Self::US,
            1762 => Self::US,
            1763 => Self::US,
            1765 => Self::US,
            1767 => Self::DM,
            1769 => Self::US,
            1770 => Self::US,
            1772 => Self::US,
            1773 => Self::US,
            1774 => Self::US,
            1775 => Self::US,
            1778 => Self::CA,
            1779 => Self::US,
            1780 => Self::CA,
            1781 => Self::US,
            1782 => Self::CA,
            1784 => Self::VC,
            1785 => Self::US,
            1786 => Self::US,
            1787 => Self::PR,
            1800 => Self::US,
            1801 => Self::US,
            1802 => Self::US,
            1803 => Self::US,
            1804 => Self::US,
            1805 => Self::US,
            1806 => Self::US,
            1807 => Self::CA,
            1808 => Self::US,
            1809 => Self::DO,
            1810 => Self::US,
            1812 => Self::US,
            1813 => Self::US,
            1814 => Self::US,
            1815 => Self::US,
            1816 => Self::US,
            1817 => Self::US,
            1818 => Self::US,
            1819 => Self::CA,
            1825 => Self::CA,
            1828 => Self::US,
            1829 => Self::DO,
            1830 => Self::US,
            1831 => Self::US,
            1832 => Self::US,
            1843 => Self::US,
            1844 => Self::US,
            1845 => Self::US,
            1847 => Self::US,
            1848 => Self::US,
            1849 => Self::DO,
            1850 => Self::US,
            1855 => Self::US,
            1856 => Self::US,
            1857 => Self::US,
            1858 => Self::US,
            1859 => Self::US,
            1860 => Self::US,
            1862 => Self::US,
            1863 => Self::US,
            1864 => Self::US,
            1865 => Self::US,
            1866 => Self::US,
            1867 => Self::CA,
            1868 => Self::TT,
            1869 => Self::KN,
            1870 => Self::US,
            1872 => Self::US,
            1873 => Self::CA,
            1876 => Self::JM,
            1877 => Self::US,
            1878 => Self::US,
            1888 => Self::US,
            1900 => Self::US,
            1901 => Self::US,
            1902 => Self::CA,
            1903 => Self::US,
            1904 => Self::US,
            1905 => Self::CA,
            1906 => Self::US,
            1907 => Self::US,
            1908 => Self::US,
            1909 => Self::US,
            1910 => Self::US,
            1912 => Self::US,
            1913 => Self::US,
            1914 => Self::US,
            1915 => Self::US,
            1916 => Self::US,
            1917 => Self::US,
            1918 => Self::US,
            1919 => Self::US,
            1920 => Self::US,
            1925 => Self::US,
            1928 => Self::US,
            1929 => Self::US,
            1930 => Self::US,
            1931 => Self::US,
            1935 => Self::US,
            1936 => Self::US,
            1937 => Self::US,
            1938 => Self::US,
            1939 => Self::PR,
            1940 => Self::US,
            1941 => Self::US,
            1947 => Self::US,
            1949 => Self::US,
            1951 => Self::US,
            1952 => Self::US,
            1954 => Self::US,
            1956 => Self::US,
            1959 => Self::US,
            1970 => Self::US,
            1971 => Self::US,
            1972 => Self::US,
            1973 => Self::US,
            1978 => Self::US,
            1979 => Self::US,
            1980 => Self::US,
            1984 => Self::US,
            1985 => Self::US,
            1989 => Self::US,
            20 => Self::EG,
            211 => Self::SS,
            212 => Self::MA,
            213 => Self::DZ,
            216 => Self::TN,
            218 => Self::LY,
            220 => Self::GM,
            221 => Self::SN,
            222 => Self::MR,
            223 => Self::ML,
            224 => Self::GN,
            225 => Self::CI,
            226 => Self::BF,
            227 => Self::NE,
            228 => Self::TG,
            229 => Self::BJ,
            230 => Self::MU,
            231 => Self::LR,
            232 => Self::SL,
            233 => Self::GH,
            234 => Self::NG,
            235 => Self::TD,
            236 => Self::CF,
            237 => Self::CM,
            238 => Self::CV,
            239 => Self::ST,
            240 => Self::GQ,
            241 => Self::GA,
            242 => Self::CG,
            243 => Self::CD,
            244 => Self::AO,
            245 => Self::GW,
            246 => Self::IO,
            247 => Self::AC,
            248 => Self::SC,
            249 => Self::SD,
            250 => Self::RW,
            251 => Self::ET,
            252 => Self::SO,
            253 => Self::DJ,
            254 => Self::KE,
            255 => Self::TZ,
            256 => Self::UG,
            257 => Self::BI,
            258 => Self::MZ,
            260 => Self::ZM,
            261 => Self::MG,
            262269 => Self::YT,
            262639 => Self::YT,
            262 => Self::RE,
            263 => Self::ZW,
            264 => Self::NA,
            265 => Self::MW,
            266 => Self::LS,
            267 => Self::BW,
            268 => Self::SZ,
            269 => Self::KM,
            27 => Self::ZA,
            290 => Self::SH,
            291 => Self::ER,
            297 => Self::AW,
            298 => Self::FO,
            299 => Self::GL,
            30 => Self::GR,
            31 => Self::NL,
            32 => Self::BE,
            33 => Self::FR,
            34 => Self::ES,
            350 => Self::GI,
            351 => Self::PT,
            352 => Self::LU,
            353 => Self::IE,
            354 => Self::IS,
            355 => Self::AL,
            356 => Self::MT,
            357 => Self::CY,
            358 => Self::FI,
            35818 => Self::AX,
            359 => Self::BG,
            36 => Self::HU,
            370 => Self::LT,
            371 => Self::LV,
            372 => Self::EE,
            373 => Self::MD,
            374 => Self::AM,
            375 => Self::BY,
            376 => Self::AD,
            377 => Self::MC,
            378 => Self::SM,
            379 => Self::VA,
            380 => Self::UA,
            381 => Self::RS,
            382 => Self::ME,
            385 => Self::HR,
            386 => Self::SI,
            387 => Self::BA,
            389 => Self::MK,
            39 => Self::IT,
            40 => Self::RO,
            41 => Self::CH,
            420 => Self::CZ,
            421 => Self::SK,
            423 => Self::LI,
            43 => Self::AT,
            441481 => Self::GG,
            441624 => Self::IM,
            441534 => Self::JE,
            44 => Self::GB,
            45 => Self::DK,
            46 => Self::SE,
            47 => Self::NO,
            4779 => Self::SJ,
            48 => Self::PL,
            49 => Self::DE,
            500 => Self::FK,
            501 => Self::BZ,
            502 => Self::GT,
            503 => Self::SV,
            504 => Self::HN,
            505 => Self::NI,
            506 => Self::CR,
            507 => Self::PA,
            508 => Self::PM,
            509 => Self::HT,
            51 => Self::PE,
            52 => Self::MX,
            53 => Self::CU,
            54 => Self::AR,
            55 => Self::BR,
            56 => Self::CL,
            57 => Self::CO,
            58 => Self::VE,
            590 => Self::GP,
            591 => Self::BO,
            592 => Self::GY,
            593 => Self::EC,
            594 => Self::GF,
            595 => Self::PY,
            596 => Self::MQ,
            597 => Self::SR,
            598 => Self::UY,
            5993 => Self::BQ,
            5994 => Self::BQ,
            5997 => Self::BQ,
            5999 => Self::CW,
            60 => Self::MY,
            61 => Self::AU,
            6189164 => Self::CX,
            6189162 => Self::CC,
            62 => Self::ID,
            63 => Self::PH,
            64 => Self::NZ,
            65 => Self::SG,
            66 => Self::TH,
            670 => Self::TL,
            6721 => Self::AQ,
            6723 => Self::NF,
            673 => Self::BN,
            674 => Self::NR,
            675 => Self::PG,
            676 => Self::TO,
            677 => Self::SB,
            678 => Self::VU,
            679 => Self::FJ,
            680 => Self::PW,
            681 => Self::WF,
            682 => Self::CK,
            683 => Self::NU,
            685 => Self::WS,
            686 => Self::KI,
            687 => Self::NC,
            688 => Self::TV,
            689 => Self::PF,
            690 => Self::TK,
            691 => Self::FM,
            692 => Self::MH,
            7 => Self::RU,
            76 => Self::KZ,
            77 => Self::KZ,
            800 => Self::XT,
            808 => Self::XS,
            81 => Self::JP,
            82 => Self::KR,
            84 => Self::VN,
            850 => Self::KP,
            852 => Self::HK,
            853 => Self::MO,
            855 => Self::KH,
            856 => Self::LA,
            86 => Self::CN,
            870 => Self::XN,
            878 => Self::XP,
            880 => Self::BD,
            881 => Self::XG,
            882 => Self::XV,
            883 => Self::XV,
            886 => Self::TW,
            90 => Self::TR,
            91 => Self::IN,
            92 => Self::PK,
            93 => Self::AF,
            94 => Self::LK,
            95 => Self::MM,
            960 => Self::MV,
            961 => Self::LB,
            962 => Self::JO,
            963 => Self::SY,
            964 => Self::IQ,
            965 => Self::KW,
            966 => Self::SA,
            967 => Self::YE,
            968 => Self::OM,
            970 => Self::PS,
            971 => Self::AE,
            972 => Self::IL,
            973 => Self::BH,
            974 => Self::QA,
            975 => Self::BT,
            976 => Self::MN,
            977 => Self::NP,
            98 => Self::IR,
            992 => Self::TJ,
            993 => Self::TM,
            994 => Self::AZ,
            995 => Self::GE,
            996 => Self::KG,
            998 => Self::UZ,
            _ => return None,
        })
    }

    /// Attempts to find the territory code from a phone number
    pub fn from_phone_number(phone: u64) -> Result<Self, FromPhoneError> {
        let phone = NonZeroU64::new(phone).ok_or(FromPhoneError::InvalidPhoneNumber)?;

        let phone_len = 1 + phone.ilog10();
        if phone_len < 10 {
            return Err(FromPhoneError::InvalidPhoneNumber);
        }
        let get_phone_leading_digits = |no_digits: u32| {
            let chop_digits = phone_len - no_digits;
            phone.get() / 10u64.pow(chop_digits)
        };
        let lookup_table = Self::lookup_table;

        // Shortcut for +1 country code
        Ok(if get_phone_leading_digits(1) == 1 {
            let first_four = get_phone_leading_digits(4);
            match lookup_table(first_four) {
                Some(found) => found,
                None => return Err(FromPhoneError::NotFound),
            }
        } else {
            // Keep trying based on a number of prefix digits
            let mut prefix_len = 7;
            loop {
                if prefix_len == 0 {
                    return Err(FromPhoneError::NotFound);
                }
                let leading_digits = get_phone_leading_digits(prefix_len);
                if let Some(found) = lookup_table(leading_digits) {
                    break found;
                }
                prefix_len -= 1;
            }
        })
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FromPhoneError {
    /// A phone number must be at least 10 digits
    InvalidPhoneNumber,
    /// The calling prefix did not match any territory code
    NotFound,
}

impl Display for FromPhoneError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Self::InvalidPhoneNumber => "Invalid phone. Must be at least 10 digits",
            Self::NotFound => "Did not match any territory code",
        })
    }
}

impl Error for FromPhoneError {}

/// A territory's calling codes. In rare cases, a territory may have more than one calling code.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CallingCodes(CallingCodesInner);

impl CallingCodes {
    /// Yields the primary calling code associated with this territory. Most countries have one code.
    pub fn primary(&self) -> NonZeroU32 {
        self.all()[0]
    }

    /// All the calling codes used by this territory
    pub fn all(&self) -> &[NonZeroU32] {
        match &self.0 {
            CallingCodesInner::Single(p) => p,
            CallingCodesInner::Two(p) => p,
            CallingCodesInner::Three(p) => p,
        }
    }

    /// Whether the territory has more than one calling code
    pub fn has_multiple(&self) -> bool {
        self.all().len() != 1
    }
}

impl Ord for CallingCodes {
    /// The ordering of a [`CallingCodes`] is dependent on the primary calling code
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.primary(), &other.primary())
    }
}

impl PartialOrd for CallingCodes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum CallingCodesInner {
    Single([NonZeroU32; 1]),
    Two([NonZeroU32; 2]),
    Three([NonZeroU32; 3]),
}

/// Finds an ISO 3166 country code from a phone number.
///
/// Deprecated: This function has poor error handling. It panics if the given number is invalid
/// or unparsable to an integer. Also, it yields an empty string when the territory code is not found.
#[deprecated]
pub fn find_iso_3166(phone: &str) -> &'static str {
    let phone = phone.parse::<u64>().unwrap();
    match TerritoryCode::from_phone_number(phone) {
        Ok(country) => country.name(),
        Err(FromPhoneError::InvalidPhoneNumber) => {
            panic!("phone length needs to be at least 10 digits")
        }
        Err(FromPhoneError::NotFound) => "",
    }
}

/// Finds a phone number calling code from an ISO 3166 country code
///
/// Deprecated: This function has poor error handling. It returns an empty string if the given code
/// is invalid. It also cannot accurately handle territories with multiple calling codes.
#[deprecated]
pub fn find_phone_cc(code: &str) -> &'static str {
    let country = match TerritoryCode::from_name(code) {
        Some(country) => country,
        None => return "",
    };
    match country {
        TerritoryCode::US => "1",
        TerritoryCode::CA => "1",
        TerritoryCode::AC => "247",
        TerritoryCode::AD => "376",
        TerritoryCode::AE => "971",
        TerritoryCode::AF => "93",
        TerritoryCode::AG => "1",
        TerritoryCode::AI => "1",
        TerritoryCode::AL => "355",
        TerritoryCode::AM => "374",
        TerritoryCode::AO => "244",
        TerritoryCode::AQ => "672",
        TerritoryCode::AR => "54",
        TerritoryCode::AS => "1",
        TerritoryCode::AT => "43",
        TerritoryCode::AU => "61",
        TerritoryCode::AW => "297",
        TerritoryCode::AX => "35818",
        TerritoryCode::AZ => "994",
        TerritoryCode::BA => "387",
        TerritoryCode::BB => "1",
        TerritoryCode::BD => "880",
        TerritoryCode::BE => "32",
        TerritoryCode::BF => "226",
        TerritoryCode::BG => "359",
        TerritoryCode::BH => "973",
        TerritoryCode::BI => "257",
        TerritoryCode::BJ => "229",
        TerritoryCode::BM => "1",
        TerritoryCode::BN => "673",
        TerritoryCode::BO => "591",
        TerritoryCode::BQ => "599", //5993,5994,5997
        TerritoryCode::BR => "55",
        TerritoryCode::BS => "1",
        TerritoryCode::BT => "975",
        TerritoryCode::BW => "267",
        TerritoryCode::BY => "375",
        TerritoryCode::BZ => "501",
        TerritoryCode::CC => "6189162",
        TerritoryCode::CD => "243",
        TerritoryCode::CF => "236",
        TerritoryCode::CG => "242",
        TerritoryCode::CH => "41",
        TerritoryCode::CI => "225",
        TerritoryCode::CK => "682",
        TerritoryCode::CL => "56",
        TerritoryCode::CM => "237",
        TerritoryCode::CN => "86",
        TerritoryCode::CO => "57",
        TerritoryCode::CR => "506",
        TerritoryCode::CU => "53",
        TerritoryCode::CV => "238",
        TerritoryCode::CW => "5999",
        TerritoryCode::CX => "6189164",
        TerritoryCode::CY => "357",
        TerritoryCode::CZ => "420",
        TerritoryCode::DE => "49",
        TerritoryCode::DJ => "253",
        TerritoryCode::DK => "45",
        TerritoryCode::DM => "1",
        TerritoryCode::DO => "1",
        TerritoryCode::DZ => "213",
        TerritoryCode::EC => "593",
        TerritoryCode::EE => "372",
        TerritoryCode::EG => "20",
        TerritoryCode::ER => "291",
        TerritoryCode::ES => "34",
        TerritoryCode::ET => "251",
        TerritoryCode::FI => "358",
        TerritoryCode::FJ => "679",
        TerritoryCode::FK => "500",
        TerritoryCode::FM => "691",
        TerritoryCode::FO => "298",
        TerritoryCode::FR => "33",
        TerritoryCode::GA => "241",
        TerritoryCode::GB => "44",
        TerritoryCode::GD => "1",
        TerritoryCode::GE => "995",
        TerritoryCode::GF => "594",
        TerritoryCode::GG => "441481",
        TerritoryCode::GH => "233",
        TerritoryCode::GI => "350",
        TerritoryCode::GL => "299",
        TerritoryCode::GM => "220",
        TerritoryCode::GN => "224",
        TerritoryCode::GP => "590",
        TerritoryCode::GQ => "240",
        TerritoryCode::GR => "30",
        TerritoryCode::GT => "502",
        TerritoryCode::GU => "1",
        TerritoryCode::GW => "245",
        TerritoryCode::GY => "592",
        TerritoryCode::HK => "852",
        TerritoryCode::HN => "504",
        TerritoryCode::HR => "385",
        TerritoryCode::HT => "509",
        TerritoryCode::HU => "36",
        TerritoryCode::ID => "62",
        TerritoryCode::IE => "353",
        TerritoryCode::IL => "972",
        TerritoryCode::IM => "441624",
        TerritoryCode::IN => "91",
        TerritoryCode::IO => "246",
        TerritoryCode::IQ => "964",
        TerritoryCode::IR => "98",
        TerritoryCode::IS => "354",
        TerritoryCode::IT => "39",
        TerritoryCode::JE => "441534",
        TerritoryCode::JM => "1",
        TerritoryCode::JO => "962",
        TerritoryCode::JP => "81",
        TerritoryCode::KE => "254",
        TerritoryCode::KG => "996",
        TerritoryCode::KH => "855",
        TerritoryCode::KI => "686",
        TerritoryCode::KM => "269",
        TerritoryCode::KN => "1",
        TerritoryCode::KP => "850",
        TerritoryCode::KR => "82",
        TerritoryCode::KW => "965",
        TerritoryCode::KY => "1",
        TerritoryCode::KZ => "7",
        TerritoryCode::LA => "856",
        TerritoryCode::LB => "961",
        TerritoryCode::LC => "1",
        TerritoryCode::LI => "423",
        TerritoryCode::LK => "94",
        TerritoryCode::LR => "231",
        TerritoryCode::LS => "266",
        TerritoryCode::LT => "370",
        TerritoryCode::LU => "352",
        TerritoryCode::LV => "371",
        TerritoryCode::LY => "218",
        TerritoryCode::MA => "212",
        TerritoryCode::MC => "377",
        TerritoryCode::MD => "373",
        TerritoryCode::ME => "382",
        TerritoryCode::MG => "261",
        TerritoryCode::MH => "692",
        TerritoryCode::MK => "389",
        TerritoryCode::ML => "223",
        TerritoryCode::MM => "95",
        TerritoryCode::MN => "976",
        TerritoryCode::MO => "853",
        TerritoryCode::MP => "1",
        TerritoryCode::MQ => "596",
        TerritoryCode::MR => "222",
        TerritoryCode::MS => "1",
        TerritoryCode::MT => "356",
        TerritoryCode::MU => "230",
        TerritoryCode::MV => "960",
        TerritoryCode::MW => "265",
        TerritoryCode::MX => "52",
        TerritoryCode::MY => "60",
        TerritoryCode::MZ => "258",
        TerritoryCode::NA => "264",
        TerritoryCode::NC => "687",
        TerritoryCode::NE => "227",
        TerritoryCode::NF => "6723",
        TerritoryCode::NG => "234",
        TerritoryCode::NI => "505",
        TerritoryCode::NL => "31",
        TerritoryCode::NO => "47",
        TerritoryCode::NP => "977",
        TerritoryCode::NR => "674",
        TerritoryCode::NU => "683",
        TerritoryCode::NZ => "64",
        TerritoryCode::OM => "968",
        TerritoryCode::PA => "507",
        TerritoryCode::PE => "51",
        TerritoryCode::PF => "689",
        TerritoryCode::PG => "675",
        TerritoryCode::PH => "63",
        TerritoryCode::PK => "92",
        TerritoryCode::PL => "48",
        TerritoryCode::PM => "508",
        TerritoryCode::PR => "1",
        TerritoryCode::PS => "970",
        TerritoryCode::PT => "351",
        TerritoryCode::PW => "680",
        TerritoryCode::PY => "595",
        TerritoryCode::QA => "974",
        TerritoryCode::RE => "262",
        TerritoryCode::RO => "40",
        TerritoryCode::RS => "381",
        TerritoryCode::RU => "7",
        TerritoryCode::RW => "250",
        TerritoryCode::SA => "966",
        TerritoryCode::SB => "677",
        TerritoryCode::SC => "248",
        TerritoryCode::SD => "249",
        TerritoryCode::SE => "46",
        TerritoryCode::SG => "65",
        TerritoryCode::SH => "290",
        TerritoryCode::SI => "386",
        TerritoryCode::SJ => "4779",
        TerritoryCode::SK => "421",
        TerritoryCode::SL => "232",
        TerritoryCode::SM => "378",
        TerritoryCode::SN => "221",
        TerritoryCode::SO => "252",
        TerritoryCode::SR => "597",
        TerritoryCode::SS => "211",
        TerritoryCode::ST => "239",
        TerritoryCode::SV => "503",
        TerritoryCode::SX => "1",
        TerritoryCode::SY => "963",
        TerritoryCode::SZ => "268",
        TerritoryCode::TC => "1",
        TerritoryCode::TD => "235",
        TerritoryCode::TG => "228",
        TerritoryCode::TH => "66",
        TerritoryCode::TJ => "992",
        TerritoryCode::TK => "690",
        TerritoryCode::TL => "670",
        TerritoryCode::TM => "993",
        TerritoryCode::TN => "216",
        TerritoryCode::TO => "676",
        TerritoryCode::TR => "90",
        TerritoryCode::TT => "1",
        TerritoryCode::TV => "688",
        TerritoryCode::TW => "886",
        TerritoryCode::TZ => "255",
        TerritoryCode::UA => "380",
        TerritoryCode::UG => "256",
        TerritoryCode::UY => "598",
        TerritoryCode::UZ => "998",
        TerritoryCode::VA => "379",
        TerritoryCode::VC => "1",
        TerritoryCode::VE => "58",
        TerritoryCode::VG => "1",
        TerritoryCode::VI => "1",
        TerritoryCode::VN => "84",
        TerritoryCode::VU => "678",
        TerritoryCode::WF => "681",
        TerritoryCode::WS => "685",
        TerritoryCode::XG => "881",
        TerritoryCode::XN => "870",
        TerritoryCode::XP => "878",
        TerritoryCode::XS => "808",
        TerritoryCode::XT => "800",
        TerritoryCode::XV => "882",
        //    CountryCode::XV => "883",
        TerritoryCode::YE => "967",
        TerritoryCode::YT => "262",
        TerritoryCode::ZA => "27",
        TerritoryCode::ZM => "260",
        TerritoryCode::ZW => "263",
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn find_iso_3166_test() {
        assert_eq!("US", find_iso_3166("12069359290"));
        assert_eq!("", find_iso_3166("12229359290"));
        assert_eq!("UZ", find_iso_3166("99893592902"));
        assert_eq!("YT", find_iso_3166("26226992902"));
    }

    #[test]
    fn find_phone_cc_test() {
        assert_eq!("1", find_phone_cc("US"));
        assert_eq!("", find_phone_cc("ZZ"));
    }

    #[test]
    fn get_country_code_equals_old_impl() {
        let mut phone_prefix = HashMap::<u32, _>::new();
        phone_prefix.insert(1201, "US");
        phone_prefix.insert(1202, "US");
        phone_prefix.insert(1203, "US");
        phone_prefix.insert(1204, "CA");
        phone_prefix.insert(1205, "US");
        phone_prefix.insert(1206, "US");
        phone_prefix.insert(1207, "US");
        phone_prefix.insert(1208, "US");
        phone_prefix.insert(1209, "US");
        phone_prefix.insert(1210, "US");
        phone_prefix.insert(1212, "US");
        phone_prefix.insert(1213, "US");
        phone_prefix.insert(1214, "US");
        phone_prefix.insert(1215, "US");
        phone_prefix.insert(1216, "US");
        phone_prefix.insert(1217, "US");
        phone_prefix.insert(1218, "US");
        phone_prefix.insert(1219, "US");
        phone_prefix.insert(1224, "US");
        phone_prefix.insert(1225, "US");
        phone_prefix.insert(1226, "CA");
        phone_prefix.insert(1228, "US");
        phone_prefix.insert(1229, "US");
        phone_prefix.insert(1231, "US");
        phone_prefix.insert(1234, "US");
        phone_prefix.insert(1236, "CA");
        phone_prefix.insert(1239, "US");
        phone_prefix.insert(1240, "US");
        phone_prefix.insert(1242, "BS");
        phone_prefix.insert(1246, "BB");
        phone_prefix.insert(1248, "US");
        phone_prefix.insert(1249, "CA");
        phone_prefix.insert(1250, "CA");
        phone_prefix.insert(1251, "US");
        phone_prefix.insert(1252, "US");
        phone_prefix.insert(1253, "US");
        phone_prefix.insert(1254, "US");
        phone_prefix.insert(1256, "US");
        phone_prefix.insert(1260, "US");
        phone_prefix.insert(1262, "US");
        phone_prefix.insert(1264, "AI");
        phone_prefix.insert(1267, "US");
        phone_prefix.insert(1268, "AG");
        phone_prefix.insert(1269, "US");
        phone_prefix.insert(1270, "US");
        phone_prefix.insert(1272, "US");
        phone_prefix.insert(1274, "US");
        phone_prefix.insert(1276, "US");
        phone_prefix.insert(1281, "US");
        phone_prefix.insert(1284, "VG");
        phone_prefix.insert(1289, "CA");
        phone_prefix.insert(1301, "US");
        phone_prefix.insert(1302, "US");
        phone_prefix.insert(1303, "US");
        phone_prefix.insert(1304, "US");
        phone_prefix.insert(1305, "US");
        phone_prefix.insert(1306, "CA");
        phone_prefix.insert(1307, "US");
        phone_prefix.insert(1308, "US");
        phone_prefix.insert(1309, "US");
        phone_prefix.insert(1310, "US");
        phone_prefix.insert(1312, "US");
        phone_prefix.insert(1313, "US");
        phone_prefix.insert(1314, "US");
        phone_prefix.insert(1315, "US");
        phone_prefix.insert(1316, "US");
        phone_prefix.insert(1317, "US");
        phone_prefix.insert(1318, "US");
        phone_prefix.insert(1319, "US");
        phone_prefix.insert(1320, "US");
        phone_prefix.insert(1321, "US");
        phone_prefix.insert(1323, "US");
        phone_prefix.insert(1325, "US");
        phone_prefix.insert(1330, "US");
        phone_prefix.insert(1331, "US");
        phone_prefix.insert(1334, "US");
        phone_prefix.insert(1336, "US");
        phone_prefix.insert(1337, "US");
        phone_prefix.insert(1339, "US");
        phone_prefix.insert(1340, "VI");
        phone_prefix.insert(1343, "CA");
        phone_prefix.insert(1345, "KY");
        phone_prefix.insert(1346, "US");
        phone_prefix.insert(1347, "US");
        phone_prefix.insert(1351, "US");
        phone_prefix.insert(1352, "US");
        phone_prefix.insert(1360, "US");
        phone_prefix.insert(1361, "US");
        phone_prefix.insert(1364, "US");
        phone_prefix.insert(1365, "CA");
        phone_prefix.insert(1385, "US");
        phone_prefix.insert(1386, "US");
        phone_prefix.insert(1401, "US");
        phone_prefix.insert(1402, "US");
        phone_prefix.insert(1403, "CA");
        phone_prefix.insert(1404, "US");
        phone_prefix.insert(1405, "US");
        phone_prefix.insert(1406, "US");
        phone_prefix.insert(1407, "US");
        phone_prefix.insert(1408, "US");
        phone_prefix.insert(1409, "US");
        phone_prefix.insert(1410, "US");
        phone_prefix.insert(1412, "US");
        phone_prefix.insert(1413, "US");
        phone_prefix.insert(1414, "US");
        phone_prefix.insert(1415, "US");
        phone_prefix.insert(1416, "CA");
        phone_prefix.insert(1417, "US");
        phone_prefix.insert(1418, "CA");
        phone_prefix.insert(1419, "US");
        phone_prefix.insert(1423, "US");
        phone_prefix.insert(1424, "US");
        phone_prefix.insert(1425, "US");
        phone_prefix.insert(1430, "US");
        phone_prefix.insert(1431, "CA");
        phone_prefix.insert(1432, "US");
        phone_prefix.insert(1434, "US");
        phone_prefix.insert(1435, "US");
        phone_prefix.insert(1437, "CA");
        phone_prefix.insert(1438, "CA");
        phone_prefix.insert(1440, "US");
        phone_prefix.insert(1441, "BM");
        phone_prefix.insert(1442, "US");
        phone_prefix.insert(1443, "US");
        phone_prefix.insert(1450, "CA");
        phone_prefix.insert(1457, "CA");
        phone_prefix.insert(1458, "US");
        phone_prefix.insert(1469, "US");
        phone_prefix.insert(1470, "US");
        phone_prefix.insert(1473, "GD");
        phone_prefix.insert(1475, "US");
        phone_prefix.insert(1478, "US");
        phone_prefix.insert(1479, "US");
        phone_prefix.insert(1480, "US");
        phone_prefix.insert(1484, "US");
        phone_prefix.insert(1500, "US");
        phone_prefix.insert(1501, "US");
        phone_prefix.insert(1502, "US");
        phone_prefix.insert(1503, "US");
        phone_prefix.insert(1504, "US");
        phone_prefix.insert(1505, "US");
        phone_prefix.insert(1506, "CA");
        phone_prefix.insert(1507, "US");
        phone_prefix.insert(1508, "US");
        phone_prefix.insert(1509, "US");
        phone_prefix.insert(1510, "US");
        phone_prefix.insert(1512, "US");
        phone_prefix.insert(1513, "US");
        phone_prefix.insert(1514, "CA");
        phone_prefix.insert(1515, "US");
        phone_prefix.insert(1516, "US");
        phone_prefix.insert(1517, "US");
        phone_prefix.insert(1518, "US");
        phone_prefix.insert(1519, "CA");
        phone_prefix.insert(1520, "US");
        phone_prefix.insert(1530, "US");
        phone_prefix.insert(1531, "US");
        phone_prefix.insert(1533, "US");
        phone_prefix.insert(1534, "US");
        phone_prefix.insert(1539, "US");
        phone_prefix.insert(1540, "US");
        phone_prefix.insert(1541, "US");
        phone_prefix.insert(1544, "US");
        phone_prefix.insert(1551, "US");
        phone_prefix.insert(1559, "US");
        phone_prefix.insert(1561, "US");
        phone_prefix.insert(1562, "US");
        phone_prefix.insert(1563, "US");
        phone_prefix.insert(1566, "US");
        phone_prefix.insert(1567, "US");
        phone_prefix.insert(1570, "US");
        phone_prefix.insert(1571, "US");
        phone_prefix.insert(1573, "US");
        phone_prefix.insert(1574, "US");
        phone_prefix.insert(1575, "US");
        phone_prefix.insert(1577, "US");
        phone_prefix.insert(1579, "CA");
        phone_prefix.insert(1580, "US");
        phone_prefix.insert(1581, "CA");
        phone_prefix.insert(1585, "US");
        phone_prefix.insert(1586, "US");
        phone_prefix.insert(1587, "CA");
        phone_prefix.insert(1600, "CA");
        phone_prefix.insert(1601, "US");
        phone_prefix.insert(1602, "US");
        phone_prefix.insert(1603, "US");
        phone_prefix.insert(1604, "CA");
        phone_prefix.insert(1605, "US");
        phone_prefix.insert(1606, "US");
        phone_prefix.insert(1607, "US");
        phone_prefix.insert(1608, "US");
        phone_prefix.insert(1609, "US");
        phone_prefix.insert(1610, "US");
        phone_prefix.insert(1612, "US");
        phone_prefix.insert(1613, "CA");
        phone_prefix.insert(1614, "US");
        phone_prefix.insert(1615, "US");
        phone_prefix.insert(1616, "US");
        phone_prefix.insert(1617, "US");
        phone_prefix.insert(1618, "US");
        phone_prefix.insert(1619, "US");
        phone_prefix.insert(1620, "US");
        phone_prefix.insert(1623, "US");
        phone_prefix.insert(1626, "US");
        phone_prefix.insert(1628, "US");
        phone_prefix.insert(1629, "US");
        phone_prefix.insert(1630, "US");
        phone_prefix.insert(1631, "US");
        phone_prefix.insert(1636, "US");
        phone_prefix.insert(1639, "CA");
        phone_prefix.insert(1641, "US");
        phone_prefix.insert(1646, "US");
        phone_prefix.insert(1647, "CA");
        phone_prefix.insert(1649, "TC");
        phone_prefix.insert(1650, "US");
        phone_prefix.insert(1651, "US");
        phone_prefix.insert(1657, "US");
        phone_prefix.insert(1660, "US");
        phone_prefix.insert(1661, "US");
        phone_prefix.insert(1662, "US");
        phone_prefix.insert(1664, "MS");
        phone_prefix.insert(1667, "US");
        phone_prefix.insert(1669, "US");
        phone_prefix.insert(1670, "MP");
        phone_prefix.insert(1671, "GU");
        phone_prefix.insert(1678, "US");
        phone_prefix.insert(1681, "US");
        phone_prefix.insert(1682, "US");
        phone_prefix.insert(1684, "AS");
        phone_prefix.insert(1700, "US");
        phone_prefix.insert(1701, "US");
        phone_prefix.insert(1702, "US");
        phone_prefix.insert(1703, "US");
        phone_prefix.insert(1704, "US");
        phone_prefix.insert(1705, "CA");
        phone_prefix.insert(1706, "US");
        phone_prefix.insert(1707, "US");
        phone_prefix.insert(1708, "US");
        phone_prefix.insert(1709, "CA");
        phone_prefix.insert(1710, "US");
        phone_prefix.insert(1712, "US");
        phone_prefix.insert(1713, "US");
        phone_prefix.insert(1714, "US");
        phone_prefix.insert(1715, "US");
        phone_prefix.insert(1716, "US");
        phone_prefix.insert(1717, "US");
        phone_prefix.insert(1718, "US");
        phone_prefix.insert(1719, "US");
        phone_prefix.insert(1720, "US");
        phone_prefix.insert(1721, "SX");
        phone_prefix.insert(1724, "US");
        phone_prefix.insert(1725, "US");
        phone_prefix.insert(1727, "US");
        phone_prefix.insert(1731, "US");
        phone_prefix.insert(1732, "US");
        phone_prefix.insert(1734, "US");
        phone_prefix.insert(1737, "US");
        phone_prefix.insert(1740, "US");
        phone_prefix.insert(1747, "US");
        phone_prefix.insert(1754, "US");
        phone_prefix.insert(1757, "US");
        phone_prefix.insert(1758, "LC");
        phone_prefix.insert(1760, "US");
        phone_prefix.insert(1762, "US");
        phone_prefix.insert(1763, "US");
        phone_prefix.insert(1765, "US");
        phone_prefix.insert(1767, "DM");
        phone_prefix.insert(1769, "US");
        phone_prefix.insert(1770, "US");
        phone_prefix.insert(1772, "US");
        phone_prefix.insert(1773, "US");
        phone_prefix.insert(1774, "US");
        phone_prefix.insert(1775, "US");
        phone_prefix.insert(1778, "CA");
        phone_prefix.insert(1779, "US");
        phone_prefix.insert(1780, "CA");
        phone_prefix.insert(1781, "US");
        phone_prefix.insert(1782, "CA");
        phone_prefix.insert(1784, "VC");
        phone_prefix.insert(1785, "US");
        phone_prefix.insert(1786, "US");
        phone_prefix.insert(1787, "PR");
        phone_prefix.insert(1800, "US");
        phone_prefix.insert(1801, "US");
        phone_prefix.insert(1802, "US");
        phone_prefix.insert(1803, "US");
        phone_prefix.insert(1804, "US");
        phone_prefix.insert(1805, "US");
        phone_prefix.insert(1806, "US");
        phone_prefix.insert(1807, "CA");
        phone_prefix.insert(1808, "US");
        phone_prefix.insert(1809, "DO");
        phone_prefix.insert(1810, "US");
        phone_prefix.insert(1812, "US");
        phone_prefix.insert(1813, "US");
        phone_prefix.insert(1814, "US");
        phone_prefix.insert(1815, "US");
        phone_prefix.insert(1816, "US");
        phone_prefix.insert(1817, "US");
        phone_prefix.insert(1818, "US");
        phone_prefix.insert(1819, "CA");
        phone_prefix.insert(1825, "CA");
        phone_prefix.insert(1828, "US");
        phone_prefix.insert(1829, "DO");
        phone_prefix.insert(1830, "US");
        phone_prefix.insert(1831, "US");
        phone_prefix.insert(1832, "US");
        phone_prefix.insert(1843, "US");
        phone_prefix.insert(1844, "US");
        phone_prefix.insert(1845, "US");
        phone_prefix.insert(1847, "US");
        phone_prefix.insert(1848, "US");
        phone_prefix.insert(1849, "DO");
        phone_prefix.insert(1850, "US");
        phone_prefix.insert(1855, "US");
        phone_prefix.insert(1856, "US");
        phone_prefix.insert(1857, "US");
        phone_prefix.insert(1858, "US");
        phone_prefix.insert(1859, "US");
        phone_prefix.insert(1860, "US");
        phone_prefix.insert(1862, "US");
        phone_prefix.insert(1863, "US");
        phone_prefix.insert(1864, "US");
        phone_prefix.insert(1865, "US");
        phone_prefix.insert(1866, "US");
        phone_prefix.insert(1867, "CA");
        phone_prefix.insert(1868, "TT");
        phone_prefix.insert(1869, "KN");
        phone_prefix.insert(1870, "US");
        phone_prefix.insert(1872, "US");
        phone_prefix.insert(1873, "CA");
        phone_prefix.insert(1876, "JM");
        phone_prefix.insert(1877, "US");
        phone_prefix.insert(1878, "US");
        phone_prefix.insert(1888, "US");
        phone_prefix.insert(1900, "US");
        phone_prefix.insert(1901, "US");
        phone_prefix.insert(1902, "CA");
        phone_prefix.insert(1903, "US");
        phone_prefix.insert(1904, "US");
        phone_prefix.insert(1905, "CA");
        phone_prefix.insert(1906, "US");
        phone_prefix.insert(1907, "US");
        phone_prefix.insert(1908, "US");
        phone_prefix.insert(1909, "US");
        phone_prefix.insert(1910, "US");
        phone_prefix.insert(1912, "US");
        phone_prefix.insert(1913, "US");
        phone_prefix.insert(1914, "US");
        phone_prefix.insert(1915, "US");
        phone_prefix.insert(1916, "US");
        phone_prefix.insert(1917, "US");
        phone_prefix.insert(1918, "US");
        phone_prefix.insert(1919, "US");
        phone_prefix.insert(1920, "US");
        phone_prefix.insert(1925, "US");
        phone_prefix.insert(1928, "US");
        phone_prefix.insert(1929, "US");
        phone_prefix.insert(1930, "US");
        phone_prefix.insert(1931, "US");
        phone_prefix.insert(1935, "US");
        phone_prefix.insert(1936, "US");
        phone_prefix.insert(1937, "US");
        phone_prefix.insert(1938, "US");
        phone_prefix.insert(1939, "PR");
        phone_prefix.insert(1940, "US");
        phone_prefix.insert(1941, "US");
        phone_prefix.insert(1947, "US");
        phone_prefix.insert(1949, "US");
        phone_prefix.insert(1951, "US");
        phone_prefix.insert(1952, "US");
        phone_prefix.insert(1954, "US");
        phone_prefix.insert(1956, "US");
        phone_prefix.insert(1959, "US");
        phone_prefix.insert(1970, "US");
        phone_prefix.insert(1971, "US");
        phone_prefix.insert(1972, "US");
        phone_prefix.insert(1973, "US");
        phone_prefix.insert(1978, "US");
        phone_prefix.insert(1979, "US");
        phone_prefix.insert(1980, "US");
        phone_prefix.insert(1984, "US");
        phone_prefix.insert(1985, "US");
        phone_prefix.insert(1989, "US");
        phone_prefix.insert(20, "EG");
        phone_prefix.insert(211, "SS");
        phone_prefix.insert(212, "MA");
        phone_prefix.insert(213, "DZ");
        phone_prefix.insert(216, "TN");
        phone_prefix.insert(218, "LY");
        phone_prefix.insert(220, "GM");
        phone_prefix.insert(221, "SN");
        phone_prefix.insert(222, "MR");
        phone_prefix.insert(223, "ML");
        phone_prefix.insert(224, "GN");
        phone_prefix.insert(225, "CI");
        phone_prefix.insert(226, "BF");
        phone_prefix.insert(227, "NE");
        phone_prefix.insert(228, "TG");
        phone_prefix.insert(229, "BJ");
        phone_prefix.insert(230, "MU");
        phone_prefix.insert(231, "LR");
        phone_prefix.insert(232, "SL");
        phone_prefix.insert(233, "GH");
        phone_prefix.insert(234, "NG");
        phone_prefix.insert(235, "TD");
        phone_prefix.insert(236, "CF");
        phone_prefix.insert(237, "CM");
        phone_prefix.insert(238, "CV");
        phone_prefix.insert(239, "ST");
        phone_prefix.insert(240, "GQ");
        phone_prefix.insert(241, "GA");
        phone_prefix.insert(242, "CG");
        phone_prefix.insert(243, "CD");
        phone_prefix.insert(244, "AO");
        phone_prefix.insert(245, "GW");
        phone_prefix.insert(246, "IO");
        phone_prefix.insert(247, "AC");
        phone_prefix.insert(248, "SC");
        phone_prefix.insert(249, "SD");
        phone_prefix.insert(250, "RW");
        phone_prefix.insert(251, "ET");
        phone_prefix.insert(252, "SO");
        phone_prefix.insert(253, "DJ");
        phone_prefix.insert(254, "KE");
        phone_prefix.insert(255, "TZ");
        phone_prefix.insert(256, "UG");
        phone_prefix.insert(257, "BI");
        phone_prefix.insert(258, "MZ");
        phone_prefix.insert(260, "ZM");
        phone_prefix.insert(261, "MG");
        phone_prefix.insert(262269, "YT");
        phone_prefix.insert(262639, "YT");
        phone_prefix.insert(262, "RE");
        phone_prefix.insert(263, "ZW");
        phone_prefix.insert(264, "NA");
        phone_prefix.insert(265, "MW");
        phone_prefix.insert(266, "LS");
        phone_prefix.insert(267, "BW");
        phone_prefix.insert(268, "SZ");
        phone_prefix.insert(269, "KM");
        phone_prefix.insert(27, "ZA");
        phone_prefix.insert(290, "SH");
        phone_prefix.insert(291, "ER");
        phone_prefix.insert(297, "AW");
        phone_prefix.insert(298, "FO");
        phone_prefix.insert(299, "GL");
        phone_prefix.insert(30, "GR");
        phone_prefix.insert(31, "NL");
        phone_prefix.insert(32, "BE");
        phone_prefix.insert(33, "FR");
        phone_prefix.insert(34, "ES");
        phone_prefix.insert(350, "GI");
        phone_prefix.insert(351, "PT");
        phone_prefix.insert(352, "LU");
        phone_prefix.insert(353, "IE");
        phone_prefix.insert(354, "IS");
        phone_prefix.insert(355, "AL");
        phone_prefix.insert(356, "MT");
        phone_prefix.insert(357, "CY");
        phone_prefix.insert(358, "FI");
        phone_prefix.insert(35818, "AX");
        phone_prefix.insert(359, "BG");
        phone_prefix.insert(36, "HU");
        phone_prefix.insert(370, "LT");
        phone_prefix.insert(371, "LV");
        phone_prefix.insert(372, "EE");
        phone_prefix.insert(373, "MD");
        phone_prefix.insert(374, "AM");
        phone_prefix.insert(375, "BY");
        phone_prefix.insert(376, "AD");
        phone_prefix.insert(377, "MC");
        phone_prefix.insert(378, "SM");
        phone_prefix.insert(379, "VA");
        phone_prefix.insert(380, "UA");
        phone_prefix.insert(381, "RS");
        phone_prefix.insert(382, "ME");
        phone_prefix.insert(385, "HR");
        phone_prefix.insert(386, "SI");
        phone_prefix.insert(387, "BA");
        phone_prefix.insert(389, "MK");
        phone_prefix.insert(39, "IT");
        phone_prefix.insert(40, "RO");
        phone_prefix.insert(41, "CH");
        phone_prefix.insert(420, "CZ");
        phone_prefix.insert(421, "SK");
        phone_prefix.insert(423, "LI");
        phone_prefix.insert(43, "AT");
        phone_prefix.insert(441481, "GG");
        phone_prefix.insert(441624, "IM");
        phone_prefix.insert(441534, "JE");
        phone_prefix.insert(44, "GB");
        phone_prefix.insert(45, "DK");
        phone_prefix.insert(46, "SE");
        phone_prefix.insert(47, "NO");
        phone_prefix.insert(4779, "SJ");
        phone_prefix.insert(48, "PL");
        phone_prefix.insert(49, "DE");
        phone_prefix.insert(500, "FK");
        phone_prefix.insert(501, "BZ");
        phone_prefix.insert(502, "GT");
        phone_prefix.insert(503, "SV");
        phone_prefix.insert(504, "HN");
        phone_prefix.insert(505, "NI");
        phone_prefix.insert(506, "CR");
        phone_prefix.insert(507, "PA");
        phone_prefix.insert(508, "PM");
        phone_prefix.insert(509, "HT");
        phone_prefix.insert(51, "PE");
        phone_prefix.insert(52, "MX");
        phone_prefix.insert(53, "CU");
        phone_prefix.insert(54, "AR");
        phone_prefix.insert(55, "BR");
        phone_prefix.insert(56, "CL");
        phone_prefix.insert(57, "CO");
        phone_prefix.insert(58, "VE");
        phone_prefix.insert(590, "GP");
        phone_prefix.insert(591, "BO");
        phone_prefix.insert(592, "GY");
        phone_prefix.insert(593, "EC");
        phone_prefix.insert(594, "GF");
        phone_prefix.insert(595, "PY");
        phone_prefix.insert(596, "MQ");
        phone_prefix.insert(597, "SR");
        phone_prefix.insert(598, "UY");
        phone_prefix.insert(5993, "BQ");
        phone_prefix.insert(5994, "BQ");
        phone_prefix.insert(5997, "BQ");
        phone_prefix.insert(5999, "CW");
        phone_prefix.insert(60, "MY");
        phone_prefix.insert(61, "AU");
        phone_prefix.insert(6189164, "CX");
        phone_prefix.insert(6189162, "CC");
        phone_prefix.insert(62, "ID");
        phone_prefix.insert(63, "PH");
        phone_prefix.insert(64, "NZ");
        phone_prefix.insert(65, "SG");
        phone_prefix.insert(66, "TH");
        phone_prefix.insert(670, "TL");
        phone_prefix.insert(6721, "AQ");
        phone_prefix.insert(6723, "NF");
        phone_prefix.insert(673, "BN");
        phone_prefix.insert(674, "NR");
        phone_prefix.insert(675, "PG");
        phone_prefix.insert(676, "TO");
        phone_prefix.insert(677, "SB");
        phone_prefix.insert(678, "VU");
        phone_prefix.insert(679, "FJ");
        phone_prefix.insert(680, "PW");
        phone_prefix.insert(681, "WF");
        phone_prefix.insert(682, "CK");
        phone_prefix.insert(683, "NU");
        phone_prefix.insert(685, "WS");
        phone_prefix.insert(686, "KI");
        phone_prefix.insert(687, "NC");
        phone_prefix.insert(688, "TV");
        phone_prefix.insert(689, "PF");
        phone_prefix.insert(690, "TK");
        phone_prefix.insert(691, "FM");
        phone_prefix.insert(692, "MH");
        phone_prefix.insert(7, "RU");
        phone_prefix.insert(76, "KZ");
        phone_prefix.insert(77, "KZ");
        phone_prefix.insert(800, "XT");
        phone_prefix.insert(808, "XS");
        phone_prefix.insert(81, "JP");
        phone_prefix.insert(82, "KR");
        phone_prefix.insert(84, "VN");
        phone_prefix.insert(850, "KP");
        phone_prefix.insert(852, "HK");
        phone_prefix.insert(853, "MO");
        phone_prefix.insert(855, "KH");
        phone_prefix.insert(856, "LA");
        phone_prefix.insert(86, "CN");
        phone_prefix.insert(870, "XN");
        phone_prefix.insert(878, "XP");
        phone_prefix.insert(880, "BD");
        phone_prefix.insert(881, "XG");
        phone_prefix.insert(882, "XV");
        phone_prefix.insert(883, "XV");
        phone_prefix.insert(886, "TW");
        phone_prefix.insert(90, "TR");
        phone_prefix.insert(91, "IN");
        phone_prefix.insert(92, "PK");
        phone_prefix.insert(93, "AF");
        phone_prefix.insert(94, "LK");
        phone_prefix.insert(95, "MM");
        phone_prefix.insert(960, "MV");
        phone_prefix.insert(961, "LB");
        phone_prefix.insert(962, "JO");
        phone_prefix.insert(963, "SY");
        phone_prefix.insert(964, "IQ");
        phone_prefix.insert(965, "KW");
        phone_prefix.insert(966, "SA");
        phone_prefix.insert(967, "YE");
        phone_prefix.insert(968, "OM");
        phone_prefix.insert(970, "PS");
        phone_prefix.insert(971, "AE");
        phone_prefix.insert(972, "IL");
        phone_prefix.insert(973, "BH");
        phone_prefix.insert(974, "QA");
        phone_prefix.insert(975, "BT");
        phone_prefix.insert(976, "MN");
        phone_prefix.insert(977, "NP");
        phone_prefix.insert(98, "IR");
        phone_prefix.insert(992, "TJ");
        phone_prefix.insert(993, "TM");
        phone_prefix.insert(994, "AZ");
        phone_prefix.insert(995, "GE");
        phone_prefix.insert(996, "KG");
        phone_prefix.insert(998, "UZ");

        for (prefix, country_name) in phone_prefix {
            // Generate a 10-digit phone number (just pretending)
            let prefix = prefix as u64;
            let prefix_len = prefix.ilog10() + 1;
            let sample_phone = prefix * 10_u64.pow(10 - prefix_len);

            let country = TerritoryCode::from_name(country_name);
            assert!(country.is_some());
            let country = country.unwrap();

            assert_eq!(Ok(country), TerritoryCode::from_phone_number(sample_phone));
            assert_eq!(
                country_name,
                find_iso_3166(&sample_phone.to_string()),
                "deprecated but functional"
            )
        }
    }

    #[test]
    fn get_phone_code_equals_old_impl() {
        let mut iso_3166 = HashMap::new();
        iso_3166.insert("US", "1");
        iso_3166.insert("CA", "1");
        iso_3166.insert("AC", "247");
        iso_3166.insert("AD", "376");
        iso_3166.insert("AE", "971");
        iso_3166.insert("AF", "93");
        iso_3166.insert("AG", "1");
        iso_3166.insert("AI", "1");
        iso_3166.insert("AL", "355");
        iso_3166.insert("AM", "374");
        iso_3166.insert("AO", "244");
        iso_3166.insert("AQ", "672");
        iso_3166.insert("AR", "54");
        iso_3166.insert("AS", "1");
        iso_3166.insert("AT", "43");
        iso_3166.insert("AU", "61");
        iso_3166.insert("AW", "297");
        iso_3166.insert("AX", "35818");
        iso_3166.insert("AZ", "994");
        iso_3166.insert("BA", "387");
        iso_3166.insert("BB", "1");
        iso_3166.insert("BD", "880");
        iso_3166.insert("BE", "32");
        iso_3166.insert("BF", "226");
        iso_3166.insert("BG", "359");
        iso_3166.insert("BH", "973");
        iso_3166.insert("BI", "257");
        iso_3166.insert("BJ", "229");
        iso_3166.insert("BM", "1");
        iso_3166.insert("BN", "673");
        iso_3166.insert("BO", "591");
        iso_3166.insert("BQ", "599"); //5993,5994,5997
        iso_3166.insert("BR", "55");
        iso_3166.insert("BS", "1");
        iso_3166.insert("BT", "975");
        iso_3166.insert("BW", "267");
        iso_3166.insert("BY", "375");
        iso_3166.insert("BZ", "501");
        iso_3166.insert("CC", "6189162");
        iso_3166.insert("CD", "243");
        iso_3166.insert("CF", "236");
        iso_3166.insert("CG", "242");
        iso_3166.insert("CH", "41");
        iso_3166.insert("CI", "225");
        iso_3166.insert("CK", "682");
        iso_3166.insert("CL", "56");
        iso_3166.insert("CM", "237");
        iso_3166.insert("CN", "86");
        iso_3166.insert("CO", "57");
        iso_3166.insert("CR", "506");
        iso_3166.insert("CU", "53");
        iso_3166.insert("CV", "238");
        iso_3166.insert("CW", "5999");
        iso_3166.insert("CX", "6189164");
        iso_3166.insert("CY", "357");
        iso_3166.insert("CZ", "420");
        iso_3166.insert("DE", "49");
        iso_3166.insert("DJ", "253");
        iso_3166.insert("DK", "45");
        iso_3166.insert("DM", "1");
        iso_3166.insert("DO", "1");
        iso_3166.insert("DZ", "213");
        iso_3166.insert("EC", "593");
        iso_3166.insert("EE", "372");
        iso_3166.insert("EG", "20");
        iso_3166.insert("ER", "291");
        iso_3166.insert("ES", "34");
        iso_3166.insert("ET", "251");
        iso_3166.insert("FI", "358");
        iso_3166.insert("FJ", "679");
        iso_3166.insert("FK", "500");
        iso_3166.insert("FM", "691");
        iso_3166.insert("FO", "298");
        iso_3166.insert("FR", "33");
        iso_3166.insert("GA", "241");
        iso_3166.insert("GB", "44");
        iso_3166.insert("GD", "1");
        iso_3166.insert("GE", "995");
        iso_3166.insert("GF", "594");
        iso_3166.insert("GG", "441481");
        iso_3166.insert("GH", "233");
        iso_3166.insert("GI", "350");
        iso_3166.insert("GL", "299");
        iso_3166.insert("GM", "220");
        iso_3166.insert("GN", "224");
        iso_3166.insert("GP", "590");
        iso_3166.insert("GQ", "240");
        iso_3166.insert("GR", "30");
        iso_3166.insert("GT", "502");
        iso_3166.insert("GU", "1");
        iso_3166.insert("GW", "245");
        iso_3166.insert("GY", "592");
        iso_3166.insert("HK", "852");
        iso_3166.insert("HN", "504");
        iso_3166.insert("HR", "385");
        iso_3166.insert("HT", "509");
        iso_3166.insert("HU", "36");
        iso_3166.insert("ID", "62");
        iso_3166.insert("IE", "353");
        iso_3166.insert("IL", "972");
        iso_3166.insert("IM", "441624");
        iso_3166.insert("IN", "91");
        iso_3166.insert("IO", "246");
        iso_3166.insert("IQ", "964");
        iso_3166.insert("IR", "98");
        iso_3166.insert("IS", "354");
        iso_3166.insert("IT", "39");
        iso_3166.insert("JE", "441534");
        iso_3166.insert("JM", "1");
        iso_3166.insert("JO", "962");
        iso_3166.insert("JP", "81");
        iso_3166.insert("KE", "254");
        iso_3166.insert("KG", "996");
        iso_3166.insert("KH", "855");
        iso_3166.insert("KI", "686");
        iso_3166.insert("KM", "269");
        iso_3166.insert("KN", "1");
        iso_3166.insert("KP", "850");
        iso_3166.insert("KR", "82");
        iso_3166.insert("KW", "965");
        iso_3166.insert("KY", "1");
        iso_3166.insert("KZ", "7");
        iso_3166.insert("LA", "856");
        iso_3166.insert("LB", "961");
        iso_3166.insert("LC", "1");
        iso_3166.insert("LI", "423");
        iso_3166.insert("LK", "94");
        iso_3166.insert("LR", "231");
        iso_3166.insert("LS", "266");
        iso_3166.insert("LT", "370");
        iso_3166.insert("LU", "352");
        iso_3166.insert("LV", "371");
        iso_3166.insert("LY", "218");
        iso_3166.insert("MA", "212");
        iso_3166.insert("MC", "377");
        iso_3166.insert("MD", "373");
        iso_3166.insert("ME", "382");
        iso_3166.insert("MG", "261");
        iso_3166.insert("MH", "692");
        iso_3166.insert("MK", "389");
        iso_3166.insert("ML", "223");
        iso_3166.insert("MM", "95");
        iso_3166.insert("MN", "976");
        iso_3166.insert("MO", "853");
        iso_3166.insert("MP", "1");
        iso_3166.insert("MQ", "596");
        iso_3166.insert("MR", "222");
        iso_3166.insert("MS", "1");
        iso_3166.insert("MT", "356");
        iso_3166.insert("MU", "230");
        iso_3166.insert("MV", "960");
        iso_3166.insert("MW", "265");
        iso_3166.insert("MX", "52");
        iso_3166.insert("MY", "60");
        iso_3166.insert("MZ", "258");
        iso_3166.insert("NA", "264");
        iso_3166.insert("NC", "687");
        iso_3166.insert("NE", "227");
        iso_3166.insert("NF", "6723");
        iso_3166.insert("NG", "234");
        iso_3166.insert("NI", "505");
        iso_3166.insert("NL", "31");
        iso_3166.insert("NO", "47");
        iso_3166.insert("NP", "977");
        iso_3166.insert("NR", "674");
        iso_3166.insert("NU", "683");
        iso_3166.insert("NZ", "64");
        iso_3166.insert("OM", "968");
        iso_3166.insert("PA", "507");
        iso_3166.insert("PE", "51");
        iso_3166.insert("PF", "689");
        iso_3166.insert("PG", "675");
        iso_3166.insert("PH", "63");
        iso_3166.insert("PK", "92");
        iso_3166.insert("PL", "48");
        iso_3166.insert("PM", "508");
        iso_3166.insert("PR", "1");
        iso_3166.insert("PS", "970");
        iso_3166.insert("PT", "351");
        iso_3166.insert("PW", "680");
        iso_3166.insert("PY", "595");
        iso_3166.insert("QA", "974");
        iso_3166.insert("RE", "262");
        iso_3166.insert("RO", "40");
        iso_3166.insert("RS", "381");
        iso_3166.insert("RU", "7");
        iso_3166.insert("RW", "250");
        iso_3166.insert("SA", "966");
        iso_3166.insert("SB", "677");
        iso_3166.insert("SC", "248");
        iso_3166.insert("SD", "249");
        iso_3166.insert("SE", "46");
        iso_3166.insert("SG", "65");
        iso_3166.insert("SH", "290");
        iso_3166.insert("SI", "386");
        iso_3166.insert("SJ", "4779");
        iso_3166.insert("SK", "421");
        iso_3166.insert("SL", "232");
        iso_3166.insert("SM", "378");
        iso_3166.insert("SN", "221");
        iso_3166.insert("SO", "252");
        iso_3166.insert("SR", "597");
        iso_3166.insert("SS", "211");
        iso_3166.insert("ST", "239");
        iso_3166.insert("SV", "503");
        iso_3166.insert("SX", "1");
        iso_3166.insert("SY", "963");
        iso_3166.insert("SZ", "268");
        iso_3166.insert("TC", "1");
        iso_3166.insert("TD", "235");
        iso_3166.insert("TG", "228");
        iso_3166.insert("TH", "66");
        iso_3166.insert("TJ", "992");
        iso_3166.insert("TK", "690");
        iso_3166.insert("TL", "670");
        iso_3166.insert("TM", "993");
        iso_3166.insert("TN", "216");
        iso_3166.insert("TO", "676");
        iso_3166.insert("TR", "90");
        iso_3166.insert("TT", "1");
        iso_3166.insert("TV", "688");
        iso_3166.insert("TW", "886");
        iso_3166.insert("TZ", "255");
        iso_3166.insert("UA", "380");
        iso_3166.insert("UG", "256");
        iso_3166.insert("UY", "598");
        iso_3166.insert("UZ", "998");
        iso_3166.insert("VA", "379");
        iso_3166.insert("VC", "1");
        iso_3166.insert("VE", "58");
        iso_3166.insert("VG", "1");
        iso_3166.insert("VI", "1");
        iso_3166.insert("VN", "84");
        iso_3166.insert("VU", "678");
        iso_3166.insert("WF", "681");
        iso_3166.insert("WS", "685");
        iso_3166.insert("XG", "881");
        iso_3166.insert("XN", "870");
        iso_3166.insert("XP", "878");
        iso_3166.insert("XS", "808");
        iso_3166.insert("XT", "800");
        iso_3166.insert("XV", "882");
        //iso_3166.insert("XV", "883");
        iso_3166.insert("YE", "967");
        iso_3166.insert("YT", "262");
        iso_3166.insert("ZA", "27");
        iso_3166.insert("ZM", "260");
        iso_3166.insert("ZW", "263");

        for (country, phone_code) in iso_3166 {
            assert_eq!(
                phone_code,
                find_phone_cc(country),
                "deprecated but functional"
            );

            let country = TerritoryCode::from_name(country);
            assert!(country.is_some());
            let country = country.unwrap();
            assert!(country
                .calling_codes()
                .primary()
                .to_string()
                .starts_with(phone_code));
        }
    }
}
