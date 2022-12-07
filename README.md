<h1 align='center'>pors</h1>

Path of Exile helper tool for Linux.

Made using X11 API (grabbing pixels from the screen) and evdev (listening to keyboard events, sending keyboard events).

#### Features

- Triggering Flasks based on HP or Mana percent
- Triggering Skills based on HP or Mana Percent
- Simple inventory managment (uniding, selling, stashing)
- PoE-specific keymaps

#### Requirements

- X11
- Game has to be in fullscreen windowed.
- 1920x1080 resolution (hardcoded values)
- User that runs the program needs to have permission to access `input` and `uinput` subsystems. More [here](https://github.com/kmonad/kmonad/blob/master/doc/faq.md#q-how-do-i-get-uinput-permissions). Can also use `sudo` to run it (not recommended and not tested)

#### Installation

```
git clone https://github.com/rareitems/pors
cd pors
cargo install --path pors
```

#### Usage

```
pors <path_to_global_config> <path_to_character_config>
```

General Usage [usage](./docs/usage.md)
Details and examples about configs can be found in [global_config](./docs/config_global.md) and [character_config](./docs/config_character.md)
