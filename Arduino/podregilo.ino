/**
*
* Copyright (c) 2015 Johannes Mueller <github@johannes-mueller.org>
* Distributed under the GNU GPL v2. For full terms see the file /LICENSE.
*
*/


const byte latchPin = 6;
const byte dataPin = 4;
const byte clockPin = 5;

const byte redPin = 2;
const byte greenPin = 3;



void setup()
{
	pinMode(latchPin,OUTPUT);
	pinMode(clockPin,OUTPUT);
	pinMode(dataPin,INPUT);
	pinMode(greenPin,OUTPUT);
	pinMode(redPin,OUTPUT);

	Serial.begin(9600);
}


#define BITNUM 16

void passButtonState()
{
	static unsigned int oldData = 0;
	unsigned int data = 0;

	digitalWrite(latchPin,LOW);
	delayMicroseconds(1);
	digitalWrite(latchPin,HIGH);

	for (int i=0; i<BITNUM; i++) {
		unsigned int bitval = digitalRead(dataPin);
		data |= (bitval << ((BITNUM-1)-i));

		digitalWrite(clockPin,HIGH);
		delayMicroseconds(1);
		digitalWrite(clockPin,LOW);
	}

	if (data != oldData) {
		Serial.write(lowByte(data));
		Serial.write(highByte(data));
	}

	oldData = data;
}

void handleState()
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
	handleState();
}
