# SMSG_BATTLEFIELD_MGR_STATE_CHANGE

## Client Version 3.3.5

### Comment

Only exists as comment in azerothcore/trinitycore.

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/_need_sorting/smsg_battlefield_mgr_state_change.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_battlefield_mgr_state_change.wowm#L1).
```rust,ignore
smsg SMSG_BATTLEFIELD_MGR_STATE_CHANGE = 0x04E8 {
    u32 unknown1;
    u32 unknown2;
}
```
### Header

SMSG have a header of 4 bytes.

#### SMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | 4 / Little | u32 | unknown1 |  |  |
| 0x08 | 4 / Little | u32 | unknown2 |  |  |
