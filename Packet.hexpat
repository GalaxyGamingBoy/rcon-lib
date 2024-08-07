struct Packet {
    u32 size;
    u32 id;
    u32 type;
    char data[size - 0x0A];
    char terminator[2];
};

Packet packet_at_0x00 @ 0x00;