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
LIBS:mainboard-cache
EELAYER 25 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title "Podregilo mainboard"
Date ""
Rev ""
Comp "https://github.com/johannes-mueller/Podregilo"
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L 74LS165 U2
U 1 1 571A5491
P 3900 1600
F 0 "U2" H 4050 1550 50  0000 C CNN
F 1 "74LS165" H 4050 1350 50  0000 C CNN
F 2 "Housings_DIP:DIP-16_W7.62mm" H 3900 1600 50  0001 C CNN
F 3 "" H 3900 1600 50  0000 C CNN
	1    3900 1600
	0    -1   -1   0   
$EndComp
$Comp
L 74LS165 U1
U 1 1 571A56ED
P 1850 1600
F 0 "U1" H 2000 1550 50  0000 C CNN
F 1 "74LS165" H 2000 1350 50  0000 C CNN
F 2 "Housings_DIP:DIP-16_W7.62mm" H 1850 1600 50  0001 C CNN
F 3 "" H 1850 1600 50  0000 C CNN
	1    1850 1600
	0    -1   -1   0   
$EndComp
$Comp
L R R4
U 1 1 571A5B0E
P 1650 3350
F 0 "R4" H 1730 3350 50  0001 C CNN
F 1 "10k" V 1650 3350 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 1580 3350 50  0001 C CNN
F 3 "" H 1650 3350 50  0000 C CNN
	1    1650 3350
	1    0    0    -1  
$EndComp
$Comp
L R R3
U 1 1 571A5CB4
P 1550 3350
F 0 "R3" V 1630 3350 50  0001 C CNN
F 1 "10k" V 1550 3350 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 1480 3350 50  0001 C CNN
F 3 "" H 1550 3350 50  0000 C CNN
	1    1550 3350
	1    0    0    -1  
$EndComp
$Comp
L R R2
U 1 1 571A5DB7
P 1450 3350
F 0 "R2" H 1530 3350 50  0001 C CNN
F 1 "10k" V 1450 3350 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 1380 3350 50  0001 C CNN
F 3 "" H 1450 3350 50  0000 C CNN
	1    1450 3350
	1    0    0    -1  
$EndComp
$Comp
L R R1
U 1 1 571A5DBD
P 1350 3350
F 0 "R1" V 1430 3350 50  0001 C CNN
F 1 "10k" V 1350 3350 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 1280 3350 50  0001 C CNN
F 3 "" H 1350 3350 50  0000 C CNN
	1    1350 3350
	1    0    0    -1  
$EndComp
$Comp
L R R7
U 1 1 571A5DCB
P 1950 3350
F 0 "R7" V 2030 3350 50  0001 C CNN
F 1 "10k" V 1950 3350 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 1880 3350 50  0001 C CNN
F 3 "" H 1950 3350 50  0000 C CNN
	1    1950 3350
	1    0    0    -1  
$EndComp
$Comp
L R R6
U 1 1 571A5DD1
P 1850 3350
F 0 "R6" H 1930 3350 50  0001 C CNN
F 1 "10k" V 1850 3350 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 1780 3350 50  0001 C CNN
F 3 "" H 1850 3350 50  0000 C CNN
	1    1850 3350
	1    0    0    -1  
$EndComp
$Comp
L R R5
U 1 1 571A5DD7
P 1750 3350
F 0 "R5" V 1830 3350 50  0001 C CNN
F 1 "10k" V 1750 3350 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 1680 3350 50  0001 C CNN
F 3 "" H 1750 3350 50  0000 C CNN
	1    1750 3350
	1    0    0    -1  
$EndComp
$Comp
L R R16
U 1 1 571A5E9A
P 3700 3350
F 0 "R16" H 3780 3350 50  0001 C CNN
F 1 "10k" V 3700 3350 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 3630 3350 50  0001 C CNN
F 3 "" H 3700 3350 50  0000 C CNN
	1    3700 3350
	1    0    0    -1  
$EndComp
$Comp
L R R15
U 1 1 571A5EA0
P 3600 3350
F 0 "R15" V 3680 3350 50  0001 C CNN
F 1 "10k" V 3600 3350 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 3530 3350 50  0001 C CNN
F 3 "" H 3600 3350 50  0000 C CNN
	1    3600 3350
	1    0    0    -1  
$EndComp
$Comp
L R R14
U 1 1 571A5EA6
P 3500 3350
F 0 "R14" H 3580 3350 50  0001 C CNN
F 1 "10k" V 3500 3350 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 3430 3350 50  0001 C CNN
F 3 "" H 3500 3350 50  0000 C CNN
	1    3500 3350
	1    0    0    -1  
$EndComp
$Comp
L R R13
U 1 1 571A5EAC
P 3400 3350
F 0 "R13" V 3480 3350 50  0001 C CNN
F 1 "10k" V 3400 3350 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 3330 3350 50  0001 C CNN
F 3 "" H 3400 3350 50  0000 C CNN
	1    3400 3350
	1    0    0    -1  
$EndComp
$Comp
L 7400 U3
U 1 1 571A6188
P 6150 1350
F 0 "U3" H 6150 1400 50  0000 C CNN
F 1 "7400" H 6150 1250 50  0000 C CNN
F 2 "Housings_DIP:DIP-14_W7.62mm" H 6150 1350 50  0001 C CNN
F 3 "" H 6150 1350 50  0000 C CNN
	1    6150 1350
	1    0    0    -1  
$EndComp
$Comp
L 7400 U3
U 2 1 571A6239
P 6150 1850
F 0 "U3" H 6150 1900 50  0000 C CNN
F 1 "7400" H 6150 1750 50  0000 C CNN
F 2 "Housings_DIP:DIP-14_W7.62mm" H 6150 1850 50  0001 C CNN
F 3 "" H 6150 1850 50  0000 C CNN
	2    6150 1850
	1    0    0    -1  
$EndComp
$Comp
L 7400 U3
U 3 1 571A631A
P 6150 2350
F 0 "U3" H 6150 2400 50  0000 C CNN
F 1 "7400" H 6150 2250 50  0000 C CNN
F 2 "Housings_DIP:DIP-14_W7.62mm" H 6150 2350 50  0001 C CNN
F 3 "" H 6150 2350 50  0000 C CNN
	3    6150 2350
	1    0    0    -1  
$EndComp
$Comp
L 7400 U3
U 4 1 571A6351
P 6150 2850
F 0 "U3" H 6150 2900 50  0000 C CNN
F 1 "7400" H 6150 2750 50  0000 C CNN
F 2 "Housings_DIP:DIP-14_W7.62mm" H 6150 2850 50  0001 C CNN
F 3 "" H 6150 2850 50  0000 C CNN
	4    6150 2850
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR01
U 1 1 571A64F2
P 3700 3650
F 0 "#PWR01" H 3700 3400 50  0001 C CNN
F 1 "GND" H 3700 3500 50  0000 C CNN
F 2 "" H 3700 3650 50  0000 C CNN
F 3 "" H 3700 3650 50  0000 C CNN
	1    3700 3650
	1    0    0    -1  
$EndComp
$Comp
L R R17
U 1 1 571A6E3B
P 6900 1350
F 0 "R17" V 6980 1350 50  0000 C CNN
F 1 "1k" V 6900 1350 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 6830 1350 50  0001 C CNN
F 3 "" H 6900 1350 50  0000 C CNN
	1    6900 1350
	0    1    1    0   
$EndComp
$Comp
L R R18
U 1 1 571A7066
P 6900 1850
F 0 "R18" V 6980 1850 50  0000 C CNN
F 1 "1k" V 6900 1850 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 6830 1850 50  0001 C CNN
F 3 "" H 6900 1850 50  0000 C CNN
	1    6900 1850
	0    1    1    0   
$EndComp
$Comp
L R R19
U 1 1 571A709A
P 6900 2350
F 0 "R19" V 6980 2350 50  0000 C CNN
F 1 "1k" V 6900 2350 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 6830 2350 50  0001 C CNN
F 3 "" H 6900 2350 50  0000 C CNN
	1    6900 2350
	0    1    1    0   
$EndComp
$Comp
L R R20
U 1 1 571A7102
P 6900 2850
F 0 "R20" V 6980 2850 50  0000 C CNN
F 1 "1k" V 6900 2850 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 6830 2850 50  0001 C CNN
F 3 "" H 6900 2850 50  0000 C CNN
	1    6900 2850
	0    1    1    0   
$EndComp
$Comp
L CONN_02X13 P1
U 1 1 571A8D2E
P 6750 4200
F 0 "P1" H 6750 4900 50  0000 C CNN
F 1 "CONN_02X13" V 6750 4200 50  0000 C CNN
F 2 "Pin_Headers:Pin_Header_Straight_2x13" H 6750 3050 50  0001 C CNN
F 3 "" H 6750 3050 50  0000 C CNN
	1    6750 4200
	1    0    0    -1  
$EndComp
$Comp
L 74LS595 U7
U 1 1 571ACC7D
P 8550 4550
F 0 "U7" H 8700 5150 50  0000 C CNN
F 1 "74LS595" H 8550 3950 50  0000 C CNN
F 2 "Housings_DIP:DIP-16_W7.62mm" H 8550 4550 50  0001 C CNN
F 3 "" H 8550 4550 50  0000 C CNN
	1    8550 4550
	-1   0    0    -1  
$EndComp
$Comp
L 74LS595 U5
U 1 1 571ACF64
P 1700 6750
F 0 "U5" H 1850 7350 50  0000 C CNN
F 1 "74LS595" H 1700 6150 50  0000 C CNN
F 2 "Housings_DIP:DIP-16_W7.62mm" H 1700 6750 50  0001 C CNN
F 3 "" H 1700 6750 50  0000 C CNN
	1    1700 6750
	1    0    0    -1  
$EndComp
$Comp
L 74LS595 U4
U 1 1 571AD01D
P 1700 5350
F 0 "U4" H 1850 5950 50  0000 C CNN
F 1 "74LS595" H 1700 4750 50  0000 C CNN
F 2 "Housings_DIP:DIP-16_W7.62mm" H 1700 5350 50  0001 C CNN
F 3 "" H 1700 5350 50  0000 C CNN
	1    1700 5350
	1    0    0    -1  
$EndComp
$Comp
L R R44
U 1 1 571AD0BB
P 7700 4100
F 0 "R44" V 7780 4100 50  0001 C CNN
F 1 "220R" V 7700 4100 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 7630 4100 50  0001 C CNN
F 3 "" H 7700 4100 50  0000 C CNN
	1    7700 4100
	0    1    1    0   
$EndComp
$Comp
L R R45
U 1 1 571AD31C
P 7700 4200
F 0 "R45" V 7780 4200 50  0001 C CNN
F 1 "220R" V 7700 4200 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 7630 4200 50  0001 C CNN
F 3 "" H 7700 4200 50  0000 C CNN
	1    7700 4200
	0    1    1    0   
$EndComp
$Comp
L R R46
U 1 1 571AD335
P 7700 4300
F 0 "R46" V 7780 4300 50  0001 C CNN
F 1 "220R" V 7700 4300 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 7630 4300 50  0001 C CNN
F 3 "" H 7700 4300 50  0000 C CNN
	1    7700 4300
	0    1    1    0   
$EndComp
$Comp
L R R47
U 1 1 571AD33B
P 7700 4400
F 0 "R47" V 7780 4400 50  0001 C CNN
F 1 "220R" V 7700 4400 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 7630 4400 50  0001 C CNN
F 3 "" H 7700 4400 50  0000 C CNN
	1    7700 4400
	0    1    1    0   
$EndComp
$Comp
L R R48
U 1 1 571AD354
P 7700 4500
F 0 "R48" V 7780 4500 50  0001 C CNN
F 1 "220R" V 7700 4500 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 7630 4500 50  0001 C CNN
F 3 "" H 7700 4500 50  0000 C CNN
	1    7700 4500
	0    1    1    0   
$EndComp
$Comp
L R R49
U 1 1 571AD35A
P 7700 4600
F 0 "R49" V 7780 4600 50  0001 C CNN
F 1 "220R" V 7700 4600 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 7630 4600 50  0001 C CNN
F 3 "" H 7700 4600 50  0000 C CNN
	1    7700 4600
	0    1    1    0   
$EndComp
$Comp
L R R50
U 1 1 571AD360
P 7700 4700
F 0 "R50" V 7780 4700 50  0001 C CNN
F 1 "220R" V 7700 4700 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 7630 4700 50  0001 C CNN
F 3 "" H 7700 4700 50  0000 C CNN
	1    7700 4700
	0    1    1    0   
$EndComp
$Comp
L R R51
U 1 1 571AD366
P 7700 4800
F 0 "R51" V 7780 4800 50  0001 C CNN
F 1 "220R" V 7700 4800 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 7630 4800 50  0001 C CNN
F 3 "" H 7700 4800 50  0000 C CNN
	1    7700 4800
	0    1    1    0   
$EndComp
$Comp
L CONN_02X13 P2
U 1 1 571AE1ED
P 3250 6400
F 0 "P2" H 3250 7100 50  0000 C CNN
F 1 "CONN_02X13" V 3250 6400 50  0000 C CNN
F 2 "Pin_Headers:Pin_Header_Straight_2x13" H 3250 5250 50  0001 C CNN
F 3 "" H 3250 5250 50  0000 C CNN
	1    3250 6400
	-1   0    0    1   
$EndComp
$Comp
L R R37
U 1 1 571B089F
P 3700 6400
F 0 "R37" V 3780 6400 50  0001 C CNN
F 1 "220R" V 3700 6400 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 3630 6400 50  0001 C CNN
F 3 "" H 3700 6400 50  0000 C CNN
	1    3700 6400
	0    1    1    0   
$EndComp
$Comp
L R R38
U 1 1 571B08A5
P 3700 6500
F 0 "R38" V 3780 6500 50  0001 C CNN
F 1 "220R" V 3700 6500 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 3630 6500 50  0001 C CNN
F 3 "" H 3700 6500 50  0000 C CNN
	1    3700 6500
	0    1    1    0   
$EndComp
$Comp
L R R39
U 1 1 571B08AB
P 3700 6600
F 0 "R39" V 3780 6600 50  0001 C CNN
F 1 "220R" V 3700 6600 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 3630 6600 50  0001 C CNN
F 3 "" H 3700 6600 50  0000 C CNN
	1    3700 6600
	0    1    1    0   
$EndComp
$Comp
L R R40
U 1 1 571B08B1
P 3700 6700
F 0 "R40" V 3780 6700 50  0001 C CNN
F 1 "220R" V 3700 6700 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 3630 6700 50  0001 C CNN
F 3 "" H 3700 6700 50  0000 C CNN
	1    3700 6700
	0    1    1    0   
$EndComp
$Comp
L R R41
U 1 1 571B08B7
P 3700 6800
F 0 "R41" V 3780 6800 50  0001 C CNN
F 1 "220R" V 3700 6800 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 3630 6800 50  0001 C CNN
F 3 "" H 3700 6800 50  0000 C CNN
	1    3700 6800
	0    1    1    0   
$EndComp
$Comp
L R R42
U 1 1 571B08BD
P 3700 6900
F 0 "R42" V 3780 6900 50  0001 C CNN
F 1 "220R" V 3700 6900 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 3630 6900 50  0001 C CNN
F 3 "" H 3700 6900 50  0000 C CNN
	1    3700 6900
	0    1    1    0   
$EndComp
$Comp
L R R43
U 1 1 571B08C3
P 3700 7000
F 0 "R43" V 3780 7000 50  0001 C CNN
F 1 "220R" V 3700 7000 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 3630 7000 50  0001 C CNN
F 3 "" H 3700 7000 50  0000 C CNN
	1    3700 7000
	0    1    1    0   
$EndComp
$Comp
L R R29
U 1 1 571B08F5
P 2550 6400
F 0 "R29" V 2630 6400 50  0001 C CNN
F 1 "220R" V 2550 6400 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 2480 6400 50  0001 C CNN
F 3 "" H 2550 6400 50  0000 C CNN
	1    2550 6400
	0    1    1    0   
$EndComp
$Comp
L R R30
U 1 1 571B08FB
P 2550 6500
F 0 "R30" V 2630 6500 50  0001 C CNN
F 1 "220R" V 2550 6500 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 2480 6500 50  0001 C CNN
F 3 "" H 2550 6500 50  0000 C CNN
	1    2550 6500
	0    1    1    0   
$EndComp
$Comp
L R R31
U 1 1 571B0901
P 2550 6600
F 0 "R31" V 2630 6600 50  0001 C CNN
F 1 "220R" V 2550 6600 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 2480 6600 50  0001 C CNN
F 3 "" H 2550 6600 50  0000 C CNN
	1    2550 6600
	0    1    1    0   
$EndComp
$Comp
L R R32
U 1 1 571B0907
P 2550 6700
F 0 "R32" V 2630 6700 50  0001 C CNN
F 1 "220R" V 2550 6700 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 2480 6700 50  0001 C CNN
F 3 "" H 2550 6700 50  0000 C CNN
	1    2550 6700
	0    1    1    0   
$EndComp
$Comp
L R R33
U 1 1 571B090D
P 2550 6800
F 0 "R33" V 2630 6800 50  0001 C CNN
F 1 "220R" V 2550 6800 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 2480 6800 50  0001 C CNN
F 3 "" H 2550 6800 50  0000 C CNN
	1    2550 6800
	0    1    1    0   
$EndComp
$Comp
L R R34
U 1 1 571B0913
P 2550 6900
F 0 "R34" V 2630 6900 50  0001 C CNN
F 1 "220R" V 2550 6900 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 2480 6900 50  0001 C CNN
F 3 "" H 2550 6900 50  0000 C CNN
	1    2550 6900
	0    1    1    0   
$EndComp
$Comp
L R R35
U 1 1 571B0919
P 2550 7000
F 0 "R35" V 2630 7000 50  0001 C CNN
F 1 "220R" V 2550 7000 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 2480 7000 50  0001 C CNN
F 3 "" H 2550 7000 50  0000 C CNN
	1    2550 7000
	0    1    1    0   
$EndComp
$Comp
L R R21
U 1 1 571B1E45
P 2550 5000
F 0 "R21" V 2630 5000 50  0001 C CNN
F 1 "220R" V 2550 5000 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 2480 5000 50  0001 C CNN
F 3 "" H 2550 5000 50  0000 C CNN
	1    2550 5000
	0    1    1    0   
$EndComp
$Comp
L R R22
U 1 1 571B1E4B
P 2550 5100
F 0 "R22" V 2630 5100 50  0001 C CNN
F 1 "220R" V 2550 5100 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 2480 5100 50  0001 C CNN
F 3 "" H 2550 5100 50  0000 C CNN
	1    2550 5100
	0    1    1    0   
$EndComp
$Comp
L R R23
U 1 1 571B1E51
P 2550 5200
F 0 "R23" V 2630 5200 50  0001 C CNN
F 1 "220R" V 2550 5200 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 2480 5200 50  0001 C CNN
F 3 "" H 2550 5200 50  0000 C CNN
	1    2550 5200
	0    1    1    0   
$EndComp
$Comp
L R R24
U 1 1 571B1E57
P 2550 5300
F 0 "R24" V 2630 5300 50  0001 C CNN
F 1 "220R" V 2550 5300 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 2480 5300 50  0001 C CNN
F 3 "" H 2550 5300 50  0000 C CNN
	1    2550 5300
	0    1    1    0   
$EndComp
$Comp
L R R25
U 1 1 571B1E5D
P 2550 5400
F 0 "R25" V 2630 5400 50  0001 C CNN
F 1 "220R" V 2550 5400 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 2480 5400 50  0001 C CNN
F 3 "" H 2550 5400 50  0000 C CNN
	1    2550 5400
	0    1    1    0   
$EndComp
$Comp
L R R26
U 1 1 571B1E63
P 2550 5500
F 0 "R26" V 2630 5500 50  0001 C CNN
F 1 "220R" V 2550 5500 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 2480 5500 50  0001 C CNN
F 3 "" H 2550 5500 50  0000 C CNN
	1    2550 5500
	0    1    1    0   
$EndComp
$Comp
L R R27
U 1 1 571B1E69
P 2550 5600
F 0 "R27" V 2630 5600 50  0001 C CNN
F 1 "220R" V 2550 5600 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 2480 5600 50  0001 C CNN
F 3 "" H 2550 5600 50  0000 C CNN
	1    2550 5600
	0    1    1    0   
$EndComp
$Comp
L 74LS595 U6
U 1 1 571ACF19
P 4550 6750
F 0 "U6" H 4550 6950 50  0000 C CNN
F 1 "74LS595" H 4550 6150 50  0000 C CNN
F 2 "Housings_DIP:DIP-16_W7.62mm" H 4550 6750 50  0001 C CNN
F 3 "" H 4550 6750 50  0000 C CNN
	1    4550 6750
	-1   0    0    -1  
$EndComp
$Comp
L GND #PWR02
U 1 1 571B5423
P 5250 6900
F 0 "#PWR02" H 5250 6650 50  0001 C CNN
F 1 "GND" H 5250 6750 50  0000 C CNN
F 2 "" H 5250 6900 50  0000 C CNN
F 3 "" H 5250 6900 50  0000 C CNN
	1    5250 6900
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR03
U 1 1 571B545B
P 1000 6900
F 0 "#PWR03" H 1000 6650 50  0001 C CNN
F 1 "GND" H 1000 6750 50  0000 C CNN
F 2 "" H 1000 6900 50  0000 C CNN
F 3 "" H 1000 6900 50  0000 C CNN
	1    1000 6900
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR04
U 1 1 571B5493
P 1000 5500
F 0 "#PWR04" H 1000 5250 50  0001 C CNN
F 1 "GND" H 1000 5350 50  0000 C CNN
F 2 "" H 1000 5500 50  0000 C CNN
F 3 "" H 1000 5500 50  0000 C CNN
	1    1000 5500
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR05
U 1 1 571B5861
P 9250 4700
F 0 "#PWR05" H 9250 4450 50  0001 C CNN
F 1 "GND" H 9250 4550 50  0000 C CNN
F 2 "" H 9250 4700 50  0000 C CNN
F 3 "" H 9250 4700 50  0000 C CNN
	1    9250 4700
	1    0    0    -1  
$EndComp
NoConn ~ 2400 5800
NoConn ~ 2400 4900
$Comp
L Arduino_Nano_Header J1
U 1 1 571B5B0A
P 10550 3100
F 0 "J1" H 10550 3900 60  0000 C CNN
F 1 "Arduino_Nano_Header" V 10600 3100 60  0000 C CNN
F 2 "w_conn_misc:arduino_nano_header" H 10550 3100 60  0001 C CNN
F 3 "" H 10550 3100 60  0000 C CNN
	1    10550 3100
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR06
U 1 1 571C0530
P 10950 4100
F 0 "#PWR06" H 10950 3850 50  0001 C CNN
F 1 "GND" H 10950 3950 50  0000 C CNN
F 2 "" H 10950 4100 50  0000 C CNN
F 3 "" H 10950 4100 50  0000 C CNN
	1    10950 4100
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR07
U 1 1 571C3761
P 2050 2300
F 0 "#PWR07" H 2050 2050 50  0001 C CNN
F 1 "GND" H 2050 2150 50  0000 C CNN
F 2 "" H 2050 2300 50  0000 C CNN
F 3 "" H 2050 2300 50  0000 C CNN
	1    2050 2300
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR08
U 1 1 571C47CF
P 4100 2300
F 0 "#PWR08" H 4100 2050 50  0001 C CNN
F 1 "GND" H 4100 2150 50  0000 C CNN
F 2 "" H 4100 2300 50  0000 C CNN
F 3 "" H 4100 2300 50  0000 C CNN
	1    4100 2300
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR09
U 1 1 571CB910
P 6500 4800
F 0 "#PWR09" H 6500 4550 50  0001 C CNN
F 1 "GND" H 6500 4650 50  0000 C CNN
F 2 "" H 6500 4800 50  0000 C CNN
F 3 "" H 6500 4800 50  0000 C CNN
	1    6500 4800
	1    0    0    -1  
$EndComp
NoConn ~ 2400 6300
NoConn ~ 3850 6300
NoConn ~ 10200 2400
NoConn ~ 10200 2500
NoConn ~ 10200 2600
NoConn ~ 10200 2700
NoConn ~ 10200 3800
NoConn ~ 10900 2400
NoConn ~ 10900 2600
NoConn ~ 10900 2800
NoConn ~ 10900 2900
NoConn ~ 10900 3000
NoConn ~ 10900 3100
NoConn ~ 10900 3200
NoConn ~ 10900 3300
NoConn ~ 10900 3400
NoConn ~ 10900 3500
NoConn ~ 10900 3600
NoConn ~ 10900 3700
NoConn ~ 10900 3800
$Comp
L GND #PWR010
U 1 1 571D0FCA
P 1250 2300
F 0 "#PWR010" H 1250 2050 50  0001 C CNN
F 1 "GND" H 1250 2150 50  0000 C CNN
F 2 "" H 1250 2300 50  0000 C CNN
F 3 "" H 1250 2300 50  0000 C CNN
	1    1250 2300
	1    0    0    -1  
$EndComp
$Comp
L VCC #PWR011
U 1 1 571D4424
P 11050 2250
F 0 "#PWR011" H 11050 2100 50  0001 C CNN
F 1 "VCC" H 11050 2400 50  0000 C CNN
F 2 "" H 11050 2250 50  0000 C CNN
F 3 "" H 11050 2250 50  0000 C CNN
	1    11050 2250
	1    0    0    -1  
$EndComp
$Comp
L VCC #PWR012
U 1 1 571D4C0D
P 5350 5550
F 0 "#PWR012" H 5350 5400 50  0001 C CNN
F 1 "VCC" H 5350 5700 50  0000 C CNN
F 2 "" H 5350 5550 50  0000 C CNN
F 3 "" H 5350 5550 50  0000 C CNN
	1    5350 5550
	1    0    0    -1  
$EndComp
$Comp
L VCC #PWR013
U 1 1 571D50D9
P 1000 4550
F 0 "#PWR013" H 1000 4400 50  0001 C CNN
F 1 "VCC" H 1000 4700 50  0000 C CNN
F 2 "" H 1000 4550 50  0000 C CNN
F 3 "" H 1000 4550 50  0000 C CNN
	1    1000 4550
	1    0    0    -1  
$EndComp
$Comp
L VCC #PWR014
U 1 1 571D53B8
P 9350 3750
F 0 "#PWR014" H 9350 3600 50  0001 C CNN
F 1 "VCC" H 9350 3900 50  0000 C CNN
F 2 "" H 9350 3750 50  0000 C CNN
F 3 "" H 9350 3750 50  0000 C CNN
	1    9350 3750
	1    0    0    -1  
$EndComp
$Comp
L VCC #PWR015
U 1 1 571D5DDA
P 1000 6000
F 0 "#PWR015" H 1000 5850 50  0001 C CNN
F 1 "VCC" H 1000 6150 50  0000 C CNN
F 2 "" H 1000 6000 50  0000 C CNN
F 3 "" H 1000 6000 50  0000 C CNN
	1    1000 6000
	1    0    0    -1  
$EndComp
$Comp
L USB_B P3
U 1 1 571D5FD0
P 10450 1050
F 0 "P3" H 10650 850 50  0000 C CNN
F 1 "USB_B" H 10400 1250 50  0000 C CNN
F 2 "Connect:USB_B" V 10400 950 50  0001 C CNN
F 3 "" V 10400 950 50  0000 C CNN
	1    10450 1050
	1    0    0    -1  
$EndComp
$Comp
L CONN_01X04 P4
U 1 1 571D60B2
P 10400 1550
F 0 "P4" H 10400 1800 50  0000 C CNN
F 1 "CONN_01X04" V 10500 1550 50  0000 C CNN
F 2 "Pin_Headers:Pin_Header_Straight_1x04" H 10400 1550 50  0001 C CNN
F 3 "" H 10400 1550 50  0000 C CNN
	1    10400 1550
	0    1    1    0   
$EndComp
$Comp
L CONN_01X02 P6
U 1 1 571CBFC1
P 4750 3700
F 0 "P6" H 4750 3850 50  0000 C CNN
F 1 "CONN_01X02" V 4900 3700 50  0000 C CNN
F 2 "Pin_Headers:Pin_Header_Straight_1x02" H 4750 3700 50  0001 C CNN
F 3 "" H 4750 3700 50  0000 C CNN
	1    4750 3700
	0    1    1    0   
$EndComp
$Comp
L CONN_01X02 P7
U 1 1 571CC050
P 4400 3700
F 0 "P7" H 4400 3850 50  0000 C CNN
F 1 "CONN_01X02" H 3950 3700 50  0000 C CNN
F 2 "Pin_Headers:Pin_Header_Straight_1x02" H 4400 3700 50  0001 C CNN
F 3 "" H 4400 3700 50  0000 C CNN
	1    4400 3700
	0    1    1    0   
$EndComp
$Comp
L CONN_01X02 P5
U 1 1 571CC15A
P 4050 3700
F 0 "P5" H 4050 3850 50  0000 C CNN
F 1 "CONN_01X02" H 3600 3700 50  0000 C CNN
F 2 "Pin_Headers:Pin_Header_Straight_1x02" H 4050 3700 50  0001 C CNN
F 3 "" H 4050 3700 50  0000 C CNN
	1    4050 3700
	0    1    1    0   
$EndComp
$Comp
L GND #PWR016
U 1 1 57211F5B
P 4150 5500
F 0 "#PWR016" H 4150 5250 50  0001 C CNN
F 1 "GND" H 4150 5350 50  0000 C CNN
F 2 "" H 4150 5500 50  0000 C CNN
F 3 "" H 4150 5500 50  0000 C CNN
	1    4150 5500
	1    0    0    -1  
$EndComp
$Comp
L R R8
U 1 1 57211F9B
P 4150 5350
F 0 "R8" V 4230 5350 50  0000 C CNN
F 1 "220R" V 4150 5350 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 4080 5350 50  0001 C CNN
F 3 "" H 4150 5350 50  0000 C CNN
	1    4150 5350
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR017
U 1 1 572278F9
P 10750 950
F 0 "#PWR017" H 10750 700 50  0001 C CNN
F 1 "GND" H 10750 800 50  0000 C CNN
F 2 "" H 10750 950 50  0000 C CNN
F 3 "" H 10750 950 50  0000 C CNN
	1    10750 950 
	1    0    0    -1  
$EndComp
$Comp
L VCC #PWR018
U 1 1 57228042
P 6500 3400
F 0 "#PWR018" H 6500 3250 50  0001 C CNN
F 1 "VCC" H 6500 3550 50  0000 C CNN
F 2 "" H 6500 3400 50  0000 C CNN
F 3 "" H 6500 3400 50  0000 C CNN
	1    6500 3400
	1    0    0    -1  
$EndComp
$Comp
L VCC #PWR019
U 1 1 57228F4F
P 4800 3500
F 0 "#PWR019" H 4800 3350 50  0001 C CNN
F 1 "VCC" H 4800 3650 50  0000 C CNN
F 2 "" H 4800 3500 50  0000 C CNN
F 3 "" H 4800 3500 50  0000 C CNN
	1    4800 3500
	1    0    0    -1  
$EndComp
$Comp
L R R52
U 1 1 587072D5
P 9500 5800
F 0 "R52" V 9450 5650 50  0000 C CNN
F 1 "1k" V 9500 5800 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" H 9430 5800 50  0001 C CNN
F 3 "" H 9500 5800 50  0000 C CNN
	1    9500 5800
	0    1    1    0   
$EndComp
$Comp
L R R53
U 1 1 5870767C
P 9500 5900
F 0 "R53" V 9450 5750 50  0000 C CNN
F 1 "1k" V 9500 5900 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 9430 5900 50  0001 C CNN
F 3 "" H 9500 5900 50  0000 C CNN
	1    9500 5900
	0    1    1    0   
$EndComp
$Comp
L R R54
U 1 1 58707841
P 9500 6000
F 0 "R54" V 9450 5850 50  0000 C CNN
F 1 "1k" V 9500 6000 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 9430 6000 50  0001 C CNN
F 3 "" H 9500 6000 50  0000 C CNN
	1    9500 6000
	0    1    1    0   
$EndComp
$Comp
L R R55
U 1 1 587078A3
P 9500 6100
F 0 "R55" V 9450 5950 50  0000 C CNN
F 1 "1k" V 9500 6100 50  0000 C CNN
F 2 "Resistors_ThroughHole:Resistor_Horizontal_RM7mm" V 9430 6100 50  0001 C CNN
F 3 "" H 9500 6100 50  0000 C CNN
	1    9500 6100
	0    1    1    0   
$EndComp
NoConn ~ 1450 900 
NoConn ~ 3500 900 
Wire Wire Line
	5350 6600 5350 5550
Wire Wire Line
	1000 5200 1000 4550
Wire Wire Line
	9350 4400 9350 3750
Wire Wire Line
	1000 6600 1000 6000
Wire Wire Line
	800  6500 1000 6500
Wire Wire Line
	800  5100 800  7550
Wire Wire Line
	1000 5100 800  5100
Wire Wire Line
	750  5400 1000 5400
Wire Wire Line
	750  5400 750  7600
Wire Wire Line
	750  6800 1000 6800
Wire Wire Line
	850  4900 1000 4900
Wire Wire Line
	850  7500 850  4900
Wire Wire Line
	2400 7500 850  7500
Wire Wire Line
	2400 7200 2400 7500
Wire Wire Line
	700  6300 1000 6300
Wire Wire Line
	700  7650 700  6300
Wire Wire Line
	3850 7650 700  7650
Wire Wire Line
	3850 7200 3850 7650
Wire Wire Line
	5600 6300 5250 6300
Wire Wire Line
	5600 5000 5600 6300
Wire Wire Line
	7850 5000 5600 5000
Wire Wire Line
	2800 5400 2800 6100
Wire Wire Line
	3500 7000 3550 7000
Wire Wire Line
	3500 6900 3550 6900
Wire Wire Line
	3500 6800 3550 6800
Wire Wire Line
	3500 6700 3550 6700
Wire Wire Line
	3500 6600 3550 6600
Wire Wire Line
	3500 6500 3550 6500
Wire Wire Line
	3500 6400 3550 6400
Wire Wire Line
	2700 7000 3000 7000
Wire Wire Line
	2700 6900 3000 6900
Wire Wire Line
	2700 6800 3000 6800
Wire Wire Line
	2700 6700 3000 6700
Wire Wire Line
	2700 6600 3000 6600
Wire Wire Line
	2700 6500 3000 6500
Wire Wire Line
	2700 6400 3000 6400
Wire Wire Line
	7000 4800 7550 4800
Wire Wire Line
	7000 4700 7550 4700
Wire Wire Line
	7000 4600 7550 4600
Wire Wire Line
	7000 4500 7550 4500
Wire Wire Line
	7550 4400 7000 4400
Wire Wire Line
	7550 4300 7000 4300
Wire Wire Line
	7000 4200 7550 4200
Wire Wire Line
	7000 4100 7550 4100
Wire Wire Line
	4800 2550 4800 750 
Wire Wire Line
	2200 2550 4800 2550
Wire Wire Line
	4250 2300 4250 2550
Wire Wire Line
	4700 2500 4700 650 
Wire Wire Line
	2350 2500 4700 2500
Wire Wire Line
	4400 2300 4400 2500
Wire Wire Line
	4700 650  8300 650 
Wire Wire Line
	7100 4000 7000 4000
Wire Wire Line
	7100 2850 7100 4000
Wire Wire Line
	7200 3900 7000 3900
Wire Wire Line
	7200 2350 7200 3900
Wire Wire Line
	7300 3800 7000 3800
Wire Wire Line
	7300 1850 7300 3800
Wire Wire Line
	7400 3700 7000 3700
Wire Wire Line
	7400 1350 7400 3700
Wire Wire Line
	5150 1850 5150 3800
Wire Wire Line
	5150 1850 5550 1850
Wire Wire Line
	3700 2850 5550 2850
Wire Wire Line
	5250 2350 5550 2350
Wire Wire Line
	5250 2350 5250 3900
Wire Wire Line
	5050 1350 5550 1350
Wire Wire Line
	5050 1350 5050 3700
Connection ~ 5550 2850
Connection ~ 5550 2350
Connection ~ 5550 1850
Connection ~ 5550 1350
Wire Wire Line
	5550 2750 5550 2950
Wire Wire Line
	5550 2250 5550 2450
Wire Wire Line
	5550 1750 5550 1950
Wire Wire Line
	5550 1250 5550 1450
Wire Wire Line
	2800 2300 3300 2300
Wire Wire Line
	2800 800  2800 2300
Wire Wire Line
	1350 800  2800 800 
Wire Wire Line
	1350 900  1350 800 
Wire Wire Line
	1950 2300 1950 3200
Wire Wire Line
	1850 2300 1850 3200
Wire Wire Line
	1750 2300 1750 3200
Wire Wire Line
	1650 2300 1650 3200
Wire Wire Line
	1550 2300 1550 3200
Wire Wire Line
	1450 2300 1450 3200
Wire Wire Line
	1350 2300 1350 3200
Wire Wire Line
	5250 6600 5350 6600
Wire Wire Line
	800  7550 5700 7550
Connection ~ 800  6500
Wire Wire Line
	750  7600 5800 7600
Connection ~ 750  6800
Wire Wire Line
	9250 4600 9700 4600
Wire Wire Line
	9250 4400 9350 4400
Wire Wire Line
	5700 7550 5700 5300
Wire Wire Line
	5700 5300 9600 5300
Wire Wire Line
	4800 750  8200 750 
Wire Wire Line
	8300 650  8300 2900
Wire Wire Line
	8200 750  8200 3000
Wire Wire Line
	8400 2800 8400 550 
Wire Wire Line
	8400 550  3400 550 
Wire Wire Line
	3400 550  3400 900 
Wire Wire Line
	9250 4100 9500 4100
Wire Wire Line
	9500 4100 9500 3100
Wire Wire Line
	9500 3100 10200 3100
Wire Wire Line
	9250 4300 9600 4300
Wire Wire Line
	9600 5300 9600 3200
Wire Wire Line
	9600 3200 10200 3200
Wire Wire Line
	9700 3300 9700 5400
Wire Wire Line
	9700 3300 10200 3300
Connection ~ 9600 4300
Wire Wire Line
	5800 7600 5800 5400
Wire Wire Line
	5800 5400 9700 5400
Connection ~ 9700 4600
Wire Wire Line
	5250 6800 5800 6800
Connection ~ 5800 6800
Wire Wire Line
	5250 6500 5700 6500
Connection ~ 5700 6500
Wire Wire Line
	10900 2700 11050 2700
Wire Wire Line
	11050 2700 11050 2250
Wire Wire Line
	5350 4000 6500 4000
Wire Wire Line
	5250 3900 6500 3900
Wire Wire Line
	5150 3800 6500 3800
Wire Wire Line
	5050 3700 6500 3700
Wire Wire Line
	5350 4000 5350 2850
Wire Wire Line
	7050 2850 7100 2850
Wire Wire Line
	7200 2350 7050 2350
Wire Wire Line
	7300 1850 7050 1850
Wire Wire Line
	7400 1350 7050 1350
Wire Wire Line
	3400 2300 3400 3200
Wire Wire Line
	3500 2300 3500 3200
Wire Wire Line
	3600 2300 3600 3200
Wire Wire Line
	3700 2300 3700 3200
Wire Wire Line
	3400 3650 3400 3500
Connection ~ 3400 3650
Wire Wire Line
	3500 3650 3500 3500
Connection ~ 3500 3650
Wire Wire Line
	3600 3650 3600 3500
Connection ~ 3600 3650
Wire Wire Line
	3700 3650 3700 3500
Connection ~ 3700 3650
Wire Wire Line
	3800 2300 4500 2300
Connection ~ 3900 2300
Connection ~ 4000 2300
Connection ~ 4100 2300
Wire Wire Line
	2200 2300 2200 2550
Connection ~ 4250 2550
Wire Wire Line
	2350 2500 2350 2300
Connection ~ 4400 2500
Wire Wire Line
	2700 5600 2700 6300
Wire Wire Line
	2700 5500 2750 5500
Wire Wire Line
	2750 5500 2750 6200
Wire Wire Line
	2700 5400 2800 5400
Wire Wire Line
	2700 5300 2850 5300
Wire Wire Line
	2850 5300 2850 6000
Wire Wire Line
	8400 2800 10200 2800
Wire Wire Line
	8300 2900 10200 2900
Wire Wire Line
	8200 3000 10200 3000
Wire Wire Line
	1450 3650 1450 3500
Wire Wire Line
	1550 3650 1550 3500
Wire Wire Line
	1650 3650 1650 3500
Wire Wire Line
	1750 3650 1750 3500
Wire Wire Line
	1850 3650 1850 3500
Wire Wire Line
	1950 3650 1950 3500
Connection ~ 1450 3650
Connection ~ 1550 3650
Connection ~ 1650 3650
Connection ~ 1750 3650
Connection ~ 1850 3650
Connection ~ 1950 3650
Wire Wire Line
	1350 3500 1350 3650
Wire Wire Line
	1350 3650 3700 3650
Wire Wire Line
	6500 4100 2650 4100
Wire Wire Line
	2650 4100 2650 2700
Wire Wire Line
	2650 2700 1350 2700
Connection ~ 1350 2700
Wire Wire Line
	6500 4200 2600 4200
Wire Wire Line
	2600 4200 2600 2750
Wire Wire Line
	2600 2750 1450 2750
Connection ~ 1450 2750
Wire Wire Line
	6500 4300 2550 4300
Wire Wire Line
	2550 4300 2550 2800
Wire Wire Line
	2550 2800 1550 2800
Connection ~ 1550 2800
Wire Wire Line
	6500 4400 2500 4400
Wire Wire Line
	2500 4400 2500 2850
Wire Wire Line
	2500 2850 1650 2850
Connection ~ 1650 2850
Wire Wire Line
	6500 4500 2450 4500
Wire Wire Line
	2450 4500 2450 2900
Wire Wire Line
	2450 2900 1750 2900
Connection ~ 1750 2900
Wire Wire Line
	6500 4600 2400 4600
Wire Wire Line
	2400 4600 2400 2950
Wire Wire Line
	2400 2950 1850 2950
Connection ~ 1850 2950
Wire Wire Line
	6500 4700 2350 4700
Wire Wire Line
	2350 4700 2350 3000
Wire Wire Line
	2350 3000 1950 3000
Connection ~ 1950 3000
Wire Wire Line
	3400 2700 5050 2700
Connection ~ 5050 2700
Connection ~ 3400 2700
Wire Wire Line
	3500 2750 5150 2750
Connection ~ 5150 2750
Connection ~ 3500 2750
Wire Wire Line
	3600 2800 5250 2800
Connection ~ 5250 2800
Connection ~ 3600 2800
Connection ~ 5350 2850
Connection ~ 3700 2850
Wire Wire Line
	2700 6300 3000 6300
Wire Wire Line
	2750 6200 3000 6200
Wire Wire Line
	2800 6100 3000 6100
Wire Wire Line
	2850 6000 3000 6000
Wire Wire Line
	2700 5200 2900 5200
Wire Wire Line
	2900 5200 2900 5900
Wire Wire Line
	2900 5900 3000 5900
Wire Wire Line
	2700 5100 2950 5100
Wire Wire Line
	2950 5100 2950 5800
Wire Wire Line
	2950 5800 3000 5800
Wire Wire Line
	2700 5000 3550 5000
Wire Wire Line
	3550 5000 3550 5800
Wire Wire Line
	3550 5800 3500 5800
Wire Wire Line
	10900 2500 10950 2500
Wire Wire Line
	10950 2500 10950 4100
Wire Wire Line
	9800 3400 10200 3400
Wire Wire Line
	9900 3500 10200 3500
Wire Wire Line
	10000 3600 10200 3600
Wire Wire Line
	10200 3700 10100 3700
Wire Wire Line
	4150 5200 4150 5100
Wire Wire Line
	4150 5100 3650 5100
Wire Wire Line
	3650 5100 3650 6300
Wire Wire Line
	3650 6300 3500 6300
Wire Wire Line
	6500 3600 6500 3400
Wire Wire Line
	6500 3400 7000 3400
Wire Wire Line
	7000 3400 7000 3600
Wire Wire Line
	4000 3500 4000 2850
Connection ~ 4000 2850
Wire Wire Line
	4350 3500 4350 2800
Connection ~ 4350 2800
Wire Wire Line
	4700 3500 4700 2750
Connection ~ 4700 2750
Wire Wire Line
	4100 3500 4800 3500
Connection ~ 4450 3500
Wire Wire Line
	3500 5900 3750 5900
Wire Wire Line
	3750 5900 3750 5800
Wire Wire Line
	9800 5800 9800 3400
Wire Wire Line
	9900 3500 9900 5900
Wire Wire Line
	3850 5900 3850 6000
Wire Wire Line
	3850 6000 3500 6000
Wire Wire Line
	10000 3600 10000 6000
Wire Wire Line
	3950 6000 3950 6100
Wire Wire Line
	3950 6100 3500 6100
Wire Wire Line
	10100 3700 10100 6100
Wire Wire Line
	4050 6100 4050 6200
Wire Wire Line
	4050 6200 3500 6200
Wire Wire Line
	9650 5800 9800 5800
Wire Wire Line
	9900 5900 9650 5900
Wire Wire Line
	10000 6000 9650 6000
Wire Wire Line
	10100 6100 9650 6100
Wire Wire Line
	3750 5800 9350 5800
Wire Wire Line
	3850 5900 9350 5900
Wire Wire Line
	3950 6000 9350 6000
Wire Wire Line
	4050 6100 9350 6100
Wire Wire Line
	2450 2300 2050 2300
Connection ~ 2050 2300
$EndSCHEMATC
