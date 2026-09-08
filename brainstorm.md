## As the name suggests this document will be my brainstorming place

---

#### Devices and their functions:
- Raspberry pi - Main brain of the entire operation. This runs the interface and actually instructs other devices what to do in what situation.
- ESP32 - Mostly for wifi and bluetooth related things. The wireless things in the ESP32 are actually pretty great so therefore I would want to use this for wireless things.
- STM32 - Mostly used for wired protocols because it is fast and has no wireless capabilities. This would be to sniff certain protocols and see if it can detect different things.

---

#### What languages to use?
- C & C++ on the ESP-, STM32.
- I would like to begin in Rust and see how I find it and if it isn't fun switch to trusted old C++. On the Raspberry pi

---

#### What functions would be fun to have?
- [ ] Bluetooth scanner (shows all information possible)
- [ ] Wifi scanner (shows all information possible)
- [ ] Create a dummy bluetooth device
- [ ] Create a dummy wifi network