import os
from create_mcq import create_random_mcq
here_path = os.path.dirname(__file__)

from mcq_solver_rustlib import py_solve_mcq_front

mcq = create_random_mcq()

grades_path = here_path + r"/grades.txt"
sheets_path = [here_path + r"/sheets/" + f"{k}.txt" for k in range(1, 401)]
grades_separator = ","
number_of_questions = 120
starting_beta = 0.1
max_beta = 0.5
lambda_inv = 1.01
answer_tokens = ["a", "b", "c", "d"]

result = py_solve_mcq_front(
    grades_path,
    sheets_path,
    grades_separator,
    number_of_questions,
    starting_beta,
    max_beta,
    lambda_inv,
    answer_tokens
)

print("Original MCQ :")
print(mcq)

if type(result) == str:
    print("Solving failed")
else:
    print("\nThe solver gives :")
print(result)

print("\nAre they equal ?")
print(mcq == result)