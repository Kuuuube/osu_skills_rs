# osu!Skills rs

osu!Skills calculator rewritten in rust.

## Usage

```
osu_skills_rs.exe {.osu file path}
```

Methods to process large amounts of maps will be added once adding accurate calculation for all skills is finished.

## Progress

- **Stamina:** Broken

- **Tenacity:** Broken

- **Agility:** Accurate within <1% in most cases. Up to 5% inaccuracy in some cases.

- **Accuracy:** Broken

- **Precision:** Accurate within <1% in most cases. Up to 5% inaccuracy in some cases. (Directly tied to Agility)

- **Reaction:** Broken

- **Memory:** Broken

## Building

```
cargo build
```