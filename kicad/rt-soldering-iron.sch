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
Sheet 1 4
Title "RT Soldering Iron"
Date "2017-06-02"
Rev "A"
Comp "Kevin Cuzner"
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Sheet
S 2600 1800 2000 2200
U 59207D3C
F0 "Analog Frontend" 60
F1 "analog-frontend.sch" 60
F2 "HEATER_CTL" I R 4600 2350 60 
F3 "HEATER_SENSE" I R 4600 2850 60 
F4 "CURRENT_SENSE" I R 4600 3000 60 
F5 "~FAULT" I R 4600 3150 60 
F6 "~HEATER_EN" I R 4600 2250 60 
$EndSheet
$Sheet
S 5400 4700 1600 1200
U 59207D73
F0 "Power Supplies" 60
F1 "power-supplies.sch" 60
$EndSheet
$Sheet
S 5400 1800 3000 2200
U 59207D7A
F0 "Digital Control" 60
F1 "digital-control.sch" 60
F2 "HEATER_SENSE" I L 5400 2850 60 
F3 "HEATER_CTL" I L 5400 2350 60 
F4 "CURRENT_SENSE" I L 5400 3000 60 
F5 "~TIP_FAULT" I L 5400 3150 60 
$EndSheet
Wire Wire Line
	4600 2850 5400 2850
Wire Wire Line
	5400 3000 4600 3000
Wire Wire Line
	4600 2350 5400 2350
Wire Wire Line
	5400 3150 4600 3150
$EndSCHEMATC
