(cd build; cmake ..)

mkdir -p  ~/pico 
(cd ~/pico/;  git clone -b master https://github.com/raspberrypi/pico-sdk.git)
(cd ~/pico/pico-sdk; git submodule update --init)

# Add SDK path to your environment
echo 'export PICO_SDK_PATH=$HOME/pico/pico-sdk' >> ~/.bashrc