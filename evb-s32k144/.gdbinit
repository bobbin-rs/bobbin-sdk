target remote :2331
monitor reset 
monitor writeu32 0x40052004 0xD928C520
monitor writeu32 0x40052000 0x00002900
monitor writeu32 0x40052008 0x0000ffff
