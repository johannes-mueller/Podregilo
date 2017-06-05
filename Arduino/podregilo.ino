/**
*
* Copyright (c) 2015 Johannes Mueller <github@johannes-mueller.org>
* Distributed under the GNU GPL v2. For full terms see the file /LICENSE.
*
*/

//! Define in/out pins of Arduino
//!
const byte latchPin_in = 4;
const byte dataPin_in = 2;
const byte clockPin_in = 3;

const byte latchPin_out = 7;
const byte dataPin_out = 5;
const byte clockPin_out = 6;

static const unsigned int LONG_PRESS_TIMEOUT = 1000;

//! number of audio channel (podcasting voices)
const byte channelnum = 4;


/** The output buffer.

    We need 4 bytes to shift out. The last one to transport the meter
    levels. The three others transport the time code to the seven
    segment displays.
 */
const byte outbuflen = 4;
byte outbuf[outbuflen];

//! The index of the meter level byte.
const byte levelind = 3;

//! The time of the last level meter update
unsigned long lastMeterUpdate;

//! The DAW timecode in seconds
unsigned int secondsDAW = 0;

//! Is there at the moment or when was the last signal from the DAW
bool haveDAWConnection = false;
unsigned long lastDAWConnectionTime = 0;

double transportSpeed;

bool recEnabled;


unsigned int xruns = 0;


enum LEDcolor { dark = 0b00, red = 0b01, green = 0b10, yellow = 0b11 };
enum LEDstate { off = false, on = true };

struct LED
{
        bool state, oldState;
        unsigned long lastChangeTime;
        unsigned int blinkTime;
        byte pin;
};

struct LED diagRed = { off,off, 0, 0, 9 };
struct LED diagGreen = { off,off, 0, 0, 8 };

struct LED recEnableLED = { off,off, 0, 0, 11 };
struct LED speedLED = { off,off, 0, 0, 10 };


void execLED(struct LED *l)
{
        unsigned long time = millis();

        if (l->blinkTime && (time >  l->lastChangeTime+l->blinkTime))
                l->state = !l->state;

        if (l->state != l->oldState) {
                digitalWrite(l->pin, l->state);
                l->oldState = l->state;
                l->lastChangeTime = time;
        }
}

void setLED(struct LED *l, LEDstate state)
{
        l->state = state;
        l->blinkTime = 0;
}

void blinkLED(struct LED *l, unsigned int period)
{
        l->blinkTime = period;
}

void indicate_xruns()
{
        blinkLED(&diagRed, 2000/xruns);
}


struct Prober
{
        unsigned long probeTime;
        bool answerReceived;
} prober = { 0, false };

const unsigned int probePeriod = 10000;
const unsigned int probeTimeOut = 1000;

void sendProbe()
{
        Serial.write('?');
        prober.answerReceived = false;
        prober.probeTime = millis();
}

bool haveConnection()
{
        static bool optimistic = true;

        if (prober.answerReceived) {
                if (!prober.probeTime| (millis() - prober.probeTime > probePeriod)) {
                        sendProbe();
                        optimistic = true;
                }
                setLED(&diagGreen, on);
                indicate_xruns();
                return true;
        }

        if (millis() > prober.probeTime + probeTimeOut) {
                setLED(&diagGreen, off);
                blinkLED(&diagRed, 200);
                sendProbe();
                optimistic = false;

                return false;
        }

        if (optimistic) {
                setLED(&diagRed, on);
                setLED(&diagGreen, off);
        }

        return false;
}


void passButtonState()
{
	static unsigned int oldData = 0;
	static unsigned int old_long_press = 0;
	static unsigned long press_times[16] = { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 };

	unsigned int data = 0;
	unsigned int long_press = 0;

	digitalWrite(latchPin_in,LOW);
	delayMicroseconds(1);
	digitalWrite(latchPin_in,HIGH);

	for (unsigned int i=0; i<8*sizeof(data); i++) {
		unsigned int bitval = digitalRead(dataPin_in) << ((8*sizeof(data)-1)-i);
		data |= bitval;

		digitalWrite(clockPin_in,HIGH);
		delayMicroseconds(1);
		digitalWrite(clockPin_in,LOW);
	}

	for (unsigned int i=0; i<8*sizeof(data); i++) {
		unsigned int mask = 1 << i;
		if (data & mask) {
			if (press_times[i]) {
				if (millis() - press_times[i] > LONG_PRESS_TIMEOUT) {
					long_press |= mask;
				}
			} else {
				press_times[i] = millis();
			}
		} else {
			press_times[i] = 0;
		}
	}

	if (data != oldData) {
                Serial.write('b');
		Serial.write((uint8_t*) &data, 2);
		oldData = data;
        }


	unsigned int change = long_press & (long_press ^ old_long_press);
	if (change) {
		Serial.write('l');
		Serial.write((uint8_t*) &change, 2);
	}
	old_long_press = long_press;
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

                if (v>200)
                        outbuf[levelind] |= (red << 2*i);
                else if (v>64)
                        outbuf[levelind] |= (yellow << 2*i);
                else if (v>8)
                        outbuf[levelind] |= (green  << 2*i);
        }

        lastMeterUpdate = millis();
}

union uintBuffer { char buffer[2]; unsigned int s; };

void updateSeconds()
{
        uintBuffer data;
        byte n = Serial.readBytes(data.buffer, 2);
        if (n!=2)
                return;

        secondsDAW = data.s;

        haveDAWConnection = true;
        lastDAWConnectionTime = millis();
}


union doubleBuffer { char buffer[sizeof(double)]; double d; };

void updateSpeed()
{
        doubleBuffer data;
        byte n = Serial.readBytes(data.buffer,sizeof(double));
        if (n!=sizeof(double))
                return;
        // FIXME: error handling

        transportSpeed = data.d;
}

void updateRecEnabled()
{
        char b;
        Serial.readBytes(&b,1);

        recEnabled = (b != '\0');
}

void new_xrun()
{
        xruns += 1;
        indicate_xruns();
}

void reset_xruns()
{
        xruns = 0;
        setLED(&diagRed, off);
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
        case 't':
                updateSeconds();
                break;
        case 's':
                updateSpeed();
                break;
        case 'r':
                updateRecEnabled();
                break;
        case 'x':
                new_xrun();
                break;
        case 'X':
                reset_xruns();
                break;
        case '!':
                prober.answerReceived = true;
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


void updateDisplay()
{
        static const byte sevenSegment[] = {
                0b00000001, // 0
                0b01001111, // 1
                0b00010010, // 2
                0b00000110, // 3
                0b01001100, // 4
                0b00100100, // 5
                0b00100000, // 6
                0b00001111, // 7
                0b00000000, // 8
                0b00000100, // 9
        };

        static const byte digit1 = 2;
        static const byte digit2 = 1;
        static const byte digit3 = 0;

        static const byte minus = 0b01111110;

        if (!haveDAWConnection) {
                outbuf[digit1] = minus;
                outbuf[digit2] = minus;
                outbuf[digit3] = minus;

                return;
        }

        unsigned int hours  = secondsDAW / 3600;
        unsigned int minutes = (secondsDAW - hours*3600)/60;

        if (hours > 9)
                outbuf[digit1] = minus;
        else
                outbuf[digit1] = sevenSegment[hours];

        byte tens = minutes/10;
        byte ones = (minutes - tens*10);

        outbuf[digit2] = sevenSegment[tens];
        outbuf[digit3] = sevenSegment[ones];
}

void updateDAWStateLeds()
{
        if (transportSpeed == 0.0)
                setLED(&speedLED, off);
        else
                blinkLED(&speedLED, (unsigned int) 1000.0/transportSpeed);

        setLED(&recEnableLED, LEDstate(recEnabled));
}


void setup()
{
	pinMode(latchPin_in,OUTPUT);
	pinMode(clockPin_in,OUTPUT);
	pinMode(dataPin_in,INPUT);

	pinMode(latchPin_out,OUTPUT);
	pinMode(clockPin_out,OUTPUT);
	pinMode(dataPin_out,OUTPUT);

        pinMode(diagRed.pin, OUTPUT);
        pinMode(diagGreen.pin, OUTPUT);
        pinMode(recEnableLED.pin, OUTPUT);
        pinMode(speedLED.pin, OUTPUT);

        transportSpeed = 0.0;
        recEnabled = false;
        xruns = 0;

        Serial.begin(9600);

        updateDAWStateLeds();

        updateDisplay();
        shiftOutData();
}


void loop()
{
        execLED(&diagRed);
        execLED(&diagGreen);
        execLED(&recEnableLED);
        execLED(&speedLED);

	checkSerialBuffer();

        if (!haveConnection()) {
                setLED(&speedLED, off);
                setLED(&recEnableLED, off);
                return;
        }

        passButtonState();
        updateDisplay();
        updateDAWStateLeds();
	shiftOutData();

        unsigned long time = millis();
        if (time-lastMeterUpdate > 200)
                outbuf[levelind] = 0b00000000;

        if (time-lastDAWConnectionTime > 2000)
                haveDAWConnection = false;
}
