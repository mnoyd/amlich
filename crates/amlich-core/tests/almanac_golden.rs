use amlich_core::almanac::calc::calculate_day_fortune;
use amlich_core::get_day_info;

/// Tết 2024 (2024-02-10): Giáp Thìn, lunar 1/1/2024
/// chi_index=4, lunar_month=1 → trực=Mãn(2,hung), lục_xung=Tuất, tam_hợp={Tý,Thìn,Thân}
#[test]
fn golden_tet_2024_truc_and_xung_hop() {
    let info = get_day_info(10, 2, 2024);
    let fortune = calculate_day_fortune(
        info.jd,
        &info.canchi.day,
        info.lunar.month,
        &info.canchi.year.can,
        &info.tiet_khi.name,
    );

    assert_eq!(info.canchi.day.chi, "Thìn");
    assert_eq!(info.lunar.day, 1);
    assert_eq!(info.lunar.month, 1);

    assert_eq!(fortune.truc.name, "Mãn");
    assert_eq!(fortune.truc.index, 2);
    assert_eq!(fortune.truc.quality, "hung");

    assert_eq!(fortune.xung_hop.luc_xung, "Tuất");
    assert_eq!(fortune.xung_hop.tam_hop.len(), 3);
    assert!(fortune.xung_hop.tam_hop.contains(&"Tý".to_string()));
    assert!(fortune.xung_hop.tam_hop.contains(&"Thìn".to_string()));
    assert!(fortune.xung_hop.tam_hop.contains(&"Thân".to_string()));
    assert_eq!(fortune.xung_hop.tu_hanh_xung.len(), 4);
}

/// Tết 2025 (2025-01-29): Mậu Tuất, lunar 1/1/2025
/// chi_index=10, lunar_month=1 → trực=Thành(8,cat), lục_xung=Thìn, tam_hợp={Dần,Ngọ,Tuất}
#[test]
fn golden_tet_2025_truc_and_xung_hop() {
    let info = get_day_info(29, 1, 2025);
    let fortune = calculate_day_fortune(
        info.jd,
        &info.canchi.day,
        info.lunar.month,
        &info.canchi.year.can,
        &info.tiet_khi.name,
    );

    assert_eq!(info.canchi.day.chi, "Tuất");
    assert_eq!(info.lunar.day, 1);
    assert_eq!(info.lunar.month, 1);

    assert_eq!(fortune.truc.name, "Thành");
    assert_eq!(fortune.truc.index, 8);
    assert_eq!(fortune.truc.quality, "cat");

    assert_eq!(fortune.xung_hop.luc_xung, "Thìn");
    assert_eq!(fortune.xung_hop.tam_hop.len(), 3);
    assert!(fortune.xung_hop.tam_hop.contains(&"Dần".to_string()));
    assert!(fortune.xung_hop.tam_hop.contains(&"Ngọ".to_string()));
    assert!(fortune.xung_hop.tam_hop.contains(&"Tuất".to_string()));
}

/// New Year 2024 (2024-01-01): Giáp Tý, lunar 20/11/2023
/// chi_index=0, lunar_month=11 → month_chi_index=(11+1)%12=0=Tý
/// trực=(0+12-0)%12=0 → Kiến(0,cat), lục_xung=Ngọ, tam_hợp={Tý,Thìn,Thân}
#[test]
fn golden_new_year_2024_truc_and_xung_hop() {
    let info = get_day_info(1, 1, 2024);
    let fortune = calculate_day_fortune(
        info.jd,
        &info.canchi.day,
        info.lunar.month,
        &info.canchi.year.can,
        &info.tiet_khi.name,
    );

    assert_eq!(info.canchi.day.chi, "Tý");
    assert_eq!(info.lunar.month, 11);

    assert_eq!(fortune.truc.name, "Kiến");
    assert_eq!(fortune.truc.index, 0);
    assert_eq!(fortune.truc.quality, "cat");

    assert_eq!(fortune.xung_hop.luc_xung, "Ngọ");
    assert!(fortune.xung_hop.tam_hop.contains(&"Tý".to_string()));
    assert!(fortune.xung_hop.tam_hop.contains(&"Thìn".to_string()));
    assert!(fortune.xung_hop.tam_hop.contains(&"Thân".to_string()));
}

/// Structural invariant: when day chi == month chi, trực must be Kiến (index 0)
#[test]
fn golden_truc_kien_when_day_chi_equals_month_chi() {
    // lunar month 1 → month_chi_index=2=Dần
    // we need a day with chi=Dần in lunar month 1 of some year
    // Try 2024-02-12: from 2024-02-10 (Thìn=4) +2 days → chi=(4+2)%12=6=Ngọ. Not Dần.
    // Actually compute: need chi=Dần(2) in lunar month 1.
    // From 2024-02-10 (Thìn=4): to Dần(2) we need (2+12-4)%12=10 days forward.
    // 2024-02-10 + 10 = 2024-02-20
    let info = get_day_info(20, 2, 2024);
    let fortune = calculate_day_fortune(
        info.jd,
        &info.canchi.day,
        info.lunar.month,
        &info.canchi.year.can,
        &info.tiet_khi.name,
    );

    // day chi should be Dần, lunar month should be 1
    assert_eq!(
        info.canchi.day.chi, "Dần",
        "expected day chi Dần for 2024-02-20"
    );
    assert_eq!(info.lunar.month, 1, "expected lunar month 1");
    assert_eq!(fortune.truc.index, 0, "trực Kiến when day chi == month chi");
    assert_eq!(fortune.truc.name, "Kiến");
}
