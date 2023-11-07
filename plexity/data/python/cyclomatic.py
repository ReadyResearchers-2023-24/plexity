x = "hello, " #1
y = " world"
z = x + y

if z == "hello, world": #2
    print(z)
elif z == "howdy": #3
    print(z)

for char in x: #4
    print(char)

a = 1
while a == 1: #5
    print(a)
    a += 1

try:
    print(y)
except: #6
    print("An exception occurred!")

with open("conditional-number.py", "r") as file: #7
    print(file)

b = 5
assert b == 5 #8

fruits = ["apple", "banana", "cherry", "kiwi", "mango"]
new_list = [x for x in fruits if "a" not in x] #9
print(new_list)

veggies = ["potato", "carrot", "broccoli", "celery", "tomato"]
new_set = {x for x in fruits if "a" not in x} #10
print(new_set)

time = ["morning", "noon", "evening"]
meal = ["breakfast", "lunch", "dinner"]
new_dict = {key:value for (key, value) in zip(time, meal)} #11
print(new_dict)

m = 0
n = 1

if m == 0 and n == 1: #12 & #13
    print(m)
elif m != 0 or n != 1: #14 & #15
    print(n)