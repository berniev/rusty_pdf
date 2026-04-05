/*
Comprehensive tests for PDF Object Stream specification compliance

PDF Reference 1.7, Section 3.4.6: Object Streams (ISO 32000-1:2008)

SPEC REQUIREMENTS (from PDF 32000):

Object Stream Dictionary Entries:
- Type (required): must be /ObjStm
- N (required): The number of compressed objects in the stream
- First (required): The byte offset (in the decoded stream) of the first compressed object
- Extends (optional): A reference to another object stream

Stream Data Format:
1. N pairs of integers: (object_number, byte_offset)
2. Byte offsets are RELATIVE TO THE FIRST OBJECT (the position indicated by /First)
3. Followed by the N objects themselves, concatenated

Example structure:
Stream data: "10 0 11 25 12 50" + [obj 10 data (25 bytes)] + [obj 11 data (25 bytes)] + [obj 12 data]
- /N = 3
- /First = 18 (length of "10 0 11 25 12 50\n")
- Object 10 at offset 0 from /First
- Object 11 at offset 25 from /First (25 bytes after start of obj 10)
- Object 12 at offset 50 from /First (50 bytes after start of obj 10)

Objects that CANNOT be compressed:
- Stream objects (objects containing a stream)
- Objects with generation number != 0
- The encryption dictionary

Test Strategy:
1. Test spec format requirements with known values
2. Test boundary conditions (1 object, 2 objects, many objects)
3. Test offset calculations match spec
4. Test exclusions (streams cannot be compressed)

// SPEC TEST 1: /Type must be /ObjStm

// SPEC TEST 2: /N must be present and be an integer

// SPEC TEST 3: /First must be present and be an integer

// SPEC TEST 4: Stream objects cannot be compressed
*/