#ifndef RTC_H_
#define RTC_H_

//==============================================================================
// Notes
//==============================================================================
// rtc.h

//==============================================================================
// Includes
//==============================================================================
#include "lib.h"

//==============================================================================
// Defines
//==============================================================================
#define RTC_SCHEDULE_MAX_SEC	(36 * 60 * 60) //36 hours
#define RTC_SCHEDULE_MIN_SEC    (1) // 1 second

//==============================================================================
// Types
//==============================================================================
typedef enum RtcScheduleState_t {
	SCHEDULE_UNINITIALIZED =	0,
	SCHEDULE_RUNNING =			2,
}RtcScheduleState_t;

typedef struct RtcSchedule_t RtcSchedule_t;

struct RtcSchedule_t {
	RtcScheduleState_t	State;
	uint32_t			Interval;
	uint32_t			NextTime;
	void				*CallbackFunc;
	RtcSchedule_t		*NextSchedule;
};

typedef enum  RtcWakeInterval_t{
	RTC_WAKE_INTERVAL_250MS	= 512,
	RTC_WAKE_INTERVAL_500MS	= 1024,
	RTC_WAKE_INTERVAL_750MS	= 2048,
	RTC_WAKE_INTERVAL_1S 	= 4096
}RtcWakeInterval_t;

//==============================================================================
// Prototypes
//==============================================================================
void RTC_Init(RtcWakeInterval_t interval);
uint32_t RTC_GetCurrentTime();
uint32_t RTC_GetCurrentTime();
uint16_t RTC_GetCurrentFractionalMs();
uint32_t RTC_GetIntervalSeconds(uint32_t seconds);
uint32_t RTC_GetTimeDiff(uint32_t time);
int64_t RTC_GetTimeDiffMs(uint32_t timeSec, uint32_t ms);
void RTC_ScheduleStart(RtcSchedule_t *schedule, uint32_t seconds, void *callBackFunc);
void RTC_ScheduleTaskHandler();

#endif /* RTC_H_ */