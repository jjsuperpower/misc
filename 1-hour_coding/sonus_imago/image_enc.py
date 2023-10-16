import cv2 as cv
from scipy.io import wavfile
import scipy.signal as signal
import numpy as np
from numba import njit


@njit   # make fast
def add_noise(img:np.ndarray, bits:int):
    '''Adds small amount of noise to image
    This function will add a small amount of noise to the least significant bits
    the goal is not to change the image visually but be able to hide info
    
    args:
        img: image to add noise to
        bits: number of bits to add noise to
        
    returns:
        img: image with noise added
    '''
    
    # loop through each pixel
    img = img.copy()
    shape = img.shape
    img = img.flatten()

    for i in range(len(img)):
        pixel = img[i]
        pixel = (pixel >> bits) << bits
        noise = np.random.randint(0, 2**bits + 1)
        pixel = pixel | noise
        img[i] = pixel

    return img.reshape(shape)


def show_img(title:str, img:np.ndarray, size=500):
    # scale window size
    scale = size / img.shape[0]
    cv.namedWindow(title, cv.WINDOW_NORMAL)
    cv.resizeWindow(title, int(img.shape[1] * scale), int(img.shape[0] * scale))
    cv.imshow(title, img)


def test_img_noise():
    # Read image
    img = cv.imread('test_img.png', cv.IMREAD_ANYDEPTH)

    # display orig image
    show_img('Original', img)

    # display noise
    show_img(f'Noise, LSBs: {1}', add_noise(img, 1))
    show_img(f'Noise, LSBs: {2}', add_noise(img, 2))
    show_img(f'Noise, LSBs: {3}', add_noise(img, 3))
    show_img(f'Noise, LSBs: {4}', add_noise(img, 4))

    cv.waitKey(0)
    
def test_sound_quant():
    
    FREQ = 16000
    QUANT_BITS = 6
    
    fs, sound = wavfile.read('test_sound.wav')
    print(f'Original - Duration: {len(sound) / fs :.1f}s, samples: {len(sound)}, bits/sample: {sound.dtype}')
    
    # resample to different frequency
    sound = signal.resample(sound, int(len(sound) * FREQ / fs))

    # convert to float 32 and scale
    orig_dtype = sound.dtype
    sound = sound.astype(np.float32)
    if orig_dtype == np.float32 or orig_dtype == np.float64:
        pass
    elif orig_dtype == np.uint8:
        sound = sound / 255
    elif orig_dtype == np.int16:
        sound = sound / (2**16) + 0.5
    elif orig_dtype == np.int32:
        sound = sound / (2**32) + 0.5
    elif sound.dtype == np.int64:
        sound = sound / (2**64) + 0.5
    else:
        raise Exception(f'Audio has unknown dtype: {orig_dtype}, supported dtypes: float32, uint8, int16, int32, int64')
    
    # normalize
    min_val = np.min(sound)
    max_val = np.max(sound)
    sound = (sound - min_val) / (max_val - min_val)
    
    # quantize to exact bits
    sound = (sound * 255).astype(np.uint8)
    sound = sound >> (8 - QUANT_BITS)
    
    # re-normalize
    min_val = np.min(sound)
    max_val = np.max(sound)
    sound = np.round((sound - min_val) * (255 / max_val), 0)
    sound = sound.astype(np.uint8)
    
    print(f'Resampled - Duration: {len(sound) / FREQ:.1f}s, samples: {len(sound)}, bits/sample: {sound.dtype}')
    
    # save sound to file
    wavfile.write('test_sound_quant.wav', FREQ, sound)
    
if __name__ == '__main__':
    test_img_noise()
    # test_sound_quant()