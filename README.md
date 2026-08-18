# vn-lunar-calendar

**Vietnamese Lunar Calendar Library** — Core in Rust (UTC+7). Other languages call the same conversions through a C ABI, or via the Python / JavaScript packages.

English | [Tiếng Việt](README.vi.md)

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.97.1-orange.svg)
![Python](https://img.shields.io/badge/python-3.15-blue.svg)
![Node](https://img.shields.io/badge/node-26-green.svg)

## Features

- **Solar ↔ Lunar conversion** (1900-2100, UTC+7 Vietnam timezone)
- **24 Solar Terms** (Tiết khí / Jieqi) with exact transition times
- **Vietnamese holidays** (Tết, Vu Lan, Trung Thu, Giỗ Tổ Hùng Vương, etc.)
- **Hoàng đạo** days for a lunar month
- **Zodiac** (12 con giáp) and **Can-Chi** (60-year cycle)
- **C ABI** (`crates/vn-lunar-core/include/vn_lunar.h`) for Go, PHP, C#, …
- **Python** and **JavaScript/WASM** bindings map 1:1 onto the core

## Use from Git

Clone the repo — nothing is published to npm, crates.io, or PyPI.

```bash
git clone https://github.com/FelixTNG/vn-lunar-calendar.git
cd vn-lunar-calendar
```

### JavaScript / TypeScript (recommended)

No Rust. From this repo:

```bash
cd packages/js
npm install
npm run build
```

In your app, depend on the folder (path relative to *your* project):

```json
{
  "dependencies": {
    "vn-lunar-calendar": "file:../vn-lunar-calendar/packages/js"
  }
}
```

```typescript
import { LunarCalendar } from "vn-lunar-calendar";

const cal = new LunarCalendar();
const lunar = cal.solarToLunar(2025, 1, 29);
console.log(lunar.vietnameseName); // "Mùng 1 Tết Ất Tỵ"
console.log(lunar.isTet);          // true
console.log(cal.lunarToSolar(2025, 1, 1, false));
console.log(cal.getJieqi(2025, 2, 4).name);
console.log(cal.getVietnamHolidays(2025));
console.log(cal.getGoodDays(2025, 1));
```

Optional WASM build (needs Rust + wasm-pack): `cd packages/js && npm run build:wasm`.

### Rust

```toml
# Cargo.toml in your project
vn-lunar-core = { git = "https://github.com/FelixTNG/vn-lunar-calendar" }
```

```rust
use vn_lunar_core::{LunarCalendar, SolarDate};

let cal = LunarCalendar::new();
let lunar = cal.solar_to_lunar(2025, 1, 29).unwrap();
println!("{}", lunar.vietnamese_name()); // "Mùng 1 Tết Ất Tỵ"
let solar = cal.lunar_to_solar(2025, 1, 1, false).unwrap();
println!("{}-{:02}-{:02}", solar.year, solar.month, solar.day);
let jieqi = cal.get_jieqi(2025, 2, 4).unwrap();
println!("{}", jieqi.name.vietnamese()); // "Lập Xuân"
```

### Python

Needs a local Rust toolchain, then from this repo:

```bash
cd packages/python
pip install maturin
maturin develop --release
```

```python
from vn_lunar import LunarCalendar

cal = LunarCalendar()
lunar = cal.solar_to_lunar(2025, 1, 29)
print(lunar.vietnamese_name())  # "Mùng 1 Tết Ất Tỵ"
print(lunar.is_tet())           # True
solar = cal.lunar_to_solar(2025, 1, 1, False)
jieqi = cal.get_jieqi(2025, 2, 4)
holidays = cal.get_vietnam_holidays(2025)
good_days = cal.get_good_days(2025, 1)
```

### C / other languages

Build `vn-lunar-core` as `cdylib`/`staticlib` and include
`crates/vn-lunar-core/include/vn_lunar.h`.

## API Reference

### Core Types

| Type | Description |
|------|-------------|
| `SolarDate` | Gregorian date (year, month, day) |
| `LunarDate` | Lunar date with metadata (zodiac, stem-branch, leap flag) |
| `JieQi` | Solar term (name, date, exact UTC time) |
| `Holiday` | Vietnamese holiday (name, solar/lunar date, public flag) |
| `GoodDay` | Auspicious day (lunar date, category, description) |
| `Zodiac` | 12 animals (Rat, Ox, Tiger...) |
| `StemBranch` | Can-Chi (Heavenly Stem + Earthly Branch) |

### Methods

| Method | Input | Output | Description |
|--------|-------|--------|-------------|
| `solar_to_lunar` | year, month, day | `LunarDate` | Gregorian → Lunar |
| `lunar_to_solar` | year, month, day, leap | `SolarDate` | Lunar → Gregorian |
| `get_jieqi` | year, month, day | `JieQi` | Solar term for date |
| `get_vietnam_holidays` | year | `Holiday[]` | All holidays in year |
| `get_good_days` | lunar_year, lunar_month | `GoodDay[]` | Auspicious days in month |
| `get_zodiac` | lunar_year | `Zodiac` | Zodiac animal |
| `get_stem_branch` | lunar_year | `StemBranch` | Can-Chi for year |

## Supported Range

- **Years**: 1900-2100 (200 years)
- **Timezone**: UTC+7 (Vietnam Standard Time)
- **Accuracy**: Astronomical algorithms (Meeus) + Vietnamese calendar rules

## Algorithm

- New moon and solar longitude after **Jean Meeus**, *Astronomical Algorithms*
- Leap month = first month after Đông Chí with no major solar term (trung khí)
- Epoch: 1984 = Giáp Tý for the 60-year cycle
- Civil timezone: UTC+7

## Development

```bash
git clone https://github.com/FelixTNG/vn-lunar-calendar.git
cd vn-lunar-calendar

cargo test -p vn-lunar-core
cargo run -p vn-lunar-core --example quick_test

cd packages/js && npm install && npm test && npm run build
```

## License

MIT © Tú (Felix) Nguyễn ([FelixTNG](https://github.com/FelixTNG/vn-lunar-calendar))

## Acknowledgments

Conversion for Vietnam civil time follows the method published by **Hồ Ngọc Đức**:
[Computing the Vietnamese lunar calendar](https://www.informatik.uni-leipzig.de/~duc/amlich/).

- **Jean Meeus** — *Astronomical Algorithms*
- Vietnamese astronomy community for civil calendar rules