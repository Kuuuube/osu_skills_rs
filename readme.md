# osu!Skills rs

osu!Skills calculator rewritten in rust.

## Usage

```
osu_skills_rs.exe {.osu file path} {calculation alg} {mod int}
```

- **Calculation Alg:**

    ` ` or `0`: Modified alg with various fixes.

    `1`: Classic alg accurate to <0.01% of the original osu!Skills calculator.

    The osu!Skills website cannot be matched to as the calculation code is not public.

- **Mod Int:**

    To get the desired mod int, add up the values for the mods you want to use. The following mods are supported:

    `2`: EZ,

    `8`: HD,

    `16`: HR,

    `64`: DT,

    `256`: HT

Methods to process large amounts of maps will be added soon.

## Building

```
cargo build
```