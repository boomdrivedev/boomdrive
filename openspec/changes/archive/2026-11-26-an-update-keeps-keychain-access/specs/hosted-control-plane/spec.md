## ADDED Requirements

### Requirement: On macOS the key is kept through macOS's own security

On macOS, the runner SHALL read and write a partition's key only through macOS's own `security`, by its path `/usr/bin/security`. The key SHALL reach `security` only on standard input, and SHALL come back only on standard output. So each build of the runner reads the item with no ask (core.32, record 0057).

#### Scenario: Kept in the keychain through security

- **WHEN** the keychain store, its program a stub `security`, reads the key of `home` at `control.example`, keeps it, then reads it again
- **THEN** each read runs `find-generic-password -s boomdrive -a home@control.example -w`, and the first read gives no key
- **AND** the write runs `-i`, with the line `add-generic-password -U -s boomdrive -a 'home@control.example' -X <the key in hex>` on standard input, and no argument holds the key
- **AND** the second read gives the key that was kept

#### Scenario: A key of printable bytes

- **WHEN** the stub `security` prints an item of 32 printable bytes, as `security` prints an item whose bytes are all printable
- **THEN** the key is those 32 bytes, also where they are hex digits

#### Scenario: The keychain refuses

- **WHEN** the stub `security` refuses a read with exit 51, or refuses a write with `add-generic-password: returned 25`
- **THEN** the read or the write fails with `security`'s words, and names the partition
