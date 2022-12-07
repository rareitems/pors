#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rarity {
    Currency,
    DivinationCard,
    Gem,
    Normal,
    Magic,
    Rare,
    Unique,
}

impl std::str::FromStr for Rarity {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Currency" => Ok(Rarity::Currency),
            "Divination Card" => Ok(Rarity::DivinationCard),
            "Gem" => Ok(Rarity::Gem),
            "Normal" => Ok(Rarity::Normal),
            "Magic" => Ok(Rarity::Magic),
            "Rare" => Ok(Rarity::Rare),
            "Unique" => Ok(Rarity::Unique),
            _ => panic!("Unmatched Rarity: {s}"),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Class {
    // Sanctum
    Relic,
    SanctifiedRelic,

    AbyssJewel,
    ActiveSkillGems,
    Amulets,
    AtlasRegionUpgradeItem,
    Belts,
    Blueprint,
    BodyArmours,
    Boots,
    Bows,
    Claws,
    Contract,
    CriticalUtilityFlasks,
    Currency,
    Daggers,
    DelveSocketableCurrency,
    DelveStackableSocketableCurrency,
    DivinationCard,
    ExpeditionLogbook,
    FishingRods,
    Gloves,
    HarvestSeed,
    HeistBrooch,
    HeistCloak,
    HeistGear,
    HeistTarget,
    HeistTool,
    Helmets,
    HideoutDoodads,
    HybridFlasks,
    Incubator,
    IncursionItem,
    Jewel,
    LabyrinthItem,
    LabyrinthMapItem,
    LabyrinthTrinket,
    Leaguestones,
    LifeFlasks,
    ManaFlasks,
    MapFragments,
    Maps,
    MetamorphSample,
    Microtransactions,
    MiscMapItems,
    OneHandAxes,
    OneHandMaces,
    OneHandSwords,
    PantheonSoul,
    Piece,
    QuestItems,
    Quivers,
    Rings,
    RuneDaggers,
    Sceptres,
    SeedEnhancer,
    Shard,
    ShardHeart,
    Shields,
    StackableCurrency,
    Staves,
    SupportSkillGems,
    ThrustingOneHandSwords,
    Trinkets,
    TwoHandAxes,
    TwoHandMaces,
    TwoHandSwords,
    Unarmed,
    UtilityFlasks,
    Wands,
    Warstaves,
}

impl Class {
    pub fn is_equipment(self) -> bool {
        matches!(
            self,
            Class::Amulets
                | Class::Belts
                | Class::Jewel
                | Class::AbyssJewel
                | Class::Rings
                | Class::BodyArmours
                | Class::Boots
                | Class::Bows
                | Class::Claws
                | Class::Daggers
                | Class::Gloves
                | Class::Helmets
                | Class::OneHandAxes
                | Class::OneHandMaces
                | Class::OneHandSwords
                | Class::Quivers
                | Class::RuneDaggers
                | Class::Sceptres
                | Class::Shields
                | Class::Staves
                | Class::ThrustingOneHandSwords
                | Class::TwoHandAxes
                | Class::TwoHandMaces
                | Class::TwoHandSwords
                | Class::Unarmed
                | Class::Wands
                | Class::Warstaves
        )
    }

    pub fn is_map(self) -> bool {
        matches!(self, Class::Maps)
    }

    /// Returns true if item with that 'Class' counts as some affintiy
    pub fn is_affinity(self) -> bool {
        match self {
            // Sanctum Chest
            Class::Relic => true,
            Class::SanctifiedRelic => true,

            Class::Amulets => false,
            Class::Belts => false,
            Class::Jewel => false,
            Class::AbyssJewel => false,
            Class::Rings => false,

            Class::BodyArmours => false,
            Class::Boots => false,
            Class::Bows => false,
            Class::Claws => false,
            Class::Daggers => false,
            Class::Gloves => false,
            Class::Helmets => false,
            Class::OneHandAxes => false,
            Class::OneHandMaces => false,
            Class::OneHandSwords => false,
            Class::Quivers => false,
            Class::RuneDaggers => false,
            Class::Sceptres => false,
            Class::Shields => false,
            Class::Staves => false,
            Class::ThrustingOneHandSwords => false,
            Class::TwoHandAxes => false,
            Class::TwoHandMaces => false,
            Class::TwoHandSwords => false,
            Class::Unarmed => false,
            Class::Wands => false,
            Class::Warstaves => false,

            Class::DivinationCard => true,
            Class::StackableCurrency => true,
            Class::Currency => true,

            Class::DelveSocketableCurrency => true,
            Class::DelveStackableSocketableCurrency => true,

            Class::ExpeditionLogbook => true,

            Class::Blueprint => true,
            Class::Contract => true,
            Class::HeistBrooch => false,
            Class::HeistCloak => false,
            Class::HeistGear => false,
            Class::HeistTarget => false,
            Class::HeistTool => false,

            Class::HideoutDoodads => true,

            Class::MapFragments => true,
            Class::Maps => true,
            Class::MetamorphSample => true,
            Class::Microtransactions => true,

            // Skill Gems
            Class::ActiveSkillGems => true,
            Class::SupportSkillGems => true,

            // Random
            Class::FishingRods => false,

            // Weird
            Class::AtlasRegionUpgradeItem => false,
            Class::QuestItems => false,

            Class::MiscMapItems => false,

            // Not sure
            Class::Incubator => todo!("{:?}", self),
            Class::LabyrinthItem => todo!("{:?}", self),
            Class::LabyrinthMapItem => todo!("{:?}", self),
            Class::LabyrinthTrinket => todo!("{:?}", self),
            Class::PantheonSoul => todo!("{:?}", self),
            Class::Piece => todo!("{:?}", self),

            // Flasks
            Class::LifeFlasks => false,
            Class::ManaFlasks => false,
            Class::UtilityFlasks => false,
            Class::CriticalUtilityFlasks => false,
            Class::HybridFlasks => false,

            // wtf?
            Class::HarvestSeed => todo!("{:?}", self),
            Class::Leaguestones => todo!("{:?}", self),
            Class::IncursionItem => todo!("{:?}", self),
            Class::SeedEnhancer => todo!("{:?}", self),
            Class::Shard => todo!("{:?}", self),
            Class::ShardHeart => todo!("{:?}", self),
            Class::Trinkets => todo!("{:?}", self),
        }
    }
}

impl std::str::FromStr for Class {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Abyss Jewels" => Ok(Class::AbyssJewel),
            "Active Skill Gems" => Ok(Class::ActiveSkillGems),
            "Amulets" => Ok(Class::Amulets),
            "Atlas Region Upgrade Item" => Ok(Class::AtlasRegionUpgradeItem),
            "Belts" => Ok(Class::Belts),
            "Blueprint" => Ok(Class::Blueprint),
            "Blueprints" => Ok(Class::Blueprint),
            "Body Armours" => Ok(Class::BodyArmours),
            "Boots" => Ok(Class::Boots),
            "Bows" => Ok(Class::Bows),
            "Claws" => Ok(Class::Claws),
            "Contracts" => Ok(Class::Contract),
            "Critical Utility Flasks" => Ok(Class::CriticalUtilityFlasks),
            "Currency" => Ok(Class::Currency),
            "Daggers" => Ok(Class::Daggers),
            "Delve Socketable Currency" => Ok(Class::DelveSocketableCurrency),
            "Delve Stackable Socketable Currency" => Ok(Class::DelveStackableSocketableCurrency),
            "Divination Cards" => Ok(Class::DivinationCard),
            "Expedition Logbooks" => Ok(Class::ExpeditionLogbook),
            "Fishing Rods" => Ok(Class::FishingRods),
            "Gloves" => Ok(Class::Gloves),
            "Harvest Seed" => Ok(Class::HarvestSeed),
            "Heist Brooch" => Ok(Class::HeistBrooch),
            "Heist Cloak" => Ok(Class::HeistCloak),
            "Heist Gear" => Ok(Class::HeistGear),
            "Heist Target" => Ok(Class::HeistTarget),
            "Heist Tool" => Ok(Class::HeistTool),
            "Helmets" => Ok(Class::Helmets),
            "Hideout Doodads" => Ok(Class::HideoutDoodads),
            "Hybrid Flasks" => Ok(Class::HybridFlasks),
            "Incubator" => Ok(Class::Incubator),
            "Incubators" => Ok(Class::Incubator),
            "Incursion Item" => Ok(Class::IncursionItem),
            "Jewels" => Ok(Class::Jewel),
            "Labyrinth Item" => Ok(Class::LabyrinthItem),
            "Labyrinth Map Item" => Ok(Class::LabyrinthMapItem),
            "Labyrinth Trinket" => Ok(Class::LabyrinthTrinket),
            "Leaguestones" => Ok(Class::Leaguestones),
            "Life Flasks" => Ok(Class::LifeFlasks),
            "Mana Flasks" => Ok(Class::ManaFlasks),
            "Map Fragments" => Ok(Class::MapFragments),
            "Maps" => Ok(Class::Maps),
            "Metamorph Samples" => Ok(Class::MetamorphSample),
            "Microtransactions" => Ok(Class::Microtransactions),
            "Misc Map Items" => Ok(Class::MiscMapItems),
            "One Hand Axes" => Ok(Class::OneHandAxes),
            "One Hand Maces" => Ok(Class::OneHandMaces),
            "One Hand Swords" => Ok(Class::OneHandSwords),
            "Pantheon Soul" => Ok(Class::PantheonSoul),
            "Piece" => Ok(Class::Piece),
            "Quest Items" => Ok(Class::QuestItems),
            "Quivers" => Ok(Class::Quivers),
            "Rings" => Ok(Class::Rings),
            "Rune Daggers" => Ok(Class::RuneDaggers),
            "Sceptres" => Ok(Class::Sceptres),
            "Seed Enhancer" => Ok(Class::SeedEnhancer),
            "Shard Heart" => Ok(Class::ShardHeart),
            "Shard" => Ok(Class::Shard),
            "Shields" => Ok(Class::Shields),
            "Stackable Currency" => Ok(Class::StackableCurrency),
            "Staves" => Ok(Class::Staves),
            "Support Skill Gems" => Ok(Class::SupportSkillGems),
            "Thrusting One Hand Swords" => Ok(Class::ThrustingOneHandSwords),
            "Trinkets" => Ok(Class::Trinkets),
            "Two Hand Axes" => Ok(Class::TwoHandAxes),
            "Two Hand Maces" => Ok(Class::TwoHandMaces),
            "Two Hand Swords" => Ok(Class::TwoHandSwords),
            "Unarmed" => Ok(Class::Unarmed),
            "Utility Flasks" => Ok(Class::UtilityFlasks),
            "Wands" => Ok(Class::Wands),
            "Warstaves" => Ok(Class::Warstaves),
            "Relic" => Ok(Class::Relic),
            "Sanctified Relic" => Ok(Class::SanctifiedRelic),
            _ => panic!("Unmatched Class: {s}"),
        }
    }
}
