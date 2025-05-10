# Swish

This is the documentation for Swish, the AI gesture recognition lock/app.

## About the project

### About Swish

Swish is the software for an AI gesture recognition lock. The idea is pretty simple, you open the app or have it installed on a device for use 24/7 (e.g. circuit with a camera), and using its camera and gesture recognition AI, alongside the front-end you enter a lock combination consisting of certain gestures defined as a password earlier.

![screenshot](https://github.com/user-attachments/assets/b08f9825-f131-481b-88f9-434fe8a2e3bf)

For example: 👍 👎 ✌️ ✊ 👍 ✌️. If you enter the password, hurray 🎉, you got in\!

### Specifics & challenges

The biggest hurdle, of course, was implementing the gesture recognizer. It’s been quite a bit of work, particularly making it function with video. But in our current world, AI is getting adopted more and more, and developing such apps and interesting things becomes easier and easier, which opens doors for a lot of possibilities and innovation.

One thing I feared during the development is that the AI would end up not accurate enough for production levels, but the AI is actually incredibly accurate, on the hands we tested, it has a pretty much 100% success rate in recognition of what we would call a “commonsensibly recognizable form of a gesture”, although our testing dataset is small.

In addition to that, it doesn’t require a powerful device to run, as long as it has Webview and Node.js, it only consumes a tiny portion of RAM and CPU/GPU, which of these two is used for the AI is defined in a setting in code, GPU by default. It should be runnable and functional on weak devices, due to the amazing magic of Tauri’s Rust back-end.

## Technologies and people involved

### Software:

- **Vue & Vite**, my beloved. Used them for the front-end, made writing it easy and fast.
- **Tauri**. Used it for making a desktop app. You can technically use the app as a website as well, but for the target audience and possible applications, an app would be way more practical. Tauri uses Rust on the back-end to keep the app blazing fast and the bundle size tiny, which is super nice in comparison to its alternatives like Electron which is notably slow unlike Tauri, this app only uses ~5% CPU power and ~400 mb of RAM in active use on my machine, it should work on a potato as long as it has a gpu, and that doesn't even have to be mid-range either. Talking about Tauri a lot, but I'm just baffled by how good it is.
- **Google’s Mediapipe API**. Used for the AI, greatly simplifies the process, is incredibly useful. And their example [Codepen](https://codepen.io/mediapipe-preview/details/zYamdVd) as well. Originally used Tensorflow.
- **JetBrains WebStorm**, just an IDE but a good one at that.
- **Obsidian**, made some really nice flow charts because thinking is pain.
- **Figma**. Used it for laying out the design.

### Hardware:

- We're also slapping it on an ESP32 and an electromechanical lock to make it an actual physical lock. And then using that as our course project alongside the app and sending it to a competition. The app and the lock are connected via BLE (Bluetooth Low Energy).
  
![Swish collage](https://github.com/user-attachments/assets/c03206e7-1443-4913-8c04-048f52a36fa4)



### People:

- Maksiks (Maksym Zadvorniy), architect.
- Maksym (Vitvinio) Vitvitskiy, helped me come up with the idea and helped along the way with things, testing it and more.
- Zaliska Svitlana, teacher.


### Known issues:
- Lack of security, if someone actually wants to use this project in production, they'd have to implement Elliptic Curve Diffie Hellman encryption for optimal security and store the password in a secure store with something a la Stronghold. You don't want someone to break into your house. Yes, getting a pick and picking an average lock in 20 minutes would probably be easier but shhhh, we love our security.
