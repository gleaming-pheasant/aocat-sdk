//! Edit this with EXTREME caution. Reordering the ISO 639-1 codes by anything other than
//! alphabetical, or adding/removing any of the codes will break the indexing.
use std::fmt;

use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::{Serialize, Serializer};

use crate::Error;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Language {
    Aa,
    Ab,
    Ae,
    Af,
    Ak,
    Am,
    An,
    Ar,
    As,
    Av,
    Ay,
    Az,
    Ba,
    Be,
    Bg,
    Bi,
    Bm,
    Bn,
    Bo,
    Br,
    Bs,
    Ca,
    Ce,
    Ch,
    Co,
    Cr,
    Cs,
    Cu,
    Cv,
    Cy,
    Da,
    De,
    Dv,
    Dz,
    Ee,
    El,
    #[default]
    En,
    Eo,
    Es,
    Et,
    Eu,
    Fa,
    Ff,
    Fi,
    Fj,
    Fo,
    Fr,
    Fy,
    Ga,
    Gd,
    Gl,
    Gn,
    Gu,
    Gv,
    Ha,
    He,
    Hi,
    Ho,
    Hr,
    Ht,
    Hu,
    Hy,
    Hz,
    Ia,
    Id,
    Ie,
    Ig,
    Ii,
    Ik,
    Io,
    Is,
    It,
    Iu,
    Ja,
    Jv,
    Ka,
    Kg,
    Ki,
    Kk,
    Kl,
    Km,
    Kn,
    Ko,
    Kr,
    Ks,
    Ku,
    Kv,
    Kw,
    Ky,
    La,
    Lb,
    Lg,
    Li,
    Ln,
    Lo,
    Lt,
    Lu,
    Lv,
    Mg,
    Mh,
    Mi,
    Mk,
    Ml,
    Mn,
    Mr,
    Ms,
    Mt,
    My,
    Na,
    Nb,
    Ne,
    Ng,
    Nl,
    Nn,
    No,
    Nr,
    Nv,
    Ny,
    Oc,
    Oj,
    Om,
    Or,
    Os,
    Pa,
    Pi,
    Pl,
    Ps,
    Pt,
    Qu,
    Rm,
    Rn,
    Ro,
    Ru,
    Rw,
    Sa,
    Sc,
    Sd,
    Se,
    Sg,
    Si,
    Sk,
    Sl,
    Sm,
    Sn,
    So,
    Sq,
    Sr,
    Ss,
    St,
    Su,
    Sv,
    Sw,
    Ta,
    Te,
    Tg,
    Th,
    Ti,
    Tk,
    Tl,
    Tn,
    To,
    Tr,
    Ts,
    Tt,
    Tw,
    Ty,
    Ug,
    Uk,
    Ur,
    Uz,
    Ve,
    Vi,
    Vo,
    Wa,
    Wo,
    Xh,
    Yi,
    Yo,
    Za,
    Zh,
    Zu,
}

struct LanguageInner {
    iso_639_1: [u8; 2],
    iso_639_2: [u8; 3],
    english_name: &'static str, // optional; drop it if you want the absolute minimum
}

// Index == Language discriminant. Keep the order identical to the enum.
static LANGUAGES: [LanguageInner; Language::COUNT] = [
    LanguageInner {
        iso_639_1: *b"aa",
        iso_639_2: *b"aar",
        english_name: "Afar",
    },
    LanguageInner {
        iso_639_1: *b"ab",
        iso_639_2: *b"abk",
        english_name: "Abkhazian",
    },
    LanguageInner {
        iso_639_1: *b"ae",
        iso_639_2: *b"ave",
        english_name: "Avestan",
    },
    LanguageInner {
        iso_639_1: *b"af",
        iso_639_2: *b"afr",
        english_name: "Afrikaans",
    },
    LanguageInner {
        iso_639_1: *b"ak",
        iso_639_2: *b"aka",
        english_name: "Akan",
    },
    LanguageInner {
        iso_639_1: *b"am",
        iso_639_2: *b"amh",
        english_name: "Amharic",
    },
    LanguageInner {
        iso_639_1: *b"an",
        iso_639_2: *b"arg",
        english_name: "Aragonese",
    },
    LanguageInner {
        iso_639_1: *b"ar",
        iso_639_2: *b"ara",
        english_name: "Arabic",
    },
    LanguageInner {
        iso_639_1: *b"as",
        iso_639_2: *b"asm",
        english_name: "Assamese",
    },
    LanguageInner {
        iso_639_1: *b"av",
        iso_639_2: *b"ava",
        english_name: "Avaric",
    },
    LanguageInner {
        iso_639_1: *b"ay",
        iso_639_2: *b"aym",
        english_name: "Aymara",
    },
    LanguageInner {
        iso_639_1: *b"az",
        iso_639_2: *b"aze",
        english_name: "Azerbaijani",
    },
    LanguageInner {
        iso_639_1: *b"ba",
        iso_639_2: *b"bak",
        english_name: "Bashkir",
    },
    LanguageInner {
        iso_639_1: *b"be",
        iso_639_2: *b"bel",
        english_name: "Belarusian",
    },
    LanguageInner {
        iso_639_1: *b"bg",
        iso_639_2: *b"bul",
        english_name: "Bulgarian",
    },
    LanguageInner {
        iso_639_1: *b"bi",
        iso_639_2: *b"bis",
        english_name: "Bislama",
    },
    LanguageInner {
        iso_639_1: *b"bm",
        iso_639_2: *b"bam",
        english_name: "Bambara",
    },
    LanguageInner {
        iso_639_1: *b"bn",
        iso_639_2: *b"ben",
        english_name: "Bengali",
    },
    LanguageInner {
        iso_639_1: *b"bo",
        iso_639_2: *b"bod",
        english_name: "Tibetan",
    },
    LanguageInner {
        iso_639_1: *b"br",
        iso_639_2: *b"bre",
        english_name: "Breton",
    },
    LanguageInner {
        iso_639_1: *b"bs",
        iso_639_2: *b"bos",
        english_name: "Bosnian",
    },
    LanguageInner {
        iso_639_1: *b"ca",
        iso_639_2: *b"cat",
        english_name: "Valencian",
    },
    LanguageInner {
        iso_639_1: *b"ce",
        iso_639_2: *b"che",
        english_name: "Chechen",
    },
    LanguageInner {
        iso_639_1: *b"ch",
        iso_639_2: *b"cha",
        english_name: "Chamorro",
    },
    LanguageInner {
        iso_639_1: *b"co",
        iso_639_2: *b"cos",
        english_name: "Corsican",
    },
    LanguageInner {
        iso_639_1: *b"cr",
        iso_639_2: *b"cre",
        english_name: "Cree",
    },
    LanguageInner {
        iso_639_1: *b"cs",
        iso_639_2: *b"cze",
        english_name: "Czech",
    },
    LanguageInner {
        iso_639_1: *b"cu",
        iso_639_2: *b"chu",
        english_name: "Church Slavic",
    },
    LanguageInner {
        iso_639_1: *b"cv",
        iso_639_2: *b"chv",
        english_name: "Chuvash",
    },
    LanguageInner {
        iso_639_1: *b"cy",
        iso_639_2: *b"cym",
        english_name: "Welsh",
    },
    LanguageInner {
        iso_639_1: *b"da",
        iso_639_2: *b"dan",
        english_name: "Danish",
    },
    LanguageInner {
        iso_639_1: *b"de",
        iso_639_2: *b"deu",
        english_name: "German",
    },
    LanguageInner {
        iso_639_1: *b"dv",
        iso_639_2: *b"div",
        english_name: "Dhivehi",
    },
    LanguageInner {
        iso_639_1: *b"dz",
        iso_639_2: *b"dzo",
        english_name: "Dzongkha",
    },
    LanguageInner {
        iso_639_1: *b"ee",
        iso_639_2: *b"ewe",
        english_name: "Ewe",
    },
    LanguageInner {
        iso_639_1: *b"el",
        iso_639_2: *b"gre",
        english_name: "Greek, Modern (1453-)",
    },
    LanguageInner {
        iso_639_1: *b"en",
        iso_639_2: *b"eng",
        english_name: "English",
    },
    LanguageInner {
        iso_639_1: *b"eo",
        iso_639_2: *b"epo",
        english_name: "Esperanto",
    },
    LanguageInner {
        iso_639_1: *b"es",
        iso_639_2: *b"spa",
        english_name: "Spanish",
    },
    LanguageInner {
        iso_639_1: *b"et",
        iso_639_2: *b"est",
        english_name: "Estonian",
    },
    LanguageInner {
        iso_639_1: *b"eu",
        iso_639_2: *b"eus",
        english_name: "Basque",
    },
    LanguageInner {
        iso_639_1: *b"fa",
        iso_639_2: *b"fas",
        english_name: "Persian",
    },
    LanguageInner {
        iso_639_1: *b"ff",
        iso_639_2: *b"ful",
        english_name: "Fulah",
    },
    LanguageInner {
        iso_639_1: *b"fi",
        iso_639_2: *b"fin",
        english_name: "Finnish",
    },
    LanguageInner {
        iso_639_1: *b"fj",
        iso_639_2: *b"fij",
        english_name: "Fijian",
    },
    LanguageInner {
        iso_639_1: *b"fo",
        iso_639_2: *b"fao",
        english_name: "Faroese",
    },
    LanguageInner {
        iso_639_1: *b"fr",
        iso_639_2: *b"fra",
        english_name: "French",
    },
    LanguageInner {
        iso_639_1: *b"fy",
        iso_639_2: *b"fry",
        english_name: "Western Frisian",
    },
    LanguageInner {
        iso_639_1: *b"ga",
        iso_639_2: *b"gle",
        english_name: "Irish",
    },
    LanguageInner {
        iso_639_1: *b"gd",
        iso_639_2: *b"gla",
        english_name: "Gaelic",
    },
    LanguageInner {
        iso_639_1: *b"gl",
        iso_639_2: *b"glg",
        english_name: "Galician",
    },
    LanguageInner {
        iso_639_1: *b"gn",
        iso_639_2: *b"grn",
        english_name: "Guarani",
    },
    LanguageInner {
        iso_639_1: *b"gu",
        iso_639_2: *b"guj",
        english_name: "Gujarati",
    },
    LanguageInner {
        iso_639_1: *b"gv",
        iso_639_2: *b"glv",
        english_name: "Manx",
    },
    LanguageInner {
        iso_639_1: *b"ha",
        iso_639_2: *b"hau",
        english_name: "Hausa",
    },
    LanguageInner {
        iso_639_1: *b"he",
        iso_639_2: *b"heb",
        english_name: "Hebrew",
    },
    LanguageInner {
        iso_639_1: *b"hi",
        iso_639_2: *b"hin",
        english_name: "Hindi",
    },
    LanguageInner {
        iso_639_1: *b"ho",
        iso_639_2: *b"hmo",
        english_name: "Hiri Motu",
    },
    LanguageInner {
        iso_639_1: *b"hr",
        iso_639_2: *b"hrv",
        english_name: "Croatian",
    },
    LanguageInner {
        iso_639_1: *b"ht",
        iso_639_2: *b"hat",
        english_name: "Haitian",
    },
    LanguageInner {
        iso_639_1: *b"hu",
        iso_639_2: *b"hun",
        english_name: "Hungarian",
    },
    LanguageInner {
        iso_639_1: *b"hy",
        iso_639_2: *b"arm",
        english_name: "Armenian",
    },
    LanguageInner {
        iso_639_1: *b"hz",
        iso_639_2: *b"her",
        english_name: "Herero",
    },
    LanguageInner {
        iso_639_1: *b"ia",
        iso_639_2: *b"ina",
        english_name: "Interlingua (International Auxiliary Language Association)",
    },
    LanguageInner {
        iso_639_1: *b"id",
        iso_639_2: *b"ind",
        english_name: "Indonesian",
    },
    LanguageInner {
        iso_639_1: *b"ie",
        iso_639_2: *b"ile",
        english_name: "Interlingue",
    },
    LanguageInner {
        iso_639_1: *b"ig",
        iso_639_2: *b"ibo",
        english_name: "Igbo",
    },
    LanguageInner {
        iso_639_1: *b"ii",
        iso_639_2: *b"iii",
        english_name: "Sichuan Yi",
    },
    LanguageInner {
        iso_639_1: *b"ik",
        iso_639_2: *b"ipk",
        english_name: "Inupiaq",
    },
    LanguageInner {
        iso_639_1: *b"io",
        iso_639_2: *b"ido",
        english_name: "Ido",
    },
    LanguageInner {
        iso_639_1: *b"is",
        iso_639_2: *b"isl",
        english_name: "Icelandic",
    },
    LanguageInner {
        iso_639_1: *b"it",
        iso_639_2: *b"ita",
        english_name: "Italian",
    },
    LanguageInner {
        iso_639_1: *b"iu",
        iso_639_2: *b"iku",
        english_name: "Inuktitut",
    },
    LanguageInner {
        iso_639_1: *b"ja",
        iso_639_2: *b"jpn",
        english_name: "Japanese",
    },
    LanguageInner {
        iso_639_1: *b"jv",
        iso_639_2: *b"jav",
        english_name: "Javanese",
    },
    LanguageInner {
        iso_639_1: *b"ka",
        iso_639_2: *b"geo",
        english_name: "Georgian",
    },
    LanguageInner {
        iso_639_1: *b"kg",
        iso_639_2: *b"kon",
        english_name: "Kongo",
    },
    LanguageInner {
        iso_639_1: *b"ki",
        iso_639_2: *b"kik",
        english_name: "Gikuyu",
    },
    LanguageInner {
        iso_639_1: *b"kk",
        iso_639_2: *b"kaz",
        english_name: "Kazakh",
    },
    LanguageInner {
        iso_639_1: *b"kl",
        iso_639_2: *b"kal",
        english_name: "Greenlandic",
    },
    LanguageInner {
        iso_639_1: *b"km",
        iso_639_2: *b"khm",
        english_name: "Central Khmer",
    },
    LanguageInner {
        iso_639_1: *b"kn",
        iso_639_2: *b"kan",
        english_name: "Kannada",
    },
    LanguageInner {
        iso_639_1: *b"ko",
        iso_639_2: *b"kor",
        english_name: "Korean",
    },
    LanguageInner {
        iso_639_1: *b"kr",
        iso_639_2: *b"kau",
        english_name: "Kanuri",
    },
    LanguageInner {
        iso_639_1: *b"ks",
        iso_639_2: *b"kas",
        english_name: "Kashmiri",
    },
    LanguageInner {
        iso_639_1: *b"ku",
        iso_639_2: *b"kur",
        english_name: "Kurdish",
    },
    LanguageInner {
        iso_639_1: *b"kv",
        iso_639_2: *b"kom",
        english_name: "Komi",
    },
    LanguageInner {
        iso_639_1: *b"kw",
        iso_639_2: *b"cor",
        english_name: "Cornish",
    },
    LanguageInner {
        iso_639_1: *b"ky",
        iso_639_2: *b"kir",
        english_name: "Kyrgyz",
    },
    LanguageInner {
        iso_639_1: *b"la",
        iso_639_2: *b"lat",
        english_name: "Latin",
    },
    LanguageInner {
        iso_639_1: *b"lb",
        iso_639_2: *b"ltz",
        english_name: "Luxembourgish",
    },
    LanguageInner {
        iso_639_1: *b"lg",
        iso_639_2: *b"lug",
        english_name: "Ganda",
    },
    LanguageInner {
        iso_639_1: *b"li",
        iso_639_2: *b"lim",
        english_name: "Limburger",
    },
    LanguageInner {
        iso_639_1: *b"ln",
        iso_639_2: *b"lin",
        english_name: "Lingala",
    },
    LanguageInner {
        iso_639_1: *b"lo",
        iso_639_2: *b"lao",
        english_name: "Lao",
    },
    LanguageInner {
        iso_639_1: *b"lt",
        iso_639_2: *b"lit",
        english_name: "Lithuanian",
    },
    LanguageInner {
        iso_639_1: *b"lu",
        iso_639_2: *b"lub",
        english_name: "Luba-Katanga",
    },
    LanguageInner {
        iso_639_1: *b"lv",
        iso_639_2: *b"lav",
        english_name: "Latvian",
    },
    LanguageInner {
        iso_639_1: *b"mg",
        iso_639_2: *b"mlg",
        english_name: "Malagasy",
    },
    LanguageInner {
        iso_639_1: *b"mh",
        iso_639_2: *b"mah",
        english_name: "Marshallese",
    },
    LanguageInner {
        iso_639_1: *b"mi",
        iso_639_2: *b"mao",
        english_name: "Maori",
    },
    LanguageInner {
        iso_639_1: *b"mk",
        iso_639_2: *b"mac",
        english_name: "Macedonian",
    },
    LanguageInner {
        iso_639_1: *b"ml",
        iso_639_2: *b"mal",
        english_name: "Malayalam",
    },
    LanguageInner {
        iso_639_1: *b"mn",
        iso_639_2: *b"mon",
        english_name: "Mongolian",
    },
    LanguageInner {
        iso_639_1: *b"mr",
        iso_639_2: *b"mar",
        english_name: "Marathi",
    },
    LanguageInner {
        iso_639_1: *b"ms",
        iso_639_2: *b"may",
        english_name: "Malay",
    },
    LanguageInner {
        iso_639_1: *b"mt",
        iso_639_2: *b"mlt",
        english_name: "Maltese",
    },
    LanguageInner {
        iso_639_1: *b"my",
        iso_639_2: *b"bur",
        english_name: "Burmese",
    },
    LanguageInner {
        iso_639_1: *b"na",
        iso_639_2: *b"nau",
        english_name: "Nauru",
    },
    LanguageInner {
        iso_639_1: *b"nb",
        iso_639_2: *b"nob",
        english_name: "Norwegian Bokmål",
    },
    LanguageInner {
        iso_639_1: *b"ne",
        iso_639_2: *b"nep",
        english_name: "Nepali",
    },
    LanguageInner {
        iso_639_1: *b"ng",
        iso_639_2: *b"ndo",
        english_name: "Ndonga",
    },
    LanguageInner {
        iso_639_1: *b"nl",
        iso_639_2: *b"dut",
        english_name: "Dutch",
    },
    LanguageInner {
        iso_639_1: *b"nn",
        iso_639_2: *b"nno",
        english_name: "Norwegian Nynorsk",
    },
    LanguageInner {
        iso_639_1: *b"no",
        iso_639_2: *b"nor",
        english_name: "Norwegian",
    },
    LanguageInner {
        iso_639_1: *b"nr",
        iso_639_2: *b"nbl",
        english_name: "South Ndebele",
    },
    LanguageInner {
        iso_639_1: *b"nv",
        iso_639_2: *b"nav",
        english_name: "Navajo",
    },
    LanguageInner {
        iso_639_1: *b"ny",
        iso_639_2: *b"nya",
        english_name: "Nyanja",
    },
    LanguageInner {
        iso_639_1: *b"oc",
        iso_639_2: *b"oci",
        english_name: "Occitan (post 1500)",
    },
    LanguageInner {
        iso_639_1: *b"oj",
        iso_639_2: *b"oji",
        english_name: "Ojibwa",
    },
    LanguageInner {
        iso_639_1: *b"om",
        iso_639_2: *b"orm",
        english_name: "Oromo",
    },
    LanguageInner {
        iso_639_1: *b"or",
        iso_639_2: *b"ori",
        english_name: "Oriya",
    },
    LanguageInner {
        iso_639_1: *b"os",
        iso_639_2: *b"oss",
        english_name: "Ossetian",
    },
    LanguageInner {
        iso_639_1: *b"pa",
        iso_639_2: *b"pan",
        english_name: "Punjabi",
    },
    LanguageInner {
        iso_639_1: *b"pi",
        iso_639_2: *b"pli",
        english_name: "Pali",
    },
    LanguageInner {
        iso_639_1: *b"pl",
        iso_639_2: *b"pol",
        english_name: "Polish",
    },
    LanguageInner {
        iso_639_1: *b"ps",
        iso_639_2: *b"pus",
        english_name: "Pashto",
    },
    LanguageInner {
        iso_639_1: *b"pt",
        iso_639_2: *b"por",
        english_name: "Portuguese",
    },
    LanguageInner {
        iso_639_1: *b"qu",
        iso_639_2: *b"que",
        english_name: "Quechua",
    },
    LanguageInner {
        iso_639_1: *b"rm",
        iso_639_2: *b"roh",
        english_name: "Romansh",
    },
    LanguageInner {
        iso_639_1: *b"rn",
        iso_639_2: *b"run",
        english_name: "Rundi",
    },
    LanguageInner {
        iso_639_1: *b"ro",
        iso_639_2: *b"rum",
        english_name: "Romanian",
    },
    LanguageInner {
        iso_639_1: *b"ru",
        iso_639_2: *b"rus",
        english_name: "Russian",
    },
    LanguageInner {
        iso_639_1: *b"rw",
        iso_639_2: *b"kin",
        english_name: "Kinyarwanda",
    },
    LanguageInner {
        iso_639_1: *b"sa",
        iso_639_2: *b"san",
        english_name: "Sanskrit",
    },
    LanguageInner {
        iso_639_1: *b"sc",
        iso_639_2: *b"srd",
        english_name: "Sardinian",
    },
    LanguageInner {
        iso_639_1: *b"sd",
        iso_639_2: *b"snd",
        english_name: "Sindhi",
    },
    LanguageInner {
        iso_639_1: *b"se",
        iso_639_2: *b"sme",
        english_name: "Northern Sami",
    },
    LanguageInner {
        iso_639_1: *b"sg",
        iso_639_2: *b"sag",
        english_name: "Sango",
    },
    LanguageInner {
        iso_639_1: *b"si",
        iso_639_2: *b"sin",
        english_name: "Sinhalese",
    },
    LanguageInner {
        iso_639_1: *b"sk",
        iso_639_2: *b"slk",
        english_name: "Slovak",
    },
    LanguageInner {
        iso_639_1: *b"sl",
        iso_639_2: *b"slv",
        english_name: "Slovenian",
    },
    LanguageInner {
        iso_639_1: *b"sm",
        iso_639_2: *b"smo",
        english_name: "Samoan",
    },
    LanguageInner {
        iso_639_1: *b"sn",
        iso_639_2: *b"sna",
        english_name: "Shona",
    },
    LanguageInner {
        iso_639_1: *b"so",
        iso_639_2: *b"som",
        english_name: "Somali",
    },
    LanguageInner {
        iso_639_1: *b"sq",
        iso_639_2: *b"alb",
        english_name: "Albanian",
    },
    LanguageInner {
        iso_639_1: *b"sr",
        iso_639_2: *b"srp",
        english_name: "Serbian",
    },
    LanguageInner {
        iso_639_1: *b"ss",
        iso_639_2: *b"ssw",
        english_name: "Swati",
    },
    LanguageInner {
        iso_639_1: *b"st",
        iso_639_2: *b"sot",
        english_name: "Sotho, Southern",
    },
    LanguageInner {
        iso_639_1: *b"su",
        iso_639_2: *b"sun",
        english_name: "Sundanese",
    },
    LanguageInner {
        iso_639_1: *b"sv",
        iso_639_2: *b"swe",
        english_name: "Swedish",
    },
    LanguageInner {
        iso_639_1: *b"sw",
        iso_639_2: *b"swa",
        english_name: "Swahili",
    },
    LanguageInner {
        iso_639_1: *b"ta",
        iso_639_2: *b"tam",
        english_name: "Tamil",
    },
    LanguageInner {
        iso_639_1: *b"te",
        iso_639_2: *b"tel",
        english_name: "Telugu",
    },
    LanguageInner {
        iso_639_1: *b"tg",
        iso_639_2: *b"tgk",
        english_name: "Tajik",
    },
    LanguageInner {
        iso_639_1: *b"th",
        iso_639_2: *b"tha",
        english_name: "Thai",
    },
    LanguageInner {
        iso_639_1: *b"ti",
        iso_639_2: *b"tir",
        english_name: "Tigrinya",
    },
    LanguageInner {
        iso_639_1: *b"tk",
        iso_639_2: *b"tuk",
        english_name: "Turkmen",
    },
    LanguageInner {
        iso_639_1: *b"tl",
        iso_639_2: *b"tgl",
        english_name: "Tagalog",
    },
    LanguageInner {
        iso_639_1: *b"tn",
        iso_639_2: *b"tsn",
        english_name: "Tswana",
    },
    LanguageInner {
        iso_639_1: *b"to",
        iso_639_2: *b"ton",
        english_name: "Tonga (Tonga Islands)",
    },
    LanguageInner {
        iso_639_1: *b"tr",
        iso_639_2: *b"tur",
        english_name: "Turkish",
    },
    LanguageInner {
        iso_639_1: *b"ts",
        iso_639_2: *b"tso",
        english_name: "Tsonga",
    },
    LanguageInner {
        iso_639_1: *b"tt",
        iso_639_2: *b"tat",
        english_name: "Tatar",
    },
    LanguageInner {
        iso_639_1: *b"tw",
        iso_639_2: *b"twi",
        english_name: "Twi",
    },
    LanguageInner {
        iso_639_1: *b"ty",
        iso_639_2: *b"tah",
        english_name: "Tahitian",
    },
    LanguageInner {
        iso_639_1: *b"ug",
        iso_639_2: *b"uig",
        english_name: "Uyghur",
    },
    LanguageInner {
        iso_639_1: *b"uk",
        iso_639_2: *b"ukr",
        english_name: "Ukrainian",
    },
    LanguageInner {
        iso_639_1: *b"ur",
        iso_639_2: *b"urd",
        english_name: "Urdu",
    },
    LanguageInner {
        iso_639_1: *b"uz",
        iso_639_2: *b"uzb",
        english_name: "Uzbek",
    },
    LanguageInner {
        iso_639_1: *b"ve",
        iso_639_2: *b"ven",
        english_name: "Venda",
    },
    LanguageInner {
        iso_639_1: *b"vi",
        iso_639_2: *b"vie",
        english_name: "Vietnamese",
    },
    LanguageInner {
        iso_639_1: *b"vo",
        iso_639_2: *b"vol",
        english_name: "Volapük",
    },
    LanguageInner {
        iso_639_1: *b"wa",
        iso_639_2: *b"wln",
        english_name: "Walloon",
    },
    LanguageInner {
        iso_639_1: *b"wo",
        iso_639_2: *b"wol",
        english_name: "Wolof",
    },
    LanguageInner {
        iso_639_1: *b"xh",
        iso_639_2: *b"xho",
        english_name: "Xhosa",
    },
    LanguageInner {
        iso_639_1: *b"yi",
        iso_639_2: *b"yid",
        english_name: "Yiddish",
    },
    LanguageInner {
        iso_639_1: *b"yo",
        iso_639_2: *b"yor",
        english_name: "Yoruba",
    },
    LanguageInner {
        iso_639_1: *b"za",
        iso_639_2: *b"zha",
        english_name: "Zhuang",
    },
    LanguageInner {
        iso_639_1: *b"zh",
        iso_639_2: *b"chi",
        english_name: "Chinese",
    },
    LanguageInner {
        iso_639_1: *b"zu",
        iso_639_2: *b"zul",
        english_name: "Zulu",
    },
];

#[allow(clippy::cast_possible_truncation)]
static ISO_639_1_LOOKUP: [Option<Language>; 676] = {
    let mut table = [None; 676]; // 26*26 all possible pairs
    let mut i = 0;

    while i < LANGUAGES.len() {
        let code = LANGUAGES[i].iso_639_1;
        let b0 = code[0];
        let b1 = code[1];

        // Map ASCII bytes ('a'..='z') to range 0..25
        let row = (b0 - b'a') as usize;
        let col = (b1 - b'a') as usize;
        let idx = row * 26 + col;

        table[idx] = Some(unsafe { std::mem::transmute::<u8, Language>(i as u8) });
        i += 1;
    }

    table
};

impl Language {
    pub const COUNT: usize = 181;

    #[inline]
    fn inner(self) -> &'static LanguageInner {
        &LANGUAGES[self as usize]
    }

    #[must_use]
    pub fn iso_639_1(self) -> &'static str {
        // table is ASCII-only by construction
        unsafe { std::str::from_utf8_unchecked(&self.inner().iso_639_1) }
    }

    #[must_use]
    pub fn iso_639_2(self) -> &'static str {
        unsafe { std::str::from_utf8_unchecked(&self.inner().iso_639_2) }
    }

    #[must_use]
    pub fn name(self) -> &'static str {
        self.inner().english_name
    }

    // Because the table is sorted by 2-letter code, lookup is a binary search.
    /// # Errors
    /// If lanauge `code` is not in valid list of ISO 639-1 codes.
    pub fn try_from_iso_639_1(code: &str) -> Result<Self, Error> {
        let b = code.as_bytes();
        let key: [u8; 2] = b
            .try_into()
            .map_err(|_| Error::InvalidLanguageCode(code.to_string()))?;

        let b0 = key[0].wrapping_sub(b'a');
        let b1 = key[1].wrapping_sub(b'a');

        // Check if the bytes are valid ASCII letters
        if b0 < 26 && b1 < 26 {
            let index = (b0 as usize) * 26 + (b1 as usize);
            if let Some(lang) = ISO_639_1_LOOKUP[index] {
                return Ok(lang);
            }
        }

        Err(Error::InvalidLanguageCode(code.to_string()))
    }
}

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LanguageVisitor;

        impl<'de> Visitor<'de> for LanguageVisitor {
            type Value = Language;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid ISO 639-1 language code")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Language::try_from_iso_639_1(v).map_err(de::Error::custom)
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Language::try_from_iso_639_1(v).map_err(de::Error::custom)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Language::try_from_iso_639_1(&v).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(LanguageVisitor)
    }
}

impl Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.iso_639_1())
    }
}

impl TryFrom<&str> for Language {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from_iso_639_1(value)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use serde::de::IntoDeserializer;

    #[test]
    fn test_deserialize_borrowed_str() {
        let de: serde::de::value::StrDeserializer<'_, serde::de::value::Error> =
            "en".into_deserializer();
        let lang = Language::deserialize(de).unwrap();
        assert_eq!(lang, Language::En);
    }

    #[test]
    fn test_deserialize_owned_string() {
        let de: serde::de::value::StringDeserializer<serde::de::value::Error> =
            "fr".to_string().into_deserializer();
        let lang = Language::deserialize(de).unwrap();
        assert_eq!(lang, Language::Fr);
    }

    #[test]
    fn test_deserialize_invalid_code() {
        let de: serde::de::value::StrDeserializer<'_, serde::de::value::Error> =
            "xx".into_deserializer();
        let res = Language::deserialize(de);
        assert!(res.is_err());
    }
}
