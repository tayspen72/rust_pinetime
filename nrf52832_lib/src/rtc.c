//==============================================================================
// Notes
//==============================================================================
// rtc.c

//==============================================================================
// Includes
//==============================================================================
#include "rtc.h"

//==============================================================================
// Defines
//==============================================================================


//==============================================================================
// Types
//==============================================================================
typedef void (*scheduleCbFunction)(RtcSchedule_t *schedule);

//==============================================================================
// Private Prototypes
//==============================================================================
static void rtc_initMcu(uint8_t enable);
static inline uint16_t rtc_getFracTimeMcu();
static void rtc_scheduleAdd(RtcSchedule_t *schedule);
static uint8_t rtc_scheduleExists(RtcSchedule_t *schedule);
static uint32_t rtc_scheduleGetNextTime(uint32_t interval);

//==============================================================================
// Variables
//==============================================================================
static uint8_t _initialized = false;
static volatile uint32_t _seconds = 0;
static volatile uint32_t _fraction = 0;
static RtcWakeInterval_t _interval;
static RtcSchedule_t *_firstSchedule = NULL;

//==============================================================================
// Public Functions
//==============================================================================
void RTC_Init(RtcWakeInterval_t interval){
	_interval = interval;

	rtc_initMcu(true);
	
	_initialized = true;
}

uint32_t RTC_GetCurrentTime(){
	return _seconds;
}

uint16_t RTC_GetCurrentFractionalMs(){
	float frac = rtc_getFracTimeMcu();
	frac += _fraction;
	return frac * ((float)1000 / (float) RTC_WAKE_INTERVAL_1S);
}

uint32_t RTC_GetIntervalSeconds(uint32_t seconds)
{
	seconds = (seconds > RTC_SCHEDULE_MAX_SEC) ? RTC_SCHEDULE_MAX_SEC : seconds;
	seconds = (seconds < RTC_SCHEDULE_MIN_SEC) ? RTC_SCHEDULE_MIN_SEC : seconds;
	return seconds;
}

uint32_t RTC_GetTimeDiff(uint32_t time){
	if(time > _seconds)
		return 0;
		
	return _seconds - time;
}

int64_t RTC_GetTimeDiffMs(uint32_t timeSec, uint32_t ms){
	int64_t seconds = (int64_t)RTC_GetCurrentTime();
	int64_t fraction = (int64_t)RTC_GetCurrentFractionalMs();

	return (int64_t)(((seconds - timeSec) * 1000) - (int64_t)ms + (int64_t)fraction);
}

void RTC_ScheduleStart(RtcSchedule_t *schedule, uint32_t seconds, void *callBackFunc){
	if(!_initialized)
		RTC_Init(RTC_WAKE_INTERVAL_1S);
		
	rtc_scheduleAdd(schedule);

	seconds = RTC_GetIntervalSeconds(seconds);
	schedule->Interval = seconds;
	schedule->CallbackFunc = callBackFunc;
	schedule->NextTime = rtc_scheduleGetNextTime(seconds);
	schedule->State = SCHEDULE_RUNNING;
}

//==============================================================================
// Private Functions
//==============================================================================
static void rtc_initMcu(uint8_t enable){			
	if(enable){
		//start the RTC
		NRF_CLOCK->LFCLKSRC = 0x01;	//configured for external XTAL on RTC
		NRF_CLOCK->TASKS_LFCLKSTART = 1;	
		while(!NRF_CLOCK->EVENTS_LFCLKSTARTED);
	
		//Disable RTC
		NRF_RTC0->TASKS_STOP = 1;
	
		//prescale by 8 : 32768 / 8 = 4096 Hz
		NRF_RTC0->PRESCALER = 7;
		
		//define the wake interval
		NRF_RTC0->CC[0] = _interval;
		
		//connect the interrupt event signal on CC[0] compare match
		NRF_RTC0->INTENSET |= 0x00010000;
	
		NVIC_EnableIRQ(RTC0_IRQn);
	
		//Enable RTC
		NRF_RTC0->TASKS_START = 1;
	}
	else{
		NRF_RTC0->TASKS_STOP = 1;
		NRF_RTC0->TASKS_CLEAR = 0;
		_initialized = false;
	}

	//Set current time to 0
	_seconds = 0;
	_fraction = 0;
}

static inline uint16_t rtc_getFracTimeMcu(){
	//the internals to the MCU ensure a safe read
	return NRF_RTC0->COUNTER;
}

static void rtc_scheduleAdd(RtcSchedule_t *schedule){
	if(rtc_scheduleExists(schedule))
		return;
	
	RtcSchedule_t *lastSchedule = _firstSchedule;
	schedule->NextSchedule = NULL;
	
	if(lastSchedule == NULL)
		_firstSchedule = schedule;
	else{
		while(lastSchedule->NextSchedule)
			lastSchedule = lastSchedule->NextSchedule;
		
		lastSchedule->NextSchedule = schedule;
	}
}

static uint8_t rtc_scheduleExists(RtcSchedule_t *schedule){
	RtcSchedule_t *s = _firstSchedule;

	if(s == NULL)
		return false;
	
	for(; s; s = s->NextSchedule){
		if(s == schedule)
			return true;
	}
	
	return false;
}

static uint32_t rtc_scheduleGetNextTime(uint32_t interval){
	if(interval > RTC_SCHEDULE_MAX_SEC)
		interval = RTC_SCHEDULE_MAX_SEC;

	return RTC_GetCurrentTime() + interval;
}

//==============================================================================
// Task Handler
//==============================================================================
void RTC_ScheduleTaskHandler(){
	static uint32_t lastSeconds;

	uint32_t seconds = RTC_GetCurrentTime();
	
	if(lastSeconds == seconds)
		return;
	
	RtcSchedule_t *s = _firstSchedule;
	while (s){
		if(s->State == SCHEDULE_RUNNING && s->NextTime <= seconds)
		{
			scheduleCbFunction func = (scheduleCbFunction)s->CallbackFunc;
			s->NextTime = rtc_scheduleGetNextTime(s->Interval);
			if(func)
				func(s);
		}
		s = s->NextSchedule;
	}
	
	lastSeconds = seconds;
}

//==============================================================================
// Interrupt Handler
//==============================================================================
void RTC0_IRQHandler(void){
	if(NRF_RTC0->EVENTS_COMPARE[0]){
		_fraction += _interval;
		if(_fraction >= RTC_WAKE_INTERVAL_1S)	{
			_seconds += (_fraction / RTC_WAKE_INTERVAL_1S);
			_fraction = 0;
		}
			
		NRF_RTC0->EVENTS_COMPARE[0] = 0;
		NRF_RTC0->TASKS_CLEAR = 1;
	}
}