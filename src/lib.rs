use asr::{
    future::{next_tick, retry},
    settings::Gui,
    timer::{self, TimerState},
    watcher::Watcher,
    Address, Process, PointerSize,
    string::{ArrayCString, ArrayString},
};
mod settings;
use settings::Settings;

asr::async_main!(stable);

const PROCESS_NAME: &str = "swkotor2.exe";

async fn main() {
    let mut settings = Settings::register();

    loop {
        let process = retry(|| Process::attach(PROCESS_NAME)).await;

        process
        .until_closes(async {
            asr::print_message("Attached to process.");

            let game_version = detect_game_version(&process);
            asr::print_message(&format!("Detected game version: {}", game_version));

            let mut watchers = Watchers::default();
            let addresses = Addresses::init(&process, &game_version).await;

            loop {
                settings.update();
                if let Err(err) = update_loop(&process, &addresses, &mut watchers, &settings) {
                    asr::print_message(&format!("Update loop error: {}", err));
                }

                match timer::state() {
                    TimerState::Running | TimerState::Paused => {
                        handle_running_state(&mut watchers, &settings);
                    }
                    TimerState::NotRunning => {
                        if start_timer(&watchers, &settings) {
                            timer::start();
                        }
                    }
                    TimerState::Ended => timer::reset(),
                      _ => {}
                }

                next_tick().await;
            }
        })
        .await;
    }

}

#[derive(Default)]
struct Watchers {
    area: Watcher<asr::string::ArrayCString<6>>,
    is_loading: Watcher<bool>,
    is_active_window: Watcher<i32>,
    is_not_amg: Watcher<i32>,
    traya_battle_state: Watcher<i32>,
    entered_areas: Vec<asr::string::ArrayCString<6>>,
    did_final_split: bool,
}

struct Addresses {
    area: Address,
    is_loading: Address,
    is_active_window: Address,
    is_not_amg: Address,
    traya_battle_state: Address,
}


impl Addresses {
    async fn init(process: &Process, game_version: &str) -> Self {
        asr::print_message("Initializing memory addresses...");
        let base_address = process.get_module_address(PROCESS_NAME).expect("Failed to get module address");

        match game_version {
            "Steam" => Self {
                area: base_address + 0x0061B4A4,
                is_loading: base_address + 0x0061B4A4,
                is_active_window: base_address + 0x0061B4E0,
                is_not_amg: base_address + 0x006309D0,
                traya_battle_state: base_address + 0x0061B4A4,
            },
            "GOG" => Self {
                area: base_address + 0x00611C04,
                is_loading: base_address + 0x00611C04,
                is_active_window: base_address + 0x00611C40,
                is_not_amg: base_address + 0x00632BA0,
                traya_battle_state: base_address + 0x00611C04,
            },
            _ => Self {
                area: Address::NULL,
                is_loading: Address::NULL,
                is_active_window: Address::NULL,
                is_not_amg: Address::NULL,
                traya_battle_state: Address::NULL,
            },
        }
    }
}

fn detect_game_version(process: &Process) -> String {
    match process.get_module_size(PROCESS_NAME) {
        Ok(size) => match size {
            7049216 => "Steam".to_string(),
            7012352 => "GOG".to_string(),
            _ => {
                asr::print_message(&format!("Unknown module size: {}", size));
                "Unknown".to_string()
            }
        },
        Err(e) => {
            asr::print_message(&format!("Error getting module size: {:?}", e));
            "Error".to_string()
        }
    }
}

fn update_loop(
    process: &Process,
    addresses: &Addresses,
    watchers: &mut Watchers,
    settings: &Settings,
) -> Result<(), String> {
    let resolve_pointer_chain = |base_address: Address, offsets: &[u32]| -> Result<Address, String> {
        let mut current_address = base_address;
        for &offset in offsets {
            current_address = match process.read::<u32>(current_address) {
                Ok(addr) => Address::new((addr as usize + offset as usize) as u64),
                Err(e) => return Err(format!("Failed to resolve pointer chain at offset {:X}: {:?}", offset, e)),
            };
        }
        Ok(current_address)
    };


    // Resolve 'area'
    let area_value = resolve_pointer_chain(addresses.area, &[0x4, 0x4, 0x2FC, 0x5])
    .and_then(|final_address| process.read::<ArrayCString<6>>(final_address).map_err(|e| format!("{:?}", e)))
    .unwrap_or_else(|e| {
        asr::print_message(&format!("Failed to read area: {:?}", e));
        ArrayCString::new()
    });
    watchers.area.update(Some(area_value));

    // Resolve 'is_loading'
    let is_loading_value = resolve_pointer_chain(addresses.is_loading, &[0x4, 0x4, 0x278, 0xCC])
    .and_then(|final_address| process.read::<i32>(final_address).map_err(|e| format!("{:?}", e)))
    .unwrap_or(0) != 0;
    watchers.is_loading.update(Some(is_loading_value));

    // Direct reads
    let is_active_window = process.read::<i32>(addresses.is_active_window).unwrap_or(0);
    watchers.is_active_window.update(Some(is_active_window));

    let is_not_amg = process.read::<i32>(addresses.is_not_amg).unwrap_or(0);
    watchers.is_not_amg.update(Some(is_not_amg));

    // Resolve 'traya_battle_state'
    let traya_battle_state_value = resolve_pointer_chain(addresses.traya_battle_state, &[0x8, 0x4, 0x1E136])
    .and_then(|final_address| process.read::<i32>(final_address).map_err(|e| format!("{:?}", e)))
    .unwrap_or(0);
    watchers.traya_battle_state.update(Some(traya_battle_state_value));

    Ok(())
}





fn handle_running_state(watchers: &mut Watchers, settings: &Settings) {
    if let Some(is_loading) = &watchers.is_loading.pair {
        if is_loading.current
            && watchers.is_not_amg.pair.as_ref().unwrap().current == 0
            && watchers.is_active_window.pair.as_ref().unwrap().current == 1
            {
                timer::pause_game_time();
            } else {
                timer::resume_game_time();
            }
    }

    if let Some(area) = &watchers.area.pair {
        if area.old != area.current {
            if should_split(&area.current, settings, &mut watchers.entered_areas) {
                watchers.entered_areas.push(area.current.clone());
                timer::split();
                asr::print_message(&format!("Split triggered for area: {}", area.current.validate_utf8().unwrap_or("<Invalid UTF-8>")));
            }
        }
    }


    if let Some(traya_state) = &watchers.traya_battle_state.pair {
        if let Some(area) = &watchers.area.pair { // Ensure `area` is available
            if area.current.validate_utf8().unwrap_or("<Invalid UTF-8>").to_uppercase() == "904MAL"
                && traya_state.current == 3
                && traya_state.current != traya_state.old
                && !watchers.did_final_split
                {
                    watchers.did_final_split = true;
                    timer::split();
                    asr::print_message("Final split at Traya battle.");
                }
        }
    }

}

fn start_timer(watchers: &Watchers, settings: &Settings) -> bool {
    if let Some(area) = &watchers.area.pair {
        // Convert the area string to uppercase
        let area_str = area.current.validate_utf8().unwrap_or("<Invalid UTF-8>").to_uppercase();

        if area_str == "001EBO" && watchers.is_loading.pair.as_ref().map_or(false, |l| l.current) {
            // Start the timer
            timer::start();

            // Apply the offset of 0.735 seconds
            timer::set_game_time(asr::time::Duration::seconds_f64(0.735));
            return true;
        }
    }
    false
}


fn should_split(
    area: &ArrayCString<6>,
    settings: &Settings,
    entered_areas: &mut Vec<ArrayCString<6>>,
) -> bool {
    let area_str = area.validate_utf8().unwrap_or("<Invalid UTF-8>").to_uppercase();

    match area_str.as_str() {
        // EBON HAWK
        "001EBO" => settings.split_001EBO && (settings.split_001EBO_unlim || !entered_areas.contains(area)),
        "002EBO" => settings.split_002EBO && (settings.split_002EBO_unlim || !entered_areas.contains(area)),
        "003EBO" => settings.split_003EBO && (settings.split_003EBO_unlim || !entered_areas.contains(area)),
        "004EBO" => settings.split_004EBO && (settings.split_004EBO_unlim || !entered_areas.contains(area)),
        "005EBO" => settings.split_005EBO && (settings.split_005EBO_unlim || !entered_areas.contains(area)),
        "006EBO" => settings.split_006EBO && (settings.split_006EBO_unlim || !entered_areas.contains(area)),
        "007EBO" => settings.split_007EBO && (settings.split_007EBO_unlim || !entered_areas.contains(area)),

        // PERAGUS
        "101PER" => settings.split_101PER && (settings.split_101PER_unlim || !entered_areas.contains(area)),
        "102PER" => settings.split_102PER && (settings.split_102PER_unlim || !entered_areas.contains(area)),
        "103PER" => settings.split_103PER && (settings.split_103PER_unlim || !entered_areas.contains(area)),
        "104PER" => settings.split_104PER && (settings.split_104PER_unlim || !entered_areas.contains(area)),
        "105PER" => settings.split_105PER && (settings.split_105PER_unlim || !entered_areas.contains(area)),
        "106PER" => settings.split_106PER && (settings.split_106PER_unlim || !entered_areas.contains(area)),
        "107PER" => settings.split_107PER && (settings.split_107PER_unlim || !entered_areas.contains(area)),

        // HARBINGER
        "151HAR" => settings.split_151HAR && (settings.split_151HAR_unlim || !entered_areas.contains(area)),
        "152HAR" => settings.split_152HAR && (settings.split_152HAR_unlim || !entered_areas.contains(area)),
        "153HAR" => settings.split_153HAR && (settings.split_153HAR_unlim || !entered_areas.contains(area)),
        "154HAR" => settings.split_154HAR && (settings.split_154HAR_unlim || !entered_areas.contains(area)),

        // TELOS - CITADEL STATION
        "201TEL" => settings.split_201TEL && (settings.split_201TEL_unlim || !entered_areas.contains(area)),
        "202TEL" => settings.split_202TEL && (settings.split_202TEL_unlim || !entered_areas.contains(area)),
        "203TEL" => settings.split_203TEL && (settings.split_203TEL_unlim || !entered_areas.contains(area)),
        "204TEL" => settings.split_204TEL && (settings.split_204TEL_unlim || !entered_areas.contains(area)),
        "205TEL" => settings.split_205TEL && (settings.split_205TEL_unlim || !entered_areas.contains(area)),
        "207TEL" => settings.split_207TEL && (settings.split_207TEL_unlim || !entered_areas.contains(area)),
        "208TEL" => settings.split_208TEL && (settings.split_208TEL_unlim || !entered_areas.contains(area)),
        "209TEL" => settings.split_209TEL && (settings.split_209TEL_unlim || !entered_areas.contains(area)),
        "211TEL" => settings.split_211TEL && (settings.split_211TEL_unlim || !entered_areas.contains(area)),
        "220TEL" => settings.split_220TEL && (settings.split_220TEL_unlim || !entered_areas.contains(area)),
        "221TEL" => settings.split_221TEL && (settings.split_221TEL_unlim || !entered_areas.contains(area)),
        "222TEL" => settings.split_222TEL && (settings.split_222TEL_unlim || !entered_areas.contains(area)),

        // TELOS - SURFACE
        "231TEL" => settings.split_231TEL && (settings.split_231TEL_unlim || !entered_areas.contains(area)),
        "232TEL" => settings.split_232TEL && (settings.split_232TEL_unlim || !entered_areas.contains(area)),
        "233TEL" => settings.split_233TEL && (settings.split_233TEL_unlim || !entered_areas.contains(area)),
        "261TEL" => settings.split_261TEL && (settings.split_261TEL_unlim || !entered_areas.contains(area)),
        "262TEL" => settings.split_262TEL && (settings.split_262TEL_unlim || !entered_areas.contains(area)),
        "950COR" => settings.split_950COR && (settings.split_950COR_unlim || !entered_areas.contains(area)),

        // NAR SHADDAA
        "301NAR" => settings.split_301NAR && (settings.split_301NAR_unlim || !entered_areas.contains(area)),
        "302NAR" => settings.split_302NAR && (settings.split_302NAR_unlim || !entered_areas.contains(area)),
        "303NAR" => settings.split_303NAR && (settings.split_303NAR_unlim || !entered_areas.contains(area)),
        "304NAR" => settings.split_304NAR && (settings.split_304NAR_unlim || !entered_areas.contains(area)),
        "305NAR" => settings.split_305NAR && (settings.split_305NAR_unlim || !entered_areas.contains(area)),
        "306NAR" => settings.split_306NAR && (settings.split_306NAR_unlim || !entered_areas.contains(area)),
        "351NAR" => settings.split_351NAR && (settings.split_351NAR_unlim || !entered_areas.contains(area)),
        "352NAR" => settings.split_352NAR && (settings.split_352NAR_unlim || !entered_areas.contains(area)),
        "371NAR" => settings.split_371NAR && (settings.split_371NAR_unlim || !entered_areas.contains(area)),

        // DXUN
        "401DXN" => settings.split_401DXN && (settings.split_401DXN_unlim || !entered_areas.contains(area)),
        "402DXN" => settings.split_402DXN && (settings.split_402DXN_unlim || !entered_areas.contains(area)),
        "403DXN" => settings.split_403DXN && (settings.split_403DXN_unlim || !entered_areas.contains(area)),
        "404DXN" => settings.split_404DXN && (settings.split_404DXN_unlim || !entered_areas.contains(area)),
        "410DXN" => settings.split_410DXN && (settings.split_410DXN_unlim || !entered_areas.contains(area)),
        "411DXN" => settings.split_411DXN && (settings.split_411DXN_unlim || !entered_areas.contains(area)),
        "421DXN" => settings.split_421DXN && (settings.split_421DXN_unlim || !entered_areas.contains(area)),

        // ONDERON
        "501OND" => settings.split_501OND && (settings.split_501OND_unlim || !entered_areas.contains(area)),
        "502OND" => settings.split_502OND && (settings.split_502OND_unlim || !entered_areas.contains(area)),
        "503OND" => settings.split_503OND && (settings.split_503OND_unlim || !entered_areas.contains(area)),
        "504OND" => settings.split_504OND && (settings.split_504OND_unlim || !entered_areas.contains(area)),
        "505OND" => settings.split_505OND && (settings.split_505OND_unlim || !entered_areas.contains(area)),
        "506OND" => settings.split_506OND && (settings.split_506OND_unlim || !entered_areas.contains(area)),
        "510OND" => settings.split_510OND && (settings.split_510OND_unlim || !entered_areas.contains(area)),
        "511OND" => settings.split_511OND && (settings.split_511OND_unlim || !entered_areas.contains(area)),
        "512OND" => settings.split_512OND && (settings.split_512OND_unlim || !entered_areas.contains(area)),

        // DANTOOINE
        "601DAN" => settings.split_601DAN && (settings.split_601DAN_unlim || !entered_areas.contains(area)),
        "602DAN" => settings.split_602DAN && (settings.split_602DAN_unlim || !entered_areas.contains(area)),
        "603DAN" => settings.split_603DAN && (settings.split_603DAN_unlim || !entered_areas.contains(area)),
        "604DAN" => settings.split_604DAN && (settings.split_604DAN_unlim || !entered_areas.contains(area)),
        "605DAN" => settings.split_605DAN && (settings.split_605DAN_unlim || !entered_areas.contains(area)),
        "610DAN" => settings.split_610DAN && (settings.split_610DAN_unlim || !entered_areas.contains(area)),
        "650DAN" => settings.split_650DAN && (settings.split_650DAN_unlim || !entered_areas.contains(area)),

        // KORRIBAN
        "701KOR" => settings.split_701KOR && (settings.split_701KOR_unlim || !entered_areas.contains(area)),
        "702KOR" => settings.split_702KOR && (settings.split_702KOR_unlim || !entered_areas.contains(area)),
        "710KOR" => settings.split_710KOR && (settings.split_710KOR_unlim || !entered_areas.contains(area)),
        "711KOR" => settings.split_711KOR && (settings.split_711KOR_unlim || !entered_areas.contains(area)),

        // RAVAGER
        "851NIH" => settings.split_851NIH && (settings.split_851NIH_unlim || !entered_areas.contains(area)),
        "852NIH" => settings.split_852NIH && (settings.split_852NIH_unlim || !entered_areas.contains(area)),
        "853NIH" => settings.split_853NIH && (settings.split_853NIH_unlim || !entered_areas.contains(area)),

        // MALACHOR V
        "901MAL" => settings.split_901MAL && (settings.split_901MAL_unlim || !entered_areas.contains(area)),
        "902MAL" => settings.split_902MAL && (settings.split_902MAL_unlim || !entered_areas.contains(area)),
        "903MAL" => settings.split_903MAL && (settings.split_903MAL_unlim || !entered_areas.contains(area)),
        "904MAL" => settings.split_904MAL && (settings.split_904MAL_unlim || !entered_areas.contains(area)),
        "905MAL" => settings.split_905MAL && (settings.split_905MAL_unlim || !entered_areas.contains(area)),
        "906MAL" => settings.split_906MAL && (settings.split_906MAL_unlim || !entered_areas.contains(area)),
        "907MAL" => settings.split_907MAL && (settings.split_907MAL_unlim || !entered_areas.contains(area)),

        _ => false,
    }
}

