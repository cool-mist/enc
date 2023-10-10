# enc-check #

## Installation ##

`cargo install enc-check`

## Usage ##

Inspect character encodings.

```
enc-check --help

Usage: enc-check [OPTIONS] <-8|-6> <NAME>

Arguments:
  <NAME>
          The string to inspect

Options:
  -8
          Inspect utf-8
  -6
          Inspect utf-16
  -j, --json
          Output as json. Useful as a command line tool
  -h, --help
          Print help
```


```
enc-check -8 asdᚢ𐌰

┌───────┬───────┬───────────┬──────┬─────┬─────┬──────────┐
│ U+dec │ U+hex │ character │ byte │ hex │ dec │ bin      │
├───────┼───────┼───────────┼──────┼─────┼─────┼──────────┤
│ 97    │ 61    │ a         │ 0    │ 61  │ 97  │ 01100001 │
│ 115   │ 73    │ s         │ 1    │ 73  │ 115 │ 01110011 │
│ 100   │ 64    │ d         │ 2    │ 64  │ 100 │ 01100100 │
│ 5794  │ 16a2  │ ᚢ         │ 3    │ e1  │ 225 │ 11100001 │
│       │       │           │ 4    │ 9a  │ 154 │ 10011010 │
│       │       │           │ 5    │ a2  │ 162 │ 10100010 │
│ 66352 │ 10330 │ 𐌰         │ 6    │ f0  │ 240 │ 11110000 │
│       │       │           │ 7    │ 90  │ 144 │ 10010000 │
│       │       │           │ 8    │ 8c  │ 140 │ 10001100 │
│       │       │           │ 9    │ b0  │ 176 │ 10110000 │
└───────┴───────┴───────────┴──────┴─────┴─────┴──────────┘
```

