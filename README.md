# Podregilo

Arduino based hardware to control a podcasting DAW.


## Disclaimer

By now this project is far from being useful. It might be interesting to other developers though.


# Goal

We are developing and constructing an Arduino based hardware box to control our
DAW while podcasting. The following features are planned:

* mute buttons for all speakers

* buttons to start, stop and prepare the recording

* LED indications of the meter levels of the voices

* buttons to start jingles and other clips

* displaying the current time


# Status

## Prototypes

* First prototype on strip board

  The first prototype was soldered and wired completely manually on stripboards
  using magnet wire for every single comnnection. It was quite a tedious work
  building it but it has the advantage that we could test and develop it
  actually while building the first prototype. In the end the prototype worked
  but turned out to be far to unstable for serious work.

* Second prototype on an etched PCB.

  With what we've learned from the first prototype design, we designed the
  circuit in KiCad and then designed two PCBs, one for the front panel and one
  for the actual circuit. These PCBs we had produced by a professional PCB
  manufacturer. It turned out that we had some bugs in the circuit design which
  we for the second prototype fixed using a grinding tool and again a bit of
  magnet wire.

## Current Status

Right now we have the second prototype up and running and plan to use it in
actual everyday podcasting for a couple of months. Depending on our experiences
we will either have new PCBs produced or we will add some features, that need
hardware modifications.

# Used technology

The hardware is build around an Arduino compatible board, probably the Arduino
nano. We are using parallel shift registers SN74HC165 and SN74HC595 to get and
set the states of our hardware items. On the software side we speak OSC to our
DAW and also probably MIDI to other software. The meter levels we get via
jack. The software is written in python.


# Aimed platform

We are developing on Linux/Ardour. It shouldn't be too difficult to port the
stuff to Mac/Ardour or even Mac/Reaper.
