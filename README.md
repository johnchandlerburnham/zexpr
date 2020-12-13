# Zexpr

ZExpr is composed of several formats:

- ZType: Self-describing codecs
- ZBase: Self-describing numerical print formats
- ZLink: Hash-linked content-identifiers
- ZAtom: Efficiently-packed typed bytes
- ZCons: Efficiently-packed canonical trees

# ZType

ZType is a table of prefix codes which describe the type of the encoded bytes.
This table can be found in `ztype.csv`.

For example, the `ZAtom`:

```
xdead_beef_dead_beef:int
```

indicates that that `xdead_beef_dead_beef` should be interpreted as a signed
integer.

All ztypes can be optionally extended with a numeric decimal suffix which lifts
the bit-length of the value into the type. Without any annotation, the type is
assumed to be variable length (i.e. the length is not type-relevant). 

For example

```
xdead_beef_dead_beef:int64
```

Indicates that `xdead_beef_dead_beef` is a 64-bit integer. This bit-number
indicated by this suffix must be byte-aligned, so that e.g. `int63` is not a
valid `ztype`. The textual format of ZType uses bit-length rather than
byte-length to align better with the common convention of saying "64-bit" rather
than "8-byte" or "8-word." However, the binary encoding of ZType uses
byte-length, so the data_length of an `int64` is encoded as the unsigned integer
`8`


## ZBase

ZBase defines a self-describing print format for bytes. This is not necessarily
a numeric base, since bijective numerations (useful for *canonically* printing
numbers as text) could be valid print formats too. (Consider the sequence:
"A, B, C, ..., X, Y, Z, AA, AB, AC, ..., AX, AY, AZ, BA, BB, BC, ..." which
Microsoft Excel uses to number its columns.)

| encoding   | code | description         |
|------------|------|---------------------|
| base2,     | 'b', | binary (01010101),  |
| base8,     | 'o', | octal,              |
| base10,    | 'd', | decimal,            |
| base16,    | 'x', | hexadecimal,        |
| base32z,   | 'v', | z-base-32           |
| base58btc, | 'I', | base58 bitcoin,     |
| base64url, | '~', | rfc4648 no padding, |

## ZExpr syntax

ZAtom:
```
<bytes>:<ztype>

wbafyasdfoiuwer:int
xdead_beef_dead_beef:bytes
```

ZCons:

```
(<zatom> <zatom> <zatom> ... <zatom>)
```

## FAQ

[TODO]

- Why use ZExpr instead of Multiformats?
- Why use ZExpr instead of JSON?
- Why use ZExpr instead of csexp?

