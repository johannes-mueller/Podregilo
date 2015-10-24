#!/usr/bin/python

from twisted.internet import reactor
from twisted.internet.protocol import Factory, Protocol
from twisted.internet.serialport import SerialPort

import jack

import numpy as np

arduino = None


class ArduinoConnection(Protocol):
    def connectionMade(self):
        global arduino
        arduino = self
        print 'Arduino device: ', self, ' is connected.'
        self.state = 0

    def sendData(self, data):
        self.transport.write(chr(data))

    def dataReceived(self,data):
        self.state = ord(data[-1])
        print 'Buttons changed {:#010b}'.format(self.state)


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
    jc = JackClient()
    reactor.run()
