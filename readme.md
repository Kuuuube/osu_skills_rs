# osu!Skills rs

osu!Skills calculator rewritten in rust.

## Usage

```
osu_skills_rs.exe {.osu file path}
```

Methods to process large amounts of maps will be added once adding accurate calculation for all skills is finished.

## Progress

- **Stamina:** Accurate within <0.01% in all tested cases.

- **Tenacity:** Broken

- **Agility:** Accurate within <1% in most cases. Up to 5% inaccuracy in some cases.

- **Accuracy:** Accurate within <0.01% in all tested cases.

- **Precision:** Accurate within <1% in most cases. Up to 5% inaccuracy in some cases. (Directly tied to Agility)

- **Reaction:** Broken

- **Memory:** Accurate within <1% in most cases. Up to 200% inaccuracy in some cases.

## Building

```
cargo build
```