#ifndef SOFT_FIFO_H_
#define SOFT_FIFO_H_

//==============================================================================
// Notes
//==============================================================================
// softFifo.h

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
typedef enum FifoState_t {
	FIFO_NOT_INITIALIZED =	0,
	FIFO_READY =		1,
	FIFO_FULL =		2,
}FifoState_t;

typedef struct FifoBuffer_t {
	FifoState_t		State;
	uint16_t		Size;
	uint16_t		Head;
	uint16_t		Tail;
	uint8_t			*RawBuffer;
}FifoBuffer_t;

//==============================================================================
// Prototypes
//==============================================================================
void FIFO_Init(FifoBuffer_t *fifo);
uint16_t FIFO_GetAvailableSize(FifoBuffer_t *fifo);
uint16_t FIFO_GetUsedSize(FifoBuffer_t *fifo);
uint16_t FIFO_WriteData(FifoBuffer_t *fifo, uint8_t *data, uint16_t dataSize);
void FIFO_WriteByte(FifoBuffer_t *fifo, uint8_t data);
uint16_t FIFO_ReadByte(FifoBuffer_t *fifo);
void FIFO_ClearBuffer(FifoBuffer_t *fifo, uint16_t eraseSize);
void FIFO_ClearBufferEnd(FifoBuffer_t *fifo, uint16_t eraseSize);
uint16_t FIFO_GetNextByte(FifoBuffer_t *fifo, uint16_t offset);
uint8_t *FIFO_GetNextBytePointer(FifoBuffer_t *fifo, uint16_t offset);

//==============================================================================
// Macros
//==============================================================================
#define FIFO_BUFFER_EMPTY	0x100
#define FIFO_BUFFER_OVERRUN 0x200
#define FIFO_CREATE_NEW(name, size) \
static uint8_t name##_rawBuffer[size]; \
static FifoBuffer_t name = { \
	.State = FIFO_NOT_INITIALIZED, \
	.Size = size, \
	.Head = 0, \
	.Tail = 0, \
	.RawBuffer = &name##_rawBuffer[0], \
}

//==============================================================================
// Inline Functions
//==============================================================================
static INLINE void FIFO_Init_INLINE(FifoBuffer_t *fifo)
{
	if(!fifo->State)
	{
		fifo->State = FIFO_READY;
		fifo->Head = 0;
		fifo->Tail = 0;
	}
}

static INLINE void FIFO_WriteByte_INLINE(FifoBuffer_t *fifo, uint8_t data)
{
	//Check that fifo is Initialized
	FIFO_Init_INLINE(fifo);

	//Check that space is left
	if(fifo->State == FIFO_FULL)
		return;

	fifo->RawBuffer[fifo->Head++] = data;

	if(fifo->Head >= fifo->Size)
		fifo->Head = 0;

	if(fifo->Head == fifo->Tail)
		fifo->State = FIFO_FULL;
}

static INLINE uint16_t FIFO_ReadByte_INLINE(FifoBuffer_t *fifo)
{
	//Check that fifo is Initialized
	FIFO_Init_INLINE(fifo);
	
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

#endif /* SOFT_FIFO_H_ */