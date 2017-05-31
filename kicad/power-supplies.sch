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
$Descr A4 11693 8268
encoding utf-8
Sheet 3 4
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L BARREL_JACK J2
U 1 1 592082C4
P 1200 1400
F 0 "J2" H 1200 1595 50  0000 C CNN
F 1 "BARREL_JACK" H 1200 1245 50  0000 C CNN
F 2 "rt-soldering-iron:PJ-069B-SMT" H 1200 1400 50  0001 C CNN
F 3 "" H 1200 1400 50  0001 C CNN
F 4 "CP-069BPJCT-ND" H 1200 1400 60  0001 C CNN "Part No."
	1    1200 1400
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR20
U 1 1 592084EC
P 1600 1900
F 0 "#PWR20" H 1600 1650 50  0001 C CNN
F 1 "GND" H 1600 1750 50  0000 C CNN
F 2 "" H 1600 1900 50  0001 C CNN
F 3 "" H 1600 1900 50  0001 C CNN
	1    1600 1900
	1    0    0    -1  
$EndComp
$Comp
L +12V #PWR19
U 1 1 59208505
P 1600 1200
F 0 "#PWR19" H 1600 1050 50  0001 C CNN
F 1 "+12V" H 1600 1340 50  0000 C CNN
F 2 "" H 1600 1200 50  0001 C CNN
F 3 "" H 1600 1200 50  0001 C CNN
	1    1600 1200
	1    0    0    -1  
$EndComp
NoConn ~ 1500 1400
$Comp
L L L1
U 1 1 592087DE
P 5850 1300
F 0 "L1" V 5800 1300 50  0000 C CNN
F 1 "4.7u" V 5925 1300 50  0000 C CNN
F 2 "Inductors:Inductor_Taiyo-Yuden_NR-30xx" H 5850 1300 50  0001 C CNN
F 3 "" H 5850 1300 50  0001 C CNN
F 4 "587-2591-1-ND" H 5850 1300 60  0001 C CNN "Part No."
	1    5850 1300
	0    -1   -1   0   
$EndComp
$Comp
L D_Schottky D1
U 1 1 59208876
P 5500 1550
F 0 "D1" H 5500 1650 50  0000 C CNN
F 1 "CUS10F30" H 5500 1450 50  0000 C CNN
F 2 "Diodes_SMD:D_SOD-323" H 5500 1550 50  0001 C CNN
F 3 "" H 5500 1550 50  0001 C CNN
F 4 "CUS10F30H3FCT-ND" H 5500 1550 60  0001 C CNN "Part No."
	1    5500 1550
	0    1    1    0   
$EndComp
$Comp
L GND #PWR25
U 1 1 592088C5
P 5500 2500
F 0 "#PWR25" H 5500 2250 50  0001 C CNN
F 1 "GND" H 5500 2350 50  0000 C CNN
F 2 "" H 5500 2500 50  0001 C CNN
F 3 "" H 5500 2500 50  0001 C CNN
	1    5500 2500
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR24
U 1 1 59208941
P 4500 2500
F 0 "#PWR24" H 4500 2250 50  0001 C CNN
F 1 "GND" H 4500 2350 50  0000 C CNN
F 2 "" H 4500 2500 50  0001 C CNN
F 3 "" H 4500 2500 50  0001 C CNN
	1    4500 2500
	1    0    0    -1  
$EndComp
$Comp
L +3.3V #PWR17
U 1 1 59208C89
P 7300 1100
F 0 "#PWR17" H 7300 950 50  0001 C CNN
F 1 "+3.3V" H 7300 1240 50  0000 C CNN
F 2 "" H 7300 1100 50  0001 C CNN
F 3 "" H 7300 1100 50  0001 C CNN
	1    7300 1100
	1    0    0    -1  
$EndComp
$Comp
L C C8
U 1 1 59208CC1
P 7000 1550
F 0 "C8" H 7025 1650 50  0000 L CNN
F 1 "10u" H 7025 1450 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 7038 1400 50  0001 C CNN
F 3 "" H 7000 1550 50  0001 C CNN
F 4 "1276-6830-1-ND" H 7000 1550 60  0001 C CNN "Part No."
	1    7000 1550
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR29
U 1 1 59208D24
P 7300 2500
F 0 "#PWR29" H 7300 2250 50  0001 C CNN
F 1 "GND" H 7300 2350 50  0000 C CNN
F 2 "" H 7300 2500 50  0001 C CNN
F 3 "" H 7300 2500 50  0001 C CNN
	1    7300 2500
	1    0    0    -1  
$EndComp
$Comp
L C C5
U 1 1 59208DBB
P 3150 1550
F 0 "C5" H 3175 1650 50  0000 L CNN
F 1 "10u" H 3175 1450 50  0000 L CNN
F 2 "Capacitors_SMD:C_0603" H 3188 1400 50  0001 C CNN
F 3 "" H 3150 1550 50  0001 C CNN
F 4 "490-7202-1-ND" H 3150 1550 60  0001 C CNN "Part No."
	1    3150 1550
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR21
U 1 1 59208EA1
P 3150 2500
F 0 "#PWR21" H 3150 2250 50  0001 C CNN
F 1 "GND" H 3150 2350 50  0000 C CNN
F 2 "" H 3150 2500 50  0001 C CNN
F 3 "" H 3150 2500 50  0001 C CNN
	1    3150 2500
	1    0    0    -1  
$EndComp
Wire Wire Line
	1600 1500 1600 1900
Wire Wire Line
	1600 1200 1600 1300
Wire Wire Line
	5100 1300 5700 1300
Wire Wire Line
	5200 1100 5100 1100
Wire Wire Line
	5500 1100 5600 1100
Wire Wire Line
	5600 1100 5600 1300
Connection ~ 5600 1300
Wire Wire Line
	5500 2500 5500 1700
Wire Wire Line
	5500 1400 5500 1300
Connection ~ 5500 1300
Wire Wire Line
	1500 1500 1600 1500
Wire Wire Line
	1500 1300 3900 1300
Wire Wire Line
	4500 2500 4500 1900
Connection ~ 1600 1300
Wire Wire Line
	6000 1300 7650 1300
Wire Wire Line
	7300 1100 7300 1400
Connection ~ 7000 1300
Wire Wire Line
	7300 2500 7300 1700
Wire Wire Line
	3150 2500 3150 1700
Wire Wire Line
	3150 1300 3150 1400
Connection ~ 3150 1300
Wire Wire Line
	6700 1400 6700 1300
Connection ~ 6700 1300
$Comp
L GND #PWR27
U 1 1 5920907B
P 6700 2500
F 0 "#PWR27" H 6700 2250 50  0001 C CNN
F 1 "GND" H 6700 2350 50  0000 C CNN
F 2 "" H 6700 2500 50  0001 C CNN
F 3 "" H 6700 2500 50  0001 C CNN
	1    6700 2500
	1    0    0    -1  
$EndComp
Wire Wire Line
	6700 2500 6700 1700
$Comp
L C C6
U 1 1 592090D7
P 3400 1550
F 0 "C6" H 3425 1650 50  0000 L CNN
F 1 "0.1u" H 3425 1450 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 3438 1400 50  0001 C CNN
F 3 "" H 3400 1550 50  0001 C CNN
F 4 "490-14603-1-ND" H 3400 1550 60  0001 C CNN "Part No."
	1    3400 1550
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR22
U 1 1 592091A5
P 3400 2500
F 0 "#PWR22" H 3400 2250 50  0001 C CNN
F 1 "GND" H 3400 2350 50  0000 C CNN
F 2 "" H 3400 2500 50  0001 C CNN
F 3 "" H 3400 2500 50  0001 C CNN
	1    3400 2500
	1    0    0    -1  
$EndComp
Wire Wire Line
	3400 2500 3400 1700
$Comp
L R R10
U 1 1 592092E7
P 6200 1550
F 0 "R10" V 6280 1550 50  0000 C CNN
F 1 "49K9" V 6200 1550 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 6130 1550 50  0001 C CNN
F 3 "" H 6200 1550 50  0001 C CNN
F 4 "311-2296-1-ND" H 6200 1550 60  0001 C CNN "Part No."
	1    6200 1550
	1    0    0    -1  
$EndComp
$Comp
L R R12
U 1 1 59209360
P 6200 2050
F 0 "R12" V 6280 2050 50  0000 C CNN
F 1 "16K2" V 6200 2050 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 6130 2050 50  0001 C CNN
F 3 "" H 6200 2050 50  0001 C CNN
F 4 "311-2144-1-ND" H 6200 2050 60  0001 C CNN "Part No."
	1    6200 2050
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR26
U 1 1 592093A2
P 6200 2500
F 0 "#PWR26" H 6200 2250 50  0001 C CNN
F 1 "GND" H 6200 2350 50  0000 C CNN
F 2 "" H 6200 2500 50  0001 C CNN
F 3 "" H 6200 2500 50  0001 C CNN
	1    6200 2500
	1    0    0    -1  
$EndComp
Wire Wire Line
	6200 2500 6200 2200
Wire Wire Line
	6200 1700 6200 1900
Wire Wire Line
	6200 1400 6200 1300
Connection ~ 6200 1300
Wire Wire Line
	6200 1800 5900 1800
Wire Wire Line
	5900 1800 5900 2850
Wire Wire Line
	5900 2850 5200 2850
Wire Wire Line
	5200 2850 5200 1600
Wire Wire Line
	5200 1600 5100 1600
Connection ~ 6200 1800
$Comp
L R R11
U 1 1 592096C6
P 3700 2050
F 0 "R11" V 3780 2050 50  0000 C CNN
F 1 "10K" V 3700 2050 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 3630 2050 50  0001 C CNN
F 3 "" H 3700 2050 50  0001 C CNN
F 4 "311-10.0KLRCT-ND" H 3700 2050 60  0001 C CNN "Part No."
	1    3700 2050
	1    0    0    -1  
$EndComp
$Comp
L R R9
U 1 1 592097A6
P 3700 1550
F 0 "R9" V 3780 1550 50  0000 C CNN
F 1 "49K9" V 3700 1550 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 3630 1550 50  0001 C CNN
F 3 "" H 3700 1550 50  0001 C CNN
F 4 "311-2296-1-ND" H 3700 1550 60  0001 C CNN "Part No."
	1    3700 1550
	1    0    0    -1  
$EndComp
Wire Wire Line
	3700 1300 3700 1400
Wire Wire Line
	3700 1700 3700 1900
$Comp
L GND #PWR23
U 1 1 59209C78
P 3700 2500
F 0 "#PWR23" H 3700 2250 50  0001 C CNN
F 1 "GND" H 3700 2350 50  0000 C CNN
F 2 "" H 3700 2500 50  0001 C CNN
F 3 "" H 3700 2500 50  0001 C CNN
	1    3700 2500
	1    0    0    -1  
$EndComp
Wire Wire Line
	3700 2500 3700 2200
Connection ~ 3700 1300
Wire Wire Line
	3400 1400 3400 1300
Connection ~ 3400 1300
Wire Wire Line
	3700 1800 3850 1800
Wire Wire Line
	3850 1600 3900 1600
Connection ~ 3700 1800
$Comp
L C C9
U 1 1 5920A352
P 7300 1550
F 0 "C9" H 7325 1650 50  0000 L CNN
F 1 "10u" H 7325 1450 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 7338 1400 50  0001 C CNN
F 3 "" H 7300 1550 50  0001 C CNN
F 4 "1276-6830-1-ND" H 7300 1550 60  0001 C CNN "Part No."
	1    7300 1550
	1    0    0    -1  
$EndComp
Connection ~ 7300 1300
Wire Wire Line
	7000 1400 7000 1300
$Comp
L GND #PWR28
U 1 1 5920A4A0
P 7000 2500
F 0 "#PWR28" H 7000 2250 50  0001 C CNN
F 1 "GND" H 7000 2350 50  0000 C CNN
F 2 "" H 7000 2500 50  0001 C CNN
F 3 "" H 7000 2500 50  0001 C CNN
	1    7000 2500
	1    0    0    -1  
$EndComp
Wire Wire Line
	7000 2500 7000 1700
Text Label 5400 2850 0    60   ~ 0
3V3_FB
Wire Wire Line
	3850 1800 3850 1600
$Comp
L L L2
U 1 1 592C89B7
P 7800 1300
F 0 "L2" V 7750 1300 50  0000 C CNN
F 1 "10u" V 7875 1300 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" H 7800 1300 50  0001 C CNN
F 3 "" H 7800 1300 50  0001 C CNN
F 4 "445-6396-1-ND" H 7800 1300 60  0001 C CNN "Part No."
	1    7800 1300
	0    -1   -1   0   
$EndComp
$Comp
L C C10
U 1 1 592C89D3
P 8500 1550
F 0 "C10" H 8525 1650 50  0000 L CNN
F 1 "10u" H 8525 1450 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 8538 1400 50  0001 C CNN
F 3 "" H 8500 1550 50  0001 C CNN
F 4 "1276-6830-1-ND" H 8500 1550 60  0001 C CNN "Part No."
	1    8500 1550
	1    0    0    -1  
$EndComp
Wire Wire Line
	8500 1100 8500 1400
Wire Wire Line
	8500 1700 8500 2500
Wire Wire Line
	7950 1300 8500 1300
$Comp
L GND #PWR30
U 1 1 592C8FE8
P 8500 2500
F 0 "#PWR30" H 8500 2250 50  0001 C CNN
F 1 "GND" H 8500 2350 50  0000 C CNN
F 2 "" H 8500 2500 50  0001 C CNN
F 3 "" H 8500 2500 50  0001 C CNN
	1    8500 2500
	1    0    0    -1  
$EndComp
$Comp
L +3.3VA #PWR18
U 1 1 592C9106
P 8500 1100
F 0 "#PWR18" H 8500 950 50  0001 C CNN
F 1 "+3.3VA" H 8500 1240 50  0000 C CNN
F 2 "" H 8500 1100 50  0001 C CNN
F 3 "" H 8500 1100 50  0001 C CNN
	1    8500 1100
	1    0    0    -1  
$EndComp
Connection ~ 8500 1300
$Comp
L C C4
U 1 1 592C9DF6
P 5350 1100
F 0 "C4" H 5375 1200 50  0000 L CNN
F 1 "0.1u" H 5375 1000 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 5388 950 50  0001 C CNN
F 3 "" H 5350 1100 50  0001 C CNN
F 4 "490-14603-1-ND" H 5350 1100 60  0001 C CNN "Part No."
	1    5350 1100
	0    1    1    0   
$EndComp
$Comp
L C C7
U 1 1 592C9E7F
P 6700 1550
F 0 "C7" H 6725 1650 50  0000 L CNN
F 1 "0.1u" H 6725 1450 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 6738 1400 50  0001 C CNN
F 3 "" H 6700 1550 50  0001 C CNN
F 4 "490-14603-1-ND" H 6700 1550 60  0001 C CNN "Part No."
	1    6700 1550
	1    0    0    -1  
$EndComp
$Comp
L AP3211 U3
U 1 1 592CBD17
P 4500 1300
F 0 "U3" H 4150 850 60  0000 C CNN
F 1 "AP3211" H 4500 1650 60  0000 C CNN
F 2 "TO_SOT_Packages_SMD:SOT-23-6" H 4500 1300 60  0001 C CNN
F 3 "" H 4500 1300 60  0001 C CNN
F 4 "AP3211KTR-G1DICT-ND" H 4500 1300 60  0001 C CNN "Part No."
	1    4500 1300
	1    0    0    -1  
$EndComp
Text Label 5100 1300 0    60   ~ 0
+3.3V_SW
$EndSCHEMATC
