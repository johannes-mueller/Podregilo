/**
*
* Copyright (c) 2015 Johannes Mueller <github@johannes-mueller.org>
* Distributed under the GNU GPL v2. For full terms see the file /LICENSE.
*
*/


const byte latchPin_in = 3;
const byte dataPin_in = 4;
const byte clockPin_in = 2;

const byte latchPin_out = 6;
const byte dataPin_out = 5;
const byte clockPin_out = 7;


const byte channelnum = 4;

const byte outbuflen = 1;
byte outbuf[outbuflen] = { 0b11011011 };//, 0b01010101, 0b01000001, 0b10101010 };

const byte levelind = 0;

enum LedState { dark = 0b00, red = 0b10, green = 0b01, yellow = 0b11 };

long lastMeterUpdate;


void setup()
{
	pinMode(latchPin_in,OUTPUT);
	pinMode(clockPin_in,OUTPUT);
	pinMode(dataPin_in,INPUT);

	pinMode(latchPin_out,OUTPUT);
	pinMode(clockPin_out,OUTPUT);
	pinMode(dataPin_out,OUTPUT);
	Serial.begin(9600);
}


void passButtonState()
{
	static unsigned int oldData = 0;
	unsigned int data = 0;

	digitalWrite(latchPin_in,LOW);
	delayMicroseconds(1);
	digitalWrite(latchPin_in,HIGH);

	for (unsigned int i=0; i<8*sizeof(data); i++) {
		unsigned int bitval = digitalRead(dataPin_in);
		data |= (bitval << ((8*sizeof(data)-1)-i));

		digitalWrite(clockPin_in,HIGH);
		delayMicroseconds(1);
		digitalWrite(clockPin_in,LOW);
	}

	if (data != oldData)
		Serial.write((uint8_t*) &data,2);

	oldData = data;
}


void adjustLevels()
{
        outbuf[levelind] = 0b00000000;
        char buffer[channelnum];

        byte n = Serial.readBytes(buffer,channelnum);
        if (n!=4)
                outbuf[levelind] = 0b11111111;

        // FIXME: error handling; what if there's not enough bytes available

        for (byte i=0; i<channelnum; i++) {
                byte v = buffer[i];

                if (v>192)
                        outbuf[levelind] |= (red << 2*i);
                else if (v>96)
                        outbuf[levelind] |= (yellow << 2*i);
                else if (v>64)
                        outbuf[levelind] |= (green  << 2*i);
        }

        lastMeterUpdate = millis();
}


void checkSerialBuffer()
{
	if (!Serial.available())
		return;

	char c = Serial.read();

        switch(c) {
        case 'l':
                adjustLevels();
                break;
        default:
                break;
                // FIXME: error handling
        }
}

void shiftOutData()
{
	for (byte i=0; i<outbuflen; i++) {
		byte p = 1;
		for (byte j=0; j<8; j++) {
			digitalWrite(dataPin_out, p & outbuf[i]);
			p = p << 1;

			digitalWrite(clockPin_out, HIGH);
			delayMicroseconds(1);
			digitalWrite(clockPin_out, LOW);
		}
	}

	digitalWrite(latchPin_out, LOW);
	delayMicroseconds(1);
	digitalWrite(latchPin_out, HIGH);
}

void loop()
{
	passButtonState();
	checkSerialBuffer();
	shiftOutData();

        if (millis()-lastMeterUpdate > 200)
                outbuf[levelind] = 0b00000000;

}
