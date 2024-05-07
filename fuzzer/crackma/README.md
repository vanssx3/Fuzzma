# Crackma - Python Password Cracking Program!
(Don't ask what the name means...)

This program is capable of Brute Force attacks and Dictionary attacks, and can be used to crack plaintext passwords, 
SHA-256 passwords, MD5 passwords, and Bcrypt passwords!

# Dependencies
In order for Crackma to function, you must have Python3 installed and the bcrypt module!

# To install Python3:

For Windows Users, download it here: https://www.python.org/downloads/windows/

For MacOS Users, download it here: https://www.python.org/downloads/macos/

For Ubuntu/Debian based Linux Distros, run
```sh
sudo apt-get install python3
```

For Arch based Linux Distros, run
```sh
sudo pacman -S python3
```

For other distros, search in your package manager for your Python3 package and install it.

# To install Bcrypt:

Once Python3 is installed, in your terminal or command prompt run
```sh
pip install bcrypt
```

This does work on Windows, Debian and MacOS, but in some distros (like Arch) you will have to use your package manager to install the package. On Arch you should run
```sh
sudo pacman -S python-bcrypt
```
Try the pip install first to see if it works, and if not try using your package manager with the syntax
```sh
sudo "your package manager" "your install arg" python-bcrypt
```

# Installation
If git is already installed and configured on your system, cd to where you want the program installed and run:
```sh
git clone https://github.com/vanssx3/Crackma.git
```

If you do not have git installed already install it here with instructions for your OS: https://git-scm.com/downloads

If for some reason you are unable to install git on your computer, you can download crackma.py and passwords.txt and put them into the same folder to run them.

# Usage
This program runs in a terminal or command prompt. To run the program, make sure that you are in the same directory you installed the program in, then run 
```sh
python3 crackma.py
```
This will show you how use to the program, or you can read it here.

To get started, run the program with the first argument corresponding to the crack type:
* -b For a Brute Force Attack
* -d For a Dictionary Attack

The second argument should be the password you want cracked, hashed or plaintext. Any brute force password should be no more than 10 characters when fully decrypted!

The third argument should correspond to the hash algorithm used:
* -p For plaintext (No hash algorithm)
* -m For MD5
* -b for BCrypt (Hash must be enclosed in ' ' to function!)
* -s for SHA-256

At the end you can optionally add -v to list all guesses (takes SIGNIFICANTLY more time)

An example call if you wanted to brute force Hi! in plaintext while seeing all guesses:
```sh
 python3 crackma.py -b Hi! -p -v
```
You can run the program with --help to see this message again!

