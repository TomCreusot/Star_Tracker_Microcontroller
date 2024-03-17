// serial_com handles the graphical elements and events.
// This handles the direct communication.
//
//

void receiveThreshold ( String [] values )
{
  println("\nDONE Threshing: " + (millis() - task_time) + "ms.");
  println();
  task_time = millis();
}


boolean foundBlob = false;
void receiveBlob ( String [] values )
{
  if ( !foundBlob ) println("\nDONE BLOBING: " + (millis() - task_time) + "ms.");
  foundBlob = true;
  float x = Float.parseFloat(values[1]);
  float y = Float.parseFloat(values[2]);

  img_real.set(int(x), int(y), color(255, 0, 255, 255));
  img_mcu.set(int(x+1), int(y+1), color(255, 0, 255, 255));
  img_real.set(int(x+1), int(y+1), color(255, 0, 255, 255));
  img_mcu.set(int(x+1), int(y-1), color(255, 0, 255, 255));
  img_real.set(int(x+1), int(y-1), color(255, 0, 255, 255));
  img_real.set(int(x), int(y), color(255, 0, 255, 255));
  img_mcu.set(int(x-1), int(y+1), color(255, 0, 255, 255));
  img_real.set(int(x-1), int(y+1), color(255, 0, 255, 255));
  img_mcu.set(int(x-1), int(y-1), color(255, 0, 255, 255));
  img_real.set(int(x-1), int(y-1), color(255, 0, 255, 255));

  img_mcu.set(int(x-1), int(y), color(255, 0, 255, 255));
  img_real.set(int(x-1), int(y), color(255, 0, 255, 255));
  img_mcu.set(int(x+1), int(y), color(255, 0, 255, 255));
  img_real.set(int(x+1), int(y), color(255, 0, 255, 255));
  img_mcu.set(int(x), int(y+1), color(255, 0, 255, 255));
  img_real.set(int(x), int(y+1), color(255, 0, 255, 255));
  img_mcu.set(int(x), int(y-1), color(255, 0, 255, 255));
  img_real.set(int(x), int(y-1), color(255, 0, 255, 255));

  img_real.updatePixels();
  img_mcu.updatePixels();

  task_time = millis();
  println("Blob       | " + x + " " + y);
}


boolean foundProject = false;
void receiveProject ( String [] values )
{
  if ( !foundProject ) println("\nDONE Projecting: " + (millis() - task_time) + "ms.");
  foundProject = true;
  float ra = Float.parseFloat(values[1]);
  float dec = Float.parseFloat(values[2]);

  task_time = millis();
  println("Projection | ra:" + ra + ", dec: " + dec);
  println();
}

boolean foundTrack = false;
void receiveTrack ( String [] values )
{
  if ( !foundTrack ) println("\nDONE Tracking: " + (millis() - task_time) + "ms.");
  foundTrack = true;

  float ra = 0;
  float dec = 0;
  if ( 2 < values.length )
  {
    ra = Float.parseFloat(values[1]);
    dec = Float.parseFloat(values[2]);
  }
  else
    println("FAILED to find constellation.");

  task_time = millis();
  println("Track      | ra:" + ra + ", dec: " + dec);
  println();
}


void receiveVote ( String [] values )
{
  println("\nDONE Vote: " + (millis() - task_time) + "ms.");

  float ra = Float.parseFloat(values[1]);
  float dec = Float.parseFloat(values[2]);

  task_time = millis();
  println("Vote       | ra:" + ra + ", dec: " + dec);
  println();
  stateRunning = false;
}






void runStateSend ( )
{
  if ( 0 < port.available() )
  {
    if (img_real.pixels.length <= pixelCur + 4)
    {
      stateSending = false;
      port.clear();
      println("SENT IMAGE");
      return;
    }
    byte [] statusBuffer = port.readBytes();
    if ( statusBuffer != null && 0 < statusBuffer.length )
    {
      if ( statusBuffer[0] == SEND_ACK )     pixelCur += 4;
      else if ( statusBuffer[0] == SEND_ERR) System.out.println("WAITING");
      else System.out.println("SEND FAILED: " + (char)statusBuffer[0]);
      sendAddress(pixelCur);
    }
  }
}


void runStateReceive ( )
{
  if ( 4 <= port.available() )
  {
    byte[] value = port.readBytes();
    if ( value.length != 0 )
    {
      img_mcu.pixels[pixelCur] = color(Byte.toUnsignedInt(value[0]), Byte.toUnsignedInt(value[0]), Byte.toUnsignedInt(value[0]), 255);
      pixelCur++;
      if ( pixelCur < img_mcu.pixels.length )
        img_mcu.pixels[pixelCur] = color(Byte.toUnsignedInt(value[1]), Byte.toUnsignedInt(value[1]), Byte.toUnsignedInt(value[1]), 255);
      pixelCur++;
      if ( pixelCur < img_mcu.pixels.length )
        img_mcu.pixels[pixelCur] = color(Byte.toUnsignedInt(value[2]), Byte.toUnsignedInt(value[2]), Byte.toUnsignedInt(value[2]), 255);
      pixelCur++;
      if ( pixelCur < img_mcu.pixels.length )
        img_mcu.pixels[pixelCur] = color(Byte.toUnsignedInt(value[3]), Byte.toUnsignedInt(value[3]), Byte.toUnsignedInt(value[3]), 255);
      pixelCur++;
      requestAddress(pixelCur);
      img_mcu.updatePixels();

      if ( img_mcu.pixels.length <= pixelCur )
      {
        stateReceiving = false;
        System.out.println("RECEIVED IMAGE");
      }
    }
  }
}



void sendAddress ( int i )
{
  byte [] sendBuffer   = new byte[4 + 4]; // Message, address, value 32_bit

  // command
  port.write(SEND_SET);

  // address
  int address = i / 4;
  sendBuffer[0] = (byte)((address >> (8*0)) & 255);
  sendBuffer[1] = (byte)((address >> (8*1)) & 255);
  sendBuffer[2] = (byte)((address >> (8*2)) & 255);
  sendBuffer[3] = (byte)((address >> (8*3)) & 255);

  // value
  color pixel   = ( img_real.pixels.length <= i ) ? 0 : img_real.pixels[i];
  sendBuffer[4] = (byte)(pixel & 0xFF);
  pixel         = ( img_real.pixels.length <= i ) ? 0 : img_real.pixels[i + 1];
  sendBuffer[5] = (byte)(pixel & 0xFF);
  pixel         = ( img_real.pixels.length <= i ) ? 0 : img_real.pixels[i + 2];
  sendBuffer[6] = (byte)(pixel & 0xFF);
  pixel         = ( img_real.pixels.length <= i ) ? 0 : img_real.pixels[i + 3];
  sendBuffer[7] = (byte)(pixel & 0xFF);

  img_real.pixels[i]   = color(Byte.toUnsignedInt(sendBuffer[4]), Byte.toUnsignedInt(sendBuffer[4]), Byte.toUnsignedInt(sendBuffer[4]), 255);
  img_real.pixels[i+1] = color(Byte.toUnsignedInt(sendBuffer[5]), Byte.toUnsignedInt(sendBuffer[5]), Byte.toUnsignedInt(sendBuffer[5]), 255);
  img_real.pixels[i+2] = color(Byte.toUnsignedInt(sendBuffer[6]), Byte.toUnsignedInt(sendBuffer[6]), Byte.toUnsignedInt(sendBuffer[6]), 255);
  img_real.pixels[i+3] = color(Byte.toUnsignedInt(sendBuffer[7]), Byte.toUnsignedInt(sendBuffer[7]), Byte.toUnsignedInt(sendBuffer[7]), 255);
  img_real.updatePixels();
  port.write(sendBuffer);
}


void requestAddress ( int i )
{
  byte [] sendBuffer   = new byte[4]; // Message, address, value 32_bit

  // command
  port.write(SEND_GET);

  // address
  int address = i / 4;
  sendBuffer[0] = (byte)((address >> (8*0)) & 255);
  sendBuffer[1] = (byte)((address >> (8*1)) & 255);
  sendBuffer[2] = (byte)((address >> (8*2)) & 255);
  sendBuffer[3] = (byte)((address >> (8*3)) & 255);

  port.write(sendBuffer);
}
