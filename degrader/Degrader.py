from PIL import Image
from random import random

# Path to image to be degraded
path = "../images/image.png"
# Path to newly degraded image
new_path = "../images/image-degraded.png"
# Percent random degraded pixels
p = 0.8

def main():
    image = Image.open(path)
    for x in range(image.size[0]):
        for y in range(image.size[1]):
            if random() <= p:
                image.putpixel((x, y), (0, 0, 0))
    image.show()
    image.save(new_path)

main()