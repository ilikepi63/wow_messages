# CMSG_ALTER_APPEARANCE

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/character_screen/cmsg_alter_appearance.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_alter_appearance.wowm#L1).
```rust,ignore
cmsg CMSG_ALTER_APPEARANCE = 0x0426 {
    u32 hair;
    u32 hair_color;
    u32 facial_hair;
}
```
### Header

CMSG have a header of 6 bytes.

#### CMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x06 | 4 / Little | u32 | hair |  |  |
| 0x0A | 4 / Little | u32 | hair_color |  |  |
| 0x0E | 4 / Little | u32 | facial_hair |  |  |

