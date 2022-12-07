#### General Form

```toml
{
    key = "<KEY>", # evtest or https://github.com/emberian/evdev/blob/master/src/scancodes.rs
    action = "<ACTION>",
    args = ["<ARGUMENT0>", "<ARGUMENT1>", ...],
}
```

##### Example

```toml
[Keymaps]
normal = [ # without any modifiers
  { key = "KEY_Y", action = "FlaskRotate", args = ["4", "5"] },
  { key = "KEY_GRAVE", action = "Exit" },
]
withShift = [ # with left shift modifier
  { key = "KEY_D", action = "KeyRight" },
  { key = "KEY_A", action = "KeyLeft" },
]
```

---

#### Available Actions

- [FlaskPop](#flaskpop)
- [FlaskRotate](#flaskrotate)
- [KeyRight](#keyright)
- [KeyLeft](#keyleft)
- [Exit](#exit)
- [Hideout](#hideout)
- [InvSimpleStash](#invsimplestash)
- [InvSimpleSell](#invsimplesell)
- [InvUnid](#invunid)
- [InvSimpleStashOrSell](#invsimplestashorsell)

---

##### FlaskPop

Tries to use the flask in specified spot.

It will passthrough (also click) the button it is keymapped to.

###### args

One of `1`, `2`, `3`, `4`, `5`. Where `1` specifies the leftmost flask and `5` specifies rightmost flask.

###### Example:

```toml
{
    key = "KEY_A",
    action = "FlaskPop",
    args = ["1"],
}
```

When `a` is pressed, it will try to use the flask in the first slot.

##### FlaskRotate

Tries to use one of the flasks in a list.
But it not use the flask previously used unless all the other specified flasks cannot be used.

It will passthrough (also click) the button it is keymapped to.

###### args

List of `1`, `2`, `3`, `4`, `5`. Where `1` specifies the leftmost flask and `5` specifies rightmost flask.

###### Example:

```toml
{
    key = "KEY_A",
    action = "FlaskRotate",
    args = ["1", "2"],
}
```

When `a` is pressed, it will try to use the flask `1`.

If it has been used previously it will try to use flask `2`, if that cannot be used, it will try to use flask `1`.

---

##### KeyRight

Taps `KEY_RIGHT` button. Which is right arrow key.

###### args

None

##### KeyLeft

Taps `KEY_LEFT` button. Which is left arrow key.

###### args

None

##### Exit

Opens up the chat and enters `/exit`. Which exits the game to the character login screen.

Then waits until character login screen and presses `Enter`.

###### args

None

##### Hideout

Opens up the chat and enters `/hideout`. Which teleports the character to hideout.

###### args

None

---

##### InvSimpleStash

If stash window is open it will drop (ctrl left click) items.

The algorithm to pick items is ignore positions specified in `ignore_items`, ignore maps according to `ignore_maps` and then drop all items that have a related affinity (including Heist and Expedition stuff).

Items to ignore are specified in [global](./config_global.md) config in `ignore_items`.

Maps to ignore are specified in [global](./config_global.md) config in `ignore_maps`.

<!-- TODO: add link -->

###### args

None

##### InvSimpleSell

If vendor window is open it, it will identify items with red background (position of the Wisdom Scroll is specified in `wisdom_scroll` global config) and then ctrl left click items.

The algorithm is to ignore positions specified in `ignore_items`, and then ctrl left click all equipment items (weapons, jewelry, body armors, shield etc.)

Scroll of wisdom position is specified in [global](./config_global.md) config in `wisdom_scroll`.

Items to ignore are specified in [global](./config_global.md) config in `ignore_items`.

<!-- TODO: add link -->

###### args

None

##### InvUnid

If vendor window is open it, it will identify items with red background (position of the Wisdom Scroll is specified in `wisdom_scroll` global config).

Scroll of wisdom position is specified in [global](./config_global.md) config in `wisdom_scroll`.

<!-- TODO: add link -->

###### args

None

##### InvSimpleStashOrSell

If vendor window is open, it does the same thing as [InvSimpleSell](#invsimplesell).

If stash window is open, it [InvSimpleStash](#invsimplestash).

###### args

None

---
