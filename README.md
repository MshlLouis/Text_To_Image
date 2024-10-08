# A Rust program that converts a text file into an image and vice versa


## 1: Notes

As of the initial commit (08.09.2024), only a small collection of chars is supported, namely all chars with ASCII code 0-127 (aka signed byte).
Those include all standard letters + numbers + some special chars. To see all available chars see https://www.ascii-code.com.

I don't recommend converting a random image to text but only those images that have been created with this program. I might add support for more chars in the future.

## 2: How it works

Every pixel consists of 3 values which are RGB. Each value ranges from 0-255 (8 bits), giving a total of 16,777,216 different colors. Since the entire Latin alphabet including
uppercase letters and numbers are internally represented with values lower than 128, they can be directly translated into pixel values (exactly like a = 1, b = 2, c = 3 ... just that
the numbers are a little higher to start with). 

This way it's possible to store 3 chars in 1 pixel. This pixel is then set onto the first position (0,0). 

To make the logic behind everything easier the image is a square, meaning x and y are equal. The problem with this approach is that unless the total number of chars used in the text file is a
perfect square of (total number of chars / 3), there will be black pixels filling up the bottom of the image. A more efficient approach would be to find all divisors of
(total number of chars / 3) and then taking the two values closest to each other. If those values are too far apart (for example for prime numbers), the program increases the number
of total chars, creating black pixels in the process but overall less than the initial approach.

## 3: An example

The following is an example of the entire code converted into an image:


![img_out](https://github.com/user-attachments/assets/fb493cf7-b738-4be7-aedb-fac996a964e0)


The following is an example of the entire code of my TwitchBot main file converted into an image:


![img_out2](https://github.com/user-attachments/assets/0e714fb6-c2ce-448e-a400-f6e4e0fb08da)


Lastly the entire movie script for Shrek 1 converted into an image:


![shrek](https://github.com/user-attachments/assets/bab8b069-ccf7-43b0-ba4c-570ad4de54be)


(The first 2 images were scaled up to increase visibility. Changing them back to their original size WILL NOT work since important information is lost in the process of scaling. The image for Shrek 1 is unchanged and is convertable back to text)
