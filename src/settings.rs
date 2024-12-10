use asr::settings::gui::Title;
use asr::settings::Gui;

#[derive(Gui)]
pub struct Settings {
    // EBON HAWK

    /// Ebon Hawk Splits
    title_ebo_splits: Title,

    /// 001EBO - Interior (Tutorial)
    #[default = false]
    pub split_001EBO: bool,

    /// 001EBO Unlimited Splits
    #[default = false]
    pub split_001EBO_unlim: bool,

    /// 002EBO - Exterior Hull (Tutorial)
    #[default = false]
    pub split_002EBO: bool,

    /// 002EBO Unlimited Splits
    #[default = false]
    pub split_002EBO_unlim: bool,

    /// 003EBO - Interior
    #[default = false]
    pub split_003EBO: bool,

    /// 003EBO Unlimited Splits
    #[default = false]
    pub split_003EBO_unlim: bool,

    /// 004EBO - Interior (Red Eclipse Ambush)
    #[default = false]
    pub split_004EBO: bool,

    /// 004EBO Unlimited Splits
    #[default = false]
    pub split_004EBO_unlim: bool,

    /// 005EBO - Interior (Peragus Escape after Turret Minigame)
    #[default = false]
    pub split_005EBO: bool,

    /// 005EBO Unlimited Splits
    #[default = false]
    pub split_005EBO_unlim: bool,

    /// 006EBO - Interior (Post Jedi Enclave)
    #[default = false]
    pub split_006EBO: bool,

    /// 006EBO Unlimited Splits
    #[default = false]
    pub split_006EBO_unlim: bool,

    /// 007EBO - Interior (Post Goto's Yacht)
    #[default = false]
    pub split_007EBO: bool,

    /// 007EBO Unlimited Splits
    #[default = false]
    pub split_007EBO_unlim: bool,

    // PERAGUS

    /// Peragus Splits
    title_per_splits: Title,

    /// 101PER - Administration Level
    #[default = false]
    pub split_101PER: bool,

    /// 101PER Unlimited Splits
    #[default = false]
    pub split_101PER_unlim: bool,

    /// 102PER - Mining Tunnels
    #[default = false]
    pub split_102PER: bool,

    /// 102PER Unlimited Splits
    #[default = false]
    pub split_102PER_unlim: bool,

    /// 103PER - Fuel Depot
    #[default = false]
    pub split_103PER: bool,

    /// 103PER Unlimited Splits
    #[default = false]
    pub split_103PER_unlim: bool,

    /// 104PER - Asteroid Exterior
    #[default = false]
    pub split_104PER: bool,

    /// 104PER Unlimited Splits
    #[default = false]
    pub split_104PER_unlim: bool,

    /// 105PER - Dormitories
    #[default = false]
    pub split_105PER: bool,

    /// 105PER Unlimited Splits
    #[default = false]
    pub split_105PER_unlim: bool,

    /// 106PER - Hangar Bay
    #[default = false]
    pub split_106PER: bool,

    /// 106PER Unlimited Splits
    #[default = false]
    pub split_106PER_unlim: bool,

    /// 107PER - Turret Minigame/Escape Sequence
    #[default = false]
    pub split_107PER: bool,

    /// 107PER Unlimited Splits
    #[default = false]
    pub split_107PER_unlim: bool,

    // HARBINGER

    /// Harbinger Splits
    title_har_splits: Title,

    /// 151HAR - Command Deck
    #[default = false]
    pub split_151HAR: bool,

    /// 151HAR Unlimited Splits
    #[default = false]
    pub split_151HAR_unlim: bool,

    /// 152HAR - Crew Quarters
    #[default = false]
    pub split_152HAR: bool,

    /// 152HAR Unlimited Splits
    #[default = false]
    pub split_152HAR_unlim: bool,

    /// 153HAR - Engine Deck
    #[default = false]
    pub split_153HAR: bool,

    /// 153HAR Unlimited Splits
    #[default = false]
    pub split_153HAR_unlim: bool,

    /// 154HAR - Command Deck Cutscene
    #[default = false]
    pub split_154HAR: bool,

    /// 154HAR Unlimited Splits
    #[default = false]
    pub split_154HAR_unlim: bool,

    // TELOS - CITADEL STATION

    /// Citadel Station Splits
    title_cit_splits: Title,

    /// 201TEL - Citadel Station Docking Module
    #[default = false]
    pub split_201TEL: bool,

    /// 201TEL Unlimited Splits
    #[default = false]
    pub split_201TEL_unlim: bool,

    /// 202TEL - Citadel Station Entertainment Module 081
    #[default = false]
    pub split_202TEL: bool,

    /// 202TEL Unlimited Splits
    #[default = false]
    pub split_202TEL_unlim: bool,

    /// 203TEL - Citadel Station Residential 082 East
    #[default = false]
    pub split_203TEL: bool,

    /// 203TEL Unlimited Splits
    #[default = false]
    pub split_203TEL_unlim: bool,

    /// 204TEL - Citadel Station Residential 082 West
    #[default = false]
    pub split_204TEL: bool,

    /// 204TEL Unlimited Splits
    #[default = false]
    pub split_204TEL_unlim: bool,

    /// 205TEL - Citadel Station Residential Cutscene to Malachor V
    #[default = false]
    pub split_205TEL: bool,

    /// 205TEL Unlimited Splits
    #[default = false]
    pub split_205TEL_unlim: bool,

    /// 207TEL - Citadel Station Cantina
    #[default = false]
    pub split_207TEL: bool,

    /// 207TEL Unlimited Splits
    #[default = false]
    pub split_207TEL_unlim: bool,

    /// 208TEL - Citadel Station Bumani Exchange Corporation
    #[default = false]
    pub split_208TEL: bool,

    /// 208TEL Unlimited Splits
    #[default = false]
    pub split_208TEL_unlim: bool,

    /// 209TEL - Citadel Station Czerka Offices
    #[default = false]
    pub split_209TEL: bool,

    /// 209TEL Unlimited Splits
    #[default = false]
    pub split_209TEL_unlim: bool,

    /// 211TEL - Citadel Station Swoop Track
    #[default = false]
    pub split_211TEL: bool,

    /// 211TEL Unlimited Splits
    #[default = false]
    pub split_211TEL_unlim: bool,

    /// 220TEL - Citadel Station Residential East - Sith Assault
    #[default = false]
    pub split_220TEL: bool,

    /// 220TEL Unlimited Splits
    #[default = false]
    pub split_220TEL_unlim: bool,

    /// 221TEL - Citadel Station Residential West - Sith Assault (cutscene w/ Grenn)
    #[default = false]
    pub split_221TEL: bool,

    /// 221TEL Unlimited Splits
    #[default = false]
    pub split_221TEL_unlim: bool,

    /// 222TEL - Citadel Station Entertainment Module - Sith Assault
    #[default = false]
    pub split_222TEL: bool,

    /// 222TEL Unlimited Splits
    #[default = false]
    pub split_222TEL_unlim: bool,

    // TELOS - SURFACE

    /// Telos Surface Splits
    title_tel_splits: Title,

    /// 231TEL - Restoration Zone
    #[default = false]
    pub split_231TEL: bool,

    /// 231TEL - Unlimited Splits
    #[default = false]
    pub split_231TEL_unlim: bool,

    /// 232TEL - Underground Base
    #[default = false]
    pub split_232TEL: bool,

    /// 232TEL - Unlimited Splits
    #[default = false]
    pub split_232TEL_unlim: bool,

    /// 233TEL - Czerka Site
    #[default = false]
    pub split_233TEL: bool,

    /// 233TEL - Unlimited Splits
    #[default = false]
    pub split_233TEL_unlim: bool,

    /// 261TEL - Polar Plateau
    #[default = false]
    pub split_261TEL: bool,

    /// 261TEL - Unlimited Splits
    #[default = false]
    pub split_261TEL_unlim: bool,

    /// 262TEL - Polar Plateau Interior
    #[default = false]
    pub split_262TEL: bool,

    /// 262TEL - Unlimited Splits
    #[default = false]
    pub split_262TEL_unlim: bool,

    /// 950COR - Ebon Hawk & Coruscant Cutscene After Leaving Telos
    #[default = false]
    pub split_950COR: bool,

    /// 950COR - Unlimited Splits
    #[default = false]
    pub split_950COR_unlim: bool,

    // NAR SHADDAA

    /// Nar Shaddaa Splits
    title_nar_splits: Title,

    /// 301NAR - Refugee Landing Pad
    #[default = false]
    pub split_301NAR: bool,

    /// 301NAR - Unlimited Splits
    #[default = false]
    pub split_301NAR_unlim: bool,

    /// 302NAR - Refugee Quad
    #[default = false]
    pub split_302NAR: bool,

    /// 302NAR - Unlimited Splits
    #[default = false]
    pub split_302NAR_unlim: bool,

    /// 303NAR - Docks
    #[default = false]
    pub split_303NAR: bool,

    /// 303NAR - Unlimited Splits
    #[default = false]
    pub split_303NAR_unlim: bool,

    /// 304NAR - Jekk'Jekk Tarr
    #[default = false]
    pub split_304NAR: bool,

    /// 304NAR - Unlimited Splits
    #[default = false]
    pub split_304NAR_unlim: bool,

    /// 305NAR - Jekk'Jekk Tunnels
    #[default = false]
    pub split_305NAR: bool,

    /// 305NAR - Unlimited Splits
    #[default = false]
    pub split_305NAR_unlim: bool,

    /// 306NAR - Entertainment Promenade
    #[default = false]
    pub split_306NAR: bool,

    /// 306NAR - Unlimited Splits
    #[default = false]
    pub split_306NAR_unlim: bool,

    /// 351NAR - Goto's Yacht
    #[default = false]
    pub split_351NAR: bool,

    /// 351NAR - Unlimited Splits
    #[default = false]
    pub split_351NAR_unlim: bool,

    /// 352NAR - Goto's Yacht - Cutscene
    #[default = false]
    pub split_352NAR: bool,

    /// 352NAR - Unlimited Splits
    #[default = false]
    pub split_352NAR_unlim: bool,

    /// 371NAR - Nar Shaddaa Swoop Track
    #[default = false]
    pub split_371NAR: bool,

    /// 371NAR - Unlimited Splits
    #[default = false]
    pub split_371NAR_unlim: bool,

    // DXUN

    /// Dxun Splits
    title_dxn_splits: Title,

    /// 401DXN - Jungle Landing
    #[default = false]
    pub split_401DXN: bool,

    /// 401DXN - Unlimited Splits
    #[default = false]
    pub split_401DXN_unlim: bool,

    /// 402DXN - Jungle
    #[default = false]
    pub split_402DXN: bool,

    /// 402DXN - Unlimited Splits
    #[default = false]
    pub split_402DXN_unlim: bool,

    /// 403DXN - Mandalorian Ruins
    #[default = false]
    pub split_403DXN: bool,

    /// 403DXN - Unlimited Splits
    #[default = false]
    pub split_403DXN_unlim: bool,

    /// 404DXN - Mandalorian Cache
    #[default = false]
    pub split_404DXN: bool,

    /// 404DXN - Unlimited Splits
    #[default = false]
    pub split_404DXN_unlim: bool,

    /// 410DXN - Jungle Tomb (Exterior)
    #[default = false]
    pub split_410DXN: bool,

    /// 410DXN - Unlimited Splits
    #[default = false]
    pub split_410DXN_unlim: bool,

    /// 411DXN - Sith Tomb (Interior)
    #[default = false]
    pub split_411DXN: bool,

    /// 411DXN - Unlimited Splits
    #[default = false]
    pub split_411DXN_unlim: bool,

    /// 421DXN - Dxun Turret Minigame
    #[default = false]
    pub split_421DXN: bool,

    /// 421DXN - Unlimited Splits
    #[default = false]
    pub split_421DXN_unlim: bool,

    // ONDERON

    /// Onderon Splits
    title_ond_splits: Title,

    /// 501OND - Iziz Spaceport
    #[default = false]
    pub split_501OND: bool,

    /// 501OND - Unlimited Splits
    #[default = false]
    pub split_501OND_unlim: bool,

    /// 502OND - Merchant Square
    #[default = false]
    pub split_502OND: bool,

    /// 502OND - Unlimited Splits
    #[default = false]
    pub split_502OND_unlim: bool,

    /// 503OND - Iziz Cantina
    #[default = false]
    pub split_503OND: bool,

    /// 503OND - Unlimited Splits
    #[default = false]
    pub split_503OND_unlim: bool,

    /// 504OND - Sky Ramp
    #[default = false]
    pub split_504OND: bool,

    /// 504OND - Unlimited Splits
    #[default = false]
    pub split_504OND_unlim: bool,

    /// 505OND - Iziz Turret Minigame
    #[default = false]
    pub split_505OND: bool,

    /// 505OND - Unlimited Splits
    #[default = false]
    pub split_505OND_unlim: bool,

    /// 506OND - Royal Palace
    #[default = false]
    pub split_506OND: bool,

    /// 506OND - Unlimited Splits
    #[default = false]
    pub split_506OND_unlim: bool,

    /// 510OND - Iziz Swoop Track
    #[default = false]
    pub split_510OND: bool,

    /// 510OND - Unlimited Splits
    #[default = false]
    pub split_510OND_unlim: bool,

    /// 511OND - Merchant Quarter
    #[default = false]
    pub split_511OND: bool,

    /// 511OND - Unlimited Splits
    #[default = false]
    pub split_511OND_unlim: bool,

    /// 512OND - Iziz Western Square
    #[default = false]
    pub split_512OND: bool,

    /// 512OND - Unlimited Splits
    #[default = false]
    pub split_512OND_unlim: bool,

    // DANTOOINE

    /// Dantooine Splits
    title_dan_splits: Title,

    /// 601DAN - Khoonda
    #[default = false]
    pub split_601DAN: bool,

    /// 601DAN - Unlimited Splits
    #[default = false]
    pub split_601DAN_unlim: bool,

    /// 602DAN - Khoonda Plains
    #[default = false]
    pub split_602DAN: bool,

    /// 602DAN - Unlimited Splits
    #[default = false]
    pub split_602DAN_unlim: bool,

    /// 603DAN - Movie Terminal on Khoonda Plains
    #[default = false]
    pub split_603DAN: bool,

    /// 603DAN - Unlimited Splits
    #[default = false]
    pub split_603DAN_unlim: bool,

    /// 604DAN - Crystal Cave
    #[default = false]
    pub split_604DAN: bool,

    /// 604DAN - Unlimited Splits
    #[default = false]
    pub split_604DAN_unlim: bool,

    /// 605DAN - Enclave Courtyard
    #[default = false]
    pub split_605DAN: bool,

    /// 605DAN - Unlimited Splits
    #[default = false]
    pub split_605DAN_unlim: bool,

    /// 610DAN - Enclave Sublevel
    #[default = false]
    pub split_610DAN: bool,

    /// 610DAN - Unlimited Splits
    #[default = false]
    pub split_610DAN_unlim: bool,

    /// 650DAN - Rebuilt Jedi Enclave
    #[default = false]
    pub split_650DAN: bool,

    /// 650DAN - Unlimited Splits
    #[default = false]
    pub split_650DAN_unlim: bool,

    // KORRIBAN

    /// Korriban Splits
    title_kor_splits: Title,

    /// 701KOR - Valley of the Dark Lords
    #[default = false]
    pub split_701KOR: bool,

    /// 701KOR - Unlimited Splits
    #[default = false]
    pub split_701KOR_unlim: bool,

    /// 702KOR - Sith Academy
    #[default = false]
    pub split_702KOR: bool,

    /// 702KOR - Unlimited Splits
    #[default = false]
    pub split_702KOR_unlim: bool,

    /// 710KOR - Shyrack Cave
    #[default = false]
    pub split_710KOR: bool,

    /// 710KOR - Unlimited Splits
    #[default = false]
    pub split_710KOR_unlim: bool,

    /// 711KOR - Secret Tomb
    #[default = false]
    pub split_711KOR: bool,

    /// 711KOR - Unlimited Splits
    #[default = false]
    pub split_711KOR_unlim: bool,

    // RAVAGER

    /// Ravager Splits
    title_rav_splits: Title,

    /// 851NIH - Command Deck
    #[default = false]
    pub split_851NIH: bool,

    /// 851NIH - Unlimited Splits
    #[default = false]
    pub split_851NIH_unlim: bool,

    /// 852NIH - Bridge
    #[default = false]
    pub split_852NIH: bool,

    /// 852NIH - Unlimited Splits
    #[default = false]
    pub split_852NIH_unlim: bool,

    /// 853NIH - Visas Nihilus Cutscene
    #[default = false]
    pub split_853NIH: bool,

    /// 853NIH - Unlimited Splits
    #[default = false]
    pub split_853NIH_unlim: bool,

    // MALACHOR V

    /// Malachor V Splits
    title_mal_splits: Title,

    /// 901MAL - Surface
    #[default = false]
    pub split_901MAL: bool,

    /// 901MAL - Unlimited Splits
    #[default = false]
    pub split_901MAL_unlim: bool,

    /// 902MAL - Depths
    #[default = false]
    pub split_902MAL: bool,

    /// 902MAL - Unlimited Splits
    #[default = false]
    pub split_902MAL_unlim: bool,

    /// 903MAL - Trayus Academy
    #[default = false]
    pub split_903MAL: bool,

    /// 903MAL - Unlimited Splits
    #[default = false]
    pub split_903MAL_unlim: bool,

    /// 904MAL - Trayus Core (Final Module)
    #[default = false]
    pub split_904MAL: bool,

    /// 904MAL - Unlimited Splits
    #[default = false]
    pub split_904MAL_unlim: bool,

    /// 905MAL - Trayus Crescent
    #[default = false]
    pub split_905MAL: bool,

    /// 905MAL - Unlimited Splits
    #[default = false]
    pub split_905MAL_unlim: bool,

    /// 906MAL - Trayus Proving Grounds
    #[default = false]
    pub split_906MAL: bool,

    /// 906MAL - Unlimited Splits
    #[default = false]
    pub split_906MAL_unlim: bool,

    /// 907MAL - Trayus Core (Traya/Sion Cutscene)
    #[default = false]
    pub split_907MAL: bool,

    /// 907MAL - Unlimited Splits
    #[default = false]
    pub split_907MAL_unlim: bool,
}
