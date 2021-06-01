# Rustified Ellisya Virtual Machine V2

Ellisya Virtual Machine is a switch based VM implemented in Rust. It has 3 registers $1 $2 and $3 (aka 3 args for each instruction). Because of the limitation of register size, the VM can utilize maximum 64kb of memory address space (`0x0000 - 0xFFFF`).

Instruction Format: `opcode`-`mem addr1`-`mem addr2`-`mem addr3`-`data`

Type: `u8`-`u8`-`u8`-`u8`-`u32`

After executing of a instruction, all bits on registers & data will gone.

```text
Diagram of one instruction in bit
-------------------------------
Operator  | Mem addr1 | Mem addr1 | Mem addr1 | Data
0000 0000 | 0000 0000 | 0000 0000 | 0000 0000 | 0000 0000 0000 0000 0000 0000 0000 0000
```

### opCode

Registers ($1, $2, $3) are only used for memory address.

Hex | Word  | Usage       | Description
----|-------|-------------|------------|
0x00| halt  | halt |  |
0x01| load  | load $1 0x12AF | load `Data` to $1
0x02| inc   | inc  $1      | increase value on $1 by 1
0x03| add   | add  $3 $1 $2| add $1 and $2 then store it to $3 
0x04| print | print $1     | print the content of $1 to stdout
0x05| strPrint | print $1  | convert number at $ to ansi code and print
0xxx| TODO  | to be expanded |

## OLD SPEC
Specification <https://github.com/shenlebantongying/Ellisya_virtual_machine>