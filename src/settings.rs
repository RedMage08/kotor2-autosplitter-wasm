use asr::settings::gui::Title;
use asr::settings::Gui;

#[derive(Gui)]
pub struct Settings {

    #[default = false]

    //  EBON HAWK

    /// Ebon Hawk Splits
    _ebo_splits: Title,

    /// 002EBO - Exterior Hull (Tutorial)

    pub _002EBO: bool,
    /// 002EBO Unlimited Splits


    pub _002EBO_unlim: bool,
    /// 003EBO - Interior


    pub _003EBO: bool,
    /// 003EBO Unlimited Splits


    pub _003EBO_unlim: bool,
    /// 004EBO - Interior (Red Eclipse Ambush)


    pub _004EBO: bool,
    /// 004EBO Unlimited Splits


    pub _004EBO_unlim: bool,
    /// 005EBO - Interior (Peragus Escape after Turret Minigame)


    pub _005EBO: bool,
    /// 005EBO Unlimited Splits


    pub _005EBO_unlim: bool,
    /// 006EBO - 006EBO - Interior (Post Jedi Enclave)


    pub _006EBO: bool,
    /// 006EBO Unlimited Splits


    pub _006EBO_unlim: bool,
    /// 007EBO - Interior (Post Goto's Yacht)


    pub _007EBO: bool,
    /// 007EBO Unlimited Splits


    pub _007EBO_unlim: bool,

    //  PERAGUS

    /// Peragus Splits
    /// Peragus Splits
    _per_splits: Title,

    /// 101PER - Administration Level


    pub _101PER: bool,
    /// 101PER - Unlimited Splits


    pub _101PER_unlim: bool,
    /// 102PER - Mining Tunnels


    pub _102PER: bool,
    /// 102PER - Unlimited Splits


    pub _102PER_unlim: bool,
    /// 103PER - Fuel Depot


    pub _103PER: bool,
    /// 103PER - Unlimited Splits


    pub _103PER_unlim: bool,
    /// 104PER - Asteroid Exterior


    pub _104PER: bool,
    /// 104PER - Unlimited Splits


    pub _104PER_unlim: bool,
    /// 105PER - Dormitories


    pub _105PER: bool,
    /// 105PER - Unlimited Splits


    pub _105PER_unlim: bool,
    /// 106PER - Hangar Bay


    pub _106PER: bool,
    /// 106PER - Unlimited Splits


    pub _106PER_unlim: bool,
    /// 107PER - Turret Minigame/Escape Sequence


    pub _107PER: bool,
    /// 107PER - Unlimited Splits


    pub _107PER_unlim: bool,

    //  HARBINGER

    /// Harbinger Splits
    _har_splits: Title,

    /// 151HAR - Command Deck


    pub _151HAR: bool,
    /// 151HAR - Unlimited Splits


    pub _151HAR_unlim: bool,
    /// 152HAR - Crew Quarters


    pub _152HAR: bool,
    /// 1512HAR - Unlimited Splits


    pub _152HAR_unlim: bool,
    /// 153HAR - Engine Deck


    pub _153HAR: bool,
    /// 153HAR - Unlimited Splits


    pub _153HAR_unlim: bool,
    /// 154HAR - Command Deck Cutscene


    pub _154HAR: bool,
    /// 154HAR - Unlimited Splits


    pub _154HAR_unlim: bool,

    //  TELOS - CITADEL STATION

    /// Citadel Station Splits
    _cit_splits: Title,

    /// 201TEL - Citadel Station Docking Module


    pub _201TEL: bool,
    /// 201TEL - Unlimited Splits


    pub _201TEL_unlim: bool,
    /// 202TEL - Citadel Station Entertainment Module 081


    pub _202TEL: bool,
    /// 202TEL - Unlimited Splits


    pub _202TEL_unlim: bool,
    /// 203TEL - Citadel Station Residential 082 East


    pub _203TEL: bool,
    /// 203TEL - Unlimited Splits


    pub _203TEL_unlim: bool,
    /// 204TEL - Citadel Station Residential 082 West


    pub _204TEL: bool,
    /// 204TEL - Unlimited Splits


    pub _204TEL_unlim: bool,
    /// 205TEL - Citadel Station Residential Cutscene to Malachor V


    pub _205TEL: bool,
    /// 205TEL - Unlimited Splits


    pub _205TEL_unlim: bool,
    /// 207TEL - Citadel Station Cantina


    pub _207TEL: bool,
    /// 207TEL - Unlimited Splits


    pub _207TEL_unlim: bool,
    /// 208TEL - Citadel Station Bumani Exchange Corporation


    pub _208TEL: bool,
    /// 208TEL - Unlimited Splits


    pub _208TEL_unlim: bool,
    /// 209TEL - Citadel Statio Czerka Offices


    pub _209TEL: bool,
    /// 209TEL - Unlimited Splits


    pub _209TEL_unlim: bool,
    /// 211TEL - Citadel Station Swoop Track


    pub _211TEL: bool,
    /// 211TEL - Unlimited Splits


    pub _211TEL_unlim: bool,
    /// 220TEL - Citadel Station Residental East - Sith Assault


    pub _220TEL: bool,
    /// 220TEL - Unlimited Splits


    pub _220TEL_unlim: bool,
    /// 221TEL - Citadel Station Residental West - Sith Assault (cutscene w/ Grenn)


    pub _221TEL: bool,
    /// 221TEL - Unlimited Splits


    pub _221TEL_unlim: bool,
    /// 222TEL - Citadel Station Entertainment Module - Sith Assault


    pub _222TEL: bool,
    /// 222TEL - Unlimited Splits


    pub _222TEL_unlim: bool,

    // TELOS - SURFACE

    ///Telos Surface Splits
    _tel_splits: Title,

    /// 231TEL - Restoration Zone


    pub _231TEL: bool,
    /// 231TEL - Unlimited Splits


    pub _231TEL_unlim: bool,
    /// 232TEL - Underground Base


    pub _232TEL: bool,
    /// 232TEL - Unlimited Splits


    pub _232TEL_unlim: bool,
    /// 233TEL - Czerka Site


    pub _233TEL: bool,
    /// 233TEL - Unlimited Splits


    pub _233TEL_unlim: bool,
    /// 261TEL - Polar Plateau


    pub _261TEL: bool,
    /// 261TEL - Unlimited Splits


    pub _261TEL_unlim: bool,
    /// 262TEL - Polar Academy


    pub _262TEL: bool,
    /// 262TEL - Unlimited Splits


    pub _262TEL_unlim: bool,
    /// 950COR - Ebon Hawk & Coruscant Cutscene After Leaving Telos


    pub _950COR: bool,
    /// 950COR - Unlimited Splits


    pub _950COR_unlim: bool,

    //  NAR SHADDAA

    /// Nar Shaddaa Splits
    _nar_splits: Title,

    /// 301NAR - Refugee Landing Pad


    pub _301NAR: bool,
    /// 301NAR - Unlimited Splits


    pub _301NAR_unlim: bool,
    /// 302NAR - Refugee Quad


    pub _302NAR: bool,
    /// 302NAR - Unlimited Splits


    pub _302NAR_unlim: bool,
    /// 303NAR - Docks


    pub _303NAR: bool,
    /// 303NAR - Unlimited Splits


    pub _303NAR_unlim: bool,
    /// 304NAR - Jekk'Jekk Tarr


    pub _304NAR: bool,
    /// 304NAR - Unlimited Splits


    pub _304NAR_unlim: bool,
    /// 305NAR - Jekk'Jekk Tunnels


    pub _305NAR: bool,
    /// 305NAR - Unlimited Splits


    pub _305NAR_unlim: bool,
    /// 306NAR - Entertainment Promenade


    pub _306NAR: bool,
    /// 306NAR - Unlimited Splits


    pub _306NAR_unlim: bool,
    /// 351NAR - Goto's Yacht


    pub _351NAR: bool,
    /// 351NAR - Unlimited Splits


    pub _351NAR_unlim: bool,
    /// 352NAR - Goto's Yacht - Cutscene


    pub _352NAR: bool,
    /// 352NAR - Unlimited Splits


    pub _352NAR_unlim: bool,
    /// 371NAR - Nar Shaddaa Swoop Track


    pub _371NAR: bool,
    /// 371NAR - Unlimited Splits


    pub _371NAR_unlim: bool,

    //  DXUN

    /// Dxun Splits
    _dxn_splits: Title,

    /// 401DXN - Jungle Landing


    pub _401DXN: bool,
    /// 401DXN - Unlimited Splits


    pub _401DXN_unlim: bool,
    /// 402DXN - Jungle


    pub _402DXN: bool,
    /// 402DXN - Unlimited Splits


    pub _402DXN_unlim: bool,
    /// 403DXN - Mandalorian Ruins


    pub _403DXN: bool,
    /// 403DXN - Unlimited Splits


    pub _403DXN_unlim: bool,
    /// 404DXN - Mandalorian Cache


    pub _404DXN: bool,
    /// 404DXN - Unlimited Splits


    pub _404DXN_unlim: bool,
    /// 410DXN - Jungle Tomb(Exterior)


    pub _410DXN: bool,
    /// 410DXN - Unlimited Splits


    pub _410DXN_unlim: bool,
    /// 411DXN - Sith Tomb(Interior)


    pub _411DXN: bool,
    /// 411DXN - Unlimited Splits


    pub _411DXN_unlim: bool,
    /// 421DXN - Dxun Turret Minigame


    pub _421DXN: bool,
    /// 421DXN - Unlimited Splits


    pub _421DXN_unlim: bool,

    // ONDERON

    /// Onderon Splits
    _ond_splits: Title,

    /// 501OND - Iziz Spaceport


    pub _501OND: bool,
    /// 501OND - Unlimited Splits


    pub _501OND_unlim: bool,
    /// 502OND - Merchant Square


    pub _502OND: bool,
    /// 501OND - Unlimited Splits


    pub _502OND_unlim: bool,
    /// 503OND - Iziz Cantina


    pub _503OND: bool,
    /// 503OND - Unlimited Splits


    pub _503OND_unlim: bool,
    /// 504OND - Sky Ramp


    pub _504OND: bool,
    /// 504OND - Unlimited Splits


    pub _504OND_unlim: bool,
    /// 505OND - Iziz Turret Minigame


    pub _505OND: bool,
    /// 505OND - Unlimited Splits


    pub _505OND_unlim: bool,
    /// 506OND - Royal Palace


    pub _506OND: bool,
    /// 506OND - Unlimited Splits


    pub _506OND_unlim: bool,
    /// 510OND - Iziz Swoop Track


    pub _510OND: bool,
    /// 510OND - Unlimited Splits


    pub _510OND_unlim: bool,
    /// 511OND - Merchant Quarter


    pub _511OND: bool,
    /// 511OND - Unlimited Splits


    pub _511OND_unlim: bool,
    /// 512OND - Iziz Western Square


    pub _512OND: bool,
    /// 512OND - Unlimited Splits


    pub _512OND_unlim: bool,

    // DANTOOINE

    /// Dantooine Splits
    _dan_splits: Title,

    /// 601DAN - Khoonda


    pub _601DAN: bool,
    /// 601DAN - Unlimited Splits


    pub _601DAN_unlim: bool,
    /// 602DAN - Khoonda Plains


    pub _602DAN: bool,
    /// 602DAN - Unlimited Splits


    pub _602DAN_unlim: bool,
    /// 603DAN - Movie Terminal on Khoonda Plains


    pub _603DAN: bool,
    /// 603DAN - Unlimited Splits


    pub _603DAN_unlim: bool,
    /// 604DAN - Crystal Cave


    pub _604DAN: bool,
    /// 604DAN - Unlimited Splits


    pub _604DAN_unlim: bool,
    /// 605DAN - Enclave Courtyard


    pub _605DAN: bool,
    /// 605DAN - Unlimited Splits


    pub _605DAN_unlim: bool,
    /// 610DAN - Enclave Sublevel


    pub _610DAN: bool,
    /// 610DAN - Unlimited Splits


    pub _610DAN_unlim: bool,
    /// 650DAN - Rebuilt Jedi Enclave


    pub _650DAN: bool,
    /// 650DAN - Unlimited Splits


    pub _650DAN_unlim: bool,

    // KORRIBAN

    /// Korriban Splits
    _kor_splits: Title,

    /// 701KOR - Valley of the Dark Lords


    pub _701KOR: bool,
    /// 701KOR - Unlimited Splits


    pub _701KOR_unlim: bool,
    /// 702KOR - Sith Academy


    pub _702KOR: bool,
    /// 702KOR - Unlimited Splits


    pub _702KOR_unlim: bool,
    /// 710KOR - Shyrack Cave


    pub _710KOR: bool,
    /// 710KOR - Unlimited Splits


    pub _710KOR_unlim: bool,
    /// 711KOR - Secret Tomb


    pub _711KOR: bool,
    /// 711KOR - Unlimited Splits


    pub _711KOR_unlim: bool,

    // RAVAGER

    /// Ravager Splits
    _rav_splits: Title,

    /// 851NIH - Command Deck


    pub _851NIH: bool,
    /// 851NIH - Unlimited Splits


    pub _851NIH_unlim: bool,
    /// 852NIH - Bridge


    pub _852NIH: bool,
    /// 852NIH - Unlimited Splits


    pub _852NIH_unlim: bool,
    /// 853NIH - Visas Nihilus Cutscene


    pub _853NIH: bool,
    /// 853NIH - Unlimited Splits


    pub _853NIH_unlim: bool,

    // MALACHOR V

    /// Malachor V Splits
    _mal_splits: Title,

    /// 901MAL - Surface


    pub _901MAL: bool,
    /// 901MAL - Unlimited Splits


    pub _901MAL_unlim: bool,
    /// 902MAL - Depths


    pub _902MAL: bool,
    /// 902MAL - Unlimited Splits


    pub _902MAL_unlim: bool,
    /// 903MAL - Trayus Academy


    pub _903MAL: bool,
    /// 903MAL - Unlimited Splits


    pub _903MAL_unlim: bool,
    /// 904MAL - Trayus Core (Final Module)


    pub _904MAL: bool,
    /// 904MAL - Unlimited Splits


    pub _904MAL_unlim: bool,
    /// 905MAL - Trayus Crescent


    pub _905MAL: bool,
    /// 905MAL - Unlimited Splits


    pub _905MAL_unlim: bool,
    /// 906MAL - Trayus Proving Grounds


    pub _906MAL: bool,
    /// 906MAL - Unlimited Splits


    pub _906MAL_unlim: bool,
    /// 907MAL - Trayus Core (Traya/Sion Cutscene)


    pub _907MAL: bool,
    /// 907MAL - Unlimited Splits


    pub _907MAL_unlim: bool,

}




