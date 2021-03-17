#ifndef GPIO_H_
#define GPIO_H_

//==============================================================================
// Notes
//==============================================================================
// gpio.h

//==============================================================================
// Includes
//==============================================================================
#include "lib.h"

//==============================================================================
// Defines
//==============================================================================


//==============================================================================
// Types
//==============================================================================
typedef enum GpioPinDirection_t {
	INPUT =		0,
	OUTPUT =	1,
}GpioPinDirection_t;

//==============================================================================
// Prototypes
//==============================================================================
uint8_t GPIO_GetPinState(uint8_t pin);
void GPIO_PinSetup(uint8_t pin, GpioPinDirection_t direction, uint8_t state, uint8_t pullUpDown);
void GPIO_SetPinState(uint8_t pin, uint8_t state);

//==============================================================================
// Implementations
//==============================================================================
static INLINE void GPIO_SetPinStateHigh_INLINE(uint8_t pin){
	NRF_P0->OUTSET = (1 << pin);
}

static INLINE void GPIO_SetPinStateLow_INLINE(uint8_t pin){
	NRF_P0->OUTCLR = (1 << pin);
}

static INLINE void GPIO_SetPinState_INLINE(uint8_t pin, uint8_t state){
	if(state)
		NRF_P0->OUTSET = (1 << pin);
	else
		NRF_P0->OUTCLR = (1 << pin);
}

static INLINE uint8_t GPIO_GetPinState_INLINE(uint8_t pin){
	return NRF_P0->IN & (1 << pin) ? 1 : 0;
}

#endif /* GPIO_H_ */
