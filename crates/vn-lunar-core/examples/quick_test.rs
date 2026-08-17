use vn_lunar_core::LunarCalendar;

fn main() {
    let cal = LunarCalendar::new();

    let tet_2025 = cal.solar_to_lunar(2025, 1, 29).unwrap();
    println!("Tết 2025: {}", tet_2025.vietnamese_name());
    println!("Zodiac: {}", tet_2025.zodiac.vietnamese());
    println!("Stem-Branch: {}", tet_2025.stem_branch.vietnamese());
    println!("Is Tết: {}", tet_2025.is_tet());

    let tet_2024 = cal.solar_to_lunar(2024, 2, 10).unwrap();
    println!("\nTết 2024: {}", tet_2024.vietnamese_name());

    let solar = cal.lunar_to_solar(2025, 1, 1, false).unwrap();
    println!(
        "\nLunar 2025-01-01 -> Solar: {}-{:02}-{:02}",
        solar.year, solar.month, solar.day
    );

    println!("Leap month 2023: {:?}", cal.leap_month(2023));

    let lap_xuan = cal.get_jieqi(2025, 2, 4).unwrap();
    println!("\nLập Xuân 2025: {} ({})", lap_xuan.name.vietnamese(), lap_xuan.solar_date);

    let holidays = cal.get_vietnam_holidays(2025).unwrap();
    println!("\nPublic holidays 2025:");
    for h in holidays.iter().filter(|h| h.is_public_holiday) {
        println!(
            "  {} - {} (ÂM: {})",
            h.solar_date,
            h.name.vietnamese(),
            h.lunar_date.vietnamese_name()
        );
    }
}
