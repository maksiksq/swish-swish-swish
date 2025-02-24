# Swish

This is the documentation for Swish, the AI gesture recognition lock/application.

## About the project

### About Swish 

Swish is the software for an AI gesture recognition lock. The idea is pretty simple, you open the app or have it installed on a device for use 24/7 (e.g. circuit with a camera), and using its camera and gesture recognition AI, alongside the front-end you enter a lock combination consisting of certain gestures defined as a password earlier. 

![screenshot](https://github.com/user-attachments/assets/b08f9825-f131-481b-88f9-434fe8a2e3bf)

For example: 👍 👎 ✌️ ✊ 👍 ✌️. If you enter the password, hurray 🎉, you got in\!

### Specifics & challenges

The biggest hurdle, of course, was implementing the gesture recognizer. It’s been quite a bit of work, particularly making it function with video. But in our current world, AI is getting adopted more and more, and developing such apps and interesting things becomes easier and easier, which opens doors for a lot of possibilities and innovation.

One thing I feared during the development is that the AI would end up not accurate enough for production levels, but the AI is actually incredibly accurate, on the hands we tested, it has a pretty much 100% success rate in recognition of what we would call a “commonsensibly recognizable form of a gesture”, although our testing dataset is small.

In addition to that, it doesn’t require a powerful device to run, as long as it has Webview and Node.js (it can be bundled with the app), it only consumes a tiny portion of RAM and CPU/GPU, which of these two is used for the AI is defined in a setting in code, GPU by default. It should be runnable and functional on weak devices, due to the amazing magic of Tauri’s Rust back-end.

## Project timeline

1) ### Conceptualization and components

This project came to me as a random idea while thinking of my course project for college. I could pick something simpler, but I wanted to do something interesting, so I did.

I quickly had an idea of what it was going to be visually, even though I didn’t know Vue at the time and only used Tauri once or twice, or much of other front-end technologies, so getting here would be difficult at the time. But as time went on, I learned and improved.

2) ### Attempts at making the AI

A bit later on I used another one of Google’s models, using TensorFlow and Python, to make the first edition of the gesture recognizer. It actually only properly worked on Linux and would only work for images, and pretty slowly at that. So I had to focus on the front-end among other non-project tasks.

3) ### Developing the front-end

It’s been a few months, and I’ve learnt some front-end technologies, including Vue and Tauri which are my favorite combo for making a desktop (or even mobile) app. So I got to developing the app, I made sure to include some little things like emoji, and essentially in its current state the app serves as a tutorial \+ an outlet for something to interact with it (e.g. a very real and very tangible lock)

4) ### Mediapipe and improving UX

When it came to actually making the AI gesture recognizer, I scrapped my past attempts with Tensorflow and instead found Google’s Mediapipe API, which conveniently had an example that did a task similar to mine. After a lot of hacking around, I had a ready gesture recognizer for images, and a bit later on, for live video from a webcam as well, made in JS and based on Google’s Codepen example. 

Then I had to make the UI display your current password, added a 3-second delay between inputs to make it more comfortable to use, added the view for the current lock combination and a hint. The next stage was just implementing the password logic, which wasn’t anything extraordinary, and I’ve done multiple times before. Also, it rick rolls you if you get the password wrong.

5) ### Polishing

After the main functionality was done, I finalized the UI, added a sidebar and the ability to reset the password, a favicon that looks like it’s a jumpy cube if you press on it in the taskbar on Windows. And that’s where we’re at, now is a good time for a speech about politics, about order, brotherhood, power. But speeches are for campaigning. Now is the time for action.

## Technologies and people involved

### Software:

- **Vue & Vite**, my beloved. Used them for the front-end, made writing it easy and fast.  
- **Tauri**. Used it for making a desktop app. You can technically use the app as a website as well, but for the target audience and possible applications, an app would be way more practical. Tauri uses Rust on the back-end to keep the app blazing fast and the bundle size tiny, which is super nice in comparison to its alternatives like Electron which is notably slow and is considered an annoying “plague” on app design.  
- **Google’s Mediapipe API**. Used for the AI, greatly simplifies the process, is incredibly useful. And their example [Codepen](https://codepen.io/mediapipe-preview/details/zYamdVd) as well.   
- **JetBrains WebStorm**, just an IDE but a good one at that.  
- **Figma**. Used it for laying out the design.

### Hardware:

- In the future, there are plans to slap it on an ESP32 and a stock electrical lock to make it an actual physical lock. That would also require a camera and a display. 

### People:

- Maksiks (Maksym Zadvorniy), architect.  
- Maksym Vitvitskiy, helped me come up with the idea and helped a little along the way with minor things, testing it and some more.  
- Zaliska Svitlana, teacher. 
