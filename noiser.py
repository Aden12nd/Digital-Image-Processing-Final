from PIL import Image
import numpy as np
import argparse
import os


def add_gaussian_noise(image_array, mean=0, std=25):
    """
    Add Gaussian noise to an image array.

    Args:
        image_array (numpy.ndarray): Input image as NumPy array.
        mean (float): Mean of Gaussian noise.
        std (float): Standard deviation of Gaussian noise.

    Returns:
        numpy.ndarray: Noisy image array.
    """
    noise = np.random.normal(mean, std, image_array.shape)

    noisy_image = image_array.astype(np.float32) + noise

    # Clip values to valid image range
    noisy_image = np.clip(noisy_image, 0, 255)

    return noisy_image.astype(np.uint8)


def main():
    parser = argparse.ArgumentParser(
        description="Add Gaussian noise to an image."
    )

    parser.add_argument(
        "input_image",
        help="Path to the input image"
    )

    parser.add_argument(
        "output_image",
        help="Path to save the noisy image"
    )

    parser.add_argument(
        "--mean",
        type=float,
        default=0,
        help="Mean of Gaussian noise (default: 0)"
    )

    parser.add_argument(
        "--std",
        type=float,
        default=25,
        help="Standard deviation of Gaussian noise (default: 25)"
    )

    args = parser.parse_args()

    # Load image
    image = Image.open(args.input_image).convert("RGB")
    image_array = np.array(image)

    # Add noise
    noisy_image_array = add_gaussian_noise(
        image_array,
        mean=args.mean,
        std=args.std
    )

    # Save result
    noisy_image = Image.fromarray(noisy_image_array)

    os.makedirs(os.path.dirname(args.output_image), exist_ok=True) \
        if os.path.dirname(args.output_image) else None

    noisy_image.save(args.output_image, "PNG")

    print(f"Noisy image saved to: {args.output_image}")


if __name__ == "__main__":
    main()