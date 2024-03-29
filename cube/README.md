# CUBE (stm32h757)
Contained is code which can be used to run the algorithm on a microcontroller.  
This uses the CubeMX software to program the STM32 neuclio-H775ZI-Q board, however, for any board with enough memory, it will also run.  
The current setup will setup the neuclio for debug mode with the pc.  
Running the contained processing project will allow you to upload images to the board and run the algorithm.  

As CubeMX does not support Rust, a wrapper is made.  
Contained in this project is a C front end communicating to the Rust/C wrapper to communicate with the rust library back end.  
The C front end code is located in CM7/Core/Src/main.c.
The wrapper is located in lib/lib.
The backend is located in the other libraries.


### Processing
The computer app [Processing](https://processing.org/) reads images from the computer and can send them to the MCU to be processed.  
It is done graphically and times each section.  

### CubeMX
The computer app [CubeMX](https://www.st.com/content/st_com/en/stm32cubemx.html) is used to easily handle the clocks, memory and pinout of the microcontroller. When it does this, all the c code is generated.

### Front End (CM7/Core/Src/main.c)
The C front end was generated by CubeMX.  
The front end handles serial communication and pin assignments (for the leds).  
When a signal is sent from the computer, it identify the request and call on the Rust wrapper to handle it.  
When the Rust handler finishes, it will send a result back to the wrapper to send back to the computer.  



### Wrapper (lib/*)
The wrapper is the Rust part of the code, this contains the generated database and a set of functions to communicate between C and the libraries.



### Memory Fragments
This is kinda an important note...
The image uses 8 bit while the memory slots in ram are 32 bit (AND SOMETIMES 64 BITS???), yep thanks docs.  
I think you can treat it as 32bit and/or 64 bit...  
Anyway the largest memory fragment was AXISRAM which is 512KByte and a 608x808 image would be 490 kByte.  
While this is within bounds, I was having unexpected glitches and I was worried about memory overflow.  
So, I merged 4, 8bit pixels into a single 32bit slot.  
This isnt great for performance, however, it reduces the size to 123KByte.  
To change this, go to the AXIS_IMAGE_SIZE in main.c and threshold in lib.rs.


The database is generated by `star_tracker_database` and stored in `lib/database.rs`. 
I dont accutally know where this is stored as it doesnt seem like it can fit in the ram with the rest of the program...
Maybe SRAM 1 and 2 (128KB) were merged, their adresses are right next to each other.
None the less it does, lol, your problem now.







## How to Use
If you are trying to write your own code, write any IO stuff in the C Front end and write any Rust stuff to look at the library in the Wrapper code.

### Processing
To graphically see the power of the algorithm, run the processing application while leaving all code unchanged.  
In processing, there will be some code instructions on how to test it on other sample images, however, how it is setup, it will work.  
Run the app and press SEND and once it has loaded press RUN DECLINATION and read the terminal.


### Gen Database
If you changed the images to use a different camera you will need to generate the database,  
modify the `database.json` file to suit the requirements.  
Then run `sh generate_database.sh`.

### Compile
To compile the code run  
`sh compile.sh`.  
Then drag the output file from `output/cube_CM7.bin` into the usb which appears when the MCU is plugged in.  
This should upload the code.  