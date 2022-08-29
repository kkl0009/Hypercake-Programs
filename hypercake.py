# Name: Kollin Labowski
# Course: CS 310
# Description: This program implements the HyperCake problem using the Rust programming language.
# Academic Integrity: This program complies with the academic integrity policies of WVU and CS 310

# hypercake function uses recursion and two embedded methods combinations and factorial
def hypercake(n, k):
    def combinations(n, r):
        def factorial(n):
            if n <= 1:
                return 1
            else:
                return n * factorial(n - 1)
                
        result = factorial(n) / (factorial(r) * factorial(n - r))
        return result
    
    if k <= 0:
        return combinations(n, k)
    return combinations(n, k) + hypercake(n, k - 1)
     
# Take in the input and print out the result           
n = int(input("Enter the amount of cuts (n): "))
k = int(input("Enter the amount of dimensions (k): "))
print("The answer is: ", hypercake(n, k))