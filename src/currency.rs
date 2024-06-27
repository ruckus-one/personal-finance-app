use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum Currency {
    USD, // United States Dollar
    EUR, // Euro
    JPY, // Japanese Yen
    GBP, // British Pound Sterling
    AUD, // Australian Dollar
    CAD, // Canadian Dollar
    CHF, // Swiss Franc
    CNY, // Chinese Yuan
    SEK, // Swedish Krona
    NZD, // New Zealand Dollar
    MXN, // Mexican Peso
    SGD, // Singapore Dollar
    HKD, // Hong Kong Dollar
    NOK, // Norwegian Krone
    KRW, // South Korean Won
    TRY, // Turkish Lira
    RUB, // Russian Ruble
    INR, // Indian Rupee
    BRL, // Brazilian Real
    ZAR, // South African Rand
    PLN, // Polish Zloty
    DKK, // Danish Krone
    MYR, // Malaysian Ringgit
    THB, // Thai Baht
    IDR, // Indonesian Rupiah
    HUF, // Hungarian Forint
    CZK, // Czech Koruna
    ILS, // Israeli New Shekel
    CLP, // Chilean Peso
    PHP, // Philippine Peso
    AED, // United Arab Emirates Dirham
    COP, // Colombian Peso
    SAR, // Saudi Riyal
    RON, // Romanian Leu
    BGN, // Bulgarian Lev
    HRK, // Croatian Kuna
    ISK, // Icelandic Krona
    VND, // Vietnamese Dong
    PKR, // Pakistani Rupee
    EGP, // Egyptian Pound
    NGN, // Nigerian Naira
    KES, // Kenyan Shilling
    GHS, // Ghanaian Cedi
    UAH, // Ukrainian Hryvnia
    ARS, // Argentine Peso
    PEN, // Peruvian Sol
    LKR, // Sri Lankan Rupee
    BDT, // Bangladeshi Taka
    MAD, // Moroccan Dirham
    TWD, // New Taiwan Dollar
    JMD, // Jamaican Dollar
    XOF, // West African CFA Franc
    XAF, // Central African CFA Franc
    XCD, // East Caribbean Dollar
    BHD, // Bahraini Dinar
    QAR, // Qatari Riyal
    OMR, // Omani Rial
    KWD, // Kuwaiti Dinar
    BZD, // Belize Dollar
    FJD, // Fijian Dollar
    MUR, // Mauritian Rupee
    NPR, // Nepalese Rupee
    LBP, // Lebanese Pound
    JOD, // Jordanian Dinar
    BOB, // Bolivian Boliviano
    PYG, // Paraguayan Guarani
    UYU, // Uruguayan Peso
    GIP, // Gibraltar Pound
    KYD, // Cayman Islands Dollar
    BMD, // Bermudian Dollar
    AWG, // Aruban Florin
    ANG, // Netherlands Antillean Guilder
    SRD, // Surinamese Dollar
    TTD, // Trinidad and Tobago Dollar
    BSD, // Bahamian Dollar
    HTG, // Haitian Gourde
    DOP, // Dominican Peso
    CUP, // Cuban Peso
    CRC, // Costa Rican Colón
    GTQ, // Guatemalan Quetzal
    HNL, // Honduran Lempira
    NIO, // Nicaraguan Córdoba
    PAB, // Panamanian Balboa
    SVC, // Salvadoran Colón
    BBD, // Barbadian Dollar
    XPF, // CFP Franc
    PGK, // Papua New Guinean Kina
    WST, // Samoan Tala
    TOP, // Tongan Paʻanga
    VUV, // Vanuatu Vatu
    SBD, // Solomon Islands Dollar
    KZT, // Kazakhstani Tenge
    UZS, // Uzbekistani Som
    TJS, // Tajikistani Somoni
    KGS, // Kyrgyzstani Som
    MNT, // Mongolian Tögrög
    AMD, // Armenian Dram
    GEL, // Georgian Lari
    AZN, // Azerbaijani Manat
    BYN, // Belarusian Ruble
    MDL, // Moldovan Leu
    MKD, // Macedonian Denar
    ALL, // Albanian Lek
    BAM, // Bosnia and Herzegovina Convertible Mark
    RSD, // Serbian Dinar
    MZN, // Mozambican Metical
    ZMW, // Zambian Kwacha
    BWP, // Botswanan Pula
    NAD, // Namibian Dollar
    SZL, // Swazi Lilangeni
    LSL, // Lesotho Loti
    MOP, // Macanese Pataca
    KHR, // Cambodian Riel
    LAK, // Lao Kip
    MMK, // Myanmar Kyat
    BND, // Brunei Dollar
    BTN, // Bhutanese Ngultrum
    SCR, // Seychellois Rupee
    MVR, // Maldivian Rufiyaa
    DJF, // Djiboutian Franc
    SOS, // Somali Shilling
    RWF, // Rwandan Franc
    BIF, // Burundian Franc
    MWK, // Malawian Kwacha
    ZWL, // Zimbabwean Dollar
    GMD, // Gambian Dalasi
    SLL, // Sierra Leonean Leone
    TZS, // Tanzanian Shilling
    UGX, // Ugandan Shilling
    AOA, // Angolan Kwanza
    CVE, // Cape Verdean Escudo
    STN, // São Tomé and Príncipe Dobra
    MRO, // Mauritanian Ouguiya
    LYD, // Libyan Dinar
    SDG, // Sudanese Pound
    YER, // Yemeni Rial
    IRR, // Iranian Rial
    IQD, // Iraqi Dinar
    AFN, // Afghan Afghani
    SYP, // Syrian Pound
    KPW, // North Korean Won
    ETB, // Ethiopian Birr
    ERN, // Eritrean Nakfa
    TMT, // Turkmenistani Manat
}

impl Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}
