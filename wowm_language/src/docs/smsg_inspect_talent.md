# SMSG_INSPECT_TALENT

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/_need_sorting/smsg_inspect_talent.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_inspect_talent.wowm#L30).
```rust,ignore
smsg SMSG_INSPECT_TALENT = 0x03F4 {
    PackedGuid player;
    u32 unspent_talent_points;
    u8 amount_of_specs;
    u8 active_spec;
    InspectTalentSpec[amount_of_specs] specs;
    u8 amount_of_glyphs;
    u16[amount_of_glyphs] glyphs;
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
| 0x04 | - / - | [PackedGuid](../spec/packed-guid.md) | player |  |  |
| - | 4 / Little | u32 | unspent_talent_points |  |  |
| - | 1 / - | u8 | amount_of_specs |  |  |
| - | 1 / - | u8 | active_spec |  |  |
| - | ? / - | [InspectTalentSpec](inspecttalentspec.md)[amount_of_specs] | specs |  |  |
| - | 1 / - | u8 | amount_of_glyphs |  |  |
| - | ? / - | u16[amount_of_glyphs] | glyphs |  |  |
