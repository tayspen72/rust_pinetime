#ifndef LIB_H_
#define LIB_H_

//==============================================================================
// Notes
//==============================================================================
// rtc.h

//==============================================================================
// Includes
//==============================================================================
#include "nrf52.h"
#include "stdint.h"

//==============================================================================
// Defines
//==============================================================================
#define INLINE						inline __attribute__((always_inline))

#define true						((uint8_t) 1)
#define false						((uint8_t) 0)
#define NULL						0
#define INPUT_EXT_INT_SIGNAL_COUNT	8

//==============================================================================
// Types
//==============================================================================
typedef enum SerialCommsPort_t {
	COMM0 =			0,
	COMM1 =			1,
	COMM2 =			2,
	COMM3 =			3,
	COMM4 =			4,
	COMM5 =			5,
	COMM6 =			6,
	COMM7 =			7,
	COMM_USB =		252,
}SerialCommsPort_t;

//==============================================================================
// Prototypes
//==============================================================================


#endif /* LIB_H_ */
