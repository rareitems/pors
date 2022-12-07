use serde::{Deserialize,
            Serialize};

use self::cfg_flasks::FlaskConfig;
use self::cfg_keymaps::Keymaps;
use self::cfg_skills::SkillConfig;
use self::triggers::CfgTriggers;

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CfgCharacter {
    #[serde(default)]
    pub Flask1: Option<FlaskConfig>,
    #[serde(default)]
    pub Flask2: Option<FlaskConfig>,
    #[serde(default)]
    pub Flask3: Option<FlaskConfig>,
    #[serde(default)]
    pub Flask4: Option<FlaskConfig>,
    #[serde(default)]
    pub Flask5: Option<FlaskConfig>,

    #[serde(default)]
    pub Skill2: Option<SkillConfig>,
    #[serde(default)]
    pub Skill3: Option<SkillConfig>,

    #[serde(default)]
    pub Keymaps: Option<Keymaps>,

    #[serde(default)]
    pub Triggers: Option<CfgTriggers>,
}

impl CfgCharacter {
    pub fn parse(path: &std::path::Path) -> Self {
        toml::from_str::<CfgCharacter>(&std::fs::read_to_string(path).unwrap()).unwrap()
    }
}

pub mod cfg_skills {
    use serde::{Deserialize,
                Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct SkillConfig {
        pub active: bool,
    }
}

pub mod cfg_flasks {
    use serde::{Deserialize,
                Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct FlaskConfig {
        pub kind: FlaskKind,
        #[serde(default)]
        pub duration: Option<u64>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub enum FlaskKind {
        Duration,
        NonDuration,
    }
}

pub mod cfg_keymaps {
    use serde::{Deserialize,
                Serialize};

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Keymaps {
        pub normal: Option<Vec<Keymap>>,
        pub withShift: Option<Vec<Keymap>>,
        pub withCtrl: Option<Vec<Keymap>>,
        pub withAlt: Option<Vec<Keymap>>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone, Copy)]
    pub enum CfgAction {
        FlaskPop,
        FlaskRotate,
        KeyRight,
        KeyLeft,
        Exit,
        Hideout,
        InvSimpleStash,
        InvSimpleSell,
        InvUnid,
        InvSimpleStashOrSell,
        UseSkillsAndPass,
        SendClipboard,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Keymap {
        pub key: makro::Key,
        pub action: CfgAction,
        pub args: Option<Vec<String>>,
    }
}

pub mod triggers {
    use serde::{Deserialize,
                Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone, Copy)]
    pub enum TriggerActions {
        Flask1,
        Flask2,
        Flask3,
        Flask4,
        Flask5,
        Flask1SC,
        Flask2SC,
        Flask3SC,
        Flask4SC,
        Flask5SC,
        Skill2,
        Skill3,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize, Serialize)]
    pub struct CfgTriggers {
        #[serde(default)]
        pub byHealth: Option<Vec<CfgTrigger>>,
        #[serde(default)]
        pub byMana: Option<Vec<CfgTrigger>>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct CfgTrigger {
        pub perc: u8,
        pub actions: Vec<TriggerActions>,
    }
}
