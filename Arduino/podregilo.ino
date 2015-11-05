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


const byte outbuflen = 1;
byte outbuf[outbuflen] = { 0b11011011 };//, 0b01010101, 0b01000001, 0b10101010 };


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


#define BITNUM 16

void passButtonState()
{
	static unsigned int oldData = 0;
	unsigned int data = 0;

	digitalWrite(latchPin_in,LOW);
	delayMicroseconds(1);
	digitalWrite(latchPin_in,HIGH);

	for (int i=0; i<BITNUM; i++) {
		unsigned int bitval = digitalRead(dataPin_in);
		data |= (bitval << ((BITNUM-1)-i));

		digitalWrite(clockPin_in,HIGH);
		delayMicroseconds(1);
		digitalWrite(clockPin_in,LOW);
	}

	if (data != oldData) {
		Serial.write((uint8_t*) &data,2);
		Serial.flush();
		//		Serial.write(highByte(data));
	}

	oldData = data;
}

void checkSerialBuffer()
{
	if (!Serial.available())
		return;

	byte data = Serial.read();
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
}
