from random import choices
import numpy as np

CHOICES = 4
LETTER_TO_PICK = 'c'
NUM_SAMPLES = int(1e5)


letter_lookup = {'a':0, 'b':1, 'c':2, 'd':3, 'e':4, 'f':5, 'g':6}


key = np.random.randint(0, high=CHOICES, size=NUM_SAMPLES)

guess1 = np.full((NUM_SAMPLES), letter_lookup[LETTER_TO_PICK]) 
guess2 = np.random.randint(0, high=CHOICES, size=NUM_SAMPLES)

grade1 = key - guess1
grade2 = key - guess2
grade1 = np.count_nonzero(grade1==0)
grade2 = np.count_nonzero(grade2==0)

print(f"Guessing {LETTER_TO_PICK}      : {round(grade1/NUM_SAMPLES * 100, 4)}%")
print(f"Guessing random : {round(grade2/NUM_SAMPLES * 100, 4)}%")

if (grade1/NUM_SAMPLES) > (grade2/NUM_SAMPLES):
    print(f"Guessing c wins")
elif (grade1/NUM_SAMPLES) < (grade2/NUM_SAMPLES):
    print(f"Guessing randomly wins")
else :
    print("It was a tie")
