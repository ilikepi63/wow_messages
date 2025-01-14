# SMSG_UPDATE_LFG_LIST

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/lfg/smsg_update_lfg_list.wowm:111`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_update_lfg_list.wowm#L111).
```rust,ignore
smsg SMSG_UPDATE_LFG_LIST = 0x0360 {
    (u32)LfgType lfg_type;
    u32 dungeon_id;
    LfgListUpdateType update_type;
    if (update_type == PARTIAL) {
        u32 amount_of_deleted_guids;
        Guid[amount_of_deleted_guids] deleted_guids;
    }
    u32 amount_of_groups;
    u32 unknown1;
    LfgListGroup[amount_of_groups] groups;
    u32 amount_of_players;
    u32 unknown2;
    LfgListPlayer[amount_of_players] players;
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
| 0x04 | 4 / - | [LfgType](lfgtype.md) | lfg_type |  |  |
| 0x08 | 4 / Little | u32 | dungeon_id |  |  |
| 0x0C | 1 / - | [LfgListUpdateType](lfglistupdatetype.md) | update_type |  |  |

If update_type is equal to `PARTIAL`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x0D | 4 / Little | u32 | amount_of_deleted_guids |  |  |
| 0x11 | ? / - | [Guid](../spec/packed-guid.md)[amount_of_deleted_guids] | deleted_guids |  |  |
| - | 4 / Little | u32 | amount_of_groups |  |  |
| - | 4 / Little | u32 | unknown1 |  | emus set to 0. |
| - | ? / - | [LfgListGroup](lfglistgroup.md)[amount_of_groups] | groups |  |  |
| - | 4 / Little | u32 | amount_of_players |  |  |
| - | 4 / Little | u32 | unknown2 |  | emus set to 0. |
| - | ? / - | [LfgListPlayer](lfglistplayer.md)[amount_of_players] | players |  |  |

