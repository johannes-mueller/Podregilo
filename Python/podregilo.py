#!/usr/bin/python
"""The software side of Podregilo

For now there's not very much docu yet. We are working on the code.
"""

from twisted.internet import reactor
from twisted.internet.protocol import Factory, Protocol
from twisted.internet.serialport import SerialPort
from txosc import async

from txosc import osc

import jack
import numpy as np

import sys

__author__ = "Johannes Mueller"
__copyright__ = "Copyright 2015, The Podregilo Project"
__license__ = "GPLv2"
__email__ = "github@johannes-mueller.org"
__status__ = "Proof of concept"


channelnum = 4

arduino = None

def dummyCallback(state):
    pass


class ArduinoConnection(Protocol):
    def connectionMade(self):
        global arduino
        arduino = self
        print 'Arduino device: ', self, ' is connected.'
        self.oldButtonGroup = [ 0b0000, 0b0000, 0b0000, 0b0000 ]
        self.buttonCallback = [ dummyCallback, dummyCallback, dummyCallback, dummyCallback ]
        self.data = []
        self.bytesExpected = 2

    def setCallback(self,i,func):
        self.buttonCallback[i] = func

    def sendData(self, data):
        self.transport.write(data)

    def dataReceived(self,data):
#        print "received", len(data), "bytes, total", len(self.data), "expecting", self.bytesExpected
        self.data += data
        if (len(self.data) > self.bytesExpected):
            print "*** Too many bytes, got %d, expected %d." % (len(self.data), self.bytesExpected)
            self.data = []
            return
        if (len(self.data) < self.bytesExpected):
            return

        bs = [ord(self.data[-1]), ord(self.data[-2])]
        self.data = []
        print '{0:#010b}'.format(bs[0]), '{0:#010b}'.format(bs[1])
        bGroup = [ bs[0] >> 4, bs[0] & 0b00001111, bs[1] >> 4, bs[1] & 0b00001111 ]

        for i, bg in enumerate(bGroup):
            if bg != self.oldButtonGroup[i]:
                self.buttonCallback[i](bg)
                self.oldButtonGroup[i] = bg


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


class OSCSender(object):
    def __init__(self, host="127.0.0.1", port=3819):
        self.port = port
        self.host = host
        self.client = async.DatagramClientProtocol()
        reactor.listenUDP(0,self.client)

    def sendMessage(self,message):
        self.client.send(message, (self.host, self.port))
        print message

    def handleMuteButton(self,i,bs):
        value = 1.
        if bs:
            value = 0.
        self.sendMessage(osc.Message("/ardour/routes/gainabs", i+1, value))


def dummyHandleButton(i,bs):
    print "Dummy Button Handler %d to %d" %(i,bs)


class JackClient():
    def __init__(self):
        self.jc = jack.Client("podregilo")
        self.ports = []
        for i in range(channelnum):
            self.ports.append(self.jc.inports.register("vocxo-"+str(i+1)))
        self.jc.set_process_callback(self.process)
        self.jc.activate()

    def process(self,frames):
        data = "l"
        for p in self.ports:
            audio = p.get_array()
            m1 = np.max(audio)
            m2 = -np.min(audio)
            m = max(m1,m2)
            if m > 1.:
                m = 1.
            print int(m*255)
            data += chr(int(m*255))

        arduino.sendData(data)


if __name__ == "__main__":
    SerialPort(ArduinoConnection(), '/dev/ttyUSB0', reactor, 9600)
    oscs = OSCSender()
    MbH = ButtonHandler()
    JbH = ButtonHandler()
    MbH.setCallback(oscs.handleMuteButton)
    JbH.setCallback(dummyHandleButton)
    arduino.setCallback(3,MbH.buttonsChanged)
    arduino.setCallback(2,JbH.buttonsChanged)
    jc = JackClient()
    reactor.run()
