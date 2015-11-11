#!/usr/bin/python
"""The software side of Podregilo

For now there's not very much docu yet. We are working on the code.
"""

from twisted.internet import reactor, protocol
from twisted.internet.protocol import Factory, Protocol
from twisted.internet.serialport import SerialPort
from txosc import async

from txosc import osc

import jack
import numpy as np
import socket
import struct

import sys

__author__ = "Johannes Mueller"
__copyright__ = "Copyright 2015, The Podregilo Project"
__license__ = "GPLv2"
__email__ = "github@johannes-mueller.org"
__status__ = "Proof of concept"


channelnum = 4

arduino = None



class Int64Argument(osc.Argument):
    """An L{Argument} representing a 64-bit signed integer.

    This is derived from IntArgument of the txosc module. Will place a
    pullrequest to txosc soon.
    """
    typeTag = "h"

    def _check_type(self):
        if type(self.value) not in [int, long]:
            raise TypeError("Value %s must be an integer or a long, not a %s." % (self.value, type(self.value).__name__))

    def toBinary(self):
        if self.value >= 1<<63:
            raise OverflowError("Integer too large: %d" % self.value)
        if self.value < -1<<63:
            raise OverflowError("Integer too small: %d" % self.value)
        return struct.pack(">i", int(self.value))


    @staticmethod
    def fromBinary(data):
        try:
            i = struct.unpack(">q", data[:8])[0]
            leftover = data[8:]
        except struct.error:
            raise osc.OscError("Too few bytes left to get an int from %s." % (data))
            #FIXME: do not raise error and return leftover anyways ?
        return Int64Argument(i), leftover

    def __int__(self):
        return int(self.value)


def elementFromBinary(data):
    if data[0] == "/":
        element, data = osc.Message.fromBinary(data)
    elif data.startswith("#bundle"):
        element, data = osc.Bundle.fromBinary(data)
    elif data.startswith("#reply"):
        element, data = osc.Message.fromBinary(data)
    else:
        raise OscError("Error parsing OSC data: " + data)
    return element


osc._tags['h'] = Int64Argument
osc._types[long] = Int64Argument


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

    def sendTime(self, seconds):
        data = "t"
        for i in range(2):
            data += chr((seconds >> 8*i) & 0b11111111)

        self.transport.write(data)

    def dataReceived(self,data):
        if data[0] == '?':
            print "Probe received"
            self.transport.write('!')
            return
        self.data += data
#        print "received", len(data), "bytes, total", len(self.data), "expecting", self.bytesExpected
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


class OSCSender(protocol.DatagramProtocol):


    def __init__(self, host="127.0.0.1", port=3819):
        self.port = port
        self.host = host
        reactor.listenUDP(0,self)

        self.frameRate = 48000
        self.queryRouteList()
        self.pollTime()

    def queryRouteList(self):
        self.sendMessage(osc.Message("/routes/list"))

    def pollTime(self):
        m = osc.Message("/ardour/transport_frame")
        self.sendMessage(m)
        reactor.callLater(0.1, self.pollTime)

    def datagramReceived(self, data, (host, port)):
        element = elementFromBinary(data)
        if element.address == "/ardour/transport_frame":
            arduino.sendTime(int(element.getValues()[0])/self.frameRate)
        elif element.address == "#reply":
            self.handleReply(element)

    def handleReply(self, element):
        if element.getValues()[0] == "end_route_list":
            self.frameRate = element.getValues()[1]

    def sendMessage(self,message):
        self.transport.write(message.toBinary(), (socket.gethostbyname(self.host), self.port))

    def handleMuteButton(self,i,bs):
        value = 1.
        if bs:
            value = 0.
        self.sendMessage(osc.Message("/ardour/routes/gainabs", i+1, value))


def dummyHandleButton(i,bs):
    print "Dummy Button Handler %d to %d" %(i,bs)


class JackClient():
    def __init__(self):
        self.initialize()

    def initialize(self):
        try:
            self.jc = jack.Client("podregilo", no_start_server=True)
            self.ports = []
            for i in range(channelnum):
                self.ports.append(self.jc.inports.register("vocxo-"+str(i+1)))
                self.jc.set_process_callback(self.process)
            self.jc.activate()
        except jack.JackError:
            print "Could not connect to jackd. Will try again in 10 seconds."
            reactor.callLater(10, self.initialize)


    def process(self,frames):
        data = "l"
        for p in self.ports:
            audio = p.get_array()
            m1 = np.max(audio)
            m2 = -np.min(audio)
            m = max(m1,m2)
            if m > 1.:
                m = 1.
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
