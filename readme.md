# osu!Skills rs

osu!Skills calculator rewritten in rust.

## Usage

```
osu_skills_rs [OPTION]...
```

### Skill Calculator

Mandatory:

```
--in=FILE                 path to .osu file to parse
```

Optional:

```
--alg=ALG                   calculation alg to use
                            (classic|default)

--mods=MODS                 integer sum of all mod values to apply
                            (`2`: EZ, `8`: HD, `16`: HR, `64`: DT, `256`: HT)

--is-dir=TYPE               set FILE to DIR or SUBDIR (recursive) and parse all .osu files in 
                            (DIR|SUBDIR)

--output-type=TYPE          output stream and type
                            (stdout|file-txt|file-csv)

--out=FILE                  output to FILE
                            (output-type must be file-txt or file-csv)

--no-ext                    removes file extension check for .osu files
```

### Skill File Parser

Mandatory:

```
--parser=ARGS               args for the parser in the following format:
                            collections separated by `;`, filters separated by `,`
                            fiters separated from values by `:`, min and max values separated by `-`
                            example: "stamina:1-100,tenacity:100-200;precision:900-1000"

--in=FILE                   path to input file

--out=FILE                  path to output file
```

## Building

```
cargo build
```