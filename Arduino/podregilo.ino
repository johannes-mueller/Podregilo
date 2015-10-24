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


#define DATAWIDTH 8

byte data, oldData;


void setup()
{
	pinMode(latchPin,OUTPUT);
	pinMode(clockPin,OUTPUT);
	pinMode(dataPin,INPUT);
	pinMode(greenPin,OUTPUT);
	pinMode(redPin,OUTPUT);

	Serial.begin(9600);

	data = 0;
	oldData = 0;
}


void passButtonState()
{
	digitalWrite(latchPin,LOW);
	delayMicroseconds(1);
	digitalWrite(latchPin,HIGH);

	data = 0;

	for (int i=0; i<DATAWIDTH; i++) {
		byte bitval = digitalRead(dataPin);
		data |= (bitval << ((DATAWIDTH-1)-i));

		digitalWrite(clockPin,HIGH);
		delayMicroseconds(1);
		digitalWrite(clockPin,LOW);
	}

	if (data != oldData)
		Serial.write(data);

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
