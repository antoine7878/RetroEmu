#!/usr/bin/env python3
"""Print the decoded cartridge header of a Game Boy / Game Boy Color ROM.

Usage: python3 gbheader.py game.gb
"""

import sys

CART_TYPES = {
    0x00: "ROM ONLY",
    0x01: "MBC1",
    0x02: "MBC1+RAM",
    0x03: "MBC1+RAM+BATTERY",
    0x05: "MBC2",
    0x06: "MBC2+BATTERY",
    0x08: "ROM+RAM",
    0x09: "ROM+RAM+BATTERY",
    0x0B: "MMM01",
    0x0C: "MMM01+RAM",
    0x0D: "MMM01+RAM+BATTERY",
    0x0F: "MBC3+TIMER+BATTERY",
    0x10: "MBC3+TIMER+RAM+BATTERY",
    0x11: "MBC3",
    0x12: "MBC3+RAM",
    0x13: "MBC3+RAM+BATTERY",
    0x19: "MBC5",
    0x1A: "MBC5+RAM",
    0x1B: "MBC5+RAM+BATTERY",
    0x1C: "MBC5+RUMBLE",
    0x1D: "MBC5+RUMBLE+RAM",
    0x1E: "MBC5+RUMBLE+RAM+BATTERY",
    0x20: "MBC6",
    0x22: "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
    0xFC: "POCKET CAMERA",
    0xFD: "BANDAI TAMA5",
    0xFE: "HuC3",
    0xFF: "HuC1+RAM+BATTERY",
}

RAM_SIZES = {
    0x00: "None",
    0x01: "Unused (2 KiB on some carts)",
    0x02: "8 KiB (1 bank)",
    0x03: "32 KiB (4 banks)",
    0x04: "128 KiB (16 banks)",
    0x05: "64 KiB (8 banks)",
}

CGB_FLAGS = {0x80: "Supports CGB (also works on DMG)", 0xC0: "CGB only"}


def main(path):
    with open(path, "rb") as f:
        rom = f.read()
    if len(rom) < 0x150:
        sys.exit("File too small to contain a Game Boy header.")

    h = rom  # header fields are at absolute offsets

    title = h[0x134:0x144].split(b"\x00")[0].decode("ascii", errors="replace")
    cgb = h[0x143]
    new_lic = h[0x144:0x146].decode("ascii", errors="replace")
    sgb = h[0x146]
    cart = h[0x147]
    rom_code = h[0x148]
    ram_code = h[0x149]
    dest = h[0x14A]
    old_lic = h[0x14B]
    version = h[0x14C]
    hdr_ck = h[0x14D]
    glob_ck = (h[0x14E] << 8) | h[0x14F]

    # Header checksum: x = x - byte - 1 over 0x134..0x14C
    calc_hdr = 0
    for b in h[0x134:0x14D]:
        calc_hdr = (calc_hdr - b - 1) & 0xFF
    # Global checksum: sum of every byte except the two checksum bytes
    calc_glob = (sum(rom) - h[0x14E] - h[0x14F]) & 0xFFFF

    if rom_code <= 0x08:
        rom_kib = 32 << rom_code
        rom_desc = f"{rom_kib} KiB ({rom_kib // 16} banks)"
    else:
        rom_desc = "Unknown"

    nintendo_logo_ok = h[0x104:0x134] == bytes.fromhex(
        "CEED6666CC0D000B03730083000C000D0008111F8889000EDCCC6EE6DDDDD999"
        "BBBB67636E0EECCCDDDC999FBBB9333E"
    )

    rows = [
        ("Title", f"{title!r}"),
        ("Nintendo logo", "OK" if nintendo_logo_ok else "MISMATCH"),
        (
            "CGB flag (0x143)",
            f"0x{cgb:02X}  {CGB_FLAGS.get(cgb, 'DMG only / part of title')}",
        ),
        ("New licensee (0x144)", new_lic if old_lic == 0x33 else "(not used)"),
        (
            "SGB flag (0x146)",
            f"0x{sgb:02X}  {'Supports SGB' if sgb == 0x03 else 'No SGB'}",
        ),
        (
            "Cartridge type (0x147)",
            f"0x{cart:02X}  {CART_TYPES.get(cart, 'Unknown')}",
        ),
        ("ROM size (0x148)", f"0x{rom_code:02X}  {rom_desc}"),
        (
            "RAM size (0x149)",
            f"0x{ram_code:02X}  {RAM_SIZES.get(ram_code, 'Unknown')}",
        ),
        (
            "Destination (0x14A)",
            f"0x{dest:02X}  {'Japan' if dest == 0 else 'Overseas'}",
        ),
        ("Old licensee (0x14B)", f"0x{old_lic:02X}"),
        ("Version (0x14C)", f"{version}"),
        (
            "Header checksum (0x14D)",
            f"0x{hdr_ck:02X}  (computed 0x{calc_hdr:02X}, {'OK' if hdr_ck == calc_hdr else 'BAD'})",
        ),
        (
            "Global checksum (0x14E)",
            f"0x{glob_ck:04X}  (computed 0x{calc_glob:04X}, {'OK' if glob_ck == calc_glob else 'BAD'})",
        ),
        ("Actual file size", f"{len(rom) // 1024} KiB"),
    ]
    width = max(len(k) for k, _ in rows)
    for k, v in rows:
        print(f"{k:<{width}}  {v}")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    main(sys.argv[1])
