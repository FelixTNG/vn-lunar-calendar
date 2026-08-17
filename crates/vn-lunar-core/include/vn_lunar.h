#ifndef VN_LUNAR_H
#define VN_LUNAR_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    int32_t year;
    uint8_t month;
    uint8_t day;
    uint8_t _pad;
} VnSolarDate;

typedef struct {
    int32_t year;
    uint8_t month;
    uint8_t day;
    uint8_t leap; /* 1 = leap month */
} VnLunarDate;

/*
 * Error codes (also returned by the convert functions):
 *  0 success
 *  1 invalid Gregorian date
 *  2 year out of range (1900-2100)
 *  3 invalid lunar month
 *  4 invalid lunar day
 *  5 leap month does not exist
 *  6 new moon calculation failed
 *  7 solar term calculation failed
 *  8 null output pointer
 */

int32_t vn_lunar_solar_to_lunar(
    int32_t year,
    uint8_t month,
    uint8_t day,
    VnLunarDate *out,
    int32_t *out_err
);

int32_t vn_lunar_lunar_to_solar(
    int32_t year,
    uint8_t month,
    uint8_t day,
    uint8_t leap,
    VnSolarDate *out,
    int32_t *out_err
);

/* 1-12 if the lunar year has a leap month, 0 if not, negative on error */
int32_t vn_lunar_leap_month(int32_t lunar_year);

const char *vn_lunar_version(void);

#ifdef __cplusplus
}
#endif

#endif /* VN_LUNAR_H */
