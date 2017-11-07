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
Sheet 3 4
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
L BARREL_JACK J2
U 1 1 592082C4
P 1200 1600
F 0 "J2" H 1200 1795 50  0000 C CNN
F 1 "BARREL_JACK" H 1200 1445 50  0000 C CNN
F 2 "rt-soldering-iron:PJ-069B-SMT" H 1200 1600 50  0001 C CNN
F 3 "" H 1200 1600 50  0001 C CNN
F 4 "CP-069BPJCT-ND" H 1200 1600 60  0001 C CNN "Part No."
	1    1200 1600
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR024
U 1 1 592084EC
P 1600 2100
F 0 "#PWR024" H 1600 1850 50  0001 C CNN
F 1 "GND" H 1600 1950 50  0000 C CNN
F 2 "" H 1600 2100 50  0001 C CNN
F 3 "" H 1600 2100 50  0001 C CNN
	1    1600 2100
	1    0    0    -1  
$EndComp
$Comp
L +12V #PWR025
U 1 1 59208505
P 1600 1400
F 0 "#PWR025" H 1600 1250 50  0001 C CNN
F 1 "+12V" H 1600 1540 50  0000 C CNN
F 2 "" H 1600 1400 50  0001 C CNN
F 3 "" H 1600 1400 50  0001 C CNN
	1    1600 1400
	1    0    0    -1  
$EndComp
NoConn ~ 1500 1600
$Comp
L L L1
U 1 1 592087DE
P 5850 1500
F 0 "L1" V 5800 1500 50  0000 C CNN
F 1 "4.7u" V 5925 1500 50  0000 C CNN
F 2 "Inductors:Inductor_Taiyo-Yuden_NR-30xx" H 5850 1500 50  0001 C CNN
F 3 "" H 5850 1500 50  0001 C CNN
F 4 "587-2591-1-ND" H 5850 1500 60  0001 C CNN "Part No."
	1    5850 1500
	0    -1   -1   0   
$EndComp
$Comp
L D_Schottky D1
U 1 1 59208876
P 5500 1750
F 0 "D1" H 5500 1850 50  0000 C CNN
F 1 "CUS10F30" H 5500 1650 50  0000 C CNN
F 2 "Diodes_SMD:D_SOD-323" H 5500 1750 50  0001 C CNN
F 3 "" H 5500 1750 50  0001 C CNN
F 4 "CUS10F30H3FCT-ND" H 5500 1750 60  0001 C CNN "Part No."
	1    5500 1750
	0    1    1    0   
$EndComp
$Comp
L GND #PWR026
U 1 1 592088C5
P 5500 2700
F 0 "#PWR026" H 5500 2450 50  0001 C CNN
F 1 "GND" H 5500 2550 50  0000 C CNN
F 2 "" H 5500 2700 50  0001 C CNN
F 3 "" H 5500 2700 50  0001 C CNN
	1    5500 2700
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR027
U 1 1 59208941
P 4500 2700
F 0 "#PWR027" H 4500 2450 50  0001 C CNN
F 1 "GND" H 4500 2550 50  0000 C CNN
F 2 "" H 4500 2700 50  0001 C CNN
F 3 "" H 4500 2700 50  0001 C CNN
	1    4500 2700
	1    0    0    -1  
$EndComp
$Comp
L +3.3V #PWR028
U 1 1 59208C89
P 8200 1300
F 0 "#PWR028" H 8200 1150 50  0001 C CNN
F 1 "+3.3V" H 8200 1440 50  0000 C CNN
F 2 "" H 8200 1300 50  0001 C CNN
F 3 "" H 8200 1300 50  0001 C CNN
	1    8200 1300
	1    0    0    -1  
$EndComp
$Comp
L C C8
U 1 1 59208CC1
P 7900 1750
F 0 "C8" H 7925 1850 50  0000 L CNN
F 1 "10u" H 7925 1650 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 7938 1600 50  0001 C CNN
F 3 "" H 7900 1750 50  0001 C CNN
F 4 "1276-6830-1-ND" H 7900 1750 60  0001 C CNN "Part No."
	1    7900 1750
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR029
U 1 1 59208D24
P 8200 2700
F 0 "#PWR029" H 8200 2450 50  0001 C CNN
F 1 "GND" H 8200 2550 50  0000 C CNN
F 2 "" H 8200 2700 50  0001 C CNN
F 3 "" H 8200 2700 50  0001 C CNN
	1    8200 2700
	1    0    0    -1  
$EndComp
$Comp
L C C5
U 1 1 59208DBB
P 3150 1750
F 0 "C5" H 3175 1850 50  0000 L CNN
F 1 "10u" H 3175 1650 50  0000 L CNN
F 2 "Capacitors_SMD:C_0603" H 3188 1600 50  0001 C CNN
F 3 "" H 3150 1750 50  0001 C CNN
F 4 "490-7202-1-ND" H 3150 1750 60  0001 C CNN "Part No."
	1    3150 1750
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR030
U 1 1 59208EA1
P 3150 2700
F 0 "#PWR030" H 3150 2450 50  0001 C CNN
F 1 "GND" H 3150 2550 50  0000 C CNN
F 2 "" H 3150 2700 50  0001 C CNN
F 3 "" H 3150 2700 50  0001 C CNN
	1    3150 2700
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR031
U 1 1 5920907B
P 7600 2700
F 0 "#PWR031" H 7600 2450 50  0001 C CNN
F 1 "GND" H 7600 2550 50  0000 C CNN
F 2 "" H 7600 2700 50  0001 C CNN
F 3 "" H 7600 2700 50  0001 C CNN
	1    7600 2700
	1    0    0    -1  
$EndComp
$Comp
L C C6
U 1 1 592090D7
P 3400 1750
F 0 "C6" H 3425 1850 50  0000 L CNN
F 1 "0.1u" H 3425 1650 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 3438 1600 50  0001 C CNN
F 3 "" H 3400 1750 50  0001 C CNN
F 4 "490-14603-1-ND" H 3400 1750 60  0001 C CNN "Part No."
	1    3400 1750
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR032
U 1 1 592091A5
P 3400 2700
F 0 "#PWR032" H 3400 2450 50  0001 C CNN
F 1 "GND" H 3400 2550 50  0000 C CNN
F 2 "" H 3400 2700 50  0001 C CNN
F 3 "" H 3400 2700 50  0001 C CNN
	1    3400 2700
	1    0    0    -1  
$EndComp
$Comp
L R R10
U 1 1 592092E7
P 6100 1750
F 0 "R10" V 6180 1750 50  0000 C CNN
F 1 "49K9" V 6100 1750 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 6030 1750 50  0001 C CNN
F 3 "" H 6100 1750 50  0001 C CNN
F 4 "311-2296-1-ND" H 6100 1750 60  0001 C CNN "Part No."
	1    6100 1750
	1    0    0    -1  
$EndComp
$Comp
L R R12
U 1 1 59209360
P 6100 2250
F 0 "R12" V 6180 2250 50  0000 C CNN
F 1 "16K2" V 6100 2250 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 6030 2250 50  0001 C CNN
F 3 "" H 6100 2250 50  0001 C CNN
F 4 "311-2144-1-ND" H 6100 2250 60  0001 C CNN "Part No."
	1    6100 2250
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR033
U 1 1 592093A2
P 6100 2700
F 0 "#PWR033" H 6100 2450 50  0001 C CNN
F 1 "GND" H 6100 2550 50  0000 C CNN
F 2 "" H 6100 2700 50  0001 C CNN
F 3 "" H 6100 2700 50  0001 C CNN
	1    6100 2700
	1    0    0    -1  
$EndComp
$Comp
L R R11
U 1 1 592096C6
P 3700 2250
F 0 "R11" V 3780 2250 50  0000 C CNN
F 1 "10K" V 3700 2250 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 3630 2250 50  0001 C CNN
F 3 "" H 3700 2250 50  0001 C CNN
F 4 "311-10.0KLRCT-ND" H 3700 2250 60  0001 C CNN "Part No."
	1    3700 2250
	1    0    0    -1  
$EndComp
$Comp
L R R9
U 1 1 592097A6
P 3700 1750
F 0 "R9" V 3780 1750 50  0000 C CNN
F 1 "49K9" V 3700 1750 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 3630 1750 50  0001 C CNN
F 3 "" H 3700 1750 50  0001 C CNN
F 4 "311-2296-1-ND" H 3700 1750 60  0001 C CNN "Part No."
	1    3700 1750
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR034
U 1 1 59209C78
P 3700 2700
F 0 "#PWR034" H 3700 2450 50  0001 C CNN
F 1 "GND" H 3700 2550 50  0000 C CNN
F 2 "" H 3700 2700 50  0001 C CNN
F 3 "" H 3700 2700 50  0001 C CNN
	1    3700 2700
	1    0    0    -1  
$EndComp
$Comp
L C C9
U 1 1 5920A352
P 8200 1750
F 0 "C9" H 8225 1850 50  0000 L CNN
F 1 "10u" H 8225 1650 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 8238 1600 50  0001 C CNN
F 3 "" H 8200 1750 50  0001 C CNN
F 4 "1276-6830-1-ND" H 8200 1750 60  0001 C CNN "Part No."
	1    8200 1750
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR035
U 1 1 5920A4A0
P 7900 2700
F 0 "#PWR035" H 7900 2450 50  0001 C CNN
F 1 "GND" H 7900 2550 50  0000 C CNN
F 2 "" H 7900 2700 50  0001 C CNN
F 3 "" H 7900 2700 50  0001 C CNN
	1    7900 2700
	1    0    0    -1  
$EndComp
Text Label 5400 3050 0    60   ~ 0
3V3_FB
$Comp
L L L2
U 1 1 592C89B7
P 8700 1500
F 0 "L2" V 8650 1500 50  0000 C CNN
F 1 "10u" V 8775 1500 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" H 8700 1500 50  0001 C CNN
F 3 "" H 8700 1500 50  0001 C CNN
F 4 "445-6396-1-ND" H 8700 1500 60  0001 C CNN "Part No."
	1    8700 1500
	0    -1   -1   0   
$EndComp
$Comp
L C C10
U 1 1 592C89D3
P 9400 1750
F 0 "C10" H 9425 1850 50  0000 L CNN
F 1 "10u" H 9425 1650 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 9438 1600 50  0001 C CNN
F 3 "" H 9400 1750 50  0001 C CNN
F 4 "1276-6830-1-ND" H 9400 1750 60  0001 C CNN "Part No."
	1    9400 1750
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR036
U 1 1 592C8FE8
P 9400 2700
F 0 "#PWR036" H 9400 2450 50  0001 C CNN
F 1 "GND" H 9400 2550 50  0000 C CNN
F 2 "" H 9400 2700 50  0001 C CNN
F 3 "" H 9400 2700 50  0001 C CNN
	1    9400 2700
	1    0    0    -1  
$EndComp
$Comp
L +3.3VA #PWR037
U 1 1 592C9106
P 9400 1300
F 0 "#PWR037" H 9400 1150 50  0001 C CNN
F 1 "+3.3VA" H 9400 1440 50  0000 C CNN
F 2 "" H 9400 1300 50  0001 C CNN
F 3 "" H 9400 1300 50  0001 C CNN
	1    9400 1300
	1    0    0    -1  
$EndComp
$Comp
L C C4
U 1 1 592C9DF6
P 5350 1300
F 0 "C4" H 5375 1400 50  0000 L CNN
F 1 "0.1u" H 5375 1200 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 5388 1150 50  0001 C CNN
F 3 "" H 5350 1300 50  0001 C CNN
F 4 "490-14603-1-ND" H 5350 1300 60  0001 C CNN "Part No."
	1    5350 1300
	0    1    1    0   
$EndComp
$Comp
L C C7
U 1 1 592C9E7F
P 7600 1750
F 0 "C7" H 7625 1850 50  0000 L CNN
F 1 "0.1u" H 7625 1650 50  0000 L CNN
F 2 "Capacitors_SMD:C_0402" H 7638 1600 50  0001 C CNN
F 3 "" H 7600 1750 50  0001 C CNN
F 4 "490-14603-1-ND" H 7600 1750 60  0001 C CNN "Part No."
	1    7600 1750
	1    0    0    -1  
$EndComp
$Comp
L AP3211 U3
U 1 1 592CBD17
P 4500 1500
F 0 "U3" H 4150 1050 60  0000 C CNN
F 1 "AP3211" H 4500 1850 60  0000 C CNN
F 2 "TO_SOT_Packages_SMD:SOT-23-6" H 4500 1500 60  0001 C CNN
F 3 "" H 4500 1500 60  0001 C CNN
F 4 "AP3211KTR-G1DICT-ND" H 4500 1500 60  0001 C CNN "Part No."
	1    4500 1500
	1    0    0    -1  
$EndComp
Text Label 5100 1500 0    60   ~ 0
+3.3V_SW
$Comp
L D_Zener D3
U 1 1 5987CC93
P 7300 2150
F 0 "D3" H 7300 2250 50  0000 C CNN
F 1 "3.6V" H 7300 2050 50  0000 C CNN
F 2 "Diodes_SMD:D_SOD-523" H 7300 2150 50  0001 C CNN
F 3 "" H 7300 2150 50  0001 C CNN
F 4 "DZ2S036M0LCT-ND" H 7300 2150 60  0001 C CNN "Part No."
	1    7300 2150
	0    1    1    0   
$EndComp
$Comp
L GND #PWR038
U 1 1 5987D189
P 7300 2700
F 0 "#PWR038" H 7300 2450 50  0001 C CNN
F 1 "GND" H 7300 2550 50  0000 C CNN
F 2 "" H 7300 2700 50  0001 C CNN
F 3 "" H 7300 2700 50  0001 C CNN
	1    7300 2700
	1    0    0    -1  
$EndComp
$Comp
L TL431 U7
U 1 1 5998FA50
P 2200 5150
F 0 "U7" H 2300 4950 60  0000 C CNN
F 1 "TL431" H 2400 5300 60  0000 C CNN
F 2 "TO_SOT_Packages_SMD:SOT-363_SC-70-6" H 2200 5150 60  0001 C CNN
F 3 "" H 2200 5150 60  0001 C CNN
F 4 "296-17863-1-ND" H 2200 5150 60  0001 C CNN "Part No."
	1    2200 5150
	1    0    0    -1  
$EndComp
$Comp
L R R24
U 1 1 5998FB5B
P 1700 4950
F 0 "R24" V 1780 4950 50  0000 C CNN
F 1 "10K" V 1700 4950 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 1630 4950 50  0001 C CNN
F 3 "" H 1700 4950 50  0001 C CNN
F 4 "311-10.0KLRCT-ND" H 1700 4950 60  0001 C CNN "Part No."
	1    1700 4950
	-1   0    0    -1  
$EndComp
$Comp
L R R26
U 1 1 5998FBD8
P 1700 5450
F 0 "R26" V 1780 5450 50  0000 C CNN
F 1 "22K6" V 1700 5450 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 1630 5450 50  0001 C CNN
F 3 "" H 1700 5450 50  0001 C CNN
F 4 "YAG3061CT-ND" H 1700 5450 60  0001 C CNN "Part No."
	1    1700 5450
	-1   0    0    -1  
$EndComp
$Comp
L R R22
U 1 1 5998FD70
P 2200 4550
F 0 "R22" V 2280 4550 50  0000 C CNN
F 1 "1K" V 2200 4550 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 2130 4550 50  0001 C CNN
F 3 "" H 2200 4550 50  0001 C CNN
F 4 "311-1.0KJRCT-ND" H 2200 4550 60  0001 C CNN "Part No."
	1    2200 4550
	-1   0    0    -1  
$EndComp
$Comp
L GND #PWR039
U 1 1 5998FEAD
P 2200 5800
F 0 "#PWR039" H 2200 5550 50  0001 C CNN
F 1 "GND" H 2200 5650 50  0000 C CNN
F 2 "" H 2200 5800 50  0001 C CNN
F 3 "" H 2200 5800 50  0001 C CNN
	1    2200 5800
	-1   0    0    -1  
$EndComp
Text Notes 1600 5150 2    60   ~ 0
Trip point 3.6V
$Comp
L +12V #PWR040
U 1 1 599906B9
P 3600 3700
F 0 "#PWR040" H 3600 3550 50  0001 C CNN
F 1 "+12V" H 3600 3840 50  0000 C CNN
F 2 "" H 3600 3700 50  0001 C CNN
F 3 "" H 3600 3700 50  0001 C CNN
	1    3600 3700
	-1   0    0    -1  
$EndComp
$Comp
L R R21
U 1 1 59990783
P 2200 4050
F 0 "R21" V 2280 4050 50  0000 C CNN
F 1 "10K" V 2200 4050 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 2130 4050 50  0001 C CNN
F 3 "" H 2200 4050 50  0001 C CNN
F 4 "311-10.0KLRCT-ND" H 2200 4050 60  0001 C CNN "Part No."
	1    2200 4050
	-1   0    0    -1  
$EndComp
$Comp
L R R25
U 1 1 599908F3
P 3600 5350
F 0 "R25" V 3680 5350 50  0000 C CNN
F 1 "10K" V 3600 5350 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 3530 5350 50  0001 C CNN
F 3 "" H 3600 5350 50  0001 C CNN
F 4 "311-10.0KLRCT-ND" H 3600 5350 60  0001 C CNN "Part No."
	1    3600 5350
	-1   0    0    -1  
$EndComp
$Comp
L R R23
U 1 1 59990AC1
P 3600 4750
F 0 "R23" V 3680 4750 50  0000 C CNN
F 1 "10K" V 3600 4750 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 3530 4750 50  0001 C CNN
F 3 "" H 3600 4750 50  0001 C CNN
F 4 "311-10.0KLRCT-ND" H 3600 4750 60  0001 C CNN "Part No."
	1    3600 4750
	-1   0    0    -1  
$EndComp
$Comp
L +3.3V #PWR041
U 1 1 59991F79
P 1700 4600
F 0 "#PWR041" H 1700 4450 50  0001 C CNN
F 1 "+3.3V" H 1700 4740 50  0000 C CNN
F 2 "" H 1700 4600 50  0001 C CNN
F 3 "" H 1700 4600 50  0001 C CNN
	1    1700 4600
	-1   0    0    -1  
$EndComp
Text Label 3200 4300 2    60   ~ 0
OVP_LATCH_IN
Text Label 2950 4800 2    60   ~ 0
OVP_LATCH_REG
Text Label 1700 5250 2    60   ~ 0
OVP_LATCH_TRIP
Text Label 3800 5000 2    60   ~ 0
OVP_LATCH_OUT
Wire Wire Line
	1600 1700 1600 2100
Wire Wire Line
	1600 1400 1600 1500
Wire Wire Line
	5100 1500 5700 1500
Wire Wire Line
	5200 1300 5100 1300
Wire Wire Line
	5500 1300 5600 1300
Wire Wire Line
	5600 1300 5600 1500
Connection ~ 5600 1500
Wire Wire Line
	5500 2700 5500 1900
Wire Wire Line
	5500 1600 5500 1500
Connection ~ 5500 1500
Wire Wire Line
	1500 1700 1600 1700
Wire Wire Line
	1500 1500 3900 1500
Wire Wire Line
	4500 2700 4500 2100
Connection ~ 1600 1500
Wire Wire Line
	8200 1300 8200 1600
Connection ~ 7900 1500
Wire Wire Line
	8200 2700 8200 1900
Wire Wire Line
	3150 2700 3150 1900
Wire Wire Line
	3150 1500 3150 1600
Connection ~ 3150 1500
Wire Wire Line
	7600 1500 7600 1600
Connection ~ 7600 1500
Wire Wire Line
	7600 2700 7600 1900
Wire Wire Line
	3400 2700 3400 1900
Wire Wire Line
	6100 2700 6100 2400
Wire Wire Line
	6100 1900 6100 2100
Wire Wire Line
	6100 1600 6100 1500
Connection ~ 6100 1500
Wire Wire Line
	6100 2000 5900 2000
Wire Wire Line
	5900 2000 5900 3050
Wire Wire Line
	5900 3050 5200 3050
Wire Wire Line
	5200 3050 5200 1800
Wire Wire Line
	5200 1800 5100 1800
Connection ~ 6100 2000
Wire Wire Line
	3700 1500 3700 1600
Wire Wire Line
	3700 1900 3700 2100
Wire Wire Line
	3700 2700 3700 2400
Connection ~ 3700 1500
Wire Wire Line
	3400 1600 3400 1500
Connection ~ 3400 1500
Wire Wire Line
	3700 2000 3850 2000
Wire Wire Line
	3850 1800 3900 1800
Connection ~ 3700 2000
Connection ~ 8200 1500
Wire Wire Line
	7900 1500 7900 1600
Wire Wire Line
	7900 2700 7900 1900
Wire Wire Line
	9400 1300 9400 1600
Wire Wire Line
	9400 1900 9400 2700
Wire Wire Line
	8850 1500 9400 1500
Connection ~ 9400 1500
Wire Wire Line
	7300 1500 7300 2000
Connection ~ 7300 1500
Wire Wire Line
	7300 2700 7300 2300
Wire Wire Line
	3300 4300 2200 4300
Wire Wire Line
	2200 4200 2200 4400
Wire Wire Line
	2200 4700 2200 4850
Wire Wire Line
	2200 5500 2200 5800
Wire Wire Line
	2200 5700 1700 5700
Wire Wire Line
	1700 5700 1700 5600
Connection ~ 2200 5700
Wire Wire Line
	1900 5200 1700 5200
Wire Wire Line
	1700 5100 1700 5300
Connection ~ 1700 5200
Wire Wire Line
	3600 3700 3600 4100
Wire Wire Line
	3600 3800 2200 3800
Wire Wire Line
	2200 3800 2200 3900
Connection ~ 3600 3800
Connection ~ 2200 4300
Wire Wire Line
	2200 5600 6200 5600
Wire Wire Line
	3600 5600 3600 5500
Connection ~ 2200 5600
Wire Wire Line
	3000 5300 3000 5600
Connection ~ 3000 5600
Wire Wire Line
	3600 4900 3600 5200
Wire Wire Line
	3600 5100 3300 5100
Connection ~ 3600 5100
Wire Wire Line
	3600 4600 3600 4500
Wire Wire Line
	3000 4800 2200 4800
Connection ~ 2200 4800
Wire Wire Line
	4900 5600 4900 5200
Connection ~ 3600 5600
Connection ~ 3600 5000
Wire Wire Line
	1700 4600 1700 4800
Wire Wire Line
	3000 4900 3000 4800
$Comp
L +3.3V #PWR042
U 1 1 599981D4
P 6200 4400
F 0 "#PWR042" H 6200 4250 50  0001 C CNN
F 1 "+3.3V" H 6200 4540 50  0000 C CNN
F 2 "" H 6200 4400 50  0001 C CNN
F 3 "" H 6200 4400 50  0001 C CNN
	1    6200 4400
	1    0    0    -1  
$EndComp
Wire Wire Line
	6200 4400 6200 4750
Wire Wire Line
	3850 1800 3850 3100
Wire Wire Line
	4900 4800 4900 3100
Wire Wire Line
	4900 3100 3850 3100
Connection ~ 3850 2000
Text Label 4000 3100 0    60   ~ 0
+3.3V_EN
Wire Wire Line
	6200 5600 6200 5050
Connection ~ 4900 5600
Connection ~ 4600 5000
$Comp
L Fuse_Small F1
U 1 1 59999724
P 6400 1500
F 0 "F1" H 6400 1440 50  0000 C CNN
F 1 "Fuse_Small" H 6400 1560 50  0000 C CNN
F 2 "Resistors_SMD:R_0603" H 6400 1500 50  0001 C CNN
F 3 "" H 6400 1500 50  0001 C CNN
	1    6400 1500
	1    0    0    -1  
$EndComp
Text Notes 6300 4200 0    60   ~ 0
Zener is optional and is a fallback if the OVP circuit does not operate\nas expected.\n\nThe OVP circuit uses a TL431 as a precision comparator with integrated\nvoltage reference to set off a latch which both disables EN and throws\nan SCR crowbar on +3.3V until 12V is removed.
$Comp
L Q_NMOS_SGD_DUAL Q5
U 2 1 5999ADD8
P 3100 5100
F 0 "Q5" H 3300 5150 50  0000 L CNN
F 1 "FDG6301N_F085" H 3300 5050 50  0000 L CNN
F 2 "TO_SOT_Packages_SMD:SOT-363_SC-70-6" H 3300 5200 50  0001 C CNN
F 3 "" H 3100 5100 50  0001 C CNN
F 4 "FDG6301N-F085CT-ND" H 3100 5100 60  0001 C CNN "Part No."
	2    3100 5100
	-1   0    0    -1  
$EndComp
$Comp
L Q_NMOS_SGD_DUAL Q5
U 1 1 5999AE89
P 4800 5000
F 0 "Q5" H 5000 5050 50  0000 L CNN
F 1 "FDG6301N_F085" H 5000 4950 50  0000 L CNN
F 2 "TO_SOT_Packages_SMD:SOT-363_SC-70-6" H 5000 5100 50  0001 C CNN
F 3 "" H 4800 5000 50  0001 C CNN
F 4 "FDG6301N-F085CT-ND" H 4800 5000 60  0001 C CNN "Part No."
	1    4800 5000
	1    0    0    -1  
$EndComp
Text Notes 3100 6100 0    60   ~ 0
Alternate mosfet part: 2N7002DWCT-ND
$Comp
L Q_PNP_BEC Q4
U 1 1 59A8EA01
P 3500 4300
F 0 "Q4" H 3700 4350 50  0000 L CNN
F 1 "MMBT3906WT1G" H 3700 4250 50  0000 L CNN
F 2 "TO_SOT_Packages_SMD:SOT-323_SC-70" H 3700 4400 50  0001 C CNN
F 3 "" H 3500 4300 50  0001 C CNN
F 4 "MMBT3906WT1GOSCT-ND" H 3500 4300 60  0001 C CNN "Part No."
	1    3500 4300
	1    0    0    1   
$EndComp
Wire Wire Line
	6000 1500 6300 1500
Wire Wire Line
	6500 1500 8550 1500
Text Notes 5500 1200 0    60   ~ 0
The fuse here is probably optional. I will replace it with\na short, but the 0603 footprint should allow a standard\nSMD fuse or PTC thermistor in its place.\n\nThe AP3211 should handle a short circuit condition and\nshut down. The SCR can't handle 1.5A forever.
$Comp
L R R33
U 1 1 59AB54BD
P 5750 5000
F 0 "R33" V 5830 5000 50  0000 C CNN
F 1 "10K" V 5750 5000 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 5680 5000 50  0001 C CNN
F 3 "" H 5750 5000 50  0001 C CNN
F 4 "311-10.0KLRCT-ND" H 5750 5000 60  0001 C CNN "Part No."
	1    5750 5000
	0    1    1    0   
$EndComp
Wire Wire Line
	3600 5000 5600 5000
Wire Wire Line
	5900 5000 6050 5000
$Comp
L Q_Thyristor_KGA D4
U 1 1 59ACE5D6
P 6200 4900
F 0 "D4" H 6275 4925 50  0000 L CNN
F 1 "Q_Thyristor_KGA" H 6275 4850 50  0000 L CNN
F 2 "TO_SOT_Packages_SMD:SOT-23" V 6200 4900 50  0001 C CNN
F 3 "" V 6200 4900 50  0001 C CNN
F 4 "NYC0102BLT1GOSCT-ND" H 6200 4900 60  0001 C CNN "Part No."
	1    6200 4900
	1    0    0    -1  
$EndComp
Text Label 6000 1500 0    60   ~ 0
+3.3V_PREFUSE
$EndSCHEMATC
