/**
*
* Copyright (c) 2015 Johannes Mueller <github@johannes-mueller.org>
* Distributed under the GNU GPL v2. For full terms see the file /LICENSE.
*
*/


const byte latchPin = 3;
const byte dataPin = 4;
const byte clockPin = 2;

const byte redPin = 7;
const byte greenPin = 8;



void setup()
{
	pinMode(latchPin_in,OUTPUT);
	pinMode(clockPin_in,OUTPUT);
	pinMode(dataPin_in,INPUT);
	pinMode(greenPin,OUTPUT);
	pinMode(redPin,OUTPUT);

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
		Serial.write(lowByte(data));
		Serial.write(highByte(data));
	}

	oldData = data;
}

void checkSerialBuffer()
{
	if (!Serial.available())
		return;

	byte data = Serial.read();
	digitalWrite(redPin,LOW);
	digitalWrite(greenPin,LOW);

	if (data == 0)
		return;

	if (data < 4)
		digitalWrite(greenPin,HIGH);
	else
		digitalWrite(redPin,HIGH);
}

void loop()
{
	passButtonState();
	checkSerialBuffer();
}
