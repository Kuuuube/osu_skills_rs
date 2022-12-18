# osu!Skills rs

osu!Skills calculator rewritten in rust.

## Usage

```
osu_skills_rs [OPTION]...
```

### Skill Calculator

Mandatory:

- `--in=FILE`: Path to .osu file to parse.

Optional:

- `--is-dir=TYPE`: (DIR|SUBDIR) Sets FILE to either `DIR` or `SUBDIR` (recursive) and parses all .osu files in `DIR` or `SUBDIR`.

- `--out=FILE`: Path to output file (`--output-type` must be file-txt or file-csv).

- `--output-type=TYPE`: (stdout|file-txt|file-csv) Output stream and type. Using `file-csv` is highly recommended.

- `--mods=MODS`: (`2`: EZ, `8`: HD, `16`: HR, `64`: DT, `256`: HT) Integer sum of all mod values to apply.

- `--no-ext`: Removes file extension check for .osu files. This can be used to calculate lazer songs folders.

- `--alg=ALG`: (classic|default) Calculation alg to use: `classic` or `default`. 

    Classic matches the [osuSkills Calculator](https://github.com/Kert/osuSkills). 

    Default contains various fixes for weird decisions made in the classic alg.

### Skill File Parser

Mandatory:

- `--parser=ARGS`: Args for the parser in the following format: Collections separated by `;`, filters separated by `,`, fiters separated from values by `:`, min and max values separated by `-`. 

    The following filters are supported:

    `name`: A custom name for the collection to be filtered. 

    If unset, a default name will be generated using the other filters. Using the same name for two or more collections can also be used to combine multiple sets of filters into one collection.

    `stamina`: The stamina value of a map.

    `tenacity` or `streams`: The tenacity or streams value of a map. These filters are interchangeable.

    `agility` or `aim`: The agility or aim value of a map. These filters are interchangeable.

    `accuracy`: The accuracy value of a map.

    `precision`: The precision value of a map.

    `reaction`: The reaction value of a map.

    `memory` or `flashlight`: The memory or flashlight value of a map. These filters are interchangeable.

    Examples: 

    Creating two collections, one containing all maps with stamina 1-100 and tenacity 100-200. The other containing all maps with precision 72.7-1000. `stamina:1-100,tenacity:100-200;precision:72.7-1000`

    Creating one collection with two different filter sets by using the same name for both collections. `name:collection1,stamina:1-100,tenacity:100-200;name:collection1,precision:72.7-1000`

- `--in=FILE`: Path to input file.

- `--out=FILE`: Path to output file.

Currently parsed files can only be converted into collections using [osu! Collection Converter](https://github.com/Kuuuube/osu_CollectionConverter). Support for direct conversion to db or osdb may be added in the future.

## Building

```
cargo build --release
```