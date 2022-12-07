use std::collections::BTreeMap;

use arrayvec::ArrayVec;
use config::{CfgAction,
             Keymap};
use makro::Key;
use poe::flasks::types::FlaskIndex;
use poe::skills::Skill;
use tracing::warn;

pub type Skills = [Option<Skill>; 3];

pub type Handle = u8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    FlaskRotate {
        last_used: FlaskIndex,
        to_use: ArrayVec<FlaskIndex, 5>,
    },
    FlaskPop {
        slot: FlaskIndex,
    },

    // FlasksPop { to_use: [Option<()>; 5] },
    InvSimpleStash,
    InvSimpleSell,

    InvSimpleStashOrSell,

    InvUnid,

    KeyRight,
    KeyLeft,

    /// Goes to hideout via /hideout
    Hideout,

    /// Exit to character selection screen via /exit
    Exit,

    /// Clipboard
    SendClipboard {
        handle: Handle,
    },

    /// Clipboard
    UseSkillsAndPass {
        to_use: ArrayVec<Handle, 2>,
    },
}

impl Action {
    pub fn from_cfg_action(
        action: CfgAction,
        args: &Option<Vec<String>>,
        vec_of_clipboards: &mut Vec<String>,
    ) -> Self {
        match action {
            CfgAction::FlaskPop => {
                let args = args.as_ref().expect("FlaskPop has empty args");
                assert!(args.len() == 1, "FlaskPop has less or more than 1 argument");
                let slot: FlaskIndex = args[0].parse::<u8>().unwrap().into();
                Self::FlaskPop { slot }
            }
            CfgAction::FlaskRotate => {
                let args = args.as_ref().expect("FlaskArgs has empty args");
                let slots: ArrayVec<FlaskIndex, 5> =
                    args.iter().map(|it| it.parse::<u8>().unwrap().into()).collect();
                assert!(args.len() >= 2, "Supplied less than 2 arguments to FlaskRotate");
                Self::FlaskRotate { last_used: slots[0], to_use: slots }
            }
            CfgAction::KeyRight => Self::KeyRight,
            CfgAction::KeyLeft => Self::KeyLeft,
            CfgAction::Exit => Self::Exit,
            CfgAction::Hideout => Self::Hideout,
            CfgAction::InvSimpleStash => Self::InvSimpleStash,
            CfgAction::InvSimpleSell => Self::InvSimpleSell,
            CfgAction::InvSimpleStashOrSell => Self::InvSimpleStashOrSell,
            CfgAction::InvUnid => Self::InvUnid,
            CfgAction::SendClipboard => {
                let args = args.as_ref().expect("SendClipboard has empty args");
                assert!(
                    args.len() == 1,
                    "SendClipboard has more than 1 argument, args: {:#?}",
                    args
                );
                let handle = vec_of_clipboards.len();
                vec_of_clipboards.push(args[0].clone());
                Self::SendClipboard { handle: handle.try_into().unwrap() }
            }
            CfgAction::UseSkillsAndPass => {
                let args = args.as_ref().expect("UseSkillsAndPass has empty args");
                let to_use: ArrayVec<u8, 2> =
                    args.iter().map(|it| it.parse::<u8>().unwrap() - 1).collect();
                Self::UseSkillsAndPass { to_use }
            }
        }
    }
}

pub fn make_keymaps(
    global: Option<&Vec<Keymap>>,
    char: Option<&Vec<Keymap>>,
    vec_of_clipboards: &mut Vec<String>,
) -> BTreeMap<Key, Action> {
    let mut ret: BTreeMap<Key, Action> = BTreeMap::new();

    if let Some(keymaps) = global.as_ref() {
        keymaps.iter().for_each(|k| {
            ret.insert(k.key, Action::from_cfg_action(k.action, &k.args, vec_of_clipboards));
        })
    }

    if let Some(keymaps) = char.as_ref() {
        keymaps.iter().for_each(|k| {
            if ret.contains_key(&k.key) {
                warn!("CharacterConfig overwrote keymap for {:#?} from GlobalConfig", k.key)
            }
            ret.insert(k.key, Action::from_cfg_action(k.action, &k.args, vec_of_clipboards));
        })
    }

    ret
}
