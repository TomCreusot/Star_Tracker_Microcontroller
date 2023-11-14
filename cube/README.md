# CUBE (stm32h757)
This is a CubeMX C project using the neuclio-h755ZI-Q.  
It is wrapped with a static rust library.  

Using serial_com with processing, you can send and perform blob detection.  

## Gen Database
To generate the database,  
modify the `database.json` file to suit the requirements.  
Then run `sh generate_database.sh`.

## Compile
To compile the code run
`sh compile.sh`.