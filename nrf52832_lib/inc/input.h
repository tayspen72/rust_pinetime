#ifndef INPUT_H_
#define INPUT_H_

//==============================================================================
// Notes
//==============================================================================
// input.h

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
typedef enum InputPinState_t{
	INPUT_UNINITIALIZED =	255,
	PRESSED =				0,
	UNPRESSED =				1,
	POWER_ON =				1,
	POWER_OFF =				0,
	PIN_HIGH =				1,
	PIN_LOW =				0,
	INPUT_PAUSED =			254,
}InputPinState_t;

typedef struct Input_t Input_t;

struct Input_t{
	InputPinState_t		State;
	uint8_t				Pin;
	
	struct {
		uint8_t	RisingEdgeOnly:		1;
		uint8_t FallingEdgeOnly:	1;
		uint8_t RealTimeCallback:	1;
	}Options;

	void				*CallBackFunc;
	Input_t				*NextInput;
};

//==============================================================================
// Prototypes
//==============================================================================
void INPUT_Init(Input_t *input);
void INPUT_Pause(Input_t *input, uint8_t pause);
void INPUT_Stop(Input_t* input);
uint8_t INPUT_GetBusy();
void INPUT_ClearQueue();
void Input_SetInterruptFlag(); //For unit tests
void INPUT_TaskHandler();

#endif /* INPUT_H_ */