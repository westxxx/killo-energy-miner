float resistance = 110.0; // Ohms (from multimeter reading)

void setup() {
  Serial.begin(9600);
}

void loop() {
  int raw = analogRead(A0);
  float voltage = raw * (5.0 / 1023.0);
  float power = (voltage * voltage) / resistance;

  Serial.print("Voltage: ");
  Serial.print(voltage, 3);
  Serial.print(" V | Power: ");
  Serial.print(power, 6);
  Serial.println(" W");

  delay(250);
}
