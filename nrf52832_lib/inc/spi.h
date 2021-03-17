#ifndef SPI_H_
#define SPI_H_

//==============================================================================
// Notes
//==============================================================================
// spi.h

//==============================================================================
// Includes
//==============================================================================
#include "lib.h"
#include "softFifo.h"

//==============================================================================
// Defines
//==============================================================================


//==============================================================================
// Types
//==============================================================================
typedef enum SpiEndian_t {
	LSBF,
	MSBF
}SpiEndian_t;

typedef enum SpiLineState_t {
	SPI_UNINITIALIZED,
	SPI_READY,
}SpiLineState_t;

typedef struct SpiLine_t SpiLine_t;

struct SpiLine_t
{
	SpiLineState_t		State;
	
	struct {
		uint8_t	CPOL:			1;
		uint8_t CPHA:			1;
	} Options;
	
	uint8_t				MisoPin;
	uint8_t				MisoPad;
	uint8_t				MosiPin;
	uint8_t				MosiPad;
	uint8_t				SclkPin;
	uint8_t				SlaveSelPin;
	uint32_t			Baud;
	SpiEndian_t			Endian;
	FifoBuffer_t*		 RxBuffer;
	void*				UnitTestCallback;
	void*				UnitTestSlaveSelCallback;
	SpiLine_t*			NextSpiLine;
};

//==============================================================================
// Prototypes
//==============================================================================
void SPI_Init(SpiLine_t *spiLine);
void SPI_TxData(SpiLine_t *spiLine, uint16_t dataSize, uint8_t *data);
void SPI_RxData(SpiLine_t *spiLine, uint16_t dataSize);
void SPI_TxRxData(SpiLine_t *spiLine, uint16_t txDataSize, uint8_t *data, uint16_t rxBytes);
void SPI_SetSlaveSel(SpiLine_t *spiLine, uint8_t state);

#endif /* SPI_H_ */