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
newlist = [x for x in fruits if "a" not in x] #9
print(newlist)