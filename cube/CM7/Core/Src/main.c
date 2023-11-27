/* USER CODE BEGIN Header */
/**
	******************************************************************************
	* @file           : main.c
	* @brief          : Main program body
	******************************************************************************
	* @attention
	*
	* Copyright (c) 2023 STMicroelectronics.
	* All rights reserved.
	*
	* This software is licensed under terms that can be found in the LICENSE file
	* in the root directory of this software component.
	* If no LICENSE file comes with this software, it is provided AS-IS.
	*
	******************************************************************************
	*/
/* USER CODE END Header */
/* Includes ------------------------------------------------------------------*/
#include "main.h"

/* Private includes ----------------------------------------------------------*/
/* USER CODE BEGIN Includes */

/* USER CODE END Includes */

/* Private typedef -----------------------------------------------------------*/
/* USER CODE BEGIN PTD */

/* USER CODE END PTD */

/* Private define ------------------------------------------------------------*/
/* USER CODE BEGIN PD */

#ifndef HSEM_ID_0
#define HSEM_ID_0 (0U) /* HW semaphore 0*/
#endif

/* USER CODE END PD */

/* Private macro -------------------------------------------------------------*/
/* USER CODE BEGIN PM */

/* USER CODE END PM */

/* Private variables ---------------------------------------------------------*/

UART_HandleTypeDef huart3;

/* USER CODE BEGIN PV */


/* USER CODE END PV */

/* Private function prototypes -----------------------------------------------*/
void SystemClock_Config(void);
static void MX_GPIO_Init(void);
static void MX_USART3_UART_Init(void);
/* USER CODE BEGIN PFP */

/* USER CODE END PFP */

/* Private user code ---------------------------------------------------------*/
/* USER CODE BEGIN 0 */


//================================================
// rust wrapper
//==============

typedef struct CArray {
	size_t size;
	void* array;
} CArray;

typedef struct Vector2 {
	float x;
	float y;
} Vector2;


void threshold     ( size_t imageAddress, uint32_t size_x, uint32_t size_y);
size_t blob        ( size_t imageAddress, size_t size_x, size_t size_y );
void project       ( size_t size_x, size_t size_y);
size_t track       ( size_t allowed_failures );
size_t track_chunk ( size_t allowed_failures, float ra, float dec );
void vote          ( );
void get_vote      ( uint8_t* vote );

void set_pixel ( size_t address, size_t size_x, size_t size_y, size_t pos_x, size_t pos_y, uint8_t value );


Vector2 get_blob( size_t index );

void print_string( uint8_t* text, uint32_t size )
{
		HAL_UART_Transmit(&huart3, text, size, 100);
}

//=================================================


const uint8_t SEND_SET = 'S';
const uint8_t SEND_GET = 'G';
const uint8_t SEND_ACK = 'A';
const uint8_t SEND_ERR = 'E';
const uint8_t SEND_HANDSHAKE = 'H';
const uint8_t SEND_RUN = 'R';
const uint8_t SEND_RUN_CHUNK = 'C';

const uint8_t SEND_THRESH          = 'a';

const uint8_t SEND_BLOB     = 'e';
const uint8_t SEND_GET_STAR = 'f';
const uint8_t SEND_GET_VOTE = 'g';


// RAM    : ORIGIN = 0x20000000, LENGTH = 128K

// /* AXISRAM */
// AXISRAM : ORIGIN = 0x24000000, LENGTH = 512K

// /* SRAM */
// SRAM1 : ORIGIN = 0x30000000, LENGTH = 128K
// SRAM2 : ORIGIN = 0x30020000, LENGTH = 128K
// SRAM3 : ORIGIN = 0x30040000, LENGTH = 32K
// SRAM4 : ORIGIN = 0x38000000, LENGTH = 64K
const uintptr_t AXIS_RAM_START  = 0x24000000;
const size_t    AXIS_IMAGE_SIZE = 808 * 608 / 4;  
volatile uint32_t* image = (uint32_t*)AXIS_RAM_START; // The 8 bit image stored as a continuous set of 32 bits.


void reset_leds ( )
{
	HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0,  GPIO_PIN_RESET);
	HAL_GPIO_WritePin(GPIOB, GPIO_PIN_14, GPIO_PIN_RESET);
	HAL_GPIO_WritePin(GPIOE, GPIO_PIN_1,  GPIO_PIN_RESET);	
}


void handshake ( )
{
	reset_leds();
	uint8_t buffer[1];
	HAL_StatusTypeDef status;
	do
	{
		status = HAL_UART_Receive (&huart3, buffer, 1, 100);
		HAL_GPIO_WritePin(GPIOB, GPIO_PIN_14, GPIO_PIN_SET);
		HAL_GPIO_WritePin(GPIOB, GPIO_PIN_14, GPIO_PIN_RESET);
		HAL_Delay(100); // blinky light.
	} while ( buffer[0] != SEND_HANDSHAKE || status != HAL_OK );

	HAL_UART_Transmit(&huart3, buffer, sizeof(buffer), 100);
}


void set_address ( )
{
	reset_leds();
	uint8_t send_buffer[1];      // Status Code.
	uint8_t set_buffer[4+4];   // 32bit address 32bit size.
	HAL_StatusTypeDef status;

	HAL_GPIO_WritePin(GPIOE, GPIO_PIN_1, GPIO_PIN_SET);
	status = HAL_UART_Receive (&huart3, set_buffer, sizeof(set_buffer), 500);
	
	if ( status == HAL_OK )
	{
		uint32_t location = 
			set_buffer[3] << (3*8) | 
			set_buffer[2] << (2*8) | 
			set_buffer[1] << (1*8) |
			set_buffer[0] << (0*8);

		image[location] = 
			set_buffer[7] << (3*8) | 
			set_buffer[6] << (2*8) | 
			set_buffer[5] << (1*8) |
			set_buffer[4] << (0*8);

		send_buffer[0] = SEND_ACK; // acknowledge
		HAL_UART_Transmit(&huart3, send_buffer, sizeof(send_buffer), 100);
	}
	else
	{
		send_buffer[0] = (uint8_t)SEND_ERR; // negative acknowledge
		HAL_UART_Transmit(&huart3, send_buffer, sizeof(send_buffer), 100);
	}
}


int firstAddress = 1;
void get_address ( )
{
	reset_leds();
	uint8_t receive_buffer[4];   // memory location
	uint8_t send_buffer[4];      // value
	HAL_StatusTypeDef status;

	// if ( firstAddress )
	// {
		// set_pixel(AXIS_RAM_START, )
		// firstAddress = 0;
	// }

	HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_SET);
	status = HAL_UART_Receive (&huart3, receive_buffer, sizeof(receive_buffer), 500);
	
	if ( status == HAL_OK )
	{
		uint32_t location =		
				receive_buffer[3] << (3*8) | 
				receive_buffer[2] << (2*8) | 
				receive_buffer[1] << (1*8) |
				receive_buffer[0] << (0*8); 

		send_buffer[0] =  image[location]           & (255);
		send_buffer[1] = (image[location] >> (8*1)) & (255);
		send_buffer[2] = (image[location] >> (8*2)) & (255);
		send_buffer[3] = (image[location] >> (8*3)) & (255);
		HAL_UART_Transmit(&huart3, send_buffer, sizeof(send_buffer), 100);
	}
}

void perform_threshold ( )
{
	HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_SET);    // Green
	HAL_GPIO_WritePin(GPIOE, GPIO_PIN_1, GPIO_PIN_RESET);  // Yellow
	HAL_GPIO_WritePin(GPIOB, GPIO_PIN_14, GPIO_PIN_RESET); // Red
	threshold(AXIS_RAM_START, 808, 608); 
}

void perform_blob ( )
{
	HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_RESET);   // Green
	HAL_GPIO_WritePin(GPIOE, GPIO_PIN_1, GPIO_PIN_SET);     // Yellow
	HAL_GPIO_WritePin(GPIOB, GPIO_PIN_14, GPIO_PIN_RESET);  // Red
	size_t found = blob(AXIS_RAM_START, 808, 608); 
}   

void perform_project ( )
{ 
	HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_RESET); // Green
	HAL_GPIO_WritePin(GPIOE, GPIO_PIN_1, GPIO_PIN_RESET); // Yellow
	HAL_GPIO_WritePin(GPIOB, GPIO_PIN_14, GPIO_PIN_SET);  // Red
	project(808, 608);
}

void perform_track ( )
{
	HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_SET);   // Green
	HAL_GPIO_WritePin(GPIOE, GPIO_PIN_1, GPIO_PIN_RESET); // Yellow
	HAL_GPIO_WritePin(GPIOB, GPIO_PIN_14, GPIO_PIN_SET);  // Red
	size_t found = track(10);
}

void perform_track_chunk ( float ra, float dec )
{
	HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_SET);   // Green
	HAL_GPIO_WritePin(GPIOE, GPIO_PIN_1, GPIO_PIN_RESET); // Yellow
	HAL_GPIO_WritePin(GPIOB, GPIO_PIN_14, GPIO_PIN_SET);  // Red
	size_t found = track_chunk(10, ra, dec);
}

void perform_vote ( )
{
	HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_SET);   // Green
	HAL_GPIO_WritePin(GPIOE, GPIO_PIN_1, GPIO_PIN_SET);   // Yellow
	HAL_GPIO_WritePin(GPIOB, GPIO_PIN_14, GPIO_PIN_SET);  // Red
	vote();
}

void perform_get_vote ( )
{
	uint8_t vote[4]; // ra: __.__, dec __.__
	get_vote(&vote);
	HAL_UART_Transmit(&huart3, vote, sizeof(vote), 100);
} 



 



/* USER CODE END 0 */
 
/**
	* @brief  The application entry point.
	* @retval int
	*/
int main(void)
{
	/* USER CODE BEGIN 1 */


	/* USER CODE END 1 */
/* USER CODE BEGIN Boot_Mode_Sequence_0 */
	int32_t timeout;
/* USER CODE END Boot_Mode_Sequence_0 */

/* USER CODE BEGIN Boot_Mode_Sequence_1 */
	/* Wait until CPU2 boots and enters in stop mode or timeout*/
	timeout = 0xFFFF;
	while((__HAL_RCC_GET_FLAG(RCC_FLAG_D2CKRDY) != RESET) && (timeout-- > 0));
	if ( timeout < 0 )
	{
	Error_Handler();
	}
/* USER CODE END Boot_Mode_Sequence_1 */
	/* MCU Configuration--------------------------------------------------------*/

	/* Reset of all peripherals, Initializes the Flash interface and the Systick. */
	HAL_Init();
 
	/* USER CODE BEGIN Init */

	/* USER CODE END Init */

	/* Configure the system clock */
	SystemClock_Config();
/* USER CODE BEGIN Boot_Mode_Sequence_2 */
/* When system initialization is finished, Cortex-M7 will release Cortex-M4 by means of
HSEM notification */
/*HW semaphore Clock enable*/
__HAL_RCC_HSEM_CLK_ENABLE();
/*Take HSEM */
HAL_HSEM_FastTake(HSEM_ID_0);
/*Release HSEM in order to notify the CPU2(CM4)*/
HAL_HSEM_Release(HSEM_ID_0,0);
/* wait until CPU2 wakes up from stop mode */
timeout = 0xFFFF;
while((__HAL_RCC_GET_FLAG(RCC_FLAG_D2CKRDY) == RESET) && (timeout-- > 0));
if ( timeout < 0 )
{
Error_Handler();
}
/* USER CODE END Boot_Mode_Sequence_2 */

	/* USER CODE BEGIN SysInit */

	/* USER CODE END SysInit */

	/* Initialize all configured peripherals */
	MX_GPIO_Init();
	MX_USART3_UART_Init();
	/* USER CODE BEGIN 2 */

	/* USER CODE END 2 */  

	/* Infinite loop */
	/* USER CODE BEGIN WHILE */
	HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0,  GPIO_PIN_RESET);
	HAL_GPIO_WritePin(GPIOB, GPIO_PIN_14, GPIO_PIN_RESET);
	HAL_GPIO_WritePin(GPIOE, GPIO_PIN_1,  GPIO_PIN_RESET);

	handshake(); 
	while (1)
	{
		uint8_t command_buffer[1];
		HAL_StatusTypeDef status;
		status = HAL_UART_Receive (&huart3, command_buffer, sizeof(command_buffer), 500);

		if ( status == HAL_OK )
		{
			switch ( command_buffer[0] ) 
			{ 
				case SEND_SET: 
					set_address();
					break;
 
				case SEND_GET:   
					get_address();
					break;
 
				// case SEND_THRESH:
				// 	perform_threshold();
				// 	break; 
 
				// case SEND_BLOB:
				// 	perform_blob();
				// 	break; 
 
				// case SEND_GET_STAR:
				// 	perform_get_blob();
				// 	break;

				case SEND_RUN_CHUNK:
				{
					uint8_t position[8]; 
					status = HAL_UART_Receive (&huart3, position, sizeof(position), 500);

					int32_t ra =		
							position[3] << (3*8) | 
							position[2] << (2*8) | 
							position[1] << (1*8) |
							position[0] << (0*8); 

					int32_t dec =		
							position[7] << (3*8) | 
							position[6] << (2*8) | 
							position[5] << (1*8) |
							position[4] << (0*8); 

					perform_threshold(); 
					perform_blob();
					perform_project(); 
					perform_track_chunk((float)ra, (float)dec);
					perform_vote();
					break;
				}

				case SEND_RUN:
					perform_threshold(); 
					perform_blob();
					perform_project(); 
					perform_track();
					perform_vote();
					break;
  
				case SEND_GET_VOTE: 
					perform_get_vote();
					break;
				default: 
					break;  
			} 
  
			reset_leds();        
		}   






		// Send Image


		// HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_SET);
		// return 0;

		// sprintf(&tx_msg, "THIS IS A MESSAGE\r\n");
		// HAL_UART_Transmit(&huart3, rx_msg, sizeof(rx_msg), 100);
		// run();
		// HAL_GPIO_WritePin(GPIOE, GPIO_PIN_1, GPIO_PIN_RESET);
		// HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_SET);
		// HAL_Delay(200);
		// HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_RESET);
		// HAL_GPIO_WritePin(GPIOB, GPIO_PIN_14, GPIO_PIN_SET);
		// HAL_Delay(200);
		// HAL_GPIO_WritePin(GPIOB, GPIO_PIN_14, GPIO_PIN_RESET);
		// HAL_GPIO_WritePin(GPIOE, GPIO_PIN_1, GPIO_PIN_SET);
		// HAL_Delay(200);
		/* USER CODE END WHILE */

		/* USER CODE BEGIN 3 */
	}
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
	/* USER CODE END 3 */
}

/**
	* @brief System Clock Configuration
	* @retval None
	*/
void SystemClock_Config(void)
{
	RCC_OscInitTypeDef RCC_OscInitStruct = {0};
	RCC_ClkInitTypeDef RCC_ClkInitStruct = {0};

	/** Supply configuration update enable
	*/
	HAL_PWREx_ConfigSupply(PWR_DIRECT_SMPS_SUPPLY);

	/** Configure the main internal regulator output voltage
	*/
	__HAL_PWR_VOLTAGESCALING_CONFIG(PWR_REGULATOR_VOLTAGE_SCALE3);

	while(!__HAL_PWR_GET_FLAG(PWR_FLAG_VOSRDY)) {}

	/** Initializes the RCC Oscillators according to the specified parameters
	* in the RCC_OscInitTypeDef structure.
	*/
	RCC_OscInitStruct.OscillatorType = RCC_OSCILLATORTYPE_HSI;
	RCC_OscInitStruct.HSIState = RCC_HSI_DIV1;
	RCC_OscInitStruct.HSICalibrationValue = RCC_HSICALIBRATION_DEFAULT;
	RCC_OscInitStruct.PLL.PLLState = RCC_PLL_NONE;
	if (HAL_RCC_OscConfig(&RCC_OscInitStruct) != HAL_OK)
	{
		Error_Handler();
	}

	/** Initializes the CPU, AHB and APB buses clocks
	*/
	RCC_ClkInitStruct.ClockType = RCC_CLOCKTYPE_HCLK|RCC_CLOCKTYPE_SYSCLK
															|RCC_CLOCKTYPE_PCLK1|RCC_CLOCKTYPE_PCLK2
															|RCC_CLOCKTYPE_D3PCLK1|RCC_CLOCKTYPE_D1PCLK1;
	RCC_ClkInitStruct.SYSCLKSource = RCC_SYSCLKSOURCE_HSI;
	RCC_ClkInitStruct.SYSCLKDivider = RCC_SYSCLK_DIV1;
	RCC_ClkInitStruct.AHBCLKDivider = RCC_HCLK_DIV1;
	RCC_ClkInitStruct.APB3CLKDivider = RCC_APB3_DIV1;
	RCC_ClkInitStruct.APB1CLKDivider = RCC_APB1_DIV2;
	RCC_ClkInitStruct.APB2CLKDivider = RCC_APB2_DIV1;
	RCC_ClkInitStruct.APB4CLKDivider = RCC_APB4_DIV1;

	if (HAL_RCC_ClockConfig(&RCC_ClkInitStruct, FLASH_LATENCY_1) != HAL_OK)
	{
		Error_Handler();
	}
}

/**
	* @brief USART3 Initialization Function
	* @param None
	* @retval None
	*/
static void MX_USART3_UART_Init(void)
{

	/* USER CODE BEGIN USART3_Init 0 */

	/* USER CODE END USART3_Init 0 */

	/* USER CODE BEGIN USART3_Init 1 */

	/* USER CODE END USART3_Init 1 */
	huart3.Instance = USART3;
	huart3.Init.BaudRate = 921600;
	huart3.Init.WordLength = UART_WORDLENGTH_8B;
	huart3.Init.StopBits = UART_STOPBITS_1;
	huart3.Init.Parity = UART_PARITY_NONE;
	huart3.Init.Mode = UART_MODE_TX_RX;
	huart3.Init.HwFlowCtl = UART_HWCONTROL_NONE;
	huart3.Init.OverSampling = UART_OVERSAMPLING_16;
	huart3.Init.OneBitSampling = UART_ONE_BIT_SAMPLE_DISABLE;
	huart3.Init.ClockPrescaler = UART_PRESCALER_DIV1;
	huart3.AdvancedInit.AdvFeatureInit = UART_ADVFEATURE_NO_INIT;
	if (HAL_UART_Init(&huart3) != HAL_OK)
	{
		Error_Handler();
	}
	if (HAL_UARTEx_SetTxFifoThreshold(&huart3, UART_TXFIFO_THRESHOLD_1_8) != HAL_OK)
	{
		Error_Handler();
	}
	if (HAL_UARTEx_SetRxFifoThreshold(&huart3, UART_RXFIFO_THRESHOLD_1_8) != HAL_OK)
	{
		Error_Handler();
	}
	if (HAL_UARTEx_DisableFifoMode(&huart3) != HAL_OK)
	{
		Error_Handler();
	}
	/* USER CODE BEGIN USART3_Init 2 */

	/* USER CODE END USART3_Init 2 */

}

/**
	* @brief GPIO Initialization Function
	* @param None
	* @retval None
	*/
static void MX_GPIO_Init(void)
{
	GPIO_InitTypeDef GPIO_InitStruct = {0};
/* USER CODE BEGIN MX_GPIO_Init_1 */
/* USER CODE END MX_GPIO_Init_1 */

	/* GPIO Ports Clock Enable */
	__HAL_RCC_GPIOC_CLK_ENABLE();
	__HAL_RCC_GPIOA_CLK_ENABLE();
	__HAL_RCC_GPIOB_CLK_ENABLE();
	__HAL_RCC_GPIOD_CLK_ENABLE();
	__HAL_RCC_GPIOG_CLK_ENABLE();
	__HAL_RCC_GPIOE_CLK_ENABLE();

	/*Configure GPIO pin Output Level */
	HAL_GPIO_WritePin(GPIOB, LD1_Pin|LD3_Pin, GPIO_PIN_RESET);

	/*Configure GPIO pin Output Level */
	HAL_GPIO_WritePin(USB_OTG_FS_PWR_EN_GPIO_Port, USB_OTG_FS_PWR_EN_Pin, GPIO_PIN_RESET);

	/*Configure GPIO pin Output Level */
	HAL_GPIO_WritePin(LD2_GPIO_Port, LD2_Pin, GPIO_PIN_RESET);

	/*Configure GPIO pin : B1_Pin */
	GPIO_InitStruct.Pin = B1_Pin;
	GPIO_InitStruct.Mode = GPIO_MODE_INPUT;
	GPIO_InitStruct.Pull = GPIO_NOPULL;
	HAL_GPIO_Init(B1_GPIO_Port, &GPIO_InitStruct);

	/*Configure GPIO pins : PC1 PC4 PC5 */
	GPIO_InitStruct.Pin = GPIO_PIN_1|GPIO_PIN_4|GPIO_PIN_5;
	GPIO_InitStruct.Mode = GPIO_MODE_AF_PP;
	GPIO_InitStruct.Pull = GPIO_NOPULL;
	GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
	GPIO_InitStruct.Alternate = GPIO_AF11_ETH;
	HAL_GPIO_Init(GPIOC, &GPIO_InitStruct);

	/*Configure GPIO pins : PA1 PA2 PA7 */
	GPIO_InitStruct.Pin = GPIO_PIN_1|GPIO_PIN_2|GPIO_PIN_7;
	GPIO_InitStruct.Mode = GPIO_MODE_AF_PP;
	GPIO_InitStruct.Pull = GPIO_NOPULL;
	GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
	GPIO_InitStruct.Alternate = GPIO_AF11_ETH;
	HAL_GPIO_Init(GPIOA, &GPIO_InitStruct);

	/*Configure GPIO pins : LD1_Pin LD3_Pin */
	GPIO_InitStruct.Pin = LD1_Pin|LD3_Pin;
	GPIO_InitStruct.Mode = GPIO_MODE_OUTPUT_PP;
	GPIO_InitStruct.Pull = GPIO_NOPULL;
	GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
	HAL_GPIO_Init(GPIOB, &GPIO_InitStruct);

	/*Configure GPIO pin : PB13 */
	GPIO_InitStruct.Pin = GPIO_PIN_13;
	GPIO_InitStruct.Mode = GPIO_MODE_AF_PP;
	GPIO_InitStruct.Pull = GPIO_NOPULL;
	GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
	GPIO_InitStruct.Alternate = GPIO_AF11_ETH;
	HAL_GPIO_Init(GPIOB, &GPIO_InitStruct);

	/*Configure GPIO pin : USB_OTG_FS_PWR_EN_Pin */
	GPIO_InitStruct.Pin = USB_OTG_FS_PWR_EN_Pin;
	GPIO_InitStruct.Mode = GPIO_MODE_OUTPUT_PP;
	GPIO_InitStruct.Pull = GPIO_NOPULL;
	GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
	HAL_GPIO_Init(USB_OTG_FS_PWR_EN_GPIO_Port, &GPIO_InitStruct);

	/*Configure GPIO pin : USB_OTG_FS_OVCR_Pin */
	GPIO_InitStruct.Pin = USB_OTG_FS_OVCR_Pin;
	GPIO_InitStruct.Mode = GPIO_MODE_IT_RISING;
	GPIO_InitStruct.Pull = GPIO_NOPULL;
	HAL_GPIO_Init(USB_OTG_FS_OVCR_GPIO_Port, &GPIO_InitStruct);

	/*Configure GPIO pins : PA8 PA11 PA12 */
	GPIO_InitStruct.Pin = GPIO_PIN_8|GPIO_PIN_11|GPIO_PIN_12;
	GPIO_InitStruct.Mode = GPIO_MODE_AF_PP;
	GPIO_InitStruct.Pull = GPIO_NOPULL;
	GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
	GPIO_InitStruct.Alternate = GPIO_AF10_OTG1_FS;
	HAL_GPIO_Init(GPIOA, &GPIO_InitStruct);

	/*Configure GPIO pins : PG11 PG13 */
	GPIO_InitStruct.Pin = GPIO_PIN_11|GPIO_PIN_13;
	GPIO_InitStruct.Mode = GPIO_MODE_AF_PP;
	GPIO_InitStruct.Pull = GPIO_NOPULL;
	GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
	GPIO_InitStruct.Alternate = GPIO_AF11_ETH;
	HAL_GPIO_Init(GPIOG, &GPIO_InitStruct);

	/*Configure GPIO pin : LD2_Pin */
	GPIO_InitStruct.Pin = LD2_Pin;
	GPIO_InitStruct.Mode = GPIO_MODE_OUTPUT_PP;
	GPIO_InitStruct.Pull = GPIO_NOPULL;
	GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
	HAL_GPIO_Init(LD2_GPIO_Port, &GPIO_InitStruct);

/* USER CODE BEGIN MX_GPIO_Init_2 */
/* USER CODE END MX_GPIO_Init_2 */
}

/* USER CODE BEGIN 4 */

/* USER CODE END 4 */

/**
	* @brief  This function is executed in case of error occurrence.
	* @retval None
	*/
void Error_Handler(void)
{
	/* USER CODE BEGIN Error_Handler_Debug */
	/* User can add his own implementation to report the HAL error return state */
	__disable_irq();
	while (1)
	{
	}
	/* USER CODE END Error_Handler_Debug */
}

#ifdef  USE_FULL_ASSERT
/**
	* @brief  Reports the name of the source file and the source line number
	*         where the assert_param error has occurred.
	* @param  file: pointer to the source file name
	* @param  line: assert_param error line source number
	* @retval None
	*/
void assert_failed(uint8_t *file, uint32_t line)
{
	/* USER CODE BEGIN 6 */
	/* User can add his own implementation to report the file name and line number,
		 ex: printf("Wrong parameters value: file %s on line %d\r\n", file, line) */
	/* USER CODE END 6 */
}
#endif /* USE_FULL_ASSERT */
