# vn-lunar-calendar

**Thư viện lịch âm Việt Nam** — Core viết bằng Rust (UTC+7). Ngôn ngữ khác gọi cùng phép đổi ngày qua C ABI, hoặc qua gói Python / JavaScript.

[English](README.md) | Tiếng Việt

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.97.1-orange.svg)
![Python](https://img.shields.io/badge/python-3.15-blue.svg)
![Node](https://img.shields.io/badge/node-26-green.svg)

## Tính năng

- **Đổi Dương ↔ Âm** (1900–2100, múi giờ Việt Nam UTC+7)
- **24 tiết khí** với thời điểm chuyển tiết
- **Ngày lễ Việt Nam** (Tết, Vu Lan, Trung Thu, Giỗ Tổ Hùng Vương, …)
- **Ngày hoàng đạo** trong một tháng âm
- **12 con giáp** và **can chi** (chu kỳ 60 năm)
- **C ABI** (`crates/vn-lunar-core/include/vn_lunar.h`) cho Go, PHP, C#, …
- Binding **Python** và **JavaScript/WASM** map 1:1 với core

## Dùng từ Git

Clone repo — không publish lên npm, crates.io hay PyPI.

```bash
git clone https://github.com/FelixTNG/vn-lunar-calendar.git
cd vn-lunar-calendar
```

### JavaScript / TypeScript (khuyên dùng)

Không cần Rust. Trong repo này:

```bash
cd packages/js
npm install
npm run build
```

Trong app của bạn, trỏ dependency vào thư mục (đường dẫn tương đối so với *project của bạn*):

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

WASM tùy chọn (cần Rust + wasm-pack): `cd packages/js && npm run build:wasm`.

### Rust

```toml
# Cargo.toml trong project của bạn
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

Cần Rust toolchain trên máy, rồi trong repo này:

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

### C / ngôn ngữ khác

Build `vn-lunar-core` dạng `cdylib`/`staticlib` và include
`crates/vn-lunar-core/include/vn_lunar.h`.

## Tham chiếu API

### Kiểu dữ liệu

| Kiểu | Mô tả |
|------|--------|
| `SolarDate` | Ngày dương (năm, tháng, ngày) |
| `LunarDate` | Ngày âm kèm metadata (con giáp, can chi, cờ nhuận) |
| `JieQi` | Tiết khí (tên, ngày, thời điểm UTC) |
| `Holiday` | Ngày lễ Việt Nam (tên, ngày dương/âm, cờ ngày nghỉ) |
| `GoodDay` | Ngày hoàng đạo (ngày âm, nhóm, mô tả) |
| `Zodiac` | 12 con giáp (Tý, Sửu, Dần…) |
| `StemBranch` | Can chi (Thiên can + Địa chi) |

### Phương thức

| Phương thức | Đầu vào | Đầu ra | Mô tả |
|-------------|----------|--------|--------|
| `solar_to_lunar` | năm, tháng, ngày | `LunarDate` | Dương → Âm |
| `lunar_to_solar` | năm, tháng, ngày, nhuận | `SolarDate` | Âm → Dương |
| `get_jieqi` | năm, tháng, ngày | `JieQi` | Tiết khí của ngày |
| `get_vietnam_holidays` | năm | `Holiday[]` | Các ngày lễ trong năm |
| `get_good_days` | năm âm, tháng âm | `GoodDay[]` | Ngày hoàng đạo trong tháng |
| `get_zodiac` | năm âm | `Zodiac` | Con giáp |
| `get_stem_branch` | năm âm | `StemBranch` | Can chi của năm |

## Phạm vi hỗ trợ

- **Năm**: 1900–2100 (200 năm)
- **Múi giờ**: UTC+7 (giờ Việt Nam)
- **Độ chính xác**: thuật toán thiên văn (Meeus) + quy tắc lịch dân sự Việt Nam

## Thuật toán

- Sóc và kinh độ mặt trời theo **Jean Meeus**, *Astronomical Algorithms*
- Tháng nhuận = tháng đầu sau Đông Chí không có trung khí
- Gốc: 1984 = Giáp Tý cho chu kỳ 60 năm
- Múi giờ dân sự: UTC+7

## Phát triển

```bash
git clone https://github.com/FelixTNG/vn-lunar-calendar.git
cd vn-lunar-calendar

cargo test -p vn-lunar-core
cargo run -p vn-lunar-core --example quick_test

cd packages/js && npm install && npm test && npm run build
```

## Giấy phép

MIT © Tú (Felix) Nguyễn ([FelixTNG](https://github.com/FelixTNG/vn-lunar-calendar))

## Ghi nhận

Phép đổi ngày theo giờ dân sự Việt Nam theo phương pháp của **Hồ Ngọc Đức**:
[Computing the Vietnamese lunar calendar](https://www.informatik.uni-leipzig.de/~duc/amlich/).

- **Jean Meeus** — *Astronomical Algorithms*
- Cộng đồng thiên văn Việt Nam về quy tắc lịch dân sự
