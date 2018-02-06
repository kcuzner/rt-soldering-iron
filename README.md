# RT Soldering Iron
By Kevin Cuzner

**NOTE: THIS IS A WORK IN PROGRESS AND MAY NEVER BE COMPLETED**

## Objective

The Weller RT Soldering Iron Tips are $20-$30 apiece. They are rated to about
55W and many variants come to a very fine point. I had the pleasure of using one
of these soldering irons at my university, but the base stations cost several
hundred dollars. This project aims to build a "base station" of sorts for these
tips so that I can use them without needing to purchase a real base station.

Note that this project has been done before, but I'm doing it myself because
reasons.

## Concept

- The Weller RT soldering iron tips have a 1/8" audio plug which has ground,
  heater, and thermocouple contacts.
- The heater can be powered from 12V and will consume 55W.
- Without closed loop control, the soldering iron tip will quickly burn itself
  up.
- This driver board will provide 12V and closed loop heater control.
- The driver board has a 1/8" audio jack on one end and a barrel jack on the
  other. It is short enough and skinny enough so that it can be mounted directly
  to the tip and become part of the handpiece, with the barrel jack connecting
  to the 12V power supply.

## Builds

### Rev A Deviations

### Rev B Deviations

- 490-14603-1-ND (0.1uF 0402 25V 10% cap) was unavailable. No lead time.
  Ordered 490-13341-1-ND (0.1uf 0402 25V 20% cap) instead.
- Accelerometer was out of stock (again...). The other variant used on the LED
  watch was out of stock too. Did not order this time around.

