EESchema Schematic File Version 2
LIBS:power
LIBS:device
LIBS:transistors
LIBS:conn
LIBS:linear
LIBS:regul
LIBS:74xx
LIBS:cmos4000
LIBS:adc-dac
LIBS:memory
LIBS:xilinx
LIBS:microcontrollers
LIBS:dsp
LIBS:microchip
LIBS:analog_switches
LIBS:motorola
LIBS:texas
LIBS:intel
LIBS:audio
LIBS:interface
LIBS:digital-audio
LIBS:philips
LIBS:display
LIBS:cypress
LIBS:siliconi
LIBS:opto
LIBS:atmel
LIBS:contrib
LIBS:valves
LIBS:rt-soldering-iron
LIBS:rt-soldering-iron-cache
EELAYER 25 0
EELAYER END
$Descr USLetter 11000 8500
encoding utf-8
Sheet 4 4
Title "RT Soldering Iron"
Date "2017-06-02"
Rev "A"
Comp "Kevin Cuzner"
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L +3.3V #PWR042
U 1 1 592BB6CC
P 4600 1000
F 0 "#PWR042" H 4600 850 50  0001 C CNN
F 1 "+3.3V" H 4600 1140 50  0000 C CNN
F 2 "" H 4600 1000 50  0001 C CNN
F 3 "" H 4600 1000 50  0001 C CNN
	1    4600 1000
	1    0    0    -1  
$EndComp
Wire Wire Line
	4600 1700 4900 1700
Wire Wire Line
	4600 1000 4600 1700
$Comp
L C C14
U 1 1 592BB72D
P 3050 2050
F 0 "C14" H 3075 2150 50  0000 L CNN
F 1 "0.1u" H 3075 1950 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 3088 1900 50  0001 C CNN
F 3 "" H 3050 2050 50  0001 C CNN
F 4 "490-14603-1-ND" H 3050 2050 60  0001 C CNN "Part No."
	1    3050 2050
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR043
U 1 1 592BB766
P 3300 2400
F 0 "#PWR043" H 3300 2150 50  0001 C CNN
F 1 "GND" H 3300 2250 50  0000 C CNN
F 2 "" H 3300 2400 50  0001 C CNN
F 3 "" H 3300 2400 50  0001 C CNN
	1    3300 2400
	1    0    0    -1  
$EndComp
Wire Wire Line
	3300 1900 3300 2400
Wire Wire Line
	3300 1900 4900 1900
NoConn ~ 4900 2100
NoConn ~ 4900 2200
Wire Wire Line
	3050 1900 3050 1800
Connection ~ 3050 1800
Wire Wire Line
	2800 2300 3300 2300
Wire Wire Line
	3050 2300 3050 2200
Connection ~ 3300 2300
$Comp
L C C12
U 1 1 592BB887
P 4300 1350
F 0 "C12" H 4325 1450 50  0000 L CNN
F 1 "0.1u" H 4325 1250 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 4338 1200 50  0001 C CNN
F 3 "" H 4300 1350 50  0001 C CNN
F 4 "490-14603-1-ND" H 4300 1350 60  0001 C CNN "Part No."
	1    4300 1350
	1    0    0    -1  
$EndComp
Wire Wire Line
	4300 1200 4300 1100
Wire Wire Line
	4050 1100 4600 1100
Connection ~ 4600 1100
$Comp
L GND #PWR044
U 1 1 592BB8F3
P 4300 1600
F 0 "#PWR044" H 4300 1350 50  0001 C CNN
F 1 "GND" H 4300 1450 50  0000 C CNN
F 2 "" H 4300 1600 50  0001 C CNN
F 3 "" H 4300 1600 50  0001 C CNN
	1    4300 1600
	1    0    0    -1  
$EndComp
Wire Wire Line
	4300 1600 4300 1500
$Comp
L C C11
U 1 1 592BB924
P 4050 1350
F 0 "C11" H 4075 1450 50  0000 L CNN
F 1 "1u" H 4075 1250 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 4088 1200 50  0001 C CNN
F 3 "" H 4050 1350 50  0001 C CNN
F 4 "490-10018-1-ND" H 4050 1350 60  0001 C CNN "Part No."
	1    4050 1350
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR045
U 1 1 592BB980
P 4050 1600
F 0 "#PWR045" H 4050 1350 50  0001 C CNN
F 1 "GND" H 4050 1450 50  0000 C CNN
F 2 "" H 4050 1600 50  0001 C CNN
F 3 "" H 4050 1600 50  0001 C CNN
	1    4050 1600
	1    0    0    -1  
$EndComp
Wire Wire Line
	4050 1600 4050 1500
Wire Wire Line
	4050 1200 4050 1100
Connection ~ 4300 1100
$Comp
L C C13
U 1 1 592BBA15
P 2800 2050
F 0 "C13" H 2825 2150 50  0000 L CNN
F 1 "1u" H 2825 1950 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 2838 1900 50  0001 C CNN
F 3 "" H 2800 2050 50  0001 C CNN
F 4 "490-10018-1-ND" H 2800 2050 60  0001 C CNN "Part No."
	1    2800 2050
	1    0    0    -1  
$EndComp
Wire Wire Line
	2800 1600 2800 1900
Connection ~ 2800 1800
Wire Wire Line
	2800 2200 2800 2300
Connection ~ 3050 2300
$Comp
L R R14
U 1 1 592BBD26
P 3500 2650
F 0 "R14" V 3580 2650 50  0000 C CNN
F 1 "10K" V 3500 2650 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 3430 2650 50  0001 C CNN
F 3 "" H 3500 2650 50  0001 C CNN
F 4 "311-10.0KLRCT-ND" H 3500 2650 60  0001 C CNN "Part No."
	1    3500 2650
	1    0    0    -1  
$EndComp
Wire Wire Line
	3500 2500 3500 2400
Wire Wire Line
	3500 2400 4900 2400
$Comp
L GND #PWR046
U 1 1 592BBD79
P 3500 2900
F 0 "#PWR046" H 3500 2650 50  0001 C CNN
F 1 "GND" H 3500 2750 50  0000 C CNN
F 2 "" H 3500 2900 50  0001 C CNN
F 3 "" H 3500 2900 50  0001 C CNN
	1    3500 2900
	1    0    0    -1  
$EndComp
Wire Wire Line
	3500 2900 3500 2800
Wire Wire Line
	2800 1800 4900 1800
$Comp
L +3.3VA #PWR047
U 1 1 592C93D2
P 2800 1600
F 0 "#PWR047" H 2800 1450 50  0001 C CNN
F 1 "+3.3VA" H 2800 1740 50  0000 C CNN
F 2 "" H 2800 1600 50  0001 C CNN
F 3 "" H 2800 1600 50  0001 C CNN
	1    2800 1600
	1    0    0    -1  
$EndComp
$Comp
L SSD1306-OLED-I2C U5
U 1 1 592C9853
P 8400 4600
F 0 "U5" H 8400 3850 60  0000 C CNN
F 1 "SSD1306-OLED-I2C" H 8400 5250 60  0000 C CNN
F 2 "rt-soldering-iron:OLED-FPC-I2C" H 9150 4450 60  0001 C CNN
F 3 "" H 9150 4450 60  0001 C CNN
	1    8400 4600
	1    0    0    -1  
$EndComp
$Comp
L MMA8652FC U6
U 1 1 592CACF4
P 5000 6700
F 0 "U6" H 5000 6250 50  0000 C CNN
F 1 "MMA8652FC" H 5000 7250 50  0000 C CNN
F 2 "rt-soldering-iron:10-DFN" H 5000 6700 50  0001 C CNN
F 3 "" H 5000 6700 50  0001 C CNN
F 4 "MMA8652FCR1CT-ND" H 5000 6700 60  0001 C CNN "Part No."
	1    5000 6700
	1    0    0    -1  
$EndComp
Text Notes 7700 5850 0    60   ~ 0
I2C Addr\n7-bit: 0x4C or 0x4D\n8-bit: 0x78 or 0x7A
$Comp
L R R15
U 1 1 592CB0DF
P 3000 3450
F 0 "R15" V 3080 3450 50  0000 C CNN
F 1 "4K7" V 3000 3450 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 2930 3450 50  0001 C CNN
F 3 "" H 3000 3450 50  0001 C CNN
F 4 "311-4.7KLRCT-ND" H 3000 3450 60  0001 C CNN "Part No."
	1    3000 3450
	1    0    0    -1  
$EndComp
$Comp
L R R16
U 1 1 592CB13F
P 3200 3450
F 0 "R16" V 3280 3450 50  0000 C CNN
F 1 "4K7" V 3200 3450 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 3130 3450 50  0001 C CNN
F 3 "" H 3200 3450 50  0001 C CNN
F 4 "311-4.7KLRCT-ND" H 3200 3450 60  0001 C CNN "Part No."
	1    3200 3450
	1    0    0    -1  
$EndComp
Wire Wire Line
	4900 3100 4500 3100
Wire Wire Line
	4500 3100 4500 3800
Wire Wire Line
	4500 3800 3200 3800
Wire Wire Line
	3200 3800 3200 3600
Wire Wire Line
	4900 3200 4600 3200
Wire Wire Line
	4600 3200 4600 3900
Wire Wire Line
	4600 3900 3000 3900
Wire Wire Line
	3000 3900 3000 3600
$Comp
L +3.3V #PWR048
U 1 1 592CB325
P 3000 3100
F 0 "#PWR048" H 3000 2950 50  0001 C CNN
F 1 "+3.3V" H 3000 3240 50  0000 C CNN
F 2 "" H 3000 3100 50  0001 C CNN
F 3 "" H 3000 3100 50  0001 C CNN
	1    3000 3100
	1    0    0    -1  
$EndComp
Wire Wire Line
	3000 3100 3000 3300
Wire Wire Line
	3000 3200 3200 3200
Wire Wire Line
	3200 3200 3200 3300
Connection ~ 3000 3200
Text Label 3600 3800 0    60   ~ 0
I2C_SCL
Text Label 3600 3900 0    60   ~ 0
I2C_SDA
Text Label 7700 4500 2    60   ~ 0
I2C_SCL
Wire Wire Line
	7700 4500 7900 4500
Text Label 7700 4600 2    60   ~ 0
I2C_SDA
Wire Wire Line
	7700 4600 7900 4600
$Comp
L GND #PWR049
U 1 1 592CB60F
P 7800 5300
F 0 "#PWR049" H 7800 5050 50  0001 C CNN
F 1 "GND" H 7800 5150 50  0000 C CNN
F 2 "" H 7800 5300 50  0001 C CNN
F 3 "" H 7800 5300 50  0001 C CNN
	1    7800 5300
	1    0    0    -1  
$EndComp
Wire Wire Line
	7800 5300 7800 5200
Wire Wire Line
	7800 5200 7900 5200
$Comp
L CONN_01X04 J3
U 1 1 592CC381
P 8000 2950
F 0 "J3" H 8000 3200 50  0000 C CNN
F 1 "CONN_01X04" V 8100 2950 50  0000 C CNN
F 2 "rt-soldering-iron:JST-SH-4" H 8000 2950 50  0001 C CNN
F 3 "" H 8000 2950 50  0001 C CNN
F 4 "455-1790-1-ND" H 8000 2950 60  0001 C CNN "Part No."
	1    8000 2950
	1    0    0    1   
$EndComp
Text Notes 8200 2900 0    60   ~ 0
Housing: 455-1379-ND\nTerminals: 455-1561-1-ND
$Comp
L +3.3V #PWR050
U 1 1 592CC5EE
P 7800 3800
F 0 "#PWR050" H 7800 3650 50  0001 C CNN
F 1 "+3.3V" H 7800 3940 50  0000 C CNN
F 2 "" H 7800 3800 50  0001 C CNN
F 3 "" H 7800 3800 50  0001 C CNN
	1    7800 3800
	1    0    0    -1  
$EndComp
Wire Wire Line
	7800 3800 7800 4100
Wire Wire Line
	7800 4100 7900 4100
$Comp
L GND #PWR051
U 1 1 592CC80A
P 7700 3200
F 0 "#PWR051" H 7700 2950 50  0001 C CNN
F 1 "GND" H 7700 3050 50  0000 C CNN
F 2 "" H 7700 3200 50  0001 C CNN
F 3 "" H 7700 3200 50  0001 C CNN
	1    7700 3200
	1    0    0    -1  
$EndComp
Wire Wire Line
	7800 3100 7700 3100
Wire Wire Line
	7700 3100 7700 3200
$Comp
L C C21
U 1 1 592CCA1C
P 3000 6650
F 0 "C21" H 3025 6750 50  0000 L CNN
F 1 "1u" H 3025 6550 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 3038 6500 50  0001 C CNN
F 3 "" H 3000 6650 50  0001 C CNN
F 4 "490-10018-1-ND" H 3000 6650 60  0001 C CNN "Part No."
	1    3000 6650
	1    0    0    -1  
$EndComp
$Comp
L C C22
U 1 1 592CCABC
P 3300 6650
F 0 "C22" H 3325 6750 50  0000 L CNN
F 1 "0.1u" H 3325 6550 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 3338 6500 50  0001 C CNN
F 3 "" H 3300 6650 50  0001 C CNN
F 4 "490-14603-1-ND" H 3300 6650 60  0001 C CNN "Part No."
	1    3300 6650
	1    0    0    -1  
$EndComp
$Comp
L C C23
U 1 1 592CCB47
P 3600 6650
F 0 "C23" H 3625 6750 50  0000 L CNN
F 1 "0.1u" H 3625 6550 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 3638 6500 50  0001 C CNN
F 3 "" H 3600 6650 50  0001 C CNN
F 4 "490-14603-1-ND" H 3600 6650 60  0001 C CNN "Part No."
	1    3600 6650
	1    0    0    -1  
$EndComp
Wire Wire Line
	3000 6300 4500 6300
Wire Wire Line
	3600 6300 3600 6500
Wire Wire Line
	4500 6400 3600 6400
Connection ~ 3600 6400
Wire Wire Line
	3300 6300 3300 6500
Connection ~ 3600 6300
Wire Wire Line
	3000 6200 3000 6500
Connection ~ 3300 6300
$Comp
L GND #PWR052
U 1 1 592CCD51
P 3000 7000
F 0 "#PWR052" H 3000 6750 50  0001 C CNN
F 1 "GND" H 3000 6850 50  0000 C CNN
F 2 "" H 3000 7000 50  0001 C CNN
F 3 "" H 3000 7000 50  0001 C CNN
	1    3000 7000
	1    0    0    -1  
$EndComp
$Comp
L +3.3V #PWR053
U 1 1 592CCDF7
P 3000 6200
F 0 "#PWR053" H 3000 6050 50  0001 C CNN
F 1 "+3.3V" H 3000 6340 50  0000 C CNN
F 2 "" H 3000 6200 50  0001 C CNN
F 3 "" H 3000 6200 50  0001 C CNN
	1    3000 6200
	1    0    0    -1  
$EndComp
Connection ~ 3000 6300
Wire Wire Line
	3000 6800 3000 7000
Wire Wire Line
	3000 6900 3600 6900
Wire Wire Line
	3600 6900 3600 6800
Connection ~ 3000 6900
Wire Wire Line
	3300 6800 3300 6900
Connection ~ 3300 6900
$Comp
L C C24
U 1 1 592CCFD7
P 4100 6850
F 0 "C24" H 4125 6950 50  0000 L CNN
F 1 "0.1u" H 4125 6750 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 4138 6700 50  0001 C CNN
F 3 "" H 4100 6850 50  0001 C CNN
F 4 "490-14603-1-ND" H 4100 6850 60  0001 C CNN "Part No."
	1    4100 6850
	1    0    0    -1  
$EndComp
Wire Wire Line
	4500 6600 4100 6600
Wire Wire Line
	4100 6600 4100 6700
$Comp
L GND #PWR054
U 1 1 592CD0B5
P 4100 7200
F 0 "#PWR054" H 4100 6950 50  0001 C CNN
F 1 "GND" H 4100 7050 50  0000 C CNN
F 2 "" H 4100 7200 50  0001 C CNN
F 3 "" H 4100 7200 50  0001 C CNN
	1    4100 7200
	1    0    0    -1  
$EndComp
Wire Wire Line
	4100 7000 4100 7200
Wire Wire Line
	4300 7100 4100 7100
Wire Wire Line
	4300 6800 4300 7100
Wire Wire Line
	4300 6800 4500 6800
Connection ~ 4100 7100
Wire Wire Line
	4300 6900 4500 6900
Connection ~ 4300 6900
Wire Wire Line
	4500 7000 4300 7000
Connection ~ 4300 7000
Text Label 5850 6800 0    60   ~ 0
I2C_SCL
Text Label 5850 6900 0    60   ~ 0
I2C_SDA
Wire Wire Line
	5850 6900 5500 6900
Wire Wire Line
	5500 6800 5850 6800
Text Label 4700 2300 2    60   ~ 0
~STM32_RESET
Wire Wire Line
	4700 2300 4900 2300
Text Label 7600 3000 2    60   ~ 0
~STM32_RESET
Wire Wire Line
	7600 3000 7800 3000
Wire Wire Line
	6300 2900 7800 2900
Wire Wire Line
	7800 2800 6300 2800
Text HLabel 9350 1700 2    60   Input ~ 0
HEATER_SENSE
Wire Wire Line
	9350 1700 6300 1700
Text Label 6850 1700 0    60   ~ 0
HEATER_SENSE
$Comp
L C C17
U 1 1 592CE1BB
P 9350 4300
F 0 "C17" H 9375 4400 50  0000 L CNN
F 1 "1u" H 9375 4200 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 9388 4150 50  0001 C CNN
F 3 "" H 9350 4300 50  0001 C CNN
F 4 "490-10018-1-ND" H 9350 4300 60  0001 C CNN "Part No."
	1    9350 4300
	0    1    1    0   
$EndComp
$Comp
L C C18
U 1 1 592CE2C3
P 9350 4600
F 0 "C18" H 9375 4700 50  0000 L CNN
F 1 "1u" H 9375 4500 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 9388 4450 50  0001 C CNN
F 3 "" H 9350 4600 50  0001 C CNN
F 4 "490-10018-1-ND" H 9350 4600 60  0001 C CNN "Part No."
	1    9350 4600
	0    1    1    0   
$EndComp
NoConn ~ 8900 4200
Wire Wire Line
	7800 3900 9000 3900
Wire Wire Line
	9000 3900 9000 4100
Wire Wire Line
	9000 4100 8900 4100
Connection ~ 7800 3900
Wire Wire Line
	8900 4500 9600 4500
Wire Wire Line
	9600 4500 9600 4300
Wire Wire Line
	9600 4300 9500 4300
Wire Wire Line
	9200 4300 9050 4300
Wire Wire Line
	9050 4300 9050 4400
Wire Wire Line
	9050 4400 8900 4400
Wire Wire Line
	8900 4600 9200 4600
Wire Wire Line
	8900 4700 9050 4700
Wire Wire Line
	9050 4700 9050 4800
Wire Wire Line
	9050 4800 9600 4800
Wire Wire Line
	9600 4800 9600 4600
Wire Wire Line
	9600 4600 9500 4600
Text Notes 6300 1700 0    60   ~ 0
ADC_IN0
Text Notes 6350 2400 0    60   ~ 0
TIM14_CH1
Wire Wire Line
	6300 2400 7500 2400
Text HLabel 7500 2400 2    60   Input ~ 0
HEATER_CTL
Text Label 6900 2400 0    60   ~ 0
HEATER_CTL
Text Label 7100 2800 0    60   ~ 0
STM32_SWDIO
Text Label 7100 2900 0    60   ~ 0
STM32_SWCLK
Text Label 3500 2400 0    60   ~ 0
STM32_BOOT0
Text Notes 4500 3100 0    60   ~ 0
I2C1_SCL
Text Notes 4650 3300 0    60   ~ 0
I2C1_SDA
Text Notes 9300 4100 0    60   ~ 0
Charge pump enabled:\nVCC  = 7.5V\nRREF = 400K -> 390K
$Comp
L R R18
U 1 1 592D0E46
P 9500 5250
F 0 "R18" V 9580 5250 50  0000 C CNN
F 1 "390K" V 9500 5250 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 9430 5250 50  0001 C CNN
F 3 "" H 9500 5250 50  0001 C CNN
F 4 "311-390KJRCT-ND" H 9500 5250 60  0001 C CNN "Part No."
	1    9500 5250
	1    0    0    -1  
$EndComp
Wire Wire Line
	8900 4900 9500 4900
Wire Wire Line
	9500 4900 9500 5100
$Comp
L GND #PWR055
U 1 1 592D0F0D
P 9500 5800
F 0 "#PWR055" H 9500 5550 50  0001 C CNN
F 1 "GND" H 9500 5650 50  0000 C CNN
F 2 "" H 9500 5800 50  0001 C CNN
F 3 "" H 9500 5800 50  0001 C CNN
	1    9500 5800
	1    0    0    -1  
$EndComp
Wire Wire Line
	9500 5400 9500 5800
$Comp
L C C15
U 1 1 592D25C4
P 6800 4250
F 0 "C15" H 6825 4350 50  0000 L CNN
F 1 "1u" H 6825 4150 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 6838 4100 50  0001 C CNN
F 3 "" H 6800 4250 50  0001 C CNN
F 4 "490-10018-1-ND" H 6800 4250 60  0001 C CNN "Part No."
	1    6800 4250
	1    0    0    -1  
$EndComp
$Comp
L C C16
U 1 1 592D2733
P 7100 4250
F 0 "C16" H 7125 4350 50  0000 L CNN
F 1 "1u" H 7125 4150 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 7138 4100 50  0001 C CNN
F 3 "" H 7100 4250 50  0001 C CNN
F 4 "490-10018-1-ND" H 7100 4250 60  0001 C CNN "Part No."
	1    7100 4250
	1    0    0    -1  
$EndComp
Wire Wire Line
	6800 4000 7800 4000
Wire Wire Line
	6800 4000 6800 4100
Connection ~ 7800 4000
Wire Wire Line
	7100 4100 7100 4000
Connection ~ 7100 4000
$Comp
L GND #PWR056
U 1 1 592D28AB
P 7100 4600
F 0 "#PWR056" H 7100 4350 50  0001 C CNN
F 1 "GND" H 7100 4450 50  0000 C CNN
F 2 "" H 7100 4600 50  0001 C CNN
F 3 "" H 7100 4600 50  0001 C CNN
	1    7100 4600
	1    0    0    -1  
$EndComp
Wire Wire Line
	7100 4400 7100 4600
Wire Wire Line
	7100 4500 6800 4500
Wire Wire Line
	6800 4500 6800 4400
Connection ~ 7100 4500
Text Label 7700 4400 2    60   ~ 0
~OLED_RST
Wire Wire Line
	7700 4400 7900 4400
Text Label 5850 6300 0    60   ~ 0
ACC_INT
Wire Wire Line
	5500 6300 5850 6300
NoConn ~ 5500 6400
Text HLabel 9350 1800 2    60   Input ~ 0
CURRENT_SENSE
Wire Wire Line
	6300 1800 9350 1800
Text Notes 6300 1800 0    60   ~ 0
ADC_IN1
Text Label 6850 1800 0    60   ~ 0
CURRENT_SENSE
$Comp
L R R13
U 1 1 592D6710
P 8900 2050
F 0 "R13" V 8980 2050 50  0000 C CNN
F 1 "SPR" V 8900 2050 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 8830 2050 50  0001 C CNN
F 3 "" H 8900 2050 50  0001 C CNN
	1    8900 2050
	1    0    0    -1  
$EndComp
Wire Wire Line
	8900 1900 8900 1800
Connection ~ 8900 1800
$Comp
L GND #PWR057
U 1 1 592D683B
P 8900 2300
F 0 "#PWR057" H 8900 2050 50  0001 C CNN
F 1 "GND" H 8900 2150 50  0000 C CNN
F 2 "" H 8900 2300 50  0001 C CNN
F 3 "" H 8900 2300 50  0001 C CNN
	1    8900 2300
	1    0    0    -1  
$EndComp
Wire Wire Line
	8900 2300 8900 2200
$Comp
L C C20
U 1 1 592D6C20
P 9300 5450
F 0 "C20" H 9325 5550 50  0000 L CNN
F 1 "2u2" H 9325 5350 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 9338 5300 50  0001 C CNN
F 3 "" H 9300 5450 50  0001 C CNN
F 4 "490-13234-1-ND" H 9300 5450 60  0001 C CNN "Part No."
	1    9300 5450
	1    0    0    -1  
$EndComp
$Comp
L C C19
U 1 1 592D6D98
P 9000 5450
F 0 "C19" H 9025 5550 50  0000 L CNN
F 1 "2u2" H 9025 5350 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 9038 5300 50  0001 C CNN
F 3 "" H 9000 5450 50  0001 C CNN
F 4 "490-13234-1-ND" H 9000 5450 60  0001 C CNN "Part No."
	1    9000 5450
	1    0    0    -1  
$EndComp
Wire Wire Line
	9000 5700 9500 5700
Wire Wire Line
	9000 5700 9000 5600
Connection ~ 9500 5700
Wire Wire Line
	9300 5600 9300 5700
Connection ~ 9300 5700
Wire Wire Line
	9300 5300 9300 5100
Wire Wire Line
	9300 5100 8900 5100
Wire Wire Line
	8900 5200 9000 5200
Wire Wire Line
	9000 5200 9000 5300
Text HLabel 4300 2600 0    60   Input ~ 0
~TIP_FAULT
Wire Wire Line
	4300 2600 4900 2600
Text Label 4700 2700 2    60   ~ 0
ACC_INT
Wire Wire Line
	4700 2700 4900 2700
Text Label 4300 2800 2    60   ~ 0
~OLED_RST
Wire Wire Line
	4300 2800 4900 2800
$Comp
L R R17
U 1 1 592D91B1
P 5600 5950
F 0 "R17" V 5680 5950 50  0000 C CNN
F 1 "4K7" V 5600 5950 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 5530 5950 50  0001 C CNN
F 3 "" H 5600 5950 50  0001 C CNN
F 4 "311-4.7KLRCT-ND" H 5600 5950 60  0001 C CNN "Part No."
	1    5600 5950
	1    0    0    -1  
$EndComp
Wire Wire Line
	5600 6100 5600 6300
Connection ~ 5600 6300
$Comp
L +3.3V #PWR058
U 1 1 592D92EB
P 5600 5700
F 0 "#PWR058" H 5600 5550 50  0001 C CNN
F 1 "+3.3V" H 5600 5840 50  0000 C CNN
F 2 "" H 5600 5700 50  0001 C CNN
F 3 "" H 5600 5700 50  0001 C CNN
	1    5600 5700
	1    0    0    -1  
$EndComp
Wire Wire Line
	5600 5700 5600 5800
NoConn ~ 4900 3000
NoConn ~ 6300 2700
$Comp
L GND #PWR059
U 1 1 592E2DAB
P 1200 5000
F 0 "#PWR059" H 1200 4750 50  0001 C CNN
F 1 "GND" H 1200 4850 50  0000 C CNN
F 2 "" H 1200 5000 50  0001 C CNN
F 3 "" H 1200 5000 50  0001 C CNN
	1    1200 5000
	1    0    0    -1  
$EndComp
Wire Wire Line
	1200 4100 1200 5000
Text Label 2200 4100 0    60   ~ 0
SW_A
Wire Wire Line
	2200 4100 2000 4100
Text Label 2200 4600 0    60   ~ 0
SW_B
Wire Wire Line
	2200 4600 2000 4600
NoConn ~ 6300 3000
Text Label 6600 1900 0    60   ~ 0
SW_A
Text Label 6600 2000 0    60   ~ 0
SW_B
Wire Wire Line
	6300 2000 6600 2000
Wire Wire Line
	6600 1900 6300 1900
$Comp
L Speaker_Crystal LS1
U 1 1 592F94D2
P 6000 4900
F 0 "LS1" H 6025 5125 50  0000 R CNN
F 1 "PKMCS0909E4000" H 6025 5050 50  0000 R CNN
F 2 "rt-soldering-iron:Murata-PKMCS" H 5965 4850 50  0001 C CNN
F 3 "" H 5965 4850 50  0001 C CNN
F 4 "490-9647-1-ND" H 6000 4900 60  0001 C CNN "Part No."
	1    6000 4900
	1    0    0    -1  
$EndComp
$Comp
L Q_NPN_BEC Q3
U 1 1 592F9D98
P 4900 5000
F 0 "Q3" H 5100 5050 50  0000 L CNN
F 1 "MMBT3904" H 5100 4950 50  0000 L CNN
F 2 "TO_SOT_Packages_SMD:SOT-323_SC-70" H 5100 5100 50  0001 C CNN
F 3 "" H 4900 5000 50  0001 C CNN
F 4 "MMBT3904WT1GOSCT-ND" H 4900 5000 60  0001 C CNN "Part No."
	1    4900 5000
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR060
U 1 1 592FA0EB
P 5000 5300
F 0 "#PWR060" H 5000 5050 50  0001 C CNN
F 1 "GND" H 5000 5150 50  0000 C CNN
F 2 "" H 5000 5300 50  0001 C CNN
F 3 "" H 5000 5300 50  0001 C CNN
	1    5000 5300
	1    0    0    -1  
$EndComp
$Comp
L R R19
U 1 1 592FA1C8
P 5000 4450
F 0 "R19" V 5080 4450 50  0000 C CNN
F 1 "270" V 5000 4450 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 4930 4450 50  0001 C CNN
F 3 "" H 5000 4450 50  0001 C CNN
F 4 "RR05P270DCT-ND" H 5000 4450 60  0001 C CNN "Part No."
	1    5000 4450
	1    0    0    -1  
$EndComp
$Comp
L +3.3V #PWR061
U 1 1 592FA2D6
P 5000 4200
F 0 "#PWR061" H 5000 4050 50  0001 C CNN
F 1 "+3.3V" H 5000 4340 50  0000 C CNN
F 2 "" H 5000 4200 50  0001 C CNN
F 3 "" H 5000 4200 50  0001 C CNN
	1    5000 4200
	1    0    0    -1  
$EndComp
Wire Wire Line
	5000 4200 5000 4300
$Comp
L GND #PWR062
U 1 1 592FAF19
P 5700 5300
F 0 "#PWR062" H 5700 5050 50  0001 C CNN
F 1 "GND" H 5700 5150 50  0000 C CNN
F 2 "" H 5700 5300 50  0001 C CNN
F 3 "" H 5700 5300 50  0001 C CNN
	1    5700 5300
	1    0    0    -1  
$EndComp
Wire Wire Line
	5800 5000 5700 5000
Wire Wire Line
	5700 5000 5700 5300
Wire Wire Line
	5000 5200 5000 5300
Wire Wire Line
	5000 4600 5000 4800
Wire Wire Line
	5000 4700 5700 4700
Wire Wire Line
	5700 4700 5700 4900
Wire Wire Line
	5700 4900 5800 4900
Connection ~ 5000 4700
$Comp
L R R20
U 1 1 592FB329
P 4350 5000
F 0 "R20" V 4430 5000 50  0000 C CNN
F 1 "1K" V 4350 5000 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 4280 5000 50  0001 C CNN
F 3 "" H 4350 5000 50  0001 C CNN
F 4 "311-1.0KJRCT-ND" H 4350 5000 60  0001 C CNN "Part No."
	1    4350 5000
	0    1    1    0   
$EndComp
Wire Wire Line
	4500 5000 4700 5000
Text Notes 6350 2500 0    60   ~ 0
TIM1_CH1
Wire Wire Line
	6300 2500 6900 2500
Text Label 6900 2500 0    60   ~ 0
BUZ_CTL
Text Label 4050 5000 2    60   ~ 0
BUZ_CTL
Wire Wire Line
	4050 5000 4200 5000
Text Label 4550 5000 0    60   ~ 0
BUZ_BASE
Text Label 5100 4700 0    60   ~ 0
BUZ_COL
Text HLabel 4300 3200 0    60   Input ~ 0
HEATER_LATCH_RESET
$Comp
L STM32F031G U4
U 1 1 59208351
P 5600 2450
F 0 "U4" H 5600 1550 60  0000 C CNN
F 1 "STM32F031G" H 5600 3350 60  0000 C CNN
F 2 "rt-soldering-iron:28-UFQFPN" H 5400 2300 60  0001 C CNN
F 3 "" H 5400 2300 60  0001 C CNN
F 4 "497-13619-ND" H 5600 2450 60  0001 C CNN "Part No."
	1    5600 2450
	1    0    0    -1  
$EndComp
Wire Wire Line
	4900 2900 4400 2900
Wire Wire Line
	4400 2900 4400 3200
Wire Wire Line
	4400 3200 4300 3200
NoConn ~ 6300 2600
$Comp
L SPST SW1
U 1 1 59AB3E45
P 1650 4100
F 0 "SW1" H 1650 4000 60  0000 C CNN
F 1 "SPST" H 1650 4300 60  0000 C CNN
F 2 "rt-soldering-iron:Panasonic_EVQP4" H 1650 4100 60  0001 C CNN
F 3 "" H 1650 4100 60  0001 C CNN
F 4 "P15502CT-ND" H 1650 4100 60  0001 C CNN "Part No."
	1    1650 4100
	1    0    0    -1  
$EndComp
$Comp
L SPST SW2
U 1 1 59AB3ED8
P 1650 4600
F 0 "SW2" H 1650 4500 60  0000 C CNN
F 1 "SPST" H 1650 4800 60  0000 C CNN
F 2 "rt-soldering-iron:Panasonic_EVQP4" H 1650 4600 60  0001 C CNN
F 3 "" H 1650 4600 60  0001 C CNN
F 4 "P15502CT-ND" H 1650 4600 60  0001 C CNN "Part No."
	1    1650 4600
	1    0    0    -1  
$EndComp
Wire Wire Line
	1300 4100 1200 4100
Wire Wire Line
	1300 4600 1200 4600
Connection ~ 1200 4600
NoConn ~ 6300 2100
NoConn ~ 6300 2200
NoConn ~ 6300 2300
$EndSCHEMATC
