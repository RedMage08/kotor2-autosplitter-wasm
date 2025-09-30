// src/lib.rs
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use asr::{
    future::{next_tick, retry},
    settings::Gui,
    timer::{self, TimerState},
    watcher::Watcher,
    Address, Process,
    deep_pointer::DeepPointer,
    string::ArrayCString,
    file_format::pe,
};
mod settings;
use settings::Settings;
mod data;
use data::{AreaCode, GameVersion};

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
                let version_str = match &game_version {
                    GameVersion::Steam => "STEAM",
                    GameVersion::GOG => "GOG",
                    GameVersion::Unknown(_) => "UNKNOWN",
                };
                asr::print_message(&format!("Detected game version: {:?}", game_version));

                let mut watchers = Watchers::default();
                let addresses = Addresses::init(&process, &game_version).await;

                loop {
                    settings.update();

                    if let Err(err) = update_loop(&process, &addresses, &mut watchers) {
                        asr::print_message(&format!("Update loop error: {}", err));
                    }

                    match timer::state() {
                        TimerState::Running | TimerState::Paused => {
                            handle_running_state(&mut watchers, &settings);
                        }
                        TimerState::NotRunning => {
                            if start_timer(&watchers) {
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
    area: Watcher<AreaCode>,
    is_loading: Watcher<bool>,
    is_active_window: Watcher<i32>,
    is_not_amg: Watcher<i32>,
    traya_battle_state: Watcher<i32>,
    entered_areas: Vec<AreaCode>,
    did_final_split: bool,
}
struct Addresses {
    area: DeepPointer<5>,
    is_loading: DeepPointer<5>,
    is_active_window: Address,
    is_not_amg: Address,
    traya_battle_state: DeepPointer<4>,
}

impl Addresses {
    async fn init(process: &Process, game_version: &GameVersion) -> Self {
        asr::print_message("Initializing memory addresses...");

        // get_module_range returns (Address, u64) in the asr version you're using.
        let (base, _size) = process
            .get_module_range(PROCESS_NAME)
            .unwrap_or((Address::NULL, 0));

        if base == Address::NULL {
            asr::print_message("Failed to get module base address! Base is NULL.");
        } else {
            asr::print_message(&format!("Module base address: {base:?}"));
        }

        match game_version {
            GameVersion::Steam => Self {
                area: DeepPointer::new_32bit(base, &[0x0061B4A4, 0x4, 0x4, 0x2FC, 0x5]),
                is_loading: DeepPointer::new_32bit(base, &[0x0061B4A4, 0x4, 0x4, 0x278, 0xCC]),
                traya_battle_state: DeepPointer::new_32bit(base, &[0x0061B4A4, 0x8, 0x4, 0x1E136]),
                is_active_window: base + 0x0061B4E0,
                is_not_amg: base + 0x006309D0,
            },
            GameVersion::GOG => Self {
                area: DeepPointer::new_32bit(base, &[0x00611C04, 0x4, 0x4, 0x2FC, 0x5]),
                is_loading: DeepPointer::new_32bit(base, &[0x00611C04, 0x4, 0x4, 0x278, 0xCC]),
                traya_battle_state: DeepPointer::new_32bit(base, &[0x00611C04, 0x8, 0x4, 0x1E136]),
                is_active_window: base + 0x00611C40,
                is_not_amg: base + 0x00632BA0,
            },
            GameVersion::Unknown(_) => Self {
                area: DeepPointer::new_32bit(Address::NULL, &[]),
                is_loading: DeepPointer::new_32bit(Address::NULL, &[]),
                traya_battle_state: DeepPointer::new_32bit(Address::NULL, &[]),
                is_active_window: Address::NULL,
                is_not_amg: Address::NULL,
            },
        }
    }
}

fn detect_game_version(process: &Process) -> GameVersion {
    // Try to get base & size via get_module_range first
    match process.get_module_range(PROCESS_NAME) {
        Ok((base, size)) => {
            asr::print_message(&format!("get_module_range -> base: {base:?}, size: {size}"));

            // If we have a readable base, try parsing PE headers for SizeOfImage (better under Wine)
            if base != Address::NULL {
                if let Some(pe_size) = pe::read_size_of_image(process, base) {
                    asr::print_message(&format!("PE SizeOfImage: {}", pe_size));
                    return match pe_size as u64 {
                        7049216 => GameVersion::Steam,
                        7012352 => GameVersion::GOG,
                        other => GameVersion::Unknown(format!("PE SizeOfImage: {}", other)),
                    };
                } else {
                    asr::print_message("pe::read_size_of_image returned None; falling back to reported size");
                }
            }

            // Fallback to the size reported by get_module_range
            match size {
                7049216 => GameVersion::Steam,
                7012352 => GameVersion::GOG,
                other => {
                    asr::print_message(&format!("Unknown module size: {}", other));
                    GameVersion::Unknown(format!("Module size: {}", other))
                }
            }
        }
        Err(e) => {
            // If get_module_range fails, try the get_module_size API before giving up.
            asr::print_message(&format!("get_module_range failed: {:?}", e));
            match process.get_module_size(PROCESS_NAME) {
                Ok(size) => {
                    asr::print_message(&format!("get_module_size fallback -> {}", size));
                    match size {
                        7049216 => GameVersion::Steam,
                        7012352 => GameVersion::GOG,
                        other => GameVersion::Unknown(format!("Module size: {}", other)),
                    }
                }
                Err(e2) => {
                    asr::print_message(&format!("Error getting module size: {:?}", e2));
                    GameVersion::Unknown(format!("Error: {:?} / {:?}", e, e2))
                }
            }
        }
    }
}

fn update_loop(
    process: &Process,
    addresses: &Addresses,
    watchers: &mut Watchers,
) -> Result<(), String> {
    // --- Area ---
    if let Ok(raw_area) = addresses.area.deref::<ArrayCString<6>>(process) {
        let area_str = raw_area.validate_utf8().unwrap_or("unknown");
        let area = AreaCode::from(area_str);
        asr::print_message(&format!("Area read: '{}' -> {:?}", area_str, area));
        watchers.area.update(Some(area));
    } else {
        asr::print_message("Failed to deref area pointer");
    }

    // --- Loading ---
    if let Ok(addr) = addresses.is_loading.deref_offsets(process) {
        let is_loading = process.read::<i32>(addr).unwrap_or(0);
        asr::print_message(&format!("is_loading raw: {} -> {}", is_loading, is_loading != 0));
        watchers.is_loading.update(Some(is_loading != 0));
    } else {
        asr::print_message("Failed to deref is_loading pointer");
    }

    // --- Traya battle state ---
    if let Ok(addr) = addresses.traya_battle_state.deref_offsets(process) {
        let state = process.read::<i32>(addr).unwrap_or(0);
        asr::print_message(&format!("Traya battle state: {}", state));
        watchers.traya_battle_state.update(Some(state));
    } else {
        asr::print_message("Failed to deref traya_battle_state pointer");
    }

    // --- Direct reads ---
    let active_window = process.read::<i32>(addresses.is_active_window).unwrap_or(0);
    asr::print_message(&format!("is_active_window: {}", active_window));
    watchers.is_active_window.update(Some(active_window));

    let not_amg = process.read::<i32>(addresses.is_not_amg).unwrap_or(0);
    asr::print_message(&format!("is_not_amg: {}", not_amg));
    watchers.is_not_amg.update(Some(not_amg));

    Ok(())
}

fn handle_running_state(watchers: &mut Watchers, settings: &Settings) {
    // --- Game Time Pause/Resume ---
    if let Some(is_loading) = &watchers.is_loading.pair {
        let not_amg = watchers.is_not_amg.pair.as_ref().map_or(0, |w| w.current);
        let active_window = watchers.is_active_window.pair.as_ref().map_or(0, |w| w.current);

        if is_loading.current && not_amg == 0 && active_window == 1 {
            timer::pause_game_time();
        } else {
            timer::resume_game_time();
        }
    }

    // --- Splitting on Area Change ---
    if let Some(area_pair) = &watchers.area.pair {
        if area_pair.old != area_pair.current {
            if should_split(&area_pair.current, settings, &mut watchers.entered_areas) {
                watchers.entered_areas.push(area_pair.current.clone());
                timer::split();
                asr::print_message(&format!("Split triggered for area: {:?}", area_pair.current));
            }
        }
    }

    // --- Final Split (Traya battle) ---
    if let (Some(traya_state), Some(area_pair)) =
        (&watchers.traya_battle_state.pair, &watchers.area.pair)
    {
        if area_pair.current == AreaCode::a904MAL
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

fn start_timer(watchers: &Watchers) -> bool {
    if let Some(area_pair) = &watchers.area.pair {
        asr::print_message(&format!("Checking start condition: {:?}", area_pair.current));
        if area_pair.current == AreaCode::a001EBO {
            let loading = watchers.is_loading.pair.as_ref().map_or(false, |l| l.current);
            asr::print_message(&format!("a001EBO detected, is_loading = {}", loading));
            if loading {
                timer::start();
                timer::set_game_time(asr::time::Duration::seconds_f64(0.735));
                return true;
            }
        }
    }
    false
}

fn should_split(area: &AreaCode, settings: &Settings, entered_areas: &mut Vec<AreaCode>) -> bool {
    area.should_split(settings, entered_areas)
}

