use std::time::Duration;

use cfg::Config;
use makro::{Clipboard,
            X};
use poe::detection::{Belly,
                     Detect};
use poe::health::HpByColorsArray;
use poe::mana::SimpleMana;
use poe::skills::Skill;
use poe::{Flasks,
          PoE};
use tracing::error;

use crate::action::{make_keymaps,
                    Skills};
use crate::triggers::{TriggersHP,
                      TriggersMana};

mod action;
mod app;
mod cfg;
mod helpers;
mod keyboard_state;
mod triggers;

fn main() {
    tracing_subscriber::fmt::init();
    color_eyre::install().unwrap();

    let x = X::new();
    let args: Vec<String> = std::env::args().collect();

    assert!(args.len() == 3);

    let cfg_global = config::CfgGlobal::parse(std::path::Path::new(&args[1]));
    let cfg_character = config::CfgCharacter::parse(std::path::Path::new(&args[2]));

    let vd = makro::input::VDevice::new();

    let builder = overlay::BuilderOverlay::new();
    let ui_sender = builder.use_events();

    println!("Make sure you are on a charater. Not in town. Flasks are full");

    let poe = PoE { detection: Belly, health: HpByColorsArray, mana: SimpleMana };
    loop {
        if err!(poe.detection.is_poe_window_on(&x)) {
            if err!(poe.detection.is_on(&x)) {
                break;
            } else {
                error!("Did not detect the inside poe");
            }
        } else {
            error!("Did not detect PoE window");
        }
        std::thread::sleep(Duration::from_secs(1));
    }

    let skills: Skills = Skill::build_from_config(&cfg_global, &cfg_character, &x);
    let clip = Clipboard::new();

    // let (normal, with_shift, with_ctrl, with_alt) =
    // if let Some(keymaps) = &cfg_character.Keymaps {
    //     (normal, with_shift, with_ctrl, with_alt)
    // } else {
    //     (BTreeMap::new(), BTreeMap::new(), BTreeMap::new(), BTreeMap::new())
    // };

    let mut vec_of_clipboards = Vec::new();
    let normal = make_keymaps(
        cfg_global.Keymaps.as_ref().and_then(|i| i.normal.as_ref()),
        cfg_character.Keymaps.as_ref().and_then(|i| i.normal.as_ref()),
        &mut vec_of_clipboards,
    );
    let with_shift = make_keymaps(
        cfg_global.Keymaps.as_ref().and_then(|i| i.withShift.as_ref()),
        cfg_character.Keymaps.as_ref().and_then(|i| i.withShift.as_ref()),
        &mut vec_of_clipboards,
    );
    let with_ctrl = make_keymaps(
        cfg_global.Keymaps.as_ref().and_then(|i| i.withCtrl.as_ref()),
        cfg_character.Keymaps.as_ref().and_then(|i| i.withCtrl.as_ref()),
        &mut vec_of_clipboards,
    );
    let with_alt = make_keymaps(
        cfg_global.Keymaps.as_ref().and_then(|i| i.withAlt.as_ref()),
        cfg_character.Keymaps.as_ref().and_then(|i| i.withAlt.as_ref()),
        &mut vec_of_clipboards,
    );

    let (triggers_by_health, triggers_by_mana): (Option<TriggersHP>, Option<TriggersMana>) =
        if let Some(cfg) = cfg_character.Triggers.as_ref() {
            (
                cfg.byHealth.as_ref().map(|hp| TriggersHP::new(hp)),
                cfg.byMana.as_ref().map(|mana| TriggersMana::new(mana)),
            )
        } else {
            (None, None)
        };

    let config = Config::new(&cfg_global, &cfg_character);

    std::thread::spawn(move || {
        let flasks = Flasks::new(&cfg_global, &cfg_character, &x);
        std::thread::sleep(Duration::from_secs(1));
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        rt.block_on(async {
            let mut device =
                makro::Device::open(std::path::Path::new(&cfg_global.device_path)).unwrap();
            device.grab().unwrap();
            let device = device.into_event_stream().unwrap();
            app::run(
                config,
                skills,
                flasks,
                normal,
                with_shift,
                with_ctrl,
                with_alt,
                triggers_by_health,
                triggers_by_mana,
                x,
                device,
                vd,
                ui_sender,
                clip,
                vec_of_clipboards,
            )
            .await
        });
    });

    builder.build_and_run();
}
