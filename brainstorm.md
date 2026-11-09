## As the name suggests this document will be my brainstorming place

---

#### Devices and their functions:
- Raspberry pi - Main brain of the entire operation. This runs the interface and actually instructs other devices what to do in what situation. (runs linux)
- ESP32 - Mostly for wifi and bluetooth related things. The wireless things in the ESP32 are actually pretty great so therefore I would want to use this for wireless things.
- STM32 - Mostly used for wired protocols because it is fast and has no wireless capabilities (the specific one I am using). This would be to sniff certain protocols and see if it can detect different things.

---

#### What languages to use?
- C & C++ on the ESP-, STM32.
- I would like to begin in Rust and see how I find it and if it isn't fun switch to trusted old C++. On the Raspberry pi
- If I could I would also like to use assembly somewhere but that is really not needed lmao

---

#### General idea
I would like to have this be a base but be very modulair. Therefore I would like to start writing a base that can check if there are any "apps" present somewhere and then you as a user can click them and open them. I would like for it to support both just elf files (so compiled c or whatever) & python scripts. And all the base functionality will also be modulair applications that are just shipped with my "default" config. And the raspberry pi which is the main brain of the operation runs linux headless and the entire interface will live in the terminal. 

This also means I want to write some library files for languages like c/c++, python & rust that define what pins are used for what etc.

---

#### What functions would be fun to have?
- [ ] Bluetooth scanner (shows all information possible)
- [ ] Wifi scanner (shows all information possible)
- [ ] Create a dummy bluetooth device
- [ ] Create a dummy wifi network
- [ ] Read uart from specific pins
- [ ] A calculator (maybe with graphing tools)
- [ ] 