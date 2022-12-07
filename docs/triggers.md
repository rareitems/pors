#### General Form

```toml
[Triggers]
byHealth = [
  { perc = 90, actions = [ # trigger actions when HP is below 90%
    "Skill2",
  ] },
  { perc = 50, actions = [ # trigger actions when HP is below 50%
    "Flask1SC",
    "Flask2SC",
  ] },
]
byMana = [ # Requires
           # and it only checks one spot at around ~40% of mana
  { perc = 35, actions = [ # trigger actions when HP is below 35%
    "Flask3",
  ] },
]
```

---

#### Available Trigger Actions

- [Flask1](#flask1)
- [Flask2](#flask2)
- [Flask3](#flask3)
- [Flask4](#flask4)
- [Flask5](#flask5)
- [Flask1SC](#flask1sc)
- [Flask2SC](#flask2sc)
- [Flask3SC](#flask3sc)
- [Flask4SC](#flask4sc)
- [Flask5SC](#flask5sc)
- [Skill2](#skill2)
- [Skill3](#skill3)

---

##### Flask1

Tries to use flask in first (leftmost) spot, according to the `[Flask1]` in character config.

In-game key for it is specified in global config.

##### Flask2

Tries to use flask in second spot, according to the `[Flask2]` in character config.

In-game key for it is specified in global config.

##### Flask3

Tries to use flask in third spot, according to the `[Flask3]` in character config.

In-game key for it is specified in global config.

##### Flask4

Tries to use flask in fourth spot, according to the `[Flask4]` in character config.

In-game key for it is specified in global config.

##### Flask5

Tries to use flask in fifth (rightmost) spot, according to the `[Flask5]` in character config.

In-game key for it is specified in global config.

##### Flask1SC

Tries to use flask in first (leftmost) spot, according to the `[Flask1]` in character config. If it can be used it will short-circuit whole chain (it wont trigger anything else, be it triggers that are supposed to be triggered on higher % or on the same %).

In-game key for it is specified in global config.

##### Flask2SC

Tries to use flask in second spot, according to the `[Flask2]` in character config. If it can be used it will short-circuit whole chain (it wont trigger anything else, be it triggers that are supposed to be triggered on higher % or on the same %).

In-game key for it is specified in global config.

##### Flask3SC

Tries to use flask in third spot, according to the `[Flask3]` in character config. If it can be used it will short-circuit whole chain (it wont trigger anything else, be it triggers that are supposed to be triggered on higher % or on the same %).

In-game key for it is specified in global config.

##### Flask4SC

Tries to use flask in fourth spot, according to the `[Flask4]` in character config. If it can be used it will short-circuit whole chain (it wont trigger anything else, be it triggers that are supposed to be triggered on higher % or on the same %).

In-game key for it is specified in global config.

##### Flask5SC

Tries to use flask in fifth (rightmost) spot, according to the `[Flask5]` in character config. If it can be used it will short-circuit whole chain (it wont trigger anything else, be it triggers that are supposed to be triggered on higher % or on the same %).

In-game key for is specified in global config.

##### Skill2

Tries to use the skill in the second skill slot (where first spot is left-click slot).

In-game key for is specified in global config. If the program was started when character was in town, this will not work.

##### Skill3

Tries to use the skill in the third skill slot (where first spot is left-click slot).

In-game key for is specified in global config. If the program was started when character was in town, this will not work.
