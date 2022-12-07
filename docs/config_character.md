Config for specific character

#### Full Config Example

[example](./config_examples/character.toml)

#### Fields

- [Flask1](#flask1)
- [Flask2](#flask2)
- [Flask3](#flask3)
- [Flask4](#flask4)
- [Flask5](#flask5)
- [Skill2](#skill2)
- [Skill3](#skill3)
- [Keymaps](#keymaps)
- [Triggers](#triggers)

---

#### Flask1

Settings for the flask in the first spot (leftmost).

##### Fields

###### kind

`"Duration"` or `"NonDuration"`

`"Duration"` for flasks that have a yellow bar under them that indicates the current effect of the flask (utility flasks). Unique flasks probably do not work.

`"NonDuration"` for flask that do not have a yellow bar that indicates the current effect of the flask (mana flasks, health flasks).

###### duration

Duration of the flasks's effect in miliseconds.

Required if `kind` of flask is `"Duration"`

##### Examples

```toml
[Flask1]
kind = "Duration"
duration = 2000
```

```toml
[Flask1]
kind = "NonDuration"
```

#### Flask2

Settings for the flask in the seconds spot.

See [Flask1](#flask1) for more details.

#### Flask3

Settings for the flask in the third spot.

See [Flask1](#flask1) for more details.

#### Flask4

Settings for the flask in the fourth spot.

See [Flask1](#flask1) for more details.

#### Flask5

Settings for the flask in the fifth (rightmost) spot.

See [Flask1](#flask1) for more details.

---

#### Skill2

Settings for the skill in the second slot (where first slot is the slot for left click).

##### Fields

###### active

Whetever the skill should be used or not.

#### Skill3

Settings for the skill in the third slot (where first slot is the slot for left click).

See [Skill2](#skill2) for more details.

---

#### Keymaps

Settings for keys bounded to actions.

Keycodes for keys can be found at https://github.com/emberian/evdev/blob/master/src/scancodes.rs or using `evtest`

Details about actions which can keys can be bound to: [actions](./actions.md)

##### Fields

###### normal

List of keymaps without any modifier.

###### withShift

List of keymaps active when left shift is pressed.

###### withCtrl

List of keymaps active when left ctrl is pressed.

###### withAlt

List of keymaps active when left alt is pressed.

##### Example
```toml
[Keymaps]
normal = [
  { key = "KEY_GRAVE", action = "Exit" },
  { key = "KEY_0", action = "InvSimpleStash" },
]
withShift = [ # optional if empty
  { key = "KEY_D", action = "KeyRight" },
  { key = "KEY_A", action = "KeyLeft" },
]
withAlt = [] # optional if empty
withCtrl = [] # optional if empty
```

---

#### Triggers

Settings for triggers that will be actived when certain percent of health or mana is reached.

Iterated from the lowest percent to the highest.

Details about triggers can be found at: [triggers](./triggers.md)

##### Fields

###### byHealth

List of percents and lists of triggers that are activeted when percent of health is reached.

###### byMana

List of percents and lists of triggers that are activeted when percent of mana is reached.

##### Example
```toml
[Triggers]
byHealth = [
  { perc = 70, actions = [
    "Skill2",
  ] },
  { perc = 50, actions = [
    "Flask1",
    "Flask2",
  ] },
]
byMana = [{ perc = 45, actions = ["Skill3"] }]
```
