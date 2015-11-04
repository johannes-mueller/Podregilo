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

We are in the status of early development. Some first tests and proofs of
concept.


# Used technology

The hardware is build around an Arduino compatible board, probably the Arduino
nano. We are using parallel shift registers SN74HC165 and SN74HC595 to get and
set the states of our hardware items. On the software side we speak OSC to our
DAW and also probably MIDI to other software. The meter levels we get via
jack. The software is written in python.


# Aimed platform

We are developing on Linux/Ardour. It shouldn't be too difficult to port the
stuff to Mac/Ardour or even Mac/Reaper.
