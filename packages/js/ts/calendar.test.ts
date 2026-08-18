import { describe, expect, it } from "vitest";
import { LunarCalendar } from "./calendar.ts";

describe("LunarCalendar", () => {
  const cal = new LunarCalendar();

  it("maps Tết 2025", () => {
    const lunar = cal.solarToLunar(2025, 1, 29);
    expect(lunar).toMatchObject({ year: 2025, month: 1, day: 1, leap: false, isTet: true });
    expect(lunar.stemBranch).toBe("Ất Tỵ");
    expect(cal.lunarToSolar(2025, 1, 1, false)).toEqual({ year: 2025, month: 1, day: 29 });
  });

  it("uses Vietnam Tết 2007", () => {
    const lunar = cal.solarToLunar(2007, 2, 17);
    expect(lunar).toMatchObject({ year: 2007, month: 1, day: 1 });
  });

  it("finds leap months", () => {
    expect(cal.leapMonth(2020)).toBe(4);
    expect(cal.leapMonth(2023)).toBe(2);
    expect(cal.leapMonth(2025)).toBe(6);
    expect(cal.leapMonth(2028)).toBe(5);
  });

  it("places Lập Xuân 2025 on 3 February", () => {
    const jieqi = cal.getJieqi(2025, 2, 3);
    expect(jieqi.name).toBe("Lập Xuân");
    expect(jieqi.solarDate).toEqual({ year: 2025, month: 2, day: 3 });
    expect(cal.getJieqi(2025, 2, 4).solarDate).toEqual({ year: 2025, month: 2, day: 3 });
  });
});
