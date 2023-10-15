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
- This is an extension of ASCII and covers the Latin alphabet - think english looking alphabets with diacritics. eg: Àä
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
~~~enc-check -8 abcd

~~~
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
~~~enc-check -8 abஐह🤔

~~~
```

---

# Common encoding schemes #

- An encoding scheme will encode the number to one or more bytes.

## Multi byte encoding schemes ##


### UTF - 16 ###

- Variable byte encoding scheme.
- 2 or 4 bytes to represent a unicode code point.

```
~~~enc-check -6 abஐह🤔

~~~

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
~~~enc-check -8 &?

~~~
```

- For example, if the url string `p1&/pw?` were to be url-encoded under utf-8 encoding, then it would be `p1%26/pw%3f`


```
~~~enc-check -6 &?

~~~
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
~~~enc-check -8 ப்

~~~
```

- Number of characters in a string is often different from `string.Length`.
- Some languages (eg: python) return the number of unicode code points.
- Some languages (eg: C#) will return the number of utf-16 bytes to encode the complete string.
- The below emoji is of length 1 in python and length 4 in c#.


```
~~~enc-check -6 🤔

~~~
```

- Be careful about advertising character length limitations.
