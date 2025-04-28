const noble = require('@abandonware/noble');

const SERVICE_UUID = '4fafc2011fb5459e8fccc5c9c331914b';
const CHARACTERISTIC_UUID = 'beb5483e36e14688b7f5ea07361b26a8';
const DEVICE_NAME = "ESP32_LED_Control";

async function main() {
    noble.on('stateChange', async (state) => {
        if (state === 'poweredOn') {
            console.log('Scanning for devices...');
            await noble.startScanningAsync([], false);
        }
    });

    noble.on('discover', async (peripheral) => {
        console.log(`Discovered ${peripheral.advertisement.localName}`);

        if (peripheral.advertisement.localName === DEVICE_NAME) {
            await noble.stopScanningAsync();
            console.log('Connecting...');
            await peripheral.connectAsync();

            const { characteristics } = await peripheral.discoverSomeServicesAndCharacteristicsAsync(
                [SERVICE_UUID],
                [CHARACTERISTIC_UUID]
            );

            const ledCharacteristic = characteristics[0];

            // Sends the command
            const command = process.argv[2] === "off" ? "off" : "on";
            console.log(`Sending command: ${command}`);
            await ledCharacteristic.writeAsync(Buffer.from(command), false);

            console.log('Command sent, disconnecting...');
            await peripheral.disconnectAsync();
            process.exit(0);
        }
    });
}

main().catch(console.error);
