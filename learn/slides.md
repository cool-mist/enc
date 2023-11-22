# Characters over the wire #

Standards on sending, and parsing characters over the web.

## Basic idea ##

- **Assign** a number to each character using a Character set.
- **Encode** the number to bytes using an encoding scheme.
- Transfer bytes over the internet

The terms character set is used interchangably with character encoding and code pages.

---

# Common character sets #

## ASCII ##

- It assigns character to number mapping from 0-127 and covers english characters and some control codes (eg: new lines, tabs)
- Not everything from 0-127 is mapped.

## Latin ##

- Also called ISO-8859-1 character set.
- This is an extension of ASCII and covers the Latin alphabet - À,ä...
- Number mappings upto 255.

## Windows 1252 ##

- Super set of Latin character set.
- Introduced by Microsoft.

## Unicode ##

- Capable of defining a mapping for 1.1 million characters.
- Currently 150000 are defined.
- Each mapping is also called a unicode code point.
- Most languages - ஐ, ह
- Emojis 😮, 🤔
- Math ∫x.dx

---

# Common encoding schemes #

- An encoding scheme will encode the number to one or more bytes.

## Single byte encoding schemes ##

- Uses up only one byte.
- Suitable for ASCII, Latin and Windows 1252 character sets.
- ASCII would only take up 7 bits, while Latin and Windows 1252 would take up 8 bits.
- Because Windows 1252 is a superset of Latin, which is also a super set of ASCII, for a very long time in the past, the most used encoding scheme was Windows 1252.
- Today, it only accounts for 1.4% of the internet traffic.

```
┌───────┬───────┬───────────┬──────┬─────┬─────┬──────────┐
│ U+dec │ U+hex │ character │ byte │ hex │ dec │ bin      │
├───────┼───────┼───────────┼──────┼─────┼─────┼──────────┤
│ 97    │ 61    │ a         │ 0    │ 61  │ 97  │ 01100001 │
│ 98    │ 62    │ b         │ 1    │ 62  │ 98  │ 01100010 │
│ 99    │ 63    │ c         │ 2    │ 63  │ 99  │ 01100011 │
│ 100   │ 64    │ d         │ 3    │ 64  │ 100 │ 01100100 │
└───────┴───────┴───────────┴──────┴─────┴─────┴──────────┘
```

---

# Common encoding schemes #

- An encoding scheme will encode the number to one or more bytes.

## Multi byte encoding schemes ##

### UTF - 8 ###

- Variable byte encoding scheme.
- 1 - 4 bytes to represent a unicode code point.
- Backward compatible with ASCII.
- Can represent a maximum number of 2097152 code points.
- 99% of the internet uses this encoding scheme.


 | Byte 1   | Byte 2   | Byte 3   | Byte 4   | Available bits
 |----------|----------|----------|----------|----------------|
 | 0xxxxxxx | -        | -        | -        | 7              |
 | 110xxxxx | 10xxxxxx | -        | -        | 11             |
 | 1110xxxx | 10xxxxxx | 10xxxxxx | -        | 16             |
 | 11110xxx | 10xxxxxx | 10xxxxxx | 10xxxxxx | 21             |


```
┌────────┬───────┬───────────┬──────┬─────┬─────┬──────────┐
│ U+dec  │ U+hex │ character │ byte │ hex │ dec │ bin      │
├────────┼───────┼───────────┼──────┼─────┼─────┼──────────┤
│ 97     │ 61    │ a         │ 0    │ 61  │ 97  │ 01100001 │
│ 98     │ 62    │ b         │ 1    │ 62  │ 98  │ 01100010 │
│ 2960   │ b90   │ ஐ         │ 2    │ e0  │ 224 │ 11100000 │
│        │       │           │ 3    │ ae  │ 174 │ 10101110 │
│        │       │           │ 4    │ 90  │ 144 │ 10010000 │
│ 2361   │ 939   │ ह         │ 5    │ e0  │ 224 │ 11100000 │
│        │       │           │ 6    │ a4  │ 164 │ 10100100 │
│        │       │           │ 7    │ b9  │ 185 │ 10111001 │
│ 129300 │ 1f914 │ 🤔        │ 8    │ f0  │ 240 │ 11110000 │
│        │       │           │ 9    │ 9f  │ 159 │ 10011111 │
│        │       │           │ 10   │ a4  │ 164 │ 10100100 │
│        │       │           │ 11   │ 94  │ 148 │ 10010100 │
└────────┴───────┴───────────┴──────┴─────┴─────┴──────────┘
```

---

# Common encoding schemes #

- An encoding scheme will encode the number to one or more bytes.

## Multi byte encoding schemes ##


### UTF - 16 ###

- Variable byte encoding scheme.
- 2 or 4 bytes to represent a unicode code point.

```
┌────────┬───────┬───────────┬──────┬─────┬─────┬──────────┐
│ U+dec  │ U+hex │ character │ byte │ hex │ dec │ bin      │
├────────┼───────┼───────────┼──────┼─────┼─────┼──────────┤
│ 97     │ 61    │ a         │ 0    │ 00  │ 0   │ 00000000 │
│        │       │           │ 1    │ 61  │ 97  │ 01100001 │
│ 98     │ 62    │ b         │ 2    │ 00  │ 0   │ 00000000 │
│        │       │           │ 3    │ 62  │ 98  │ 01100010 │
│ 2960   │ b90   │ ஐ         │ 4    │ 0b  │ 11  │ 00001011 │
│        │       │           │ 5    │ 90  │ 144 │ 10010000 │
│ 2361   │ 939   │ ह         │ 6    │ 09  │ 9   │ 00001001 │
│        │       │           │ 7    │ 39  │ 57  │ 00111001 │
│ 129300 │ 1f914 │ 🤔        │ 8    │ d8  │ 216 │ 11011000 │
│        │       │           │ 9    │ 3e  │ 62  │ 00111110 │
│        │       │           │ 10   │ dd  │ 221 │ 11011101 │
│        │       │           │ 11   │ 14  │ 20  │ 00010100 │
└────────┴───────┴───────────┴──────┴─────┴─────┴──────────┘
```

---

# URL Encoding #

- Applicable only for HTTP traffic.
- Some characters have a special meaning in the url string Eg: &, #, ?
- The url string should also be only in ASCII.
- These characters should be treated differently.

## Steps to URL-encode a string ##

- Encode the string in one of the encoding schemes.
- If a particular character cannot appear in the url string, or is not ASCII, print the hex representation of the string, prefixed with a `%`.

```
┌───────┬───────┬───────────┬──────┬─────┬─────┬──────────┐
│ U+dec │ U+hex │ character │ byte │ hex │ dec │ bin      │
├───────┼───────┼───────────┼──────┼─────┼─────┼──────────┤
│ 38    │ 26    │ &         │ 0    │ 26  │ 38  │ 00100110 │
│ 63    │ 3f    │ ?         │ 1    │ 3f  │ 63  │ 00111111 │
└───────┴───────┴───────────┴──────┴─────┴─────┴──────────┘
```

- For example, if the url string `p1&/pw?` were to be url-encoded under utf-8 encoding, then it would be `p1%26/pw%3f`

```
┌───────┬───────┬───────────┬──────┬─────┬─────┬──────────┐
│ U+dec │ U+hex │ character │ byte │ hex │ dec │ bin      │
├───────┼───────┼───────────┼──────┼─────┼─────┼──────────┤
│ 38    │ 26    │ &         │ 0    │ 00  │ 0   │ 00000000 │
│       │       │           │ 1    │ 26  │ 38  │ 00100110 │
│ 63    │ 3f    │ ?         │ 2    │ 00  │ 0   │ 00000000 │
│       │       │           │ 3    │ 3f  │ 63  │ 00111111 │
└───────┴───────┴───────────┴──────┴─────┴─────┴──────────┘
```

- Under utf-16 encoding, it would be `p1%00%26/pw%00%3f`

---

# What should be supported in applications? #

- Support Unicode code points encoded as utf-8 characters.
- URL encode under utf-8.

---

# What is a character? #

- It is a group of unicode code points - also called a grapheme cluster.
- Eg: the character 'ப்' consists of 2 unicode code points as seen below.

```
┌───────┬───────┬───────────┬──────┬─────┬─────┬──────────┐
│ U+dec │ U+hex │ character │ byte │ hex │ dec │ bin      │
├───────┼───────┼───────────┼──────┼─────┼─────┼──────────┤
│ 2986  │ baa   │ ப         │ 0    │ e0  │ 224 │ 11100000 │
│       │       │           │ 1    │ ae  │ 174 │ 10101110 │
│       │       │           │ 2    │ aa  │ 170 │ 10101010 │
│ 3021  │ bcd   │ ்         | 3    │ e0  │ 224 │ 11100000 │
│       │       │           │ 4    │ af  │ 175 │ 10101111 │
│       │       │           │ 5    │ 8d  │ 141 │ 10001101 │
└───────┴───────┴───────────┴──────┴─────┴─────┴──────────┘
```

- Number of characters in a string is often different from `string.Length`.
- Some languages (eg: python) return the number of unicode code points.
- Some languages (eg: C#) will return the number of utf-16 bytes to encode the complete string.
- The below emoji is of length 1 in python and length 4 in c#.


```
┌────────┬───────┬───────────┬──────┬─────┬─────┬──────────┐
│ U+dec  │ U+hex │ character │ byte │ hex │ dec │ bin      │
├────────┼───────┼───────────┼──────┼─────┼─────┼──────────┤
│ 129300 │ 1f914 │ 🤔        │ 0    │ d8  │ 216 │ 11011000 │
│        │       │           │ 1    │ 3e  │ 62  │ 00111110 │
│        │       │           │ 2    │ dd  │ 221 │ 11011101 │
│        │       │           │ 3    │ 14  │ 20  │ 00010100 │
└────────┴───────┴───────────┴──────┴─────┴─────┴──────────┘
```

- Be careful about advertising character length limitations.
