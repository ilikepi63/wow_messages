# CMSG_CHANGE_SEATS_ON_CONTROLLED_VEHICLE

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/_need_sorting/cmsg_change_seats_on_controlled_vehicle.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_change_seats_on_controlled_vehicle.wowm#L1).
```rust,ignore
cmsg CMSG_CHANGE_SEATS_ON_CONTROLLED_VEHICLE = 0x049B {
    PackedGuid vehicle;
    MovementInfo info;
    PackedGuid accessory;
    u8 seat;
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
| 0x06 | - / - | [PackedGuid](../spec/packed-guid.md) | vehicle |  |  |
| - | - / - | [MovementInfo](movementinfo.md) | info |  |  |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | accessory |  |  |
| - | 1 / - | u8 | seat |  |  |
