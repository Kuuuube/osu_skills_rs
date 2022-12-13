# osu!Skills rs

osu!Skills calculator rewritten in rust.

## Usage

```
osu_skills_rs.exe {.osu file path} {calculation alg}
```

- **Calculation Alg:**

    ` ` or `0`: Modified alg with various fixes.

    `1`: Classic alg accurate to <0.01% of the original osu!Skills calculator.

The osu!Skills website cannot be matched to as the calculation code is not public.

Methods to process large amounts of maps will be added soon.

## Building

```
cargo build
```