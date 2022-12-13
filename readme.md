# osu!Skills rs

osu!Skills calculator rewritten in rust.

## Usage

```
osu_skills_rs [OPTION]...
```

Mandatory:

```
--file=FILE                 path to .osu file to parse
```

Optional:

```
--alg=ALG                   calculation alg to use 
                            (`classic` or `default`)

--mods=MODS                 integer sum of all mod values to apply
                            (`2`: EZ, `8`: HD, `16`: HR, `64`: DT, `256`: HT)

--is-dir                    set FILE to DIR and parse all .osu files in DIR

--output-type=TYPE          output stream and type
                            (stdout|file-txt|file-csv)

--out=FILE                  output to FILE
                            (output-type must be file-txt or file-csv)
```

## Building

```
cargo build
```