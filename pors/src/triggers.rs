use arrayvec::ArrayVec;
use config::{CfgTrigger,
             TriggerActions};

type Perc = u8;
type Handle = u8;

#[derive(Default, Debug)]
pub struct TriggersHP {
    pub percs: Vec<(Perc, Handle)>,
    pub actions: Vec<ArrayVec<TriggerActions, 5>>,
}

impl TriggersHP {
    pub fn new(trigs: &[CfgTrigger]) -> Self {
        let mut ret = TriggersHP::default();

        for (handle, cfg) in trigs.iter().enumerate() {
            let actions: ArrayVec<TriggerActions, 5> = cfg.actions.iter().copied().collect();
            ret.actions.push(actions);
            ret.percs.push((cfg.perc, handle as u8));
        }

        ret.percs.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        ret
    }
}

#[derive(Default, Debug)]
pub struct TriggersMana {
    pub percs: Vec<(Perc, Handle)>,
    pub actions: Vec<ArrayVec<TriggerActions, 5>>,
}

impl TriggersMana {
    pub fn new(trigs: &[CfgTrigger]) -> TriggersMana {
        let mut ret = TriggersMana::default();
        for (handle, cfg) in trigs.iter().enumerate() {
            let actions: ArrayVec<TriggerActions, 5> = cfg.actions.iter().copied().collect();
            ret.actions.push(actions);
            ret.percs.push((cfg.perc, handle as u8));
        }

        ret
    }
}
