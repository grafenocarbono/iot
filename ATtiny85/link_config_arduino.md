## Link to configure Arduino:

 	[Configure Arduino IDE ATtiny85 ](https://projecthub.arduino.cc/alaspuresujay/use-an-attiny85-with-arduino-ide-d847c5)

  **Download Arduino IDE:**

  `wget https://downloads.arduino.cc/arduino-ide/arduino-ide_2.3.2_Linux_64bit.AppImage`

 **File>Preferences>Additional boards manager URLs:**

  [digistump boards](https://github.com/digistump/arduino-boards-index/blob/master/package_digistump_index.json)

**Tools>Board>Boards Manager. Drop down select Contributed and type Digistump AVR Boards then click the “Install” button.**
  

File>Quit
cd /home/<username>/.arduino15/

wget http://drazzy.com/package_drazzy.com_index.json --no-check-certificate

You will get the following file:
 package_drazzy.com_index.json

 You will see a file named package_drazzy.json in that folder. Rename it to package_drazzy.com_index.json.

Comprueba si se llama así package_drazzy.com_index.json, en caso contrario tendrás que renombrarlo



cd /home/grafeno/.arduino15/staging/packages
wget http://azduino.com/bin/micronucleus/micronucleus-cli-2.5-azd1-x86_64-linux-gnu.tar.bz2


Abre de nuevo arduino y busca en 
Tools>Board>Boards manager...> Type:Contributed> Select ATTinyCore by Spence Konde

DigiKeyboard.h No such file or directory

cd /tmp
git clone https://github.com/digistump/DigisparkArduinoIntegration
cd /tmp/DigisparkArduinoIntegration/libraries
zip -r digi_key.zip ./DigisparkKeyboard/

Sketch>Include Library>Add .zip library> Busca en el directorio donde se creó el .zip y aparecerá el mensaje:
Library installed

Continuará....

