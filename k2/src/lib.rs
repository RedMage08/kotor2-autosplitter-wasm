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
        if traya_state.current == 3 && !watchers.did_final_split {
            watchers.did_final_split = true;
            timer::split();
            asr::print_message("Final split at Traya battle.");
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
        "002EBO" => settings._002EBO && (settings._002EBO_unlim || !entered_areas.contains(area)),
        "003EBO" => settings._003EBO && (settings._003EBO_unlim || !entered_areas.contains(area)),
        "004EBO" => settings._004EBO && (settings._004EBO_unlim || !entered_areas.contains(area)),
        "005EBO" => settings._005EBO && (settings._005EBO_unlim || !entered_areas.contains(area)),
        "006EBO" => settings._006EBO && (settings._006EBO_unlim || !entered_areas.contains(area)),
        "007EBO" => settings._007EBO && (settings._007EBO_unlim || !entered_areas.contains(area)),

        // PERAGUS
        "101PER" => settings._101PER && (settings._101PER_unlim || !entered_areas.contains(area)),
        "102PER" => settings._102PER && (settings._102PER_unlim || !entered_areas.contains(area)),
        "103PER" => settings._103PER && (settings._103PER_unlim || !entered_areas.contains(area)),
        "104PER" => settings._104PER && (settings._104PER_unlim || !entered_areas.contains(area)),
        "105PER" => settings._105PER && (settings._105PER_unlim || !entered_areas.contains(area)),
        "106PER" => settings._106PER && (settings._106PER_unlim || !entered_areas.contains(area)),
        "107PER" => settings._107PER && (settings._107PER_unlim || !entered_areas.contains(area)),

        // HARBINGER
        "151HAR" => settings._151HAR && (settings._151HAR_unlim || !entered_areas.contains(area)),
        "152HAR" => settings._152HAR && (settings._152HAR_unlim || !entered_areas.contains(area)),
        "153HAR" => settings._153HAR && (settings._153HAR_unlim || !entered_areas.contains(area)),
        "154HAR" => settings._154HAR && (settings._154HAR_unlim || !entered_areas.contains(area)),

        // TELOS - CITADEL STATION
        "201TEL" => settings._201TEL && (settings._201TEL_unlim || !entered_areas.contains(area)),
        "202TEL" => settings._202TEL && (settings._202TEL_unlim || !entered_areas.contains(area)),
        "203TEL" => settings._203TEL && (settings._203TEL_unlim || !entered_areas.contains(area)),
        "204TEL" => settings._204TEL && (settings._204TEL_unlim || !entered_areas.contains(area)),
        "205TEL" => settings._205TEL && (settings._205TEL_unlim || !entered_areas.contains(area)),
        "207TEL" => settings._207TEL && (settings._207TEL_unlim || !entered_areas.contains(area)),
        "208TEL" => settings._208TEL && (settings._208TEL_unlim || !entered_areas.contains(area)),
        "209TEL" => settings._209TEL && (settings._209TEL_unlim || !entered_areas.contains(area)),
        "211TEL" => settings._211TEL && (settings._211TEL_unlim || !entered_areas.contains(area)),
        "220TEL" => settings._220TEL && (settings._220TEL_unlim || !entered_areas.contains(area)),
        "221TEL" => settings._221TEL && (settings._221TEL_unlim || !entered_areas.contains(area)),
        "222TEL" => settings._222TEL && (settings._222TEL_unlim || !entered_areas.contains(area)),

        // TELOS - SURFACE
        "231TEL" => settings._231TEL && (settings._231TEL_unlim || !entered_areas.contains(area)),
        "232TEL" => settings._232TEL && (settings._232TEL_unlim || !entered_areas.contains(area)),
        "233TEL" => settings._233TEL && (settings._233TEL_unlim || !entered_areas.contains(area)),
        "261TEL" => settings._261TEL && (settings._261TEL_unlim || !entered_areas.contains(area)),
        "262TEL" => settings._262TEL && (settings._262TEL_unlim || !entered_areas.contains(area)),
        "950COR" => settings._950COR && (settings._950COR_unlim || !entered_areas.contains(area)),

        // NAR SHADDAA
        "301NAR" => settings._301NAR && (settings._301NAR_unlim || !entered_areas.contains(area)),
        "302NAR" => settings._302NAR && (settings._302NAR_unlim || !entered_areas.contains(area)),
        "303NAR" => settings._303NAR && (settings._303NAR_unlim || !entered_areas.contains(area)),
        "304NAR" => settings._304NAR && (settings._304NAR_unlim || !entered_areas.contains(area)),
        "305NAR" => settings._305NAR && (settings._305NAR_unlim || !entered_areas.contains(area)),
        "306NAR" => settings._306NAR && (settings._306NAR_unlim || !entered_areas.contains(area)),
        "351NAR" => settings._351NAR && (settings._351NAR_unlim || !entered_areas.contains(area)),
        "352NAR" => settings._352NAR && (settings._352NAR_unlim || !entered_areas.contains(area)),
        "371NAR" => settings._371NAR && (settings._371NAR_unlim || !entered_areas.contains(area)),

        // DXUN
        "401DXN" => settings._401DXN && (settings._401DXN_unlim || !entered_areas.contains(area)),
        "402DXN" => settings._402DXN && (settings._402DXN_unlim || !entered_areas.contains(area)),
        "403DXN" => settings._403DXN && (settings._403DXN_unlim || !entered_areas.contains(area)),
        "404DXN" => settings._404DXN && (settings._404DXN_unlim || !entered_areas.contains(area)),
        "410DXN" => settings._410DXN && (settings._410DXN_unlim || !entered_areas.contains(area)),
        "411DXN" => settings._411DXN && (settings._411DXN_unlim || !entered_areas.contains(area)),
        "421DXN" => settings._421DXN && (settings._421DXN_unlim || !entered_areas.contains(area)),

        // ONDERON
        "501OND" => settings._501OND && (settings._501OND_unlim || !entered_areas.contains(area)),
        "502OND" => settings._502OND && (settings._502OND_unlim || !entered_areas.contains(area)),
        "503OND" => settings._503OND && (settings._503OND_unlim || !entered_areas.contains(area)),
        "504OND" => settings._504OND && (settings._504OND_unlim || !entered_areas.contains(area)),
        "505OND" => settings._505OND && (settings._505OND_unlim || !entered_areas.contains(area)),
        "506OND" => settings._506OND && (settings._506OND_unlim || !entered_areas.contains(area)),
        "510OND" => settings._510OND && (settings._510OND_unlim || !entered_areas.contains(area)),
        "511OND" => settings._511OND && (settings._511OND_unlim || !entered_areas.contains(area)),
        "512OND" => settings._512OND && (settings._512OND_unlim || !entered_areas.contains(area)),

        // DANTOOINE
        "601DAN" => settings._601DAN && (settings._601DAN_unlim || !entered_areas.contains(area)),
        "602DAN" => settings._602DAN && (settings._602DAN_unlim || !entered_areas.contains(area)),
        "603DAN" => settings._603DAN && (settings._603DAN_unlim || !entered_areas.contains(area)),
        "604DAN" => settings._604DAN && (settings._604DAN_unlim || !entered_areas.contains(area)),
        "605DAN" => settings._605DAN && (settings._605DAN_unlim || !entered_areas.contains(area)),
        "610DAN" => settings._610DAN && (settings._610DAN_unlim || !entered_areas.contains(area)),
        "650DAN" => settings._650DAN && (settings._650DAN_unlim || !entered_areas.contains(area)),

        // KORRIBAN
        "701KOR" => settings._701KOR && (settings._701KOR_unlim || !entered_areas.contains(area)),
        "702KOR" => settings._702KOR && (settings._702KOR_unlim || !entered_areas.contains(area)),
        "710KOR" => settings._710KOR && (settings._710KOR_unlim || !entered_areas.contains(area)),
        "711KOR" => settings._711KOR && (settings._711KOR_unlim || !entered_areas.contains(area)),

        // RAVAGER
        "851NIH" => settings._851NIH && (settings._851NIH_unlim || !entered_areas.contains(area)),
        "852NIH" => settings._852NIH && (settings._852NIH_unlim || !entered_areas.contains(area)),
        "853NIH" => settings._853NIH && (settings._853NIH_unlim || !entered_areas.contains(area)),

        // MALACHOR V
        "901MAL" => settings._901MAL && (settings._901MAL_unlim || !entered_areas.contains(area)),
        "902MAL" => settings._902MAL && (settings._902MAL_unlim || !entered_areas.contains(area)),
        "903MAL" => settings._903MAL && (settings._903MAL_unlim || !entered_areas.contains(area)),
        "904MAL" => settings._904MAL && (settings._904MAL_unlim || !entered_areas.contains(area)),
        "905MAL" => settings._905MAL && (settings._905MAL_unlim || !entered_areas.contains(area)),
        "906MAL" => settings._906MAL && (settings._906MAL_unlim || !entered_areas.contains(area)),
        "907MAL" => settings._907MAL && (settings._907MAL_unlim || !entered_areas.contains(area)),

        _ => false,
    }
}

