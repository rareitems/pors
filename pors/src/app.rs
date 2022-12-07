use std::collections::BTreeMap;
use std::time::Duration;

use config::TriggerActions;
use makro::{Clipboard,
            EventType,
            InputEvent,
            Key,
            Pos,
            Rgb,
            R,
            X};
use overlay::Messages;
use poe::detection::{Belly,
                     Detect};
use poe::flasks::types::FlaskIndex;
use poe::health::{DetectHealth,
                  HpByColorsArray};
use poe::mana::SimpleMana;
use poe::{inventory,
          DetectMana,
          Flasks,
          PoE};
use tokio::select;
use tracing::info;

use crate::action::Action;
use crate::cfg::Config;
use crate::keyboard_state::KeyboardState;
use crate::triggers::{TriggersHP,
                      TriggersMana};
use crate::{err,
            Skills};

const SLEEP: Duration = Duration::from_millis(10);
macro_rules! sleep {
    () => {
        std::thread::sleep(SLEEP);
    };
}

#[allow(clippy::too_many_arguments)]
pub async fn run(
    config: Config,
    mut skills: Skills,
    mut flasks: Flasks,
    mut normal: BTreeMap<makro::Key, Action>,
    mut with_shift: BTreeMap<makro::Key, Action>,
    mut with_ctrl: BTreeMap<makro::Key, Action>,
    mut with_alt: BTreeMap<makro::Key, Action>,
    trigs_hp: Option<TriggersHP>,
    trigs_mana: Option<TriggersMana>,
    x: X,
    mut device: makro::EventStream,
    mut vd: makro::input::VDevice,
    ui: overlay::UserEventSender<Messages>,
    mut clip: Clipboard,
    vec_of_clipboards: Vec<String>,
) {
    let mut state = KeyboardState::Normal;
    let poe = PoE { detection: Belly, health: HpByColorsArray, mana: SimpleMana };
    info!("Launched");
    ui.send_event(overlay::Messages::Msg("Launched")).unwrap();

    let mut is_poewindow_on = false;
    let mut interval_title_check = tokio::time::interval(std::time::Duration::from_millis(1332));

    let mut interval_250 = tokio::time::interval(std::time::Duration::from_millis(333));

    let mut interval_ui = tokio::time::interval(std::time::Duration::from_millis(666));

    let local = tokio::task::LocalSet::new();

    local
        .run_until(async {
            loop {
                select! {
                    _ = interval_title_check.tick() => {
                        if err!(poe.detection.is_poe_window_on(&x)) {
                            is_poewindow_on = true;
                            ui.send_event(overlay::Messages::TitleOn).unwrap();
                        } else {
                            is_poewindow_on = false;
                            ui.send_event(overlay::Messages::TitleOff).unwrap();
                        }
                    },

                    event = device.next_event() => {
                        let event = event.unwrap();

                        if is_poewindow_on {
                            let key = makro::Key::new(event.code());
                            match event.value() {
                                0 => {
                                    state.on_release(key);
                                    vd.emit_event(event);
                                },
                                1 => {
                                    if state.change_on_press(key) {
                                        match state {
                                            KeyboardState::Normal => {
                                                if err!(handle_keys(key, &config, &mut normal, &mut flasks, &mut skills, state, &x, &mut vd, &mut clip, &poe, &ui, &vec_of_clipboards)) {
                                                    vd.emit_event(event);
                                                }
                                            }

                                            KeyboardState::LCtrl => {
                                                if err!(handle_keys(key, &config, &mut with_ctrl, &mut flasks, &mut skills, state, &x, &mut vd, &mut clip, &poe, &ui, &vec_of_clipboards)) {
                                                    vd.emit_event(event);
                                                }
                                            }

                                            KeyboardState::LShift => {
                                                if err!(handle_keys(key, &config, &mut with_shift, &mut flasks, &mut skills, state, &x, &mut vd, &mut clip, &poe, &ui, &vec_of_clipboards)) {
                                                    vd.emit_event(event);
                                                }
                                            }

                                            KeyboardState::LAlt => {
                                                if err!(handle_keys(key, &config, &mut with_alt, &mut flasks, &mut skills, state, &x, &mut vd, &mut clip, &poe, &ui, &vec_of_clipboards)) {
                                                    vd.emit_event(event);
                                                }
                                            }

                                            KeyboardState::LCtrlF => {
                                                ui.send_event(Messages::Msg("In state: LCtrlF")).unwrap();
                                                vd.emit_event(event);
                                            }

                                            KeyboardState::Enter => {
                                                ui.send_event(Messages::Msg("In state: Enter")).unwrap();
                                                vd.emit_event(event);
                                            }
                                        }
                                    } else {
                                        vd.emit_event(event);
                                    }
                                },
                                it => unreachable!("{it} KIND:{:?}, TYPE:{:?}", event.kind(), event.event_type()),
                                // _ => continue,
                            }
                        } else {
                            vd.emit_event(event);
                        }
                    },


                    _ = interval_250.tick(), if is_poewindow_on => {
                        if err!(poe.detection.is_on(&x)) {
                            if let Some(trigs) = trigs_hp.as_ref() {
                                err!(handle_health(trigs, &mut flasks, &skills, &poe, &x, &mut vd));
                            }

                            if let Some(trigs) = trigs_mana.as_ref() {
                                err!(handle_mana(trigs, &mut flasks, &skills, &poe, &x, &mut vd));
                            }

                            ui.send_event(overlay::Messages::IsOn).unwrap();
                        } else {
                            ui.send_event(overlay::Messages::IsOff).unwrap();
                            continue;
                        }
                    },

                    _ = interval_ui.tick() => {
                        ui.send_event(Messages::Draw).unwrap();
                    }

                    else => continue,
                }
                std::thread::sleep(Duration::from_millis(10));
            }
        })
        .await;
}

macro_rules! TriggerActionFlask {
    ($t:expr, $flasks:expr, $ret:expr, $x:expr, $vd:expr, $current:expr, $lit:literal) => {{
        if $flasks.check_and_pop($t, $x, $vd)? {
            $ret = true;
            info!("{}: Current Percent: {} TriggerActions::Flask::{:?}", $lit, $current, $t);
        }
    }};
}

macro_rules! TriggerActionFlaskSC {
    ($t:expr, $flasks:expr, $ret:expr, $x:expr, $vd:expr, $current:expr, $lit:literal) => {{
        if $flasks.is_on($t, $x)? {
            info!(
                "{}: Current Percent: {} TriggerActions::FlaskSC::{:?} IS ON SHORT-CIRCUIT",
                $lit, $current, $t
            );
            return Ok(false);
        }
        if $flasks.has_charges($t, $x)? {
            $flasks.pop($t, $vd);
            info!("{}: Current Percent: {} TriggerActions::FlaskSC::{:?}", $lit, $current, $t);
            return Ok(true);
        }
    }};
}

macro_rules! TriggerActionSkill {
    ($slot:expr, $skills:expr, $ret:expr, $x:expr, $vd:expr, $current:expr, $lit:literal) => {{
        info!("{}: Current Percent: {} TriggerActions::Skill::{:?}", $lit, $current, $slot);
        $skills[$slot]
            .as_ref()
            .unwrap_or_else(|| panic!("No skill defined in slot {}, but expected to use", $slot))
            .check_and_use($x, $vd)?;
    }};
}

fn handle_health<D, HP, MANA>(
    trigs: &TriggersHP,
    flasks: &mut Flasks,
    skills: &Skills,
    poe: &PoE<D, HP, MANA>,
    x: &X,
    vd: &mut makro::input::VDevice,
) -> R<bool>
where
    D: Detect,
    HP: DetectHealth,
    MANA: DetectMana,
{
    let current = poe.health.perc(x)?;
    let mut ret = false;

    for (perc, handle) in &trigs.percs {
        if current <= *perc {
            for action in &trigs.actions[*handle as usize] {
                match action {
                    TriggerActions::Flask1 => {
                        TriggerActionFlask!(FlaskIndex::F1, flasks, ret, x, vd, current, "LIFE");
                    }
                    TriggerActions::Flask2 => {
                        TriggerActionFlask!(FlaskIndex::F2, flasks, ret, x, vd, current, "LIFE");
                    }
                    TriggerActions::Flask3 => {
                        TriggerActionFlask!(FlaskIndex::F3, flasks, ret, x, vd, current, "LIFE");
                    }
                    TriggerActions::Flask4 => {
                        TriggerActionFlask!(FlaskIndex::F4, flasks, ret, x, vd, current, "LIFE");
                    }
                    TriggerActions::Flask5 => {
                        TriggerActionFlask!(FlaskIndex::F5, flasks, ret, x, vd, current, "LIFE");
                    }
                    TriggerActions::Flask1SC => {
                        TriggerActionFlaskSC!(FlaskIndex::F1, flasks, ret, x, vd, current, "LIFE");
                    }
                    TriggerActions::Flask2SC => {
                        TriggerActionFlaskSC!(FlaskIndex::F2, flasks, ret, x, vd, current, "LIFE");
                    }
                    TriggerActions::Flask3SC => {
                        TriggerActionFlaskSC!(FlaskIndex::F3, flasks, ret, x, vd, current, "LIFE");
                    }
                    TriggerActions::Flask4SC => {
                        TriggerActionFlaskSC!(FlaskIndex::F4, flasks, ret, x, vd, current, "LIFE");
                    }
                    TriggerActions::Flask5SC => {
                        TriggerActionFlaskSC!(FlaskIndex::F5, flasks, ret, x, vd, current, "LIFE");
                    }
                    TriggerActions::Skill2 => {
                        TriggerActionSkill!(2 - 1, skills, ret, x, vd, current, "LIFE");
                    }
                    TriggerActions::Skill3 => {
                        TriggerActionSkill!(3 - 1, skills, ret, x, vd, current, "LIFE");
                    }
                }
            }
        }
    }

    Ok(ret)
}

fn handle_mana<D, HP, MANA>(
    trigs: &TriggersMana,
    flasks: &mut Flasks,
    skills: &Skills,
    poe: &PoE<D, HP, MANA>,
    x: &X,
    vd: &mut makro::input::VDevice,
) -> R<bool>
where
    D: Detect,
    HP: DetectHealth,
    MANA: DetectMana,
{
    let current = poe.mana.perc(x)?;
    let mut ret = false;

    for (perc, handle) in &trigs.percs {
        if current <= *perc {
            for action in &trigs.actions[*handle as usize] {
                match action {
                    TriggerActions::Flask1 => {
                        TriggerActionFlask!(FlaskIndex::F1, flasks, ret, x, vd, current, "MANA");
                    }
                    TriggerActions::Flask2 => {
                        TriggerActionFlask!(FlaskIndex::F2, flasks, ret, x, vd, current, "MANA");
                    }
                    TriggerActions::Flask3 => {
                        TriggerActionFlask!(FlaskIndex::F3, flasks, ret, x, vd, current, "MANA");
                    }
                    TriggerActions::Flask4 => {
                        TriggerActionFlask!(FlaskIndex::F4, flasks, ret, x, vd, current, "MANA");
                    }
                    TriggerActions::Flask5 => {
                        TriggerActionFlask!(FlaskIndex::F5, flasks, ret, x, vd, current, "MANA");
                    }
                    TriggerActions::Flask1SC => {
                        TriggerActionFlaskSC!(FlaskIndex::F1, flasks, ret, x, vd, current, "MANA");
                    }
                    TriggerActions::Flask2SC => {
                        TriggerActionFlaskSC!(FlaskIndex::F2, flasks, ret, x, vd, current, "MANA");
                    }
                    TriggerActions::Flask3SC => {
                        TriggerActionFlaskSC!(FlaskIndex::F3, flasks, ret, x, vd, current, "MANA");
                    }
                    TriggerActions::Flask4SC => {
                        TriggerActionFlaskSC!(FlaskIndex::F4, flasks, ret, x, vd, current, "MANA");
                    }
                    TriggerActions::Flask5SC => {
                        TriggerActionFlaskSC!(FlaskIndex::F5, flasks, ret, x, vd, current, "MANA");
                    }
                    TriggerActions::Skill2 => {
                        TriggerActionSkill!(2 - 1, skills, ret, x, vd, current, "MANA");
                    }
                    TriggerActions::Skill3 => {
                        TriggerActionSkill!(3 - 1, skills, ret, x, vd, current, "MANA");
                    }
                }
            }
        }
    }

    Ok(ret)
}

#[allow(clippy::too_many_arguments)]
/// Returns true if the event should be pass through
fn handle_keys<D: Detect, HP: DetectHealth, MANA: DetectMana>(
    key: makro::Key,
    config: &Config,
    keymaps: &mut BTreeMap<makro::Key, Action>,
    flasks: &mut Flasks,
    skills: &mut Skills,
    kb_state: KeyboardState,
    x: &X,
    vd: &mut makro::input::VDevice,
    c: &mut Clipboard,
    poe: &PoE<D, HP, MANA>,
    ui: &overlay::UserEventSender<Messages>,
    vec_of_clipboards: &[String],
) -> R<bool> {
    if let Some(action) = keymaps.get_mut(&key) {
        match action {
            Action::FlaskRotate { last_used, to_use } => {
                let last_used_value = *last_used;

                if flasks.is_on(last_used_value, x)? {
                    return Ok(true);
                }

                for f in to_use.iter() {
                    let f_value = *f;
                    if f_value != last_used_value && flasks.check_and_pop(f_value, x, vd)? {
                        *last_used = f_value;
                        return Ok(true);
                    }
                }
                flasks.check_and_pop(last_used_value, x, vd)?;
                Ok(true)
            }

            Action::FlaskPop { slot } => {
                flasks.check_and_pop(*slot, x, vd)?;
                Ok(true)
            }

            Action::KeyRight => {
                vd.tap(Key::KEY_RIGHT);
                Ok(false)
            }

            Action::KeyLeft => {
                vd.tap(Key::KEY_LEFT);
                Ok(false)
            }

            Action::Hideout => {
                fn_actions::hideout(kb_state, vd);
                Ok(false)
            }

            Action::Exit => {
                fn_actions::exit(kb_state, vd, x);
                Ok(false)
            }

            Action::InvSimpleStash => {
                if poe.detection.is_stash_window_open(x)? {
                    fn_actions::simple_stash(x, config, vd, c)?;
                } else {
                    ui.send_event(Messages::Msg("Stash window is not open")).unwrap();
                }
                Ok(false)
            }

            Action::InvSimpleSell => {
                if poe.detection.is_vendor_window_open(x)? {
                    fn_actions::simple_sell(x, config, vd, c, ui)?;
                } else {
                    ui.send_event(Messages::Msg("Vendor window is not open")).unwrap();
                }
                Ok(false)
            }

            Action::InvUnid => {
                // if poe.detection.is_stash_window_open(x)? {
                //     x.move_mouse(Pos::new(1265, 645))?;
                // } else {
                //     ui_sender.send_event(Messages::Msg("Stash window is not open")).unwrap();
                // }
                // Ok(false)

                if poe.detection.is_stash_window_open(x)?
                    || poe.detection.is_vendor_window_open(x)?
                {
                    // inventory::unid_items(
                    //     config
                    //         .pos_wisdom
                    //         .expect("pos_wisdom not found in config but expected to use"),
                    //     x,
                    //     vd,
                    //     c,
                    // );
                    if let Some(msg) = inventory::unid_items(
                        config
                            .pos_wisdom
                            .expect("pos_wisdom not found in config but expected to use"),
                        x,
                        vd,
                        c,
                    ) {
                        ui.send_event(Messages::Msg(msg)).unwrap();
                        return Ok(false);
                    }

                    x.move_mouse(Pos::new(1265, 645))?;
                } else {
                    ui.send_event(Messages::Msg("Vendor nor Stash window is not open")).unwrap();
                }
                Ok(false)
            }

            Action::InvSimpleStashOrSell => {
                if poe.detection.is_stash_window_open(x)? {
                    fn_actions::simple_stash(x, config, vd, c)?;
                } else if poe.detection.is_vendor_window_open(x)? {
                    if let Some(msg) = inventory::unid_items(
                        config
                            .pos_wisdom
                            .expect("pos_wisdom not found in config but expected to use"),
                        x,
                        vd,
                        c,
                    ) {
                        ui.send_event(Messages::Msg(msg)).unwrap();
                        return Ok(false);
                    }
                    fn_actions::simple_sell(x, config, vd, c, ui)?;
                } else {
                    ui.send_event(Messages::Msg("Vendor nor Stash window is not open")).unwrap();
                }
                Ok(false)
            }

            Action::SendClipboard { handle } => {
                kb_state.release_state(vd);
                let str = vec_of_clipboards[*handle as usize].as_str();
                let prev = c.get();
                c.set(str);
                sleep!();
                sleep!();
                vd.tap_with_lctrl(Key::KEY_F);
                sleep!();
                sleep!();
                vd.tap_with_lctrl(Key::KEY_V);
                sleep!();
                sleep!();

                kb_state.press_state(vd);
                c.set(&prev);

                Ok(false)
            }

            Action::UseSkillsAndPass { to_use } => {
                for handle in to_use {
                    skills[*handle as usize]
                        .as_ref()
                        .unwrap_or_else(|| {
                            panic!("No skill defined in slot {}, but expected to use", *handle)
                        })
                        .check_and_use(x, vd)?;
                }
                Ok(true)
            }
        }
    } else {
        Ok(true)
    }
}

mod fn_actions {
    use super::*;

    pub fn simple_stash(
        x: &X,
        config: &Config,
        vd: &mut makro::input::VDevice,
        c: &mut Clipboard,
    ) -> Result<(), makro::error::Error> {
        inventory::stash_simple(
            config
                .ignore_items
                .as_ref()
                .expect("ignore_items not found in config but expected to use"),
            *config
                .ignore_maps
                .as_ref()
                .expect("ignore_maps not found in config but expected to use"),
            x,
            vd,
            c,
        );
        x.move_mouse(Pos::new(960, 420))?;
        Ok(())
    }

    pub fn simple_sell(
        x: &X,
        config: &Config,
        vd: &mut makro::input::VDevice,
        c: &mut Clipboard,
        ui: &overlay::UserEventSender<Messages>,
    ) -> Result<(), makro::error::Error> {
        if let Some(msg) = inventory::unid_items(
            config.pos_wisdom.expect("pos_wisdom not found in config but expected to use"),
            x,
            vd,
            c,
        ) {
            ui.send_event(Messages::Msg(msg)).unwrap();
            return Ok(());
        }
        inventory::sell_simple(
            config
                .ignore_items
                .as_ref()
                .expect("ignore_items not found in config but expected to use"),
            x,
            vd,
            c,
        );
        x.move_mouse(Pos::new(310, 575))?;
        Ok(())
    }

    pub fn exit(kb_state: KeyboardState, vd: &mut makro::input::VDevice, x: &X) {
        kb_state.release_state(vd);
        vd.emit_events(&[
            InputEvent::new(EventType::KEY, Key::KEY_ENTER.code(), 1),
            InputEvent::new(EventType::KEY, Key::KEY_ENTER.code(), 0),
        ]);
        std::thread::sleep(Duration::from_millis(100));
        vd.emit_events(&[
            InputEvent::new(EventType::KEY, Key::KEY_SLASH.code(), 1),
            InputEvent::new(EventType::KEY, Key::KEY_SLASH.code(), 0),
            InputEvent::new(EventType::KEY, Key::KEY_E.code(), 1),
            InputEvent::new(EventType::KEY, Key::KEY_E.code(), 0),
            InputEvent::new(EventType::KEY, Key::KEY_X.code(), 1),
            InputEvent::new(EventType::KEY, Key::KEY_X.code(), 0),
            InputEvent::new(EventType::KEY, Key::KEY_I.code(), 1),
            InputEvent::new(EventType::KEY, Key::KEY_I.code(), 0),
            InputEvent::new(EventType::KEY, Key::KEY_T.code(), 1),
            InputEvent::new(EventType::KEY, Key::KEY_T.code(), 0),
            InputEvent::new(EventType::KEY, Key::KEY_ENTER.code(), 1),
            InputEvent::new(EventType::KEY, Key::KEY_ENTER.code(), 0),
        ]);
        for _ in 0..30 {
            if x.get_pixel(Pos::new(1255, 258))
                .unwrap()
                .cmp_by_dist(Rgb::new(57, 42, 36), 7.0)
                .is_lt()
            {
                vd.tap(Key::KEY_ENTER);
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        kb_state.press_state(vd);
    }

    pub fn hideout(kb_state: KeyboardState, vd: &mut makro::input::VDevice) {
        kb_state.release_state(vd);
        vd.emit_events(&[
            InputEvent::new(EventType::KEY, Key::KEY_ENTER.code(), 1),
            InputEvent::new(EventType::KEY, Key::KEY_ENTER.code(), 0),
        ]);
        std::thread::sleep(Duration::from_millis(100));
        vd.emit_events(&[
            InputEvent::new(EventType::KEY, Key::KEY_SLASH.code(), 1),
            InputEvent::new(EventType::KEY, Key::KEY_SLASH.code(), 0),
            InputEvent::new(EventType::KEY, Key::KEY_H.code(), 1),
            InputEvent::new(EventType::KEY, Key::KEY_H.code(), 0),
            InputEvent::new(EventType::KEY, Key::KEY_I.code(), 1),
            InputEvent::new(EventType::KEY, Key::KEY_I.code(), 0),
            InputEvent::new(EventType::KEY, Key::KEY_D.code(), 1),
            InputEvent::new(EventType::KEY, Key::KEY_D.code(), 0),
            InputEvent::new(EventType::KEY, Key::KEY_E.code(), 1),
            InputEvent::new(EventType::KEY, Key::KEY_E.code(), 0),
            InputEvent::new(EventType::KEY, Key::KEY_O.code(), 1),
            InputEvent::new(EventType::KEY, Key::KEY_O.code(), 0),
            InputEvent::new(EventType::KEY, Key::KEY_U.code(), 1),
            InputEvent::new(EventType::KEY, Key::KEY_U.code(), 0),
            InputEvent::new(EventType::KEY, Key::KEY_T.code(), 1),
            InputEvent::new(EventType::KEY, Key::KEY_T.code(), 0),
            InputEvent::new(EventType::KEY, Key::KEY_ENTER.code(), 1),
            InputEvent::new(EventType::KEY, Key::KEY_ENTER.code(), 0),
        ]);
        kb_state.press_state(vd);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t0() {
        let a = Action::Exit;
        let b = std::mem::size_of_val(&a);
        dbg!(b);

        assert!(false);
    }
}
