import processing.serial.*;

Serial  port;
boolean portSetup = false;
PImage img_real;
PImage img_mcu;

boolean waitingResponse = false;
boolean stateSending    = false;
boolean stateReceiving  = false;
boolean stateRunning    = false;

byte SEND_SET = 'S';
byte SEND_GET = 'G';
byte SEND_ACK = 'A';
byte SEND_ERR = 'E';
byte SEND_RUN = 'R';
byte SEND_RUN_CHUNK = 'C';
byte SEND_HANDSHAKE = 'H';

int num_blobs = 0;
int blobCur  = 0;
int pixelCur = 0;

// in degrees
int ra  = 248;
int dec = -26;

void setup()
{
  size(808*2, 608 + 50);  // Stage size
  noStroke();      // No border on the next thing drawn
  colorMode(RGB, 255);
  img_real = loadImage("fail_10.png");
  img_mcu  = createImage(808, 608, ARGB);
  for ( int i = 0; i < img_mcu.pixels.length; i++ )  img_mcu.pixels[i] = color(255, 255, 255, 255);


  // Print a list of the serial ports, for debugging purposes:
  printArray(Serial.list());


  var serialPorts = Serial.list();
  for ( int i = 0; i < serialPorts.length && !portSetup; i++ )
  {
    try
    {
      String portName = Serial.list()[i];
      port = new Serial(this, portName, 921600);
      System.out.println("Checking Port: " + portName);

      int time = millis();
      port.clear();

      port.write(SEND_HANDSHAKE);
      while ( port.available() == 0 && millis() - time < 1000 );

      if ( port.read() == SEND_HANDSHAKE )
      {
        System.out.println("Found Port: " + portName);
        portSetup = true;
      }
      if ( !portSetup )  System.out.println("FAILED to acknowledge.");
    }
    catch ( RuntimeException e ) {
      System.out.println("FAILED to connect.");
    }
  }

  System.out.println("\n\n");
}




void draw()
{
  textAlign(CENTER, CENTER);
  textSize(30);
  image(img_real, 0, 0);

  float progress = round(pixelCur / (608.0 * 808.0) * 608.0);
  if ( stateSending ) rect(0, progress, 808, 608 - progress);

  image(img_mcu, 808, 0);

  if ( button(0, 608, 404, 50, "SEND") && idle() )
  {
    stateSending = true;
    pixelCur = 0;
    sendAddress(0);
  } else if ( button(404, 608, 404, 50, "RECEIVE") && idle() )
  {
    stateReceiving = true;
    pixelCur = 0;
    port.write(SEND_GET);
    requestAddress(pixelCur);
  } else if ( button(808, 608, 404, 50, "RUN") && idle() )
  {
    stateRunning = true;
    port.write(SEND_RUN);
    task_time = millis();
  } else if ( button(1212, 608, 404, 50, "RUN") && idle() )
  {
    stateRunning = true;
    port.write(SEND_RUN_CHUNK);
    
    byte [] sendBuffer = new byte[8];
    sendBuffer[0] = (byte)((ra >> (8*0)) & 255);
    sendBuffer[1] = (byte)((ra >> (8*1)) & 255);
    sendBuffer[2] = (byte)((ra >> (8*2)) & 255);
    sendBuffer[3] = (byte)((ra >> (8*3)) & 255);
    sendBuffer[4] = (byte)((dec >> (8*0)) & 255);
    sendBuffer[5] = (byte)((dec >> (8*1)) & 255);
    sendBuffer[6] = (byte)((dec >> (8*2)) & 255);
    sendBuffer[7] = (byte)((dec >> (8*3)) & 255);
    port.write(sendBuffer);
    task_time = millis();
  }
}


boolean button ( int x, int y, int w, int h, String title )
{
  fill(100);
  rect(x, y, w, h);
  fill(255);
  text(title, x, y, w, h);
  return mousePressed && x < mouseX && mouseX < x + w && y < mouseY && mouseY < y + h;
}

boolean idle ( )
{
  return stateSending == false && stateReceiving == false && stateRunning == false;
}


void serialEvent( Serial myPort )
{
  states();
}


int task_time = 0;
boolean reached_end = false;
void states ( )
{
  if ( stateSending ) runStateSend();
  else if ( stateReceiving ) runStateReceive();
  else if ( stateRunning )
  {
    //delay(100);
    if (0 < port.available()) {
      String inBuffer = port.readStringUntil('\n');
      if ( inBuffer != null )
      {
        String[] command = inBuffer.split("[ \n]");

        switch ( command[0] )
        {
        case "Threshold":
          receiveThreshold(command);
          task_time = millis();
          break;
        case "Blob":
          receiveBlob(command);
          task_time = millis();
          break;
        case "Project":
          receiveProject(command);
          task_time = millis();
          break;

        case "Track":
          receiveTrack(command);
          break;

        case "Vote":
          receiveVote(command);
          break;

        default:
          print("UNEXPECTED: " + inBuffer);
        }
      }
    }
  }
}




void delayNano ( long time )
{
  long start = System.nanoTime();
  while ( System.nanoTime() - start < time );
}
