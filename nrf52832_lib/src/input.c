//==============================================================================
// Notes
//==============================================================================
// input.c

//==============================================================================
// Includes
//==============================================================================
#include "input.h"
#include "gpio.h"

//==============================================================================
// Defines
//==============================================================================
#ifndef INPUT_QUEUE_SIZE
	#define INPUT_QUEUE_SIZE	10
#endif

//==============================================================================
// Types
//==============================================================================
typedef void (*inputFunction)(Input_t *sender);
typedef void (*inputFunctionRealTime)();

typedef struct InputQueueEntry_t {
	uint16_t	Flags;
	uint32_t	PinStates;
}InputQueueEntry_t;

//==============================================================================
// Private Prototypes
//==============================================================================
static void input_addInput(Input_t* input);
static void input_callCallbackFunc(Input_t *input);
static uint8_t input_exists(Input_t* input);
static uint8_t input_getExtInt(Input_t *input);
static void input_initMcu(Input_t *input);
static void input_stopMcu(Input_t* input);

//==============================================================================
// Variables
//==============================================================================
static Input_t *_firstInput = NULL;
static volatile InputQueueEntry_t _queue[INPUT_QUEUE_SIZE];
static volatile uint8_t _head = 0;
static volatile uint8_t _tail = 0;
static volatile uint8_t _full = false;
static inputFunctionRealTime _realTimeCallbacks[INPUT_EXT_INT_SIGNAL_COUNT] = { NULL };
static uint32_t _realTimeCallbackPinsEnabled = 0;

//==============================================================================
// Public Functions
//==============================================================================
void INPUT_Init(Input_t *input){
	//This helps eliminate issues with pin states being captured
	//before their interrupt has been setup. 
	while(_head != _tail)
		INPUT_TaskHandler();
	
	input_addInput(input);
	
	input_initMcu(input);

	//Set initial pin state
	input->State = (InputPinState_t) GPIO_GetPinState(input->Pin);

	if(input->Options.RealTimeCallback){
		//for this external interrupt signal, define the real time callback
		uint8_t extInt = input_getExtInt(input);
		_realTimeCallbacks[extInt] = input->CallBackFunc;
	}
}

void INPUT_Pause(Input_t *input, uint8_t pause){
	if(!input_exists(input))
		return;
	
	if(pause)
		input->State = INPUT_PAUSED;
	
	input->State = (InputPinState_t)MCU_GetPinState(input->Pin);
}

void INPUT_Stop(Input_t* input){
	if(!input_exists(input))
		return;
	
	input_stopMcu(input);
}

uint8_t INPUT_GetBusy(){
	if(_head == _tail && !_full)
		return false;
	
	return true;
}

void INPUT_ClearQueue(){
	_head = 0;
	_tail = 0;
	_full = false;
}

//==============================================================================
// Private Functions
//==============================================================================
static void input_addInput(Input_t* input){
	if(input_exists(input))
		return;
	
	Input_t *lastInput = _firstInput;
	input->NextInput = NULL;

	if(lastInput == NULL)
	_firstInput = input;
	else{
		while(lastInput->NextInput)
		lastInput = lastInput->NextInput;

		lastInput->NextInput = input;
	}
}

static void input_callCallbackFunc(Input_t *input){
	inputFunction func = (inputFunction)input->CallBackFunc;
	
	//Call the callback function if not null
	if(func)
		func(input);
}

static uint8_t input_exists(Input_t* input){
	Input_t* tmpInput = _firstInput;
	
	while(tmpInput){
		if(tmpInput == input)
			return true;
		
		tmpInput = tmpInput->NextInput;
	}
	
	return false;
}

static uint8_t input_getExtInt(Input_t *input){
	//the GPTIOE peripheral allows for any pin to be mapped to any one of
	//	the 8 available event flags. Use this to find the next available
	//	event flag
	uint8_t pin = 32;
	uint8_t eventNumber = 0;
	for(eventNumber = 0; eventNumber < 8; eventNumber++){
		//check mode for a disabled (0) value
		//if found a pin that is not enabled, use this event number
		if(!(NRF_GPIOTE->CONFIG[eventNumber] & 0x3))
			break;

		//if not disabled, check to see if this pin has already been mapped
		uint8_t pin = (NRF_GPIOTE->CONFIG[eventNumber] & 0x1F00) >> 8;
		if(pin == input->Pin)
			break;
	}

	return eventNumber;
}

static void input_initMcu(Input_t *input){
	// Setup input Pin
	GPIO_PinSetup(input->Pin, INPUT, 1, true);

	//Disable input interrupts during config
	NVIC_DisableIRQ(GPIOTE_IRQn);

	//get the event number
	uint8_t event = input_getExtInt(input);

	// disable whatever was there before
	NRF_GPIOTE->CONFIG[event] = 0;

	//define the pin this event will come from
	NRF_GPIOTE->CONFIG[event] |= (input->Pin << 8);

	//Enable Rise and Fall Detect (0=none; 1=rise; 2=fall; 3=both; 4=high; 5=low)
	uint8_t polarity = input->Options.FallingEdgeOnly ? 0x02 : input->Options.RisingEdgeOnly ? 0x01 : 0x03;
	NRF_GPIOTE->CONFIG[event] |= (polarity << 16);

	// Enable
	NRF_GPIOTE->CONFIG[event] |= 0x1;
	NRF_GPIOTE->INTENSET |= (1 << event);

	//Clear the interrupt flag
	NRF_GPIOTE->EVENTS_IN[event] = 0;

	//Enable Input Interrupts
	NVIC_EnableIRQ(GPIOTE_IRQn);
}

static void input_stopMcu(Input_t* input){
	//disable interrupts for this interrupt signal

	//get external interrupt number for this pin
	uint32_t extInt = input_getExtInt(input);
	input->State = INPUT_PAUSED;
	
	// Disable all interrupts on this pin
	NRF_GPIOTE->CONFIG[extInt] &= ~0x3;
	
	_realTimeCallbacks[extInt] = NULL;
}

//==============================================================================
// Task Handler
//==============================================================================
void INPUT_TaskHandler(){
	if(_head == _tail && !_full)
		return;
		
	Input_t *input = _firstInput;
	
	while(input){
		if(input->State == INPUT_PAUSED){
			input = input->NextInput;
			continue;
		}
		
		uint8_t extInt = input_getExtInt(input);
		if(!(_queue[_tail].Flags & (1 << extInt))){
			input = input->NextInput;
			continue;
		}
		
		if(input->Options.FallingEdgeOnly || input->Options.RisingEdgeOnly)
			input->State = input->Options.FallingEdgeOnly ? 0 : 1;
		else
			input->State = _queue[_tail].PinStates & (1 << input->Pin) ? 1 : 0;
		
		input_callCallbackFunc(input);
		
		input = input->NextInput;
	}
	
	if(++_tail >= INPUT_QUEUE_SIZE)
		_tail = 0;
	
	_full = false;
}

//==============================================================================
// Interrupt Handler
//==============================================================================
void GPIOTE_IRQHandler(){
	//determine which event number caused the interrupt
	uint8_t flags = 0, eventNumber = 0;
	for(uint8_t i = 0; i < INPUT_EXT_INT_SIGNAL_COUNT; i++){
		if(NRF_GPIOTE->EVENTS_IN[i]){
			//save the interrupt pin state
			flags |= (1 << i);
			
			//clear the interrupt
			NRF_GPIOTE->EVENTS_IN[i] = 0;

			//track the event number to make things easier
			eventNumber = i;
			break;
		}
	}

	// If the pin has a real time callback, use it, do not add to queue
	if(_realTimeCallbacks[eventNumber]){
		_realTimeCallbacks[eventNumber](NULL);
		return;
	}
			
	if(_full)
		return;
		
	_queue[_head].PinStates = NRF_P0->IN;
	
	_queue[_head].Flags = flags;
	
	// Move head and check for overruns
	if(++_head >= INPUT_QUEUE_SIZE)
		_head = 0;
			
	if(_head == _tail)
		_full = true;
}