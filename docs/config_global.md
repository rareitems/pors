Global config that in theory should be the same on all characters.

#### Full Config Example

[example](./config_examples/global.toml)

---

#### Fields

- [device_path](#device_path)
- [keys_flasks](#keys_flasks)
- [keys_skills](#keys_skills)
- [ignore_items](#ignore_items)
- [ignore_maps](#ignore_maps)
- [wisdom_scroll](#wisdom_scroll)

#### device_path

Path to the keyboard device.

Can be found by using `evtest`

If keyboard is disconnected the path will probably change.

#### keys_flasks

In-game keys for flasks.

Keycodes can be found at [keycodes](https://github.com/emberian/evdev/blob/master/src/scancodes.rs) or using `evtest`

##### Example

```toml
[keys_flasks]
slot1 = "KEY_1"
slot2 = "KEY_2"
slot3 = "KEY_3"
slot4 = "KEY_4"
slot5 = "KEY_5"
```

#### keys_skills

In-game keys for skills.

Keycodes can be found at [keycodes](https://github.com/emberian/evdev/blob/master/src/scancodes.rs) or using `evtest`

##### Example

```toml
[keys_flasks]
skill2 = "KEY_1"
skill3 = "KEY_2"
```

#### ignore_maps

Settings for ignoring maps for `InvSimpleStash` action keymap.

##### Fields

###### max_column

1-indexed column number. All maps after this column will be stashed.

###### min_tier

Number that specifies minimum tier of maps. Maps with tier lower than this will be stashed.

###### min_rarity

Can be `"Normal"`, `"Magic"`, `"Rare"` or `"Unique"`. All maps below this rarity will be dropped.

##### Example

```toml
ignore_maps = { min_tier = 14, min_rarity = "Rare", max_column = 2 }
```

All maps beyond the second column will be dropped. Maps in first and second column will be kept only if they are at least tier 14 and rare or unique.

#### ignore_items

1-indexed, row columns positions of items to always ignore during `InvSimpleStash` and `InvSimpleSell`.

##### Example

```toml
ignore_items = [
  [1, 1],
  [1, 2],
]
```

Item in first row, first column and first row, second column will be always kept in inventory.

#### wisdom_scroll

1-indexed, row columns positions of a wisdom scroll inside the inventory.
```toml
wisdom_scroll = [1, 1]
```
