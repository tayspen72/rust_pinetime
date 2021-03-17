//==============================================================================
// Notes
//==============================================================================
// softFifo.c

//==============================================================================
// Includes
//==============================================================================
#include "softFifo.h"

//==============================================================================
// Defines
//==============================================================================


//==============================================================================
// Types
//==============================================================================


//==============================================================================
// Private Prototypes
//==============================================================================


//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Public Functions
//==============================================================================
void FIFO_Init(FifoBuffer_t *fifo){
	if(!fifo->State){
		fifo->State = FIFO_READY;
		fifo->Head = 0;
		fifo->Tail = 0;
	}
}

uint16_t FIFO_GetAvailableSize(FifoBuffer_t *fifo){
	FIFO_Init_INLINE(fifo);

	uint16_t availableSize;

	if(fifo->State == FIFO_FULL)
		availableSize = 0;
	else if(fifo->Head > fifo->Tail)
		availableSize = (fifo->Size - fifo->Head) + fifo->Tail;
	else if(fifo->Head < fifo->Tail)
		availableSize = fifo->Tail - fifo->Head;
	else
		availableSize = fifo->Size;

	return availableSize;
}

uint16_t FIFO_GetUsedSize(FifoBuffer_t *fifo){
	FIFO_Init_INLINE(fifo);

	uint16_t usedSize;

	if(fifo->State == FIFO_FULL)
		usedSize = fifo->Size;
	else if(fifo->Head > fifo->Tail)
		usedSize = fifo->Head - fifo->Tail;
	else if(fifo->Head < fifo->Tail)
		usedSize = (fifo->Size - fifo->Tail) + fifo->Head;
	else
		usedSize = 0;

	return usedSize;
}

uint16_t FIFO_WriteData(FifoBuffer_t *fifo, uint8_t *data, uint16_t dataSize){
	FIFO_Init_INLINE(fifo);

	if(dataSize > FIFO_GetAvailableSize(fifo))
		return 0;

	for(uint16_t i = 0; i < dataSize; i++){
		fifo->RawBuffer[fifo->Head++] = data[i];

		if(fifo->Head >= fifo->Size) //Handle run over of buffer
			fifo->Head = 0;

		if(fifo->Head == fifo->Tail)
			fifo->State = FIFO_FULL;
	}

	return dataSize;
}

void FIFO_WriteByte(FifoBuffer_t *fifo, uint8_t data){
	//Check that fifo is Initialized
	FIFO_Init(fifo);
	
	//Check that space is left
	if(fifo->State == FIFO_FULL)
		return;

	fifo->RawBuffer[fifo->Head++] = data;

	if(fifo->Head >= fifo->Size)
		fifo->Head = 0;

	if(fifo->Head == fifo->Tail)
		fifo->State = FIFO_FULL;
}

uint16_t FIFO_ReadByte(FifoBuffer_t *fifo){
	//Check that fifo is Initialized
	FIFO_Init(fifo);
	
	//Check that there is something in buff
	if((fifo->Head == fifo->Tail) && (fifo->State != FIFO_FULL))
		return FIFO_BUFFER_EMPTY;
		
	uint8_t byte = fifo->RawBuffer[fifo->Tail++];
	
	if(fifo->Tail >= fifo->Size)
		fifo->Tail = 0;
		
	//Make sure buffer state is not full
	fifo->State = FIFO_READY;
	
	return byte;
}

void FIFO_ClearBuffer(FifoBuffer_t *fifo, uint16_t eraseSize){
	FIFO_Init_INLINE(fifo);

	if(!eraseSize || eraseSize >= FIFO_GetUsedSize(fifo))
		fifo->Tail = fifo->Head;

	else if((fifo->Tail + eraseSize) < fifo->Size)
		fifo->Tail += eraseSize;
	else
		fifo->Tail = eraseSize - (fifo->Size - fifo->Tail);

	fifo->State = FIFO_READY;
}

void FIFO_ClearBufferEnd(FifoBuffer_t *fifo, uint16_t eraseSize){
	FIFO_Init_INLINE(fifo);

	if(!eraseSize || eraseSize >= FIFO_GetUsedSize(fifo))
		fifo->Tail = fifo->Head;

	else if(fifo->Head > eraseSize)
		fifo->Head -= eraseSize;
	else
		fifo->Head = fifo->Size - (eraseSize - fifo->Head);
		
	fifo->State = FIFO_READY;
}

uint16_t FIFO_GetNextByte(FifoBuffer_t *fifo, uint16_t offset){
	FIFO_Init_INLINE(fifo);

	//Check that there is something in buffer
	if((fifo->Head == fifo->Tail) && (fifo->State != FIFO_FULL))
		return FIFO_BUFFER_EMPTY;

	uint16_t usedSize = FIFO_GetUsedSize(fifo);

	if (offset >= usedSize)
		return FIFO_BUFFER_OVERRUN;

	if(fifo->Tail + offset < fifo->Size)
		return fifo->RawBuffer[fifo->Tail + offset];
	else
		return fifo->RawBuffer[(fifo->Tail + offset) - fifo->Size];
}

uint8_t *FIFO_GetNextBytePointer(FifoBuffer_t *fifo, uint16_t offset){
	FIFO_Init_INLINE(fifo);

	uint16_t usedSize = FIFO_GetUsedSize(fifo);

	if(!usedSize || usedSize < offset)
		return NULL;

	if(fifo->Tail + offset < fifo->Size)
		return &fifo->RawBuffer[fifo->Tail + offset];
	else
		return &fifo->RawBuffer[(fifo->Tail + offset) - fifo->Size];
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================


//==============================================================================
// Interrupt Handler
//==============================================================================
