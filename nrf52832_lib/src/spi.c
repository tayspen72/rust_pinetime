//==============================================================================
// Notes
//==============================================================================
// spi.c

//==============================================================================
// Includes
//==============================================================================
#include "spi.h"
#include "gpio.h"
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
static void spi_initMcu(SpiLine_t *spiLine);
static void spi_setOptionsMcu(SpiLine_t *spiLine);
uint8_t spi_readWriteByteMcu(SpiLine_t *spiLine, uint8_t byte);

//==============================================================================
// Variables
//==============================================================================
static SpiLine_t *_firstSpiLine = NULL;

//==============================================================================
// Public Functions
//==============================================================================
void SPI_Init(SpiLine_t *spiLine){
	SpiLine_t *s = _firstSpiLine;
	spiLine->NextSpiLine = NULL;

	uint8_t add = true;
	//Add to SPI Linked List
	while(s){
        if (s == spiLine)
            add = false;

		if(!s->NextSpiLine)
			break;
			
        s = s->NextSpiLine;
	}
	
	if(!s)
		_firstSpiLine = spiLine;
	else if(add)
		s->NextSpiLine = spiLine;
	
	//If this is not for a unit test, setup hardware for SPI
	if(!spiLine->UnitTestCallback)
		spi_initMcu(spiLine);
	
	if(spiLine->RxBuffer)
		FIFO_Init_INLINE(spiLine->RxBuffer);
	
	spiLine->State = SPI_READY;
}

void SPI_TxData(SpiLine_t *spiLine, uint16_t dataSize, uint8_t *data){
	if(spiLine->State != SPI_READY)
		SPI_Init(spiLine);
		
	SPI_SetSlaveSel(spiLine, 0);
	
	for(uint16_t i = 0; i < dataSize; i++)
		spi_readWriteByteMcu(spiLine, data[i]);
}

void SPI_RxData(SpiLine_t *spiLine, uint16_t dataSize){
	if(spiLine->RxBuffer == NULL)
		return;
	else if(spiLine->State != SPI_READY)
		SPI_Init(spiLine);

	SPI_SetSlaveSel(spiLine, 0);
	
	for(uint16_t i = 0; i < dataSize; i++){
		uint8_t rx = spi_readWriteByteMcu(spiLine, 0xFF);
		FIFO_WriteByte_INLINE(spiLine->RxBuffer, rx);
	}
}

void SPI_TxRxData(SpiLine_t *spiLine, uint16_t txDataSize, uint8_t *data, uint16_t rxBytes){
	if(spiLine->RxBuffer == NULL)
		return;
	else if(spiLine->State != SPI_READY)
		SPI_Init(spiLine);
	
	SPI_SetSlaveSel(spiLine, 0);
	
	for(uint16_t i = 0; i < txDataSize; i++){
		uint8_t rx = spi_readWriteByteMcu(spiLine, data[i]);
		if(!rxBytes)
			FIFO_WriteByte_INLINE(spiLine->RxBuffer, rx);	
	}
	
	for(uint16_t i = 0; i < rxBytes; i++){
		uint8_t rx = spi_readWriteByteMcu(spiLine, 0xFF);
		FIFO_WriteByte_INLINE(spiLine->RxBuffer, rx);
	}
}

//==============================================================================
// Private Functions
//==============================================================================
static void spi_initMcu(SpiLine_t *spiLine){
	//ensure the peripheral is disabled before configuring
	NRF_SPI0->ENABLE = 0;

	// Setup MOSI I/O Line
	GPIO_PinSetup(spiLine->MosiPin, OUTPUT, 0, false);
	NRF_SPI0->PSEL.MOSI = spiLine->MosiPin;

	//Setup MISO I/O Line if needed
	if(spiLine->RxBuffer != NULL){
		GPIO_PinSetup(spiLine->MisoPin, INPUT, 1, true);
		NRF_SPI0->PSEL.MISO = spiLine->MisoPin;
	}
	
	// Setup SCLK I/O Line
	GPIO_PinSetup(spiLine->SclkPin, OUTPUT, 0, false);
	NRF_SPI0->PSEL.SCK = spiLine->SclkPin;
	
	// Setup Slave SEL I/O Line
	GPIO_PinSetup(spiLine->SlaveSelPin, OUTPUT, 1, false);

	spi_setOptionsMcu(spiLine);

	// Enable SPI
	NRF_SPI0->ENABLE = true;
}

static void spi_setOptionsMcu(SpiLine_t *spiLine)
{
	//define frequency
	if(spiLine->Baud <= 1000000)
		NRF_SPI0->FREQUENCY = 0x10000000;
	else if(spiLine->Baud <= 2000000)
		NRF_SPI0->FREQUENCY = 0x20000000;
	else if(spiLine->Baud <= 4000000)
		NRF_SPI0->FREQUENCY = 0x40000000;
	else
		NRF_SPI0->FREQUENCY = 0x80000000;

	//define config
	uint32_t config = 0;
	if(spiLine->Endian == LSBF)
		config |= 0x1;
	if(spiLine->Options.CPHA)
		config |= 0x2;
	if(spiLine->Options.CPOL)
		config |= 0x4;
	NRF_SPI0->CONFIG = config;
}

uint8_t spi_readWriteByteMcu(SpiLine_t *spiLine, uint8_t byte){
	NRF_SPI0->TXD = byte;
	while(!NRF_SPI0->EVENTS_READY);

	return spiLine->RxBuffer ? NRF_SPI0->RXD : 0xFF;
}

//==============================================================================
// Task Handler
//==============================================================================


//==============================================================================
// Interrupt Handler
//==============================================================================
