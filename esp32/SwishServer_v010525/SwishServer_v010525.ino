#include <BLEDevice.h>
#include <BLEServer.h>
#include <BLEUtils.h>
#include <BLE2902.h>
#include <string>

// set to the buily-in LED as of currently, change this if you're using this code
#define LED_PIN 15

BLECharacteristic *pCharacteristic;
BLEServer *pServer;
BLEAdvertising *pAdvertising;
bool deviceConnected = false;

#define SERVICE_UUID        "4fafc201-1fb5-459e-8fcc-c5c9c331914b"
#define CHARACTERISTIC_UUID "beb5483e-36e1-4688-b7f5-ea07361b26a8"

void startAdvertising() {
  pAdvertising = pServer->getAdvertising();
  pAdvertising->start();
  Serial.println("Advertising started");
}

class MyCallbacks: public BLEServerCallbacks {
    void onConnect(BLEServer* pServer) {
      deviceConnected = true;
      Serial.println("Client connected");
    }

    void onDisconnect(BLEServer* pServer) {
      deviceConnected = false;
      Serial.println("Client disconnected, restarting advertising...");
      delay(500); // surely we won't be having issues that require real throttling, right? right??
      startAdvertising();
    }
};

class MyCharacteristicCallbacks : public BLECharacteristicCallbacks {
  void onWrite(BLECharacteristic *pCharacteristic) {
    String value = pCharacteristic->getValue();
    Serial.println("Received command: " + value);

    if (value == "on") {
      digitalWrite(LED_PIN, HIGH);
    } else if (value == "off") {
      digitalWrite(LED_PIN, LOW);
    }
  }
};

void setup() {
  Serial.begin(115200);
  pinMode(LED_PIN, OUTPUT);

  BLEDevice::init("ESP32_LED_Control");
  pServer = BLEDevice::createServer();
  pServer->setCallbacks(new MyCallbacks());

  BLEService *pService = pServer->createService(SERVICE_UUID);
  pCharacteristic = pService->createCharacteristic(
                      CHARACTERISTIC_UUID,
                      BLECharacteristic::PROPERTY_WRITE
                    );

  pCharacteristic->setCallbacks(new MyCharacteristicCallbacks());

  pService->start();
  startAdvertising();

  Serial.println("Waiting for a client to connect...");
}

void loop() {
  // nothing ere
}
