import os
import sys
import struct

MAGIC = b"GHOSTBYTEv1"

FOOTER = "<BQHIQB"
FOOTER_SIZE = struct.calcsize(FOOTER)


def checksum(data):
    return sum(data) & 0xFFFFFFFF


def open_file(path, mode="rb"):
    sample = os.path.join("samples", path)

    if os.path.exists(sample):
        return open(sample, mode)

    return open(path, mode)


def implant(payload, host):
    with open_file(payload, "rb") as f:
        data = f.read()

    with open_file(host, "ab+") as f:
        f.seek(0, 2)

        offset = f.tell()

        footer = struct.pack(
            FOOTER,
            1,
            len(data),
            checksum(data),
            len(payload),
            offset,
            0,
        )

        for chunk in [
            data,
            payload.encode(),
            footer,
            MAGIC,
        ]:
            f.write(chunk)

    print(f"""
[GhostByte]

Payload  : {payload}
Host     : {host}
Status   : Implanted
Mode     : Append-Only
""")


def inspect(host):
    with open_file(host, "rb") as f:
        f.seek(0, 2)

        size = f.tell()

        f.seek(size - len(MAGIC))

        if f.read(len(MAGIC)) != MAGIC:
            raise Exception("GhostByte payload not found")

        footer_pos = size - len(MAGIC) - FOOTER_SIZE

        f.seek(footer_pos)

        (
            version,
            payload_size,
            check,
            name_len,
            offset,
            flags,
        ) = struct.unpack(
            FOOTER,
            f.read(FOOTER_SIZE),
        )

        name_pos = footer_pos - name_len

        f.seek(name_pos)

        name = f.read(name_len).decode()

        f.seek(offset)

        payload = f.read(payload_size)

        status = (
            "VERIFIED"
            if checksum(payload) == check
            else "CORRUPTED"
        )

        print(f"""
GhostByte Inspection Report

Host File   : {host}
Embedded    : {name}
File Size   : {size} bytes
Offset      : {offset}

Integrity   : {status}
Layout      : Append-Only
Parser      : Reverse Footer Traversal

Binary Layout
────────────────────────────
HOST      ████████████████████
PAYLOAD   ██
META      █
MAGIC     ■
""")


def extract(host):
    with open_file(host, "rb") as f:
        f.seek(0, 2)

        size = f.tell()

        f.seek(size - len(MAGIC))

        if f.read(len(MAGIC)) != MAGIC:
            raise Exception("GhostByte payload not found")

        footer_pos = size - len(MAGIC) - FOOTER_SIZE

        f.seek(footer_pos)

        (
            version,
            payload_size,
            check,
            name_len,
            offset,
            flags,
        ) = struct.unpack(
            FOOTER,
            f.read(FOOTER_SIZE),
        )

        name_pos = footer_pos - name_len

        f.seek(name_pos)

        name = f.read(name_len).decode()

        f.seek(offset)

        payload = f.read(payload_size)

        if checksum(payload) != check:
            raise Exception("Checksum mismatch")

    with open(name, "wb") as out:
        out.write(payload)

    print(f"""
[GhostByte Extraction]

Recovered : {name}
Method    : Reverse Footer Parsing
Status    : Success
""")


def usage():
    print("""
Usage:
python ghostbyte.py implant <payload> <host>
python ghostbyte.py inspect <host>
python ghostbyte.py extract <host>
""")


def main():
    if len(sys.argv) < 3:
        usage()
        return

    cmd = sys.argv[1]

    try:
        if cmd == "implant" and len(sys.argv) == 4:
            implant(sys.argv[2], sys.argv[3])

        elif cmd == "inspect" and len(sys.argv) == 3:
            inspect(sys.argv[2])

        elif cmd == "extract" and len(sys.argv) == 3:
            extract(sys.argv[2])

        else:
            usage()

    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    main()