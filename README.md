<!-- AUTOMATICALLY GENERATED, DO NOT EDIT -->
<!-- edit README.md.template instead -->

# Rust serialization benchmark

The goal of these benchmarks is to provide thorough and complete benchmarks for various rust
serialization frameworks.

## These benchmarks are a work in progress

These benchmarks are still being developed and pull requests to improve benchmarks are welcome.

You can use [this horrible tiny webpage](https://davidkoloski.me/rust_serialization_benchmark) to
turn a benchmark log into nicely-formatted markdown tables.

## Format

All tests benchmark the following properties (time or size):

* **Serialize**: serialize data into a buffer
* **Deserialize**: deserializes a buffer into a normal rust object
* **Size**: the size of the buffer when serialized
* **Zlib**: the size of the buffer after zlib compression

Zero-copy deserialization libraries have an additional set of benchmarks:

* **Access**: accesses a buffer as structured data
* **Read**: runs through a buffer and reads fields out of it
* **Update**: updates a buffer as structured data

Some benchmark results may be italicized and followed by an asterisk. Mouse over these for more details on what situation was benchmarked. Other footnotes are located at the bottom.

## Last updated: 2023-2-7

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 292.22 µs | <span title="unvalidated">*2.8610 ms\**</span> | 1705800 | 502521 | 414548 |
| bare | 903.32 µs | 4.0336 ms | 765778 | 312739 | 264630 |
| bincode | 714.21 µs | 3.7885 ms | 1045784 | 374305 | 311761 |
| borsh | 583.81 µs | 3.8601 ms | 885780 | 363280 | 286514 |
| bson | 3.6009 ms | 13.458 ms | 1924682 | 537661 | 376270 |
| capnp | 962.82 µs | † | 1443216 | 509618 | 428649 |
| cbor | 2.2577 ms | 7.9276 ms | 1407835 | 407372 | 324081 |
| flatbuffers | 2.2986 ms | † | 1276368 | 469962 | 388832 |
| nachricht | 8.2204 ms | 6.9161 ms | 818669 | 334639 | 285514 |
| postcard | 469.48 µs | 4.2310 ms | 724953 | 303462 | 253747 |
| prost | <span title="populate + encode">*3.9564 ms\**</span> <span title="encode">*685.62 µs\**</span> | 4.7753 ms | 764951 | 269811 | 227947 |
| rkyv | 393.55 µs | <span title="unvalidated">*2.8642 ms\**</span> <span title="validated upfront with error">*4.1927 ms\**</span> | 1011488 | 384230 | 336234 |
| rmp | 1.8871 ms | 5.5423 ms | 784997 | 326654 | 278219 |
| ron | 21.022 ms | 20.686 ms | 1607459 | 452648 | 349713 |
| scale | 706.20 µs | 3.9542 ms | 765778 | 312771 | 264518 |
| serde_json | 4.6766 ms | 10.728 ms | 1827461 | 474358 | 361090 |
| simd-json | 5.1659 ms | 7.1933 ms | 1827461 | 474358 | 361090 |
| speedy | 321.33 µs | 3.3646 ms | 885780 | 363280 | 286514 |
| alkahest | 300.82 µs | † | 1045784 | 454748 | 389424 |
| dlhn | 861.00 µs | 4.5646 ms | 724953 | 302512 | 253629 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*44.395 µs\**</span> | <span title="unvalidated">*68.706 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*115.62 ns\**</span> | <span title="validated on-demand with error">*433.25 µs\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.9258 ns\**</span> <span title="validated upfront with error">*2.6016 ms\**</span> | <span title="unvalidated">*115.34 µs\**</span> <span title="validated upfront with error">*2.6752 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.8270 ns\**</span> <span title="validated upfront with error">*1.0083 ms\**</span> | <span title="unvalidated">*19.694 µs\**</span> <span title="validated upfront with error">*1.0267 ms\**</span> | 15.294 µs |
| alkahest | <span title="validated on-demand with panic">*3.3236 ns\**</span> | <span title="validated on-demand with panic">*60.300 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*100.00%\**</span> | 42.50% | 53.69% | 54.99% |
| bare | 32.35% | 70.93% | 94.67% | 86.27% | 86.14% |
| bincode | 40.92% | 75.52% | 69.32% | 72.08% | 73.12% |
| borsh | 50.05% | 74.12% | 81.84% | 74.27% | 79.56% |
| bson | 8.12% | 21.26% | 37.67% | 50.18% | 60.58% |
| capnp | 30.35% | † | 50.23% | 52.94% | 53.18% |
| cbor | 12.94% | 36.09% | 51.49% | 66.23% | 70.34% |
| flatbuffers | 12.71% | † | 56.80% | 57.41% | 58.62% |
| nachricht | 3.55% | 41.37% | 88.55% | 80.63% | 79.84% |
| postcard | 62.24% | 67.62% | 100.00% | 88.91% | 89.83% |
| prost | <span title="populate + encode">*7.39%\**</span> <span title="encode">*42.62%\**</span> | 59.91% | 94.77% | 100.00% | 100.00% |
| rkyv | 74.25% | <span title="unvalidated">*99.89%\**</span> <span title="validated upfront with error">*68.24%\**</span> | 71.67% | 70.22% | 67.79% |
| rmp | 15.49% | 51.62% | 92.35% | 82.60% | 81.93% |
| ron | 1.39% | 13.83% | 45.10% | 59.61% | 65.18% |
| scale | 41.38% | 72.35% | 94.67% | 86.26% | 86.17% |
| serde_json | 6.25% | 26.67% | 39.67% | 56.88% | 63.13% |
| simd-json | 5.66% | 39.77% | 39.67% | 56.88% | 63.13% |
| speedy | 90.94% | 85.03% | 81.84% | 74.27% | 79.56% |
| alkahest | 97.14% | † | 69.32% | 59.33% | 58.53% |
| dlhn | 33.94% | 62.68% | 100.00% | 89.19% | 89.87% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*28.66%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*1.58%\**</span> | <span title="validated on-demand with error">*4.55%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.54%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.07%\**</span> <span title="validated upfront with error">*0.74%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.92%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*54.97%\**</span> | <span title="validated on-demand with panic">*32.66%\**</span> | ‡ |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 501.83 µs | <span title="unvalidated">*497.71 µs\**</span> | 6000024 | 5380836 | 5345890 |
| bare | 8.5607 ms | 7.0374 ms | 6000003 | 5380817 | 5345900 |
| bincode | 5.4467 ms | 7.1394 ms | 6000008 | 5380823 | 5345890 |
| borsh | 6.8942 ms | 6.3499 ms | 6000004 | 5380818 | 5345889 |
| bson | 66.263 ms | 152.33 ms | 23013911 | 9211138 | 7497811 |
| capnp | 15.108 ms | † | 14000088 | 6729881 | 6051062 |
| cbor | 53.239 ms | 62.066 ms | 13122324 | 7527423 | 6759658 |
| flatbuffers | 1.0326 ms | † | 6000024 | 5380800 | 5345910 |
| nachricht | 183.83 ms | 44.055 ms | 8125037 | 6495174 | 6386940 |
| postcard | 874.72 µs | 1.7402 ms | 6000003 | 5380817 | 5345900 |
| prost | <span title="populate + encode">*9.8522 ms\**</span> <span title="encode">*7.8212 ms\**</span> | 19.550 ms | 8750000 | 6683814 | 6421871 |
| rkyv | 656.70 µs | <span title="unvalidated">*528.74 µs\**</span> <span title="validated upfront with error">*528.81 µs\**</span> | 6000008 | 5380822 | 5345892 |
| rmp | 21.604 ms | 24.301 ms | 8125006 | 6496879 | 6391037 |
| ron | 269.03 ms | 356.78 ms | 22192885 | 9009575 | 8138755 |
| scale | 6.0523 ms | 7.3695 ms | 6000004 | 5380818 | 5345889 |
| serde_json | 124.30 ms | 117.89 ms | 26192883 | 9612105 | 8586741 |
| simd-json | 121.92 ms | 159.09 ms | 39152823 | 16587283 | 14549214 |
| speedy | 534.27 µs | 464.55 µs | 6000004 | 5380818 | 5345889 |
| alkahest | 532.04 µs | † | 6000008 | 5380823 | 5345890 |
| dlhn | 7.0994 ms | 9.8885 ms | 6000003 | 5380817 | 5345900 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*3.1241 ns\**</span> | <span title="unvalidated">*294.38 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*201.21 ns\**</span> | <span title="validated on-demand with error">*6.4158 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.9274 ns\**</span> <span title="validated upfront with error">*54.536 ns\**</span> | <span title="unvalidated">*56.483 µs\**</span> <span title="validated upfront with error">*56.573 µs\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.8299 ns\**</span> <span title="validated upfront with error">*17.650 ns\**</span> | <span title="unvalidated">*56.603 µs\**</span> <span title="validated upfront with error">*56.533 µs\**</span> | 285.07 µs |
| alkahest | <span title="validated on-demand with panic">*3.3233 ns\**</span> | <span title="validated on-demand with panic">*100.46 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*93.34%\**</span> | 100.00% | 100.00% | 100.00% |
| bare | 5.86% | 6.60% | 100.00% | 100.00% | 100.00% |
| bincode | 9.21% | 6.51% | 100.00% | 100.00% | 100.00% |
| borsh | 7.28% | 7.32% | 100.00% | 100.00% | 100.00% |
| bson | 0.76% | 0.30% | 26.07% | 58.42% | 71.30% |
| capnp | 3.32% | † | 42.86% | 79.95% | 88.35% |
| cbor | 0.94% | 0.75% | 45.72% | 71.48% | 79.09% |
| flatbuffers | 48.60% | † | 100.00% | 100.00% | 100.00% |
| nachricht | 0.27% | 1.05% | 73.85% | 82.84% | 83.70% |
| postcard | 57.37% | 26.70% | 100.00% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*5.09%\**</span> <span title="encode">*6.42%\**</span> | 2.38% | 68.57% | 80.50% | 83.25% |
| rkyv | 76.42% | <span title="unvalidated">*87.86%\**</span> <span title="validated upfront with error">*87.85%\**</span> | 100.00% | 100.00% | 100.00% |
| rmp | 2.32% | 1.91% | 73.85% | 82.82% | 83.65% |
| ron | 0.19% | 0.13% | 27.04% | 59.72% | 65.68% |
| scale | 8.29% | 6.30% | 100.00% | 100.00% | 100.00% |
| serde_json | 0.40% | 0.39% | 22.91% | 55.98% | 62.26% |
| simd-json | 0.41% | 0.29% | 15.32% | 32.44% | 36.74% |
| speedy | 93.93% | 100.00% | 100.00% | 100.00% | 100.00% |
| alkahest | 94.32% | † | 100.00% | 100.00% | 100.00% |
| dlhn | 7.07% | 4.70% | 100.00% | 100.00% | 100.00% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*58.57%\**</span> | <span title="unvalidated">*19.19%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.91%\**</span> | <span title="validated on-demand with error">*0.88%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.59%\**</span> <span title="validated upfront with error">*3.36%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.84%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*10.37%\**</span> | <span title="unvalidated">*99.79%\**</span> <span title="validated upfront with error">*99.91%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*55.06%\**</span> | <span title="validated on-demand with panic">*56.22%\**</span> | ‡ |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 308.84 µs | <span title="unvalidated">*2.2745 ms\**</span> | 1290592 | 396039 | 339085 |
| bare | 956.99 µs | 3.8415 ms | 356311 | 213270 | 198488 |
| bincode | 783.03 µs | 3.1041 ms | 569975 | 240897 | 232423 |
| borsh | 692.52 µs | 2.9652 ms | 446595 | 234395 | 210008 |
| bson | 5.1375 ms | 14.440 ms | 1619653 | 506953 | 328399 |
| capnp | 815.81 µs | † | 803896 | 336655 | 280851 |
| cbor | 2.2963 ms | 7.7502 ms | 1109821 | 347812 | 274526 |
| flatbuffers | 4.1814 ms | † | 844168 | 346957 | 294015 |
| nachricht | 7.6743 ms | 6.1750 ms | 449745 | 252743 | 231110 |
| postcard | 558.41 µs | 3.2794 ms | 367489 | 222144 | 207344 |
| prost | <span title="populate + encode">*4.5361 ms\**</span> <span title="encode">*1.6076 ms\**</span> | 5.5519 ms | 596811 | 306728 | 269310 |
| rkyv | 513.28 µs | <span title="unvalidated">*2.1870 ms\**</span> <span title="validated upfront with error">*3.0159 ms\**</span> | 596952 | 254678 | 220053 |
| rmp | 2.1438 ms | 4.6357 ms | 424533 | 245594 | 226188 |
| ron | 12.244 ms | 21.908 ms | 1465223 | 439761 | 343338 |
| scale | 819.08 µs | 3.2323 ms | 356311 | 213188 | 198524 |
| serde_json | 5.0238 ms | 12.285 ms | 1623191 | 472275 | 359623 |
| simd-json | 5.0236 ms | 7.1665 ms | 1663769 | 496401 | 383682 |
| speedy | 488.25 µs | 2.7077 ms | 449595 | 235136 | 210361 |
| alkahest | 348.40 µs | † | 667570 | 325536 | 320452 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*70.448 µs\**</span> | <span title="unvalidated">*70.963 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*115.63 ns\**</span> | <span title="validated on-demand with error">*1.1498 µs\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.9270 ns\**</span> <span title="validated upfront with error">*2.9410 ms\**</span> | <span title="unvalidated">*3.2090 µs\**</span> <span title="validated upfront with error">*2.9233 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.8261 ns\**</span> <span title="validated upfront with error">*810.32 µs\**</span> | <span title="unvalidated">*226.24 ns\**</span> <span title="validated upfront with error">*810.09 µs\**</span> | 1.8731 µs |
| alkahest | <span title="validated on-demand with panic">*3.3241 ns\**</span> | <span title="validated on-demand with panic">*31.564 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*96.15%\**</span> | 27.61% | 53.83% | 58.54% |
| bare | 32.27% | 56.93% | 100.00% | 99.96% | 100.00% |
| bincode | 39.44% | 70.46% | 62.51% | 88.50% | 85.40% |
| borsh | 44.60% | 73.76% | 79.78% | 90.95% | 94.51% |
| bson | 6.01% | 15.15% | 22.00% | 42.05% | 60.44% |
| capnp | 37.86% | † | 44.32% | 63.33% | 70.67% |
| cbor | 13.45% | 28.22% | 32.11% | 61.29% | 72.30% |
| flatbuffers | 7.39% | † | 42.21% | 61.45% | 67.51% |
| nachricht | 4.02% | 35.42% | 79.23% | 84.35% | 85.88% |
| postcard | 55.31% | 66.69% | 96.96% | 95.97% | 95.73% |
| prost | <span title="populate + encode">*6.81%\**</span> <span title="encode">*19.21%\**</span> | 39.39% | 59.70% | 69.50% | 73.70% |
| rkyv | 60.17% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*72.52%\**</span> | 59.69% | 83.71% | 90.20% |
| rmp | 14.41% | 47.18% | 83.93% | 86.81% | 87.75% |
| ron | 2.52% | 9.98% | 24.32% | 48.48% | 57.81% |
| scale | 37.71% | 67.66% | 100.00% | 100.00% | 99.98% |
| serde_json | 6.15% | 17.80% | 21.95% | 45.14% | 55.19% |
| simd-json | 6.15% | 30.52% | 21.42% | 42.95% | 51.73% |
| speedy | 63.25% | 80.77% | 79.25% | 90.67% | 94.36% |
| alkahest | 88.65% | † | 53.37% | 65.49% | 61.94% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.32%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*1.58%\**</span> | <span title="validated on-demand with error">*19.68%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.50%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*7.05%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*54.94%\**</span> | <span title="validated on-demand with panic">*0.72%\**</span> | ‡ |



## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
