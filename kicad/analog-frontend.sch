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
Sheet 2 4
Title "RT Soldering Iron"
Date "2017-11-06"
Rev "B"
Comp "Kevin Cuzner"
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L Q_PMOS_DGSD Q1
U 1 1 592080F2
P 7200 3100
F 0 "Q1" H 7400 3150 50  0000 L CNN
F 1 "SIA483DJ-T1-GE3" H 7400 3050 50  0000 L CNN
F 2 "rt-soldering-iron:PP-SC70-6L" H 7400 3200 50  0001 C CNN
F 3 "" H 7200 3100 50  0001 C CNN
F 4 "SIA483DJ-T1-GE3CT-ND" H 7200 3100 60  0001 C CNN "Part No."
	1    7200 3100
	1    0    0    1   
$EndComp
$Comp
L CONN_BARREL_STEREO J1
U 1 1 59208262
P 8100 3900
F 0 "J1" H 8100 3700 60  0000 C CNN
F 1 "IRON_TIP" H 8100 4100 60  0000 C CNN
F 2 "rt-soldering-iron:SJ-3502-SMT" H 8100 3900 60  0001 C CNN
F 3 "" H 8100 3900 60  0001 C CNN
F 4 "CP-3502SJCT-ND" H 8100 3900 60  0001 C CNN "Part No."
	1    8100 3900
	-1   0    0    1   
$EndComp
$Comp
L GND #PWR01
U 1 1 592A622A
P 7600 4100
F 0 "#PWR01" H 7600 3850 50  0001 C CNN
F 1 "GND" H 7600 3950 50  0000 C CNN
F 2 "" H 7600 4100 50  0001 C CNN
F 3 "" H 7600 4100 50  0001 C CNN
	1    7600 4100
	1    0    0    -1  
$EndComp
$Comp
L +12V #PWR02
U 1 1 592A6425
P 8900 800
F 0 "#PWR02" H 8900 650 50  0001 C CNN
F 1 "+12V" H 8900 940 50  0000 C CNN
F 2 "" H 8900 800 50  0001 C CNN
F 3 "" H 8900 800 50  0001 C CNN
	1    8900 800 
	-1   0    0    -1  
$EndComp
$Comp
L R R2
U 1 1 592A649F
P 6700 2850
F 0 "R2" V 6780 2850 50  0000 C CNN
F 1 "10K" V 6700 2850 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 6630 2850 50  0001 C CNN
F 3 "" H 6700 2850 50  0001 C CNN
F 4 "311-10.0KLRCT-ND" H 6700 2850 60  0001 C CNN "Part No."
	1    6700 2850
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR03
U 1 1 592A6A31
P 6700 4000
F 0 "#PWR03" H 6700 3750 50  0001 C CNN
F 1 "GND" H 6700 3850 50  0000 C CNN
F 2 "" H 6700 4000 50  0001 C CNN
F 3 "" H 6700 4000 50  0001 C CNN
	1    6700 4000
	-1   0    0    -1  
$EndComp
$Comp
L R R3
U 1 1 592A6A68
P 5550 3400
F 0 "R3" V 5630 3400 50  0000 C CNN
F 1 "1K" V 5550 3400 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 5480 3400 50  0001 C CNN
F 3 "" H 5550 3400 50  0001 C CNN
F 4 "311-1.0KJRCT-ND" H 5550 3400 60  0001 C CNN "Part No."
	1    5550 3400
	0    -1   1    0   
$EndComp
$Comp
L R R4
U 1 1 592A6B77
P 6300 3650
F 0 "R4" V 6380 3650 50  0000 C CNN
F 1 "10K" V 6300 3650 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 6230 3650 50  0001 C CNN
F 3 "" H 6300 3650 50  0001 C CNN
F 4 "311-10.0KLRCT-ND" H 6300 3650 60  0001 C CNN "Part No."
	1    6300 3650
	-1   0    0    -1  
$EndComp
Text Label 7500 3900 2    60   ~ 0
SENSE_IN
Text HLabel 4400 3400 0    60   Input ~ 0
HEATER_CTL
Text Label 4950 3400 2    60   ~ 0
HEATER_CTL
$Comp
L R R6
U 1 1 592BA873
P 5900 5250
F 0 "R6" V 5980 5250 50  0000 C CNN
F 1 "270" V 5900 5250 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 5830 5250 50  0001 C CNN
F 3 "" H 5900 5250 50  0001 C CNN
F 4 "RR05P270DCT-ND" H 5900 5250 60  0001 C CNN "Part No."
	1    5900 5250
	1    0    0    -1  
$EndComp
$Comp
L R R5
U 1 1 592BA8FA
P 7350 5000
F 0 "R5" V 7430 5000 50  0000 C CNN
F 1 "100K" V 7350 5000 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 7280 5000 50  0001 C CNN
F 3 "" H 7350 5000 50  0001 C CNN
F 4 "RR05P100KDCT-ND" H 7350 5000 60  0001 C CNN "Part No."
	1    7350 5000
	0    1    1    0   
$EndComp
$Comp
L GND #PWR04
U 1 1 592BA9A1
P 5900 5500
F 0 "#PWR04" H 5900 5250 50  0001 C CNN
F 1 "GND" H 5900 5350 50  0000 C CNN
F 2 "" H 5900 5500 50  0001 C CNN
F 3 "" H 5900 5500 50  0001 C CNN
	1    5900 5500
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR05
U 1 1 592BA9F6
P 7400 6300
F 0 "#PWR05" H 7400 6050 50  0001 C CNN
F 1 "GND" H 7400 6150 50  0000 C CNN
F 2 "" H 7400 6300 50  0001 C CNN
F 3 "" H 7400 6300 50  0001 C CNN
	1    7400 6300
	1    0    0    -1  
$EndComp
Text HLabel 9700 5800 2    60   Input ~ 0
HEATER_SENSE
Text Label 9400 5500 0    60   ~ 0
HEATER_SENSE
$Comp
L C C3
U 1 1 592CA29D
P 6050 6500
F 0 "C3" H 6075 6600 50  0000 L CNN
F 1 "0.1u" H 6075 6400 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 6088 6350 50  0001 C CNN
F 3 "" H 6050 6500 50  0001 C CNN
F 4 "490-14603-1-ND" H 6050 6500 60  0001 C CNN "Part No."
	1    6050 6500
	1    0    0    -1  
$EndComp
$Comp
L +3.3VA #PWR06
U 1 1 592CA321
P 7400 5400
F 0 "#PWR06" H 7400 5250 50  0001 C CNN
F 1 "+3.3VA" H 7400 5540 50  0000 C CNN
F 2 "" H 7400 5400 50  0001 C CNN
F 3 "" H 7400 5400 50  0001 C CNN
	1    7400 5400
	1    0    0    -1  
$EndComp
$Comp
L +3.3VA #PWR07
U 1 1 592CA3C9
P 6050 6250
F 0 "#PWR07" H 6050 6100 50  0001 C CNN
F 1 "+3.3VA" H 6050 6390 50  0000 C CNN
F 2 "" H 6050 6250 50  0001 C CNN
F 3 "" H 6050 6250 50  0001 C CNN
	1    6050 6250
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR08
U 1 1 592CA3FB
P 6050 6750
F 0 "#PWR08" H 6050 6500 50  0001 C CNN
F 1 "GND" H 6050 6600 50  0000 C CNN
F 2 "" H 6050 6750 50  0001 C CNN
F 3 "" H 6050 6750 50  0001 C CNN
	1    6050 6750
	1    0    0    -1  
$EndComp
$Comp
L R R8
U 1 1 592CA953
P 4150 5900
F 0 "R8" V 4230 5900 50  0000 C CNN
F 1 "4K7" V 4150 5900 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 4080 5900 50  0001 C CNN
F 3 "" H 4150 5900 50  0001 C CNN
F 4 "311-4.7KLRCT-ND" H 4150 5900 60  0001 C CNN "Part No."
	1    4150 5900
	0    1    1    0   
$EndComp
$Comp
L C C2
U 1 1 592CAA63
P 4900 6250
F 0 "C2" H 4925 6350 50  0000 L CNN
F 1 "0.001u" H 4925 6150 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 4938 6100 50  0001 C CNN
F 3 "" H 4900 6250 50  0001 C CNN
F 4 "1276-1057-1-ND" H 4900 6250 60  0001 C CNN "Part No."
	1    4900 6250
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR09
U 1 1 592CAAF6
P 4900 6500
F 0 "#PWR09" H 4900 6250 50  0001 C CNN
F 1 "GND" H 4900 6350 50  0000 C CNN
F 2 "" H 4900 6500 50  0001 C CNN
F 3 "" H 4900 6500 50  0001 C CNN
	1    4900 6500
	1    0    0    -1  
$EndComp
$Comp
L R R7
U 1 1 592D1D47
P 3800 5650
F 0 "R7" V 3880 5650 50  0000 C CNN
F 1 "1M" V 3800 5650 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 3730 5650 50  0001 C CNN
F 3 "" H 3800 5650 50  0001 C CNN
F 4 "311-1.00MLRCT-ND" H 3800 5650 60  0001 C CNN "Part No."
	1    3800 5650
	1    0    0    -1  
$EndComp
$Comp
L +3.3VA #PWR010
U 1 1 592D1E60
P 3800 5400
F 0 "#PWR010" H 3800 5250 50  0001 C CNN
F 1 "+3.3VA" H 3800 5540 50  0000 C CNN
F 2 "" H 3800 5400 50  0001 C CNN
F 3 "" H 3800 5400 50  0001 C CNN
	1    3800 5400
	1    0    0    -1  
$EndComp
$Comp
L ACS711xEX U1
U 1 1 592D4A15
P 8200 2100
F 0 "U1" H 8200 1550 60  0000 C CNN
F 1 "ACS711xEX" H 8200 2650 60  0000 C CNN
F 2 "rt-soldering-iron:12-WFQFN" H 8200 2100 60  0001 C CNN
F 3 "" H 8200 2100 60  0001 C CNN
F 4 "620-1483-1-ND" H 8200 2100 60  0001 C CNN "Part No."
	1    8200 2100
	-1   0    0    -1  
$EndComp
$Comp
L GND #PWR011
U 1 1 592D5006
P 7500 2600
F 0 "#PWR011" H 7500 2350 50  0001 C CNN
F 1 "GND" H 7500 2450 50  0000 C CNN
F 2 "" H 7500 2600 50  0001 C CNN
F 3 "" H 7500 2600 50  0001 C CNN
	1    7500 2600
	-1   0    0    -1  
$EndComp
$Comp
L +3.3VA #PWR012
U 1 1 592D518F
P 9200 1300
F 0 "#PWR012" H 9200 1150 50  0001 C CNN
F 1 "+3.3VA" H 9200 1440 50  0000 C CNN
F 2 "" H 9200 1300 50  0001 C CNN
F 3 "" H 9200 1300 50  0001 C CNN
	1    9200 1300
	-1   0    0    -1  
$EndComp
Text HLabel 9100 2200 2    60   Input ~ 0
CURRENT_SENSE
$Comp
L R R1
U 1 1 592D53AA
P 9800 2750
F 0 "R1" V 9880 2750 50  0000 C CNN
F 1 "4K7" V 9800 2750 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 9730 2750 50  0001 C CNN
F 3 "" H 9800 2750 50  0001 C CNN
F 4 "311-4.7KLRCT-ND" H 9800 2750 60  0001 C CNN "Part No."
	1    9800 2750
	-1   0    0    -1  
$EndComp
$Comp
L +3.3V #PWR013
U 1 1 592D545D
P 9800 2500
F 0 "#PWR013" H 9800 2350 50  0001 C CNN
F 1 "+3.3V" H 9800 2640 50  0000 C CNN
F 2 "" H 9800 2500 50  0001 C CNN
F 3 "" H 9800 2500 50  0001 C CNN
	1    9800 2500
	-1   0    0    -1  
$EndComp
$Comp
L GND #PWR014
U 1 1 592D5804
P 8900 2600
F 0 "#PWR014" H 8900 2350 50  0001 C CNN
F 1 "GND" H 8900 2450 50  0000 C CNN
F 2 "" H 8900 2600 50  0001 C CNN
F 3 "" H 8900 2600 50  0001 C CNN
	1    8900 2600
	-1   0    0    -1  
$EndComp
Text HLabel 9900 3000 2    60   Input ~ 0
~FAULT
Text Label 9700 3000 2    60   ~ 0
~TIP_FAULT
$Comp
L C C1
U 1 1 592D9EC4
P 8700 6050
F 0 "C1" H 8725 6150 50  0000 L CNN
F 1 "0.001u" H 8725 5950 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 8738 5900 50  0001 C CNN
F 3 "" H 8700 6050 50  0001 C CNN
F 4 "1276-1057-1-ND" H 8700 6050 60  0001 C CNN "Part No."
	1    8700 6050
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR015
U 1 1 592D9ECB
P 8700 6300
F 0 "#PWR015" H 8700 6050 50  0001 C CNN
F 1 "GND" H 8700 6150 50  0000 C CNN
F 2 "" H 8700 6300 50  0001 C CNN
F 3 "" H 8700 6300 50  0001 C CNN
	1    8700 6300
	1    0    0    -1  
$EndComp
Text Label 7550 1700 2    60   ~ 0
12V_TIP
Text Label 6950 3100 2    60   ~ 0
TIP_GATE
Text Label 6100 3200 2    60   ~ 0
TIP_GATE_B
Text Label 7600 3650 2    60   ~ 0
TIP_CONNECTOR
Text Label 5550 5900 0    60   ~ 0
OPAMP_SENSE_IN
Text Label 6300 5000 0    60   ~ 0
OPAMP_FB
$Comp
L C C25
U 1 1 592E6240
P 9850 1650
F 0 "C25" H 9875 1750 50  0000 L CNN
F 1 "0.1u" H 9875 1550 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 9888 1500 50  0001 C CNN
F 3 "" H 9850 1650 50  0001 C CNN
F 4 "490-14603-1-ND" H 9850 1650 60  0001 C CNN "Part No."
	1    9850 1650
	-1   0    0    -1  
$EndComp
$Comp
L GND #PWR016
U 1 1 592E63D8
P 9850 1900
F 0 "#PWR016" H 9850 1650 50  0001 C CNN
F 1 "GND" H 9850 1750 50  0000 C CNN
F 2 "" H 9850 1900 50  0001 C CNN
F 3 "" H 9850 1900 50  0001 C CNN
	1    9850 1900
	-1   0    0    -1  
$EndComp
$Comp
L MCP6V71U U2
U 1 1 5930E5C8
P 7400 5800
F 0 "U2" H 7550 5650 60  0000 C CNN
F 1 "MCP6V71U" H 7750 5950 60  0000 C CNN
F 2 "TO_SOT_Packages_SMD:SOT-353_SC-70-5" H 7400 5800 60  0001 C CNN
F 3 "" H 7400 5800 60  0001 C CNN
F 4 "MCP6V71UT-E/LTYCT-ND" H 7400 5800 60  0001 C CNN "Part No."
	1    7400 5800
	1    0    0    -1  
$EndComp
$Comp
L +3.3VA #PWR017
U 1 1 5930F993
P 3650 6300
F 0 "#PWR017" H 3650 6150 50  0001 C CNN
F 1 "+3.3VA" H 3650 6440 50  0000 C CNN
F 2 "" H 3650 6300 50  0001 C CNN
F 3 "" H 3650 6300 50  0001 C CNN
	1    3650 6300
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR018
U 1 1 5930FA31
P 4150 7350
F 0 "#PWR018" H 4150 7100 50  0001 C CNN
F 1 "GND" H 4150 7200 50  0000 C CNN
F 2 "" H 4150 7350 50  0001 C CNN
F 3 "" H 4150 7350 50  0001 C CNN
	1    4150 7350
	1    0    0    -1  
$EndComp
$Comp
L C C26
U 1 1 593100DD
P 7400 4650
F 0 "C26" H 7425 4750 50  0000 L CNN
F 1 "SPR" H 7425 4550 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 7438 4500 50  0001 C CNN
F 3 "" H 7400 4650 50  0001 C CNN
	1    7400 4650
	0    1    1    0   
$EndComp
Text Label 3300 5900 2    60   ~ 0
SENSE_IN
Text Notes 8400 5100 0    60   ~ 0
Possible drop-in replacements:\nOPA333 - 296-19547-1-ND
$Comp
L Q_NMOS_SGD_DUAL Q7
U 2 1 5999E031
P 3300 2200
F 0 "Q7" H 3500 2250 50  0000 L CNN
F 1 "FDG6301N_F085" H 3500 2150 50  0000 L CNN
F 2 "TO_SOT_Packages_SMD:SOT-363_SC-70-6" H 3500 2300 50  0001 C CNN
F 3 "" H 3300 2200 50  0001 C CNN
F 4 "FDG6301N-F085CT-ND" H 3300 2200 60  0001 C CNN "Part No."
	2    3300 2200
	-1   0    0    -1  
$EndComp
$Comp
L Q_NMOS_SGD_DUAL Q7
U 1 1 5999E0BC
P 4800 2100
F 0 "Q7" H 5000 2150 50  0000 L CNN
F 1 "FDG6301N_F085" H 5000 2050 50  0000 L CNN
F 2 "TO_SOT_Packages_SMD:SOT-363_SC-70-6" H 5000 2200 50  0001 C CNN
F 3 "" H 4800 2100 50  0001 C CNN
F 4 "FDG6301N-F085CT-ND" H 4800 2100 60  0001 C CNN "Part No."
	1    4800 2100
	1    0    0    -1  
$EndComp
Text Label 1600 2300 2    60   ~ 0
HEATER_SENSE
Wire Wire Line
	7600 4000 7600 4100
Wire Wire Line
	7700 4000 7600 4000
Wire Wire Line
	7400 2800 7400 2900
Wire Wire Line
	7600 3800 7700 3800
Wire Wire Line
	7600 3300 7600 3800
Wire Wire Line
	7300 3400 7300 3300
Wire Wire Line
	7300 3400 7600 3400
Connection ~ 7600 3400
Wire Wire Line
	7500 3300 7500 3400
Connection ~ 7500 3400
Wire Wire Line
	7400 3300 7400 3400
Connection ~ 7400 3400
Wire Wire Line
	6700 1400 6700 2700
Wire Wire Line
	6700 3000 6700 3200
Wire Wire Line
	6700 3100 7000 3100
Connection ~ 6700 3100
Wire Wire Line
	6700 3600 6700 4000
Wire Wire Line
	5700 3400 6400 3400
Wire Wire Line
	6300 3800 6300 3900
Wire Wire Line
	6300 3900 6700 3900
Connection ~ 6700 3900
Wire Wire Line
	6300 3500 6300 3400
Connection ~ 6300 3400
Wire Wire Line
	7700 3900 7500 3900
Wire Wire Line
	7400 5400 7400 5500
Wire Wire Line
	5400 3400 4400 3400
Wire Wire Line
	5900 5000 7200 5000
Wire Wire Line
	7100 5700 6200 5700
Wire Wire Line
	5900 5500 5900 5400
Wire Wire Line
	7400 6300 7400 6100
Wire Wire Line
	7700 5800 9700 5800
Wire Wire Line
	8200 5000 8200 5800
Wire Wire Line
	7500 5000 8200 5000
Connection ~ 8200 5800
Wire Wire Line
	6050 6750 6050 6650
Wire Wire Line
	6050 6350 6050 6250
Wire Wire Line
	5900 5100 5900 5000
Wire Wire Line
	6200 5700 6200 5000
Connection ~ 6200 5000
Wire Wire Line
	3300 5900 4000 5900
Wire Wire Line
	4300 5900 7100 5900
Wire Wire Line
	4900 6100 4900 5900
Connection ~ 4900 5900
Wire Wire Line
	4900 6500 4900 6400
Wire Wire Line
	3800 5400 3800 5500
Wire Wire Line
	3800 5800 3800 5900
Connection ~ 3800 5900
Wire Wire Line
	8900 800  8900 1700
Wire Wire Line
	8900 1700 8800 1700
Wire Wire Line
	7600 1700 7300 1700
Wire Wire Line
	7300 1700 7300 2900
Wire Wire Line
	7400 2800 7300 2800
Connection ~ 7300 2800
Wire Wire Line
	6700 1400 8900 1400
Connection ~ 8900 1400
Wire Wire Line
	7500 2200 7500 2600
Wire Wire Line
	7500 2200 7600 2200
Wire Wire Line
	7600 2300 7500 2300
Connection ~ 7500 2300
Wire Wire Line
	7600 2400 7500 2400
Connection ~ 7500 2400
Wire Wire Line
	7600 2500 7500 2500
Connection ~ 7500 2500
Wire Wire Line
	9200 1300 9200 2000
Wire Wire Line
	9200 2000 8800 2000
Wire Wire Line
	9100 2200 8800 2200
Wire Wire Line
	9800 2500 9800 2600
Wire Wire Line
	9800 2900 9800 3000
Wire Wire Line
	9200 3000 9900 3000
Wire Wire Line
	9200 3000 9200 2400
Wire Wire Line
	9200 2400 8800 2400
Wire Wire Line
	8900 2600 8900 2500
Wire Wire Line
	8900 2500 8800 2500
Connection ~ 9800 3000
Wire Wire Line
	8700 6300 8700 6200
Wire Wire Line
	8700 5900 8700 5800
Connection ~ 8700 5800
Wire Wire Line
	9850 1900 9850 1800
Wire Wire Line
	9850 1500 9850 1400
Wire Wire Line
	9850 1400 9200 1400
Connection ~ 9200 1400
Wire Wire Line
	3650 6300 3650 6400
Wire Wire Line
	4150 7350 4150 7250
Wire Wire Line
	4350 6950 4450 6950
Wire Wire Line
	4450 6950 4450 5900
Connection ~ 4450 5900
Wire Wire Line
	7250 4650 6900 4650
Wire Wire Line
	6900 4650 6900 5000
Connection ~ 6900 5000
Wire Wire Line
	7550 4650 7700 4650
Wire Wire Line
	7700 4650 7700 5000
Connection ~ 7700 5000
$Comp
L R R29
U 1 1 5999F62C
P 800 1850
F 0 "R29" V 880 1850 50  0000 C CNN
F 1 "16K2" V 800 1850 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 730 1850 50  0001 C CNN
F 3 "" H 800 1850 50  0001 C CNN
F 4 "311-2144-1-ND" H 800 1850 60  0001 C CNN "Part No."
	1    800  1850
	1    0    0    -1  
$EndComp
$Comp
L R R31
U 1 1 5999F691
P 800 2350
F 0 "R31" V 880 2350 50  0000 C CNN
F 1 "100K" V 800 2350 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 730 2350 50  0001 C CNN
F 3 "" H 800 2350 50  0001 C CNN
F 4 "RR05P100KDCT-ND" H 800 2350 60  0001 C CNN "Part No."
	1    800  2350
	1    0    0    -1  
$EndComp
Wire Wire Line
	1600 2300 1700 2300
Wire Wire Line
	800  1600 800  1700
Wire Wire Line
	800  2000 800  2200
$Comp
L GND #PWR019
U 1 1 5999F8D4
P 800 2800
F 0 "#PWR019" H 800 2550 50  0001 C CNN
F 1 "GND" H 800 2650 50  0000 C CNN
F 2 "" H 800 2800 50  0001 C CNN
F 3 "" H 800 2800 50  0001 C CNN
	1    800  2800
	1    0    0    -1  
$EndComp
Wire Wire Line
	800  2500 800  2800
Wire Wire Line
	800  2700 4900 2700
Wire Wire Line
	3200 2700 3200 2400
$Comp
L R R27
U 1 1 5999FB38
P 2400 1050
F 0 "R27" V 2480 1050 50  0000 C CNN
F 1 "10K" V 2400 1050 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 2330 1050 50  0001 C CNN
F 3 "" H 2400 1050 50  0001 C CNN
F 4 "311-10.0KLRCT-ND" H 2400 1050 60  0001 C CNN "Part No."
	1    2400 1050
	1    0    0    -1  
$EndComp
$Comp
L R R28
U 1 1 5999FBC7
P 2400 1550
F 0 "R28" V 2480 1550 50  0000 C CNN
F 1 "1K" V 2400 1550 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 2330 1550 50  0001 C CNN
F 3 "" H 2400 1550 50  0001 C CNN
F 4 "311-1.0KJRCT-ND" H 2400 1550 60  0001 C CNN "Part No."
	1    2400 1550
	1    0    0    -1  
$EndComp
Wire Wire Line
	2400 1700 2400 2200
Wire Wire Line
	2400 1200 2400 1400
Wire Wire Line
	2400 1800 3200 1800
Wire Wire Line
	3200 1800 3200 2000
Connection ~ 2400 1800
$Comp
L +12V #PWR020
U 1 1 5999FEE9
P 2400 700
F 0 "#PWR020" H 2400 550 50  0001 C CNN
F 1 "+12V" H 2400 840 50  0000 C CNN
F 2 "" H 2400 700 50  0001 C CNN
F 3 "" H 2400 700 50  0001 C CNN
	1    2400 700 
	1    0    0    -1  
$EndComp
Wire Wire Line
	2400 700  2400 900 
Wire Wire Line
	2400 1300 3300 1300
Connection ~ 2400 1300
Wire Wire Line
	2400 800  3600 800 
Wire Wire Line
	3600 800  3600 1100
Connection ~ 2400 800 
$Comp
L R R32
U 1 1 599A05FD
P 3600 2450
F 0 "R32" V 3680 2450 50  0000 C CNN
F 1 "10K" V 3600 2450 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 3530 2450 50  0001 C CNN
F 3 "" H 3600 2450 50  0001 C CNN
F 4 "311-10.0KLRCT-ND" H 3600 2450 60  0001 C CNN "Part No."
	1    3600 2450
	1    0    0    -1  
$EndComp
$Comp
L R R30
U 1 1 599A0680
P 3600 1850
F 0 "R30" V 3680 1850 50  0000 C CNN
F 1 "10K" V 3600 1850 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 3530 1850 50  0001 C CNN
F 3 "" H 3600 1850 50  0001 C CNN
F 4 "311-10.0KLRCT-ND" H 3600 1850 60  0001 C CNN "Part No."
	1    3600 1850
	1    0    0    -1  
$EndComp
Wire Wire Line
	4900 2700 4900 2300
Connection ~ 3200 2700
Wire Wire Line
	3600 2700 3600 2600
Connection ~ 3600 2700
Wire Wire Line
	3600 1500 3600 1700
Wire Wire Line
	3600 2000 3600 2300
Wire Wire Line
	3500 2200 3600 2200
Connection ~ 3600 2200
Wire Wire Line
	3600 2100 4600 2100
Connection ~ 3600 2100
Text Label 5000 1800 0    60   ~ 0
TIP_GATE_B
Wire Wire Line
	5000 1800 4900 1800
Wire Wire Line
	4900 1800 4900 1900
$Comp
L Q_NMOS_SGD_DUAL Q2
U 1 1 599A0F0F
P 2700 4100
F 0 "Q2" H 2900 4150 50  0000 L CNN
F 1 "FDG6301N_F085" H 2900 4050 50  0000 L CNN
F 2 "TO_SOT_Packages_SMD:SOT-363_SC-70-6" H 2900 4200 50  0001 C CNN
F 3 "" H 2700 4100 50  0001 C CNN
F 4 "FDG6301N-F085CT-ND" H 2700 4100 60  0001 C CNN "Part No."
	1    2700 4100
	1    0    0    -1  
$EndComp
Text HLabel 2300 4100 0    60   Input ~ 0
HEATER_LATCH_RESET
Wire Wire Line
	6100 3200 6200 3200
Wire Wire Line
	6200 3200 6200 3400
Connection ~ 6200 3400
Text Label 3900 1900 0    60   ~ 0
HEATER_LATCH
Wire Wire Line
	3900 1900 3800 1900
Wire Wire Line
	3800 1900 3800 2100
Connection ~ 3800 2100
$Comp
L GND #PWR021
U 1 1 599A1D11
P 2800 4400
F 0 "#PWR021" H 2800 4150 50  0001 C CNN
F 1 "GND" H 2800 4250 50  0000 C CNN
F 2 "" H 2800 4400 50  0001 C CNN
F 3 "" H 2800 4400 50  0001 C CNN
	1    2800 4400
	1    0    0    -1  
$EndComp
Wire Wire Line
	2300 4100 2500 4100
Wire Wire Line
	2800 4300 2800 4400
Text Label 3000 3700 0    60   ~ 0
HEATER_LATCH
Wire Wire Line
	2800 3700 3000 3700
Wire Wire Line
	2800 3700 2800 3900
$Comp
L Q_NMOS_SGD_DUAL Q2
U 2 1 599A21BF
P 6600 3400
F 0 "Q2" H 6800 3450 50  0000 L CNN
F 1 "FDG6301N_F085" H 6800 3350 50  0000 L CNN
F 2 "TO_SOT_Packages_SMD:SOT-363_SC-70-6" H 6800 3500 50  0001 C CNN
F 3 "" H 6600 3400 50  0001 C CNN
F 4 "FDG6301N-F085CT-ND" H 6600 3400 60  0001 C CNN "Part No."
	2    6600 3400
	1    0    0    -1  
$EndComp
Wire Wire Line
	9400 5500 9200 5500
Wire Wire Line
	9200 5500 9200 5800
Connection ~ 9200 5800
Text Notes 800  1550 0    60   ~ 0
85% trip point\n\n(2.8V at 3.3V)
Wire Wire Line
	2400 2200 2300 2200
Connection ~ 800  2700
Wire Wire Line
	1700 2100 800  2100
Connection ~ 800  2100
$Comp
L +3.3VA #PWR022
U 1 1 59A7A75B
P 2000 1500
F 0 "#PWR022" H 2000 1350 50  0001 C CNN
F 1 "+3.3VA" H 2000 1640 50  0000 C CNN
F 2 "" H 2000 1500 50  0001 C CNN
F 3 "" H 2000 1500 50  0001 C CNN
	1    2000 1500
	1    0    0    -1  
$EndComp
Wire Wire Line
	2000 1500 2000 1900
Wire Wire Line
	2000 1600 800  1600
Wire Wire Line
	2000 2500 2000 2700
Connection ~ 2000 2700
Connection ~ 2000 1600
$Comp
L TLV1701 U8
U 1 1 59A8D87F
P 2000 2200
F 0 "U8" H 2150 2050 60  0000 C CNN
F 1 "TLV1701" H 2250 1950 60  0000 C CNN
F 2 "TO_SOT_Packages_SMD:SOT-353_SC-70-5" H 2000 2200 60  0001 C CNN
F 3 "" H 2000 2200 60  0001 C CNN
F 4 "296-40600-1-ND" H 2000 2200 60  0001 C CNN "Part No."
	1    2000 2200
	1    0    0    -1  
$EndComp
$Comp
L Q_PNP_BEC Q6
U 1 1 59A8DF25
P 3500 1300
F 0 "Q6" H 3700 1350 50  0000 L CNN
F 1 "MMBT3906WT1G" H 3700 1250 50  0000 L CNN
F 2 "TO_SOT_Packages_SMD:SOT-323_SC-70" H 3700 1400 50  0001 C CNN
F 3 "" H 3500 1300 50  0001 C CNN
F 4 "MMBT3906WT1GOSCT-ND" H 3500 1300 60  0001 C CNN "Part No."
	1    3500 1300
	1    0    0    1   
$EndComp
Text Notes 1700 2850 0    60   ~ 0
U8 was chosen for its high output voltage tolerance
$Comp
L D_x2_Serial_KAC D2
U 1 1 59D1D460
P 4150 6950
F 0 "D2" H 4200 6850 50  0000 C CNN
F 1 "BAV99WT1G" H 4150 7050 50  0000 C CNN
F 2 "TO_SOT_Packages_SMD:SOT-323_SC-70" H 4150 6950 50  0001 C CNN
F 3 "" H 4150 6950 50  0001 C CNN
F 4 "BAV99WT1GOSCT-ND" H 4150 6950 60  0001 C CNN "Part No."
	1    4150 6950
	0    -1   -1   0   
$EndComp
$Comp
L R R34
U 1 1 59D1D623
P 3900 6400
F 0 "R34" V 3980 6400 50  0000 C CNN
F 1 "SPR" V 3900 6400 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 3830 6400 50  0001 C CNN
F 3 "" H 3900 6400 50  0001 C CNN
	1    3900 6400
	0    1    1    0   
$EndComp
$Comp
L R R35
U 1 1 59D1D6E8
P 3900 6550
F 0 "R35" V 3980 6550 50  0000 C CNN
F 1 "0" V 3900 6550 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 3830 6550 50  0001 C CNN
F 3 "" H 3900 6550 50  0001 C CNN
F 4 "311-0.0JRCT-ND" H 3900 6550 60  0001 C CNN "Part No."
	1    3900 6550
	0    1    1    0   
$EndComp
Wire Wire Line
	4050 6400 4150 6400
Wire Wire Line
	4150 6400 4150 6650
Wire Wire Line
	3650 6400 3750 6400
Wire Wire Line
	4050 6550 4150 6550
Connection ~ 4150 6550
$Comp
L GND #PWR023
U 1 1 59D1E7A5
P 3650 6650
F 0 "#PWR023" H 3650 6400 50  0001 C CNN
F 1 "GND" H 3650 6500 50  0000 C CNN
F 2 "" H 3650 6650 50  0001 C CNN
F 3 "" H 3650 6650 50  0001 C CNN
	1    3650 6650
	1    0    0    -1  
$EndComp
Wire Wire Line
	3650 6650 3650 6550
Wire Wire Line
	3650 6550 3750 6550
Text Notes 1850 6150 0    60   ~ 0
K-type thermocouple:\nSeebeck coefficient is 40.46uV/C
Text Notes 9300 6100 0    60   ~ 0
Gain of 371.3 gives\n15.026mV/C
$EndSCHEMATC
