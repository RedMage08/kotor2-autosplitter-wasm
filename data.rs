#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::settings::Settings;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AreaCode {
    // EBON HAWK
    a001EBO,
    a002EBO,
    a003EBO,
    a004EBO,
    a005EBO,
    a006EBO,
    a007EBO,

    // PERAGUS
    a101PER,
    a102PER,
    a103PER,
    a104PER,
    a105PER,
    a106PER,
    a107PER,

    // HARBINGER
    a151HAR,
    a152HAR,
    a153HAR,
    a154HAR,

    // TELOS - CITADEL STATION
    a201TEL,
    a202TEL,
    a203TEL,
    a204TEL,
    a205TEL,
    a207TEL,
    a208TEL,
    a209TEL,
    a211TEL,
    a220TEL,
    a221TEL,
    a222TEL,

    // TELOS - SURFACE
    a231TEL,
    a232TEL,
    a233TEL,
    a261TEL,
    a262TEL,
    a950COR,

    // NAR SHADDAA
    a301NAR,
    a302NAR,
    a303NAR,
    a304NAR,
    a305NAR,
    a306NAR,
    a351NAR,
    a352NAR,
    a371NAR,

    // DXUN
    a401DXN,
    a402DXN,
    a403DXN,
    a404DXN,
    a410DXN,
    a411DXN,
    a421DXN,

    // ONDERON
    a501OND,
    a502OND,
    a503OND,
    a504OND,
    a505OND,
    a506OND,
    a510OND,
    a511OND,
    a512OND,

    // DANTOOINE
    a601DAN,
    a602DAN,
    a603DAN,
    a604DAN,
    a605DAN,
    a610DAN,
    a650DAN,

    // KORRIBAN
    a701KOR,
    a702KOR,
    a710KOR,
    a711KOR,

    // RAVAGER
    a851NIH,
    a852NIH,
    a853NIH,

    // MALACHOR V
    a901MAL,
    a902MAL,
    a903MAL,
    a904MAL,
    a905MAL,
    a906MAL,
    a907MAL,

    // Any code not recognized.
    Unknown(String),
}

impl From<&str> for AreaCode {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            // EBON HAWK
            "001EBO" => Self::a001EBO,
            "002EBO" => Self::a002EBO,
            "003EBO" => Self::a003EBO,
            "004EBO" => Self::a004EBO,
            "005EBO" => Self::a005EBO,
            "006EBO" => Self::a006EBO,
            "007EBO" => Self::a007EBO,

            // PERAGUS
            "101PER" => Self::a101PER,
            "102PER" => Self::a102PER,
            "103PER" => Self::a103PER,
            "104PER" => Self::a104PER,
            "105PER" => Self::a105PER,
            "106PER" => Self::a106PER,
            "107PER" => Self::a107PER,

            // HARBINGER
            "151HAR" => Self::a151HAR,
            "152HAR" => Self::a152HAR,
            "153HAR" => Self::a153HAR,
            "154HAR" => Self::a154HAR,

            // TELOS - CITADEL STATION
            "201TEL" => Self::a201TEL,
            "202TEL" => Self::a202TEL,
            "203TEL" => Self::a203TEL,
            "204TEL" => Self::a204TEL,
            "205TEL" => Self::a205TEL,
            "207TEL" => Self::a207TEL,
            "208TEL" => Self::a208TEL,
            "209TEL" => Self::a209TEL,
            "211TEL" => Self::a211TEL,
            "220TEL" => Self::a220TEL,
            "221TEL" => Self::a221TEL,
            "222TEL" => Self::a222TEL,

            // TELOS - SURFACE
            "231TEL" => Self::a231TEL,
            "232TEL" => Self::a232TEL,
            "233TEL" => Self::a233TEL,
            "261TEL" => Self::a261TEL,
            "262TEL" => Self::a262TEL,
            "950COR" => Self::a950COR,

            // NAR SHADDAA
            "301NAR" => Self::a301NAR,
            "302NAR" => Self::a302NAR,
            "303NAR" => Self::a303NAR,
            "304NAR" => Self::a304NAR,
            "305NAR" => Self::a305NAR,
            "306NAR" => Self::a306NAR,
            "351NAR" => Self::a351NAR,
            "352NAR" => Self::a352NAR,
            "371NAR" => Self::a371NAR,

            // DXUN
            "401DXN" => Self::a401DXN,
            "402DXN" => Self::a402DXN,
            "403DXN" => Self::a403DXN,
            "404DXN" => Self::a404DXN,
            "410DXN" => Self::a410DXN,
            "411DXN" => Self::a411DXN,
            "421DXN" => Self::a421DXN,

            // ONDERON
            "501OND" => Self::a501OND,
            "502OND" => Self::a502OND,
            "503OND" => Self::a503OND,
            "504OND" => Self::a504OND,
            "505OND" => Self::a505OND,
            "506OND" => Self::a506OND,
            "510OND" => Self::a510OND,
            "511OND" => Self::a511OND,
            "512OND" => Self::a512OND,

            // DANTOOINE
            "601DAN" => Self::a601DAN,
            "602DAN" => Self::a602DAN,
            "603DAN" => Self::a603DAN,
            "604DAN" => Self::a604DAN,
            "605DAN" => Self::a605DAN,
            "610DAN" => Self::a610DAN,
            "650DAN" => Self::a650DAN,

            // KORRIBAN
            "701KOR" => Self::a701KOR,
            "702KOR" => Self::a702KOR,
            "710KOR" => Self::a710KOR,
            "711KOR" => Self::a711KOR,

            // RAVAGER
            "851NIH" => Self::a851NIH,
            "852NIH" => Self::a852NIH,
            "853NIH" => Self::a853NIH,

            // MALACHOR V
            "901MAL" => Self::a901MAL,
            "902MAL" => Self::a902MAL,
            "903MAL" => Self::a903MAL,
            "904MAL" => Self::a904MAL,
            "905MAL" => Self::a905MAL,
            "906MAL" => Self::a906MAL,
            "907MAL" => Self::a907MAL,

            other => Self::Unknown(other.to_string()),
        }
    }
}

impl AreaCode {
    /// Checks if this area code should trigger a split.
    pub fn should_split(
        &self,
        settings: &Settings,
        entered_areas: &mut Vec<AreaCode>,
    ) -> bool {
        match self {

        // EBON HAWK
        Self::a001EBO => settings.split_001EBO_unlim || !entered_areas.contains(self),
        Self::a002EBO => settings.split_002EBO && (settings.split_002EBO_unlim || !entered_areas.contains(self)),
        Self::a003EBO => settings.split_003EBO && (settings.split_003EBO_unlim || !entered_areas.contains(self)),
        Self::a004EBO => settings.split_004EBO && (settings.split_004EBO_unlim || !entered_areas.contains(self)),
        Self::a005EBO => settings.split_005EBO && (settings.split_005EBO_unlim || !entered_areas.contains(self)),
        Self::a006EBO => settings.split_006EBO && (settings.split_006EBO_unlim || !entered_areas.contains(self)),
        Self::a007EBO => settings.split_007EBO && (settings.split_007EBO_unlim || !entered_areas.contains(self)),

        // PERAGUS
        Self::a101PER => settings.split_101PER && (settings.split_101PER_unlim || !entered_areas.contains(self)),
        Self::a102PER => settings.split_102PER && (settings.split_102PER_unlim || !entered_areas.contains(self)),
        Self::a103PER => settings.split_103PER && (settings.split_103PER_unlim || !entered_areas.contains(self)),
        Self::a104PER => settings.split_104PER && (settings.split_104PER_unlim || !entered_areas.contains(self)),
        Self::a105PER => settings.split_105PER && (settings.split_105PER_unlim || !entered_areas.contains(self)),
        Self::a106PER => settings.split_106PER && (settings.split_106PER_unlim || !entered_areas.contains(self)),
        Self::a107PER => settings.split_107PER && (settings.split_107PER_unlim || !entered_areas.contains(self)),

        // HARBINGER
        Self::a151HAR => settings.split_151HAR && (settings.split_151HAR_unlim || !entered_areas.contains(self)),
        Self::a152HAR => settings.split_152HAR && (settings.split_152HAR_unlim || !entered_areas.contains(self)),
        Self::a153HAR => settings.split_153HAR && (settings.split_153HAR_unlim || !entered_areas.contains(self)),
        Self::a154HAR => settings.split_154HAR && (settings.split_154HAR_unlim || !entered_areas.contains(self)),

        // TELOS - CITADEL STATION
        Self::a201TEL => settings.split_201TEL && (settings.split_201TEL_unlim || !entered_areas.contains(self)),
        Self::a202TEL => settings.split_202TEL && (settings.split_202TEL_unlim || !entered_areas.contains(self)),
        Self::a203TEL => settings.split_203TEL && (settings.split_203TEL_unlim || !entered_areas.contains(self)),
        Self::a204TEL => settings.split_204TEL && (settings.split_204TEL_unlim || !entered_areas.contains(self)),
        Self::a205TEL => settings.split_205TEL && (settings.split_205TEL_unlim || !entered_areas.contains(self)),
        Self::a207TEL => settings.split_207TEL && (settings.split_207TEL_unlim || !entered_areas.contains(self)),
        Self::a208TEL => settings.split_208TEL && (settings.split_208TEL_unlim || !entered_areas.contains(self)),
        Self::a209TEL => settings.split_209TEL && (settings.split_209TEL_unlim || !entered_areas.contains(self)),
        Self::a211TEL => settings.split_211TEL && (settings.split_211TEL_unlim || !entered_areas.contains(self)),
        Self::a220TEL => settings.split_220TEL && (settings.split_220TEL_unlim || !entered_areas.contains(self)),
        Self::a221TEL => settings.split_221TEL && (settings.split_221TEL_unlim || !entered_areas.contains(self)),
        Self::a222TEL => settings.split_222TEL && (settings.split_222TEL_unlim || !entered_areas.contains(self)),

        // TELOS - SURFACE
        Self::a231TEL => settings.split_231TEL && (settings.split_231TEL_unlim || !entered_areas.contains(self)),
        Self::a232TEL => settings.split_232TEL && (settings.split_232TEL_unlim || !entered_areas.contains(self)),
        Self::a233TEL => settings.split_233TEL && (settings.split_233TEL_unlim || !entered_areas.contains(self)),
        Self::a261TEL => settings.split_261TEL && (settings.split_261TEL_unlim || !entered_areas.contains(self)),
        Self::a262TEL => settings.split_262TEL && (settings.split_262TEL_unlim || !entered_areas.contains(self)),
        Self::a950COR => settings.split_950COR && (settings.split_950COR_unlim || !entered_areas.contains(self)),

        // NAR SHADDAA
        Self::a301NAR => settings.split_301NAR && (settings.split_301NAR_unlim || !entered_areas.contains(self)),
        Self::a302NAR => settings.split_302NAR && (settings.split_302NAR_unlim || !entered_areas.contains(self)),
        Self::a303NAR => settings.split_303NAR && (settings.split_303NAR_unlim || !entered_areas.contains(self)),
        Self::a304NAR => settings.split_304NAR && (settings.split_304NAR_unlim || !entered_areas.contains(self)),
        Self::a305NAR => settings.split_305NAR && (settings.split_305NAR_unlim || !entered_areas.contains(self)),
        Self::a306NAR => settings.split_306NAR && (settings.split_306NAR_unlim || !entered_areas.contains(self)),
        Self::a351NAR => settings.split_351NAR && (settings.split_351NAR_unlim || !entered_areas.contains(self)),
        Self::a352NAR => settings.split_352NAR && (settings.split_352NAR_unlim || !entered_areas.contains(self)),
        Self::a371NAR => settings.split_371NAR && (settings.split_371NAR_unlim || !entered_areas.contains(self)),

        // DXUN
        Self::a401DXN => settings.split_401DXN && (settings.split_401DXN_unlim || !entered_areas.contains(self)),
        Self::a402DXN => settings.split_402DXN && (settings.split_402DXN_unlim || !entered_areas.contains(self)),
        Self::a403DXN => settings.split_403DXN && (settings.split_403DXN_unlim || !entered_areas.contains(self)),
        Self::a404DXN => settings.split_404DXN && (settings.split_404DXN_unlim || !entered_areas.contains(self)),
        Self::a410DXN => settings.split_410DXN && (settings.split_410DXN_unlim || !entered_areas.contains(self)),
        Self::a411DXN => settings.split_411DXN && (settings.split_411DXN_unlim || !entered_areas.contains(self)),
        Self::a421DXN => settings.split_421DXN && (settings.split_421DXN_unlim || !entered_areas.contains(self)),

        // ONDERON
        Self::a501OND => settings.split_501OND && (settings.split_501OND_unlim || !entered_areas.contains(self)),
        Self::a502OND => settings.split_502OND && (settings.split_502OND_unlim || !entered_areas.contains(self)),
        Self::a503OND => settings.split_503OND && (settings.split_503OND_unlim || !entered_areas.contains(self)),
        Self::a504OND => settings.split_504OND && (settings.split_504OND_unlim || !entered_areas.contains(self)),
        Self::a505OND => settings.split_505OND && (settings.split_505OND_unlim || !entered_areas.contains(self)),
        Self::a506OND => settings.split_506OND && (settings.split_506OND_unlim || !entered_areas.contains(self)),
        Self::a510OND => settings.split_510OND && (settings.split_510OND_unlim || !entered_areas.contains(self)),
        Self::a511OND => settings.split_511OND && (settings.split_511OND_unlim || !entered_areas.contains(self)),
        Self::a512OND => settings.split_512OND && (settings.split_512OND_unlim || !entered_areas.contains(self)),

        // DANTOOINE#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        Self::a601DAN => settings.split_601DAN && (settings.split_601DAN_unlim || !entered_areas.contains(self)),
        Self::a602DAN => settings.split_602DAN && (settings.split_602DAN_unlim || !entered_areas.contains(self)),
        Self::a603DAN => settings.split_603DAN && (settings.split_603DAN_unlim || !entered_areas.contains(self)),
        Self::a604DAN => settings.split_604DAN && (settings.split_604DAN_unlim || !entered_areas.contains(self)),
        Self::a605DAN => settings.split_605DAN && (settings.split_605DAN_unlim || !entered_areas.contains(self)),
        Self::a610DAN => settings.split_610DAN && (settings.split_610DAN_unlim || !entered_areas.contains(self)),
        Self::a650DAN => settings.split_650DAN && (settings.split_650DAN_unlim || !entered_areas.contains(self)),

        // KORRIBAN
        Self::a701KOR => settings.split_701KOR && (settings.split_701KOR_unlim || !entered_areas.contains(self)),
        Self::a702KOR => settings.split_702KOR && (settings.split_702KOR_unlim || !entered_areas.contains(self)),
        Self::a710KOR => settings.split_710KOR && (settings.split_710KOR_unlim || !entered_areas.contains(self)),
        Self::a711KOR => settings.split_711KOR && (settings.split_711KOR_unlim || !entered_areas.contains(self)),

        // RAVAGER
        Self::a851NIH => settings.split_851NIH && (settings.split_851NIH_unlim || !entered_areas.contains(self)),
        Self::a852NIH => settings.split_852NIH && (settings.split_852NIH_unlim || !entered_areas.contains(self)),
        Self::a853NIH => settings.split_853NIH && (settings.split_853NIH_unlim || !entered_areas.contains(self)),

        // MALACHOR V
        Self::a901MAL => settings.split_901MAL && (settings.split_901MAL_unlim || !entered_areas.contains(self)),
        Self::a902MAL => settings.split_902MAL && (settings.split_902MAL_unlim || !entered_areas.contains(self)),
        Self::a903MAL => settings.split_903MAL && (settings.split_903MAL_unlim || !entered_areas.contains(self)),
        Self::a904MAL => settings.split_904MAL && (settings.split_904MAL_unlim || !entered_areas.contains(self)),
        Self::a905MAL => settings.split_905MAL && (settings.split_905MAL_unlim || !entered_areas.contains(self)),
        Self::a906MAL => settings.split_906MAL && (settings.split_906MAL_unlim || !entered_areas.contains(self)),
        Self::a907MAL => settings.split_907MAL && (settings.split_907MAL_unlim || !entered_areas.contains(self)),
            _ => false,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameVersion {
    Steam,
    GOG,
    Unknown(String),
}

impl From<&str> for GameVersion {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "STEAM" => Self::Steam,
            "GOG" => Self::GOG,
            other => Self::Unknown(other.to_string()),
        }
    }
}
