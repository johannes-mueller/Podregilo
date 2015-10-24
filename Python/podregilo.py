#!/usr/bin/python

from twisted.internet import reactor
from twisted.internet.protocol import Factory, Protocol
from twisted.internet.serialport import SerialPort

import jack

import numpy as np

arduino = None

def dummyCallback(state):
    pass


class ArduinoConnection(Protocol):
    def connectionMade(self):
        global arduino
        arduino = self
        print 'Arduino device: ', self, ' is connected.'
        self.oldMuteButtons = 0b0000
        self.oldOtherButtons = 0b0000
        self.muteCallback = dummyCallback
        self.otherCallback = dummyCallback

    def setMuteCallback(self,func):
        self.muteCallback = func

    def sendData(self, data):
        self.transport.write(chr(data))

    def dataReceived(self,data):
        buttonState = ord(data[-1])
        muteButtons = buttonState >> 4
        otherButtons = buttonState & 0b00001111

        if muteButtons != self.oldMuteButtons:
            self.muteCallback(muteButtons)
            self.oldMuteButtons = muteButtons


class ButtonHandler():
    def __init__(self):
        self.buttons = [False, False, False, False]

    def setCallback(self,func):
        self.callback = func

    def buttonsChanged(self,state):
        bn = 1
        for i,b in enumerate(self.buttons):
            bs = bool(state & bn)
            bn = bn << 1
            if bs != self.buttons[i]:
                self.callback(i,bs)
                self.buttons[i] = bs


def handleMuteButton(i,bs):
    if bs:
        print "Muting",
    else:
        print "Unmuting",
    print "channel number %d" % (i+1)


class JackClient():
    def __init__(self):
        self.jc = jack.Client("podregilo")
        self.port1 = self.jc.inports.register("vocxo-1")
        self.jc.set_process_callback(self.process)
        self.jc.activate()

    def process(self,frames):
        audio = self.port1.get_array()
        m1 = np.max(audio)
        m2 = -np.min(audio)
        m = max(m1,m2)
        arduino.sendData(int(m*10))

if __name__ == "__main__":
    SerialPort(ArduinoConnection(), '/dev/ttyUSB0', reactor, 9600)
    MbH = ButtonHandler()
    MbH.setCallback(handleMuteButton)
    arduino.setMuteCallback(MbH.buttonsChanged)
    jc = JackClient()
    reactor.run()
